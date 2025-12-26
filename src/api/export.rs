//! Export API route handlers.
//!
//! Implements REST endpoints for exporting profiles to StructureDefinition JSON
//! and FSH formats with deterministic output, caching, and validation.

use axum::{
    Json, Router,
    body::Body,
    extract::{Path, Query, State},
    http::{
        StatusCode,
        header::{self, HeaderMap, HeaderValue},
    },
    response::{IntoResponse, Response},
    routing::get,
};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path as FsPath;
use zip::write::SimpleFileOptions;

use crate::decompiler::{DecompilerError, decompile_sd_value_to_fsh};
use crate::export::{ExportConfig, StructureDefinitionExporter, merge_original_sd_fields};
use crate::ir::ProfileDocument;
use crate::state::AppState;

use super::dto::{ApiResponse, Diagnostic, DiagnosticSeverity};
use super::export_dto::*;
use super::profile_merge::hydrate_profile_document;
use super::profiles::{ErrorResponse, ProfilePath, ProjectPath};
use super::storage::{ProfileStorage, StorageError};

/// Create export routes.
pub fn export_routes() -> Router<AppState> {
    Router::new()
        // Single resource exports
        .route(
            "/{profileId}/export/sd",
            get(export_sd).head(export_sd_headers),
        )
        .route("/{profileId}/export/sd/base", get(export_base_sd))
        .route(
            "/{profileId}/export/fsh",
            get(export_fsh).head(export_fsh_headers),
        )
        .route("/{profileId}/export/schema", get(export_schema))
        .route("/{profileId}/preview", get(preview))
}

/// Create project-level export routes.
pub fn project_export_routes() -> Router<AppState> {
    Router::new().route("/export", get(bulk_export))
}

// === Path Parameters ===

/// Path parameters for export routes (reuses ProfilePath from profiles module).
pub use super::profiles::ProfilePath as ExportPath;

// === SD Export (R1) ===

/// GET /api/projects/:projectId/profiles/:profileId/export/sd
///
/// Export a profile as StructureDefinition JSON.
async fn export_sd(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Query(query): Query<SdExportQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(StorageError::NotFound(_)) => {
            return ErrorResponse::not_found("Profile", &params.profile_id).into_response();
        }
        Err(e) => return ErrorResponse::internal_error(e.to_string()).into_response(),
    };
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return e.into_response(),
    };
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return e.into_response(),
    };
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return e.into_response(),
    };

    // Validate before export
    let validation = validate_for_export(&doc);
    if !validation.can_export(query.force) {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({
                "error": "Validation failed",
                "code": "VALIDATION_ERROR",
                "validation": validation
            })),
        )
            .into_response();
    }

    // Build export config
    let config = match query.format {
        SdExportFormat::Differential => ExportConfig::differential_only(),
        SdExportFormat::Snapshot => ExportConfig::snapshot_only(),
        SdExportFormat::Both => ExportConfig::default(),
    };
    let config = if query.pretty {
        config.pretty()
    } else {
        config
    };

    // Export to JSON
    let mut exporter = StructureDefinitionExporter::with_config(config);
    let mut json_value = match exporter.export_value(&doc).await {
        Ok(v) => v,
        Err(e) => {
            return ErrorResponse::internal_error(format!("Export failed: {}", e)).into_response();
        }
    };
    merge_original_sd_for_export(&project_dir, &doc, &mut json_value).await;

    // Serialize for content and ETag
    let json_value = crate::export::recursively_sort_value(&json_value);
    let json_string = if query.pretty {
        serde_json::to_string_pretty(&json_value).unwrap_or_default()
    } else {
        serde_json::to_string(&json_value).unwrap_or_default()
    };

    // Calculate ETag
    let etag = calculate_etag(&json_string);

    // Check If-None-Match for caching
    if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH) {
        if let Ok(value) = if_none_match.to_str() {
            if value == etag || value == format!("\"{}\"", etag) {
                return StatusCode::NOT_MODIFIED.into_response();
            }
        }
    }

    // Persist if requested
    let persisted_path = if query.persist {
        match storage.save_sd_json(&doc.metadata.name, &json_string).await {
            Ok(path) => Some(path.display().to_string()),
            Err(e) => {
                tracing::warn!("Failed to persist SD export: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Build metadata
    let metadata = ExportMetadata {
        resource_id: doc.metadata.id.clone(),
        name: doc.metadata.name.clone(),
        url: doc.metadata.url.clone(),
        fhir_version: doc.resource.fhir_version.as_str().to_string(),
        filename: format!("{}.json", doc.metadata.name),
        content_type: "application/fhir+json".to_string(),
        etag: etag.clone(),
        persisted_path,
    };

    // Build response with proper headers
    let response = SdExportResponse {
        data: json_value,
        metadata,
        diagnostics: validation.diagnostics,
    };

    let mut resp = Json(ApiResponse::ok(response)).into_response();
    let headers = resp.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/fhir+json"),
    );
    headers.insert(
        header::ETAG,
        HeaderValue::from_str(&format!("\"{}\"", etag)).unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename=\"{}.json\"",
            doc.metadata.name
        ))
        .unwrap(),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, must-revalidate"),
    );

    resp
}

/// GET /api/projects/:projectId/profiles/:profileId/export/sd/base
///
/// Export the base StructureDefinition JSON.
async fn export_base_sd(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(StorageError::NotFound(_)) => {
            return ErrorResponse::not_found("Profile", &params.profile_id).into_response();
        }
        Err(e) => return ErrorResponse::internal_error(e.to_string()).into_response(),
    };

    let base_url = doc.resource.base.url.clone();
    let base_name = doc
        .resource
        .base
        .name
        .clone()
        .or_else(|| base_url.rsplit('/').next().map(String::from))
        .unwrap_or_else(|| "Base".to_string());

    let canonical_manager = match state.canonical_manager().await {
        Ok(mgr) => mgr.clone(),
        Err(e) => {
            return ErrorResponse::internal_error(format!("Canonical manager error: {}", e))
                .into_response();
        }
    };
    let resolver = crate::base::BaseResolver::new(canonical_manager);

    let json_value = match resolver.load_base_sd_json(&base_url).await {
        Ok(v) => crate::export::recursively_sort_value(&v),
        Err(e) => {
            return ErrorResponse::internal_error(format!("Failed to load base SD: {}", e))
                .into_response();
        }
    };

    let json_string = serde_json::to_string(&json_value).unwrap_or_default();
    let etag = calculate_etag(&json_string);

    let metadata = ExportMetadata {
        resource_id: base_name.clone(),
        name: base_name.clone(),
        url: base_url,
        fhir_version: doc.resource.fhir_version.as_str().to_string(),
        filename: format!("{}.json", base_name),
        content_type: "application/fhir+json".to_string(),
        etag: etag.clone(),
        persisted_path: None,
    };

    let response = SdExportResponse {
        data: json_value,
        metadata,
        diagnostics: Vec::new(),
    };

    let mut resp = Json(ApiResponse::ok(response)).into_response();
    let headers = resp.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/fhir+json"),
    );
    headers.insert(
        header::ETAG,
        HeaderValue::from_str(&format!("\"{}\"", etag)).unwrap(),
    );

    resp
}

/// HEAD /api/projects/:projectId/profiles/:profileId/export/sd
///
/// Get headers for SD export (for caching checks).
async fn export_sd_headers(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Query(query): Query<SdExportQuery>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // Build minimal export to calculate ETag
    let config = match query.format {
        SdExportFormat::Differential => ExportConfig::differential_only(),
        SdExportFormat::Snapshot => ExportConfig::snapshot_only(),
        SdExportFormat::Both => ExportConfig::default(),
    };
    let config = if query.pretty {
        config.pretty()
    } else {
        config
    };

    let mut exporter = StructureDefinitionExporter::with_config(config);
    let mut json_value = match exporter.export_value(&doc).await {
        Ok(v) => v,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    merge_original_sd_for_export(&project_dir, &doc, &mut json_value).await;

    let json_string = if query.pretty {
        serde_json::to_string_pretty(&json_value).unwrap_or_default()
    } else {
        serde_json::to_string(&json_value).unwrap_or_default()
    };

    let etag = calculate_etag(&json_string);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/fhir+json")
        .header(header::ETAG, format!("\"{}\"", etag))
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.json\"", doc.metadata.name),
        )
        .header(header::CACHE_CONTROL, "private, must-revalidate")
        .body(Body::empty())
        .unwrap()
        .into_response()
}

// === FSH Export (R2) ===

/// GET /api/projects/:projectId/profiles/:profileId/export/fsh
///
/// Export a profile as FHIR Shorthand.
#[axum::debug_handler]
async fn export_fsh(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Query(query): Query<FshExportQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(StorageError::NotFound(_)) => {
            return ErrorResponse::not_found("Profile", &params.profile_id).into_response();
        }
        Err(e) => return ErrorResponse::internal_error(e.to_string()).into_response(),
    };

    // Hydrate the profile (merge base + differential into root tree)
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return e.into_response(),
    };

    // Validate before export
    let validation = validate_for_export(&doc);
    if !validation.can_export(query.force) {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({
                "error": "Validation failed",
                "code": "VALIDATION_ERROR",
                "validation": validation
            })),
        )
            .into_response();
    }

    // Export to SD JSON, then decompile to FSH using maki-decompiler
    let fsh_content = match generate_fsh_via_decompiler(&project_dir, &doc).await {
        Ok(fsh) => fsh,
        Err(e) => {
            return ErrorResponse::internal_error(format!("FSH decompilation failed: {}", e))
                .into_response();
        }
    };

    // Calculate ETag
    let etag = calculate_etag(&fsh_content);

    // Check If-None-Match for caching
    if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH) {
        if let Ok(value) = if_none_match.to_str() {
            if value == etag || value == format!("\"{}\"", etag) {
                return StatusCode::NOT_MODIFIED.into_response();
            }
        }
    }

    // Persist if requested
    let persisted_path = if query.persist {
        match storage.save_fsh(&doc.metadata.name, &fsh_content).await {
            Ok(path) => Some(path.display().to_string()),
            Err(e) => {
                tracing::warn!("Failed to persist FSH export: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Build metadata
    let metadata = ExportMetadata {
        resource_id: doc.metadata.id.clone(),
        name: doc.metadata.name.clone(),
        url: doc.metadata.url.clone(),
        fhir_version: doc.resource.fhir_version.as_str().to_string(),
        filename: format!("{}.fsh", doc.metadata.name),
        content_type: "text/plain; charset=utf-8".to_string(),
        etag: etag.clone(),
        persisted_path,
    };

    let response = FshExportResponse {
        data: fsh_content,
        metadata,
        diagnostics: validation.diagnostics,
    };

    let mut resp = Json(ApiResponse::ok(response)).into_response();
    let headers = resp.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    headers.insert(
        header::ETAG,
        HeaderValue::from_str(&format!("\"{}\"", etag)).unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename=\"{}.fsh\"",
            doc.metadata.name
        ))
        .unwrap(),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, must-revalidate"),
    );

    resp
}

/// HEAD /api/projects/:projectId/profiles/:profileId/export/fsh
///
/// Get headers for FSH export (for caching checks).
#[axum::debug_handler]
async fn export_fsh_headers(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    // Hydrate the profile
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let fsh_content = match generate_fsh_via_decompiler(&project_dir, &doc).await {
        Ok(fsh) => fsh,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let etag = calculate_etag(&fsh_content);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .header(header::ETAG, format!("\"{}\"", etag))
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.fsh\"", doc.metadata.name),
        )
        .header(header::CACHE_CONTROL, "private, must-revalidate")
        .body(Body::empty())
        .unwrap()
        .into_response()
}

// === FHIR Schema Export (R4) ===

/// GET /api/projects/:projectId/profiles/:profileId/export/schema
///
/// Export a profile as FHIR Schema JSON.
async fn export_schema(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Query(query): Query<FshExportQuery>, // Reuse FshExportQuery for persist/force
    headers: HeaderMap,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(StorageError::NotFound(_)) => {
            return ErrorResponse::not_found("Profile", &params.profile_id).into_response();
        }
        Err(e) => return ErrorResponse::internal_error(e.to_string()).into_response(),
    };

    // Hydrate the profile
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return e.into_response(),
    };

    // Validate before export
    let validation = validate_for_export(&doc);
    if !validation.can_export(query.force) {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({
                "error": "Validation failed",
                "code": "VALIDATION_ERROR",
                "validation": validation
            })),
        )
            .into_response();
    }

    // Export to SD JSON, then convert to FHIR Schema
    let schema_content = match generate_fhirschema(&project_dir, &doc).await {
        Ok(s) => s,
        Err(e) => {
            return ErrorResponse::internal_error(format!("FHIR Schema conversion failed: {}", e))
                .into_response();
        }
    };

    // Calculate ETag
    let etag = calculate_etag(&schema_content);

    // Check If-None-Match for caching
    if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH) {
        if let Ok(value) = if_none_match.to_str() {
            if value == etag || value == format!("\"{}\"", etag) {
                return StatusCode::NOT_MODIFIED.into_response();
            }
        }
    }

    // Persist if requested
    let persisted_path = if query.persist {
        match storage
            .save_sd_json(&format!("{}.schema", doc.metadata.name), &schema_content)
            .await
        {
            Ok(path) => Some(path.display().to_string()),
            Err(e) => {
                tracing::warn!("Failed to persist FHIR Schema export: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Build metadata
    let metadata = ExportMetadata {
        resource_id: doc.metadata.id.clone(),
        name: doc.metadata.name.clone(),
        url: doc.metadata.url.clone(),
        fhir_version: doc.resource.fhir_version.as_str().to_string(),
        filename: format!("{}.schema.json", doc.metadata.name),
        content_type: "application/schema+json".to_string(),
        etag: etag.clone(),
        persisted_path,
    };

    let response = SdExportResponse {
        data: serde_json::from_str(&schema_content).unwrap_or(serde_json::Value::Null),
        metadata,
        diagnostics: validation.diagnostics,
    };

    let mut resp = Json(ApiResponse::ok(response)).into_response();
    let headers = resp.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/schema+json"),
    );
    headers.insert(
        header::ETAG,
        HeaderValue::from_str(&format!("\"{}\"", etag)).unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename=\"{}.schema.json\"",
            doc.metadata.name
        ))
        .unwrap(),
    );

    resp
}

// === Bulk Export (R3) ===

/// GET /api/projects/:projectId/export
///
/// Export all project resources.
#[axum::debug_handler]
async fn bulk_export(
    State(state): State<AppState>,
    Path(params): Path<ProjectPath>,
    Query(query): Query<BulkExportQuery>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load all profiles
    let profiles = match storage.list_profiles().await {
        Ok(p) => p,
        Err(e) => return ErrorResponse::internal_error(e.to_string()).into_response(),
    };

    if profiles.is_empty() {
        return (
            StatusCode::OK,
            Json(ApiResponse::ok(BulkExportResponse {
                project_id: params.project_id.clone(),
                files: Vec::new(),
                summary: ExportSummary {
                    total_resources: 0,
                    success_count: 0,
                    failed_count: 0,
                    skipped_count: 0,
                    formats: Vec::new(),
                },
                diagnostics: Vec::new(),
            })),
        )
            .into_response();
    }

    match query.structure {
        ExportStructure::Flat => {
            bulk_export_flat(&state, &params.project_id, profiles, &query).await
        }
        ExportStructure::Packaged => {
            bulk_export_packaged(&state, &params.project_id, profiles, &query).await
        }
    }
}

/// Export profiles as flat structure (JSON response).
async fn bulk_export_flat(
    state: &AppState,
    project_id: &str,
    profiles: Vec<ProfileDocument>,
    query: &BulkExportQuery,
) -> Response<Body> {
    let project_dir = state.project_path(project_id);
    let mut files = Vec::new();
    let mut diagnostics = Vec::new();
    let mut success_count = 0u32;
    let mut failed_count = 0u32;
    let total = profiles.len() as u32;

    for doc in profiles {
        let resource_id = doc.metadata.id.clone();
        let resource_name = doc.metadata.name.clone();
        let doc = match hydrate_profile_document(state, doc).await {
            Ok(d) => d,
            Err(_e) => {
                failed_count += 1;
                diagnostics.push(ResourceDiagnostic {
                    resource_id,
                    name: resource_name,
                    diagnostics: vec![Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        code: "HYDRATION_FAILED".to_string(),
                        message: "Failed to hydrate profile for export".to_string(),
                        path: None,
                    }],
                });
                continue;
            }
        };
        let validation = validate_for_export(&doc);
        let resource_diag = ResourceDiagnostic {
            resource_id: doc.metadata.id.clone(),
            name: doc.metadata.name.clone(),
            diagnostics: validation.diagnostics.clone(),
        };

        if !validation.can_export(true) {
            failed_count += 1;
            diagnostics.push(resource_diag);
            continue;
        }

        // Export SD if requested
        if matches!(query.format, BulkExportFormat::Sd | BulkExportFormat::Both) {
            let config = if query.pretty {
                ExportConfig::default().pretty()
            } else {
                ExportConfig::default()
            };
            let mut exporter = StructureDefinitionExporter::with_config(config);

            match exporter.export_value(&doc).await {
                Ok(mut value) => {
                    merge_original_sd_for_export(&project_dir, &doc, &mut value).await;
                    let content = if query.pretty {
                        serde_json::to_string_pretty(&value).unwrap_or_default()
                    } else {
                        serde_json::to_string(&value).unwrap_or_default()
                    };
                    files.push(ExportedFile {
                        resource_id: doc.metadata.id.clone(),
                        name: doc.metadata.name.clone(),
                        path: format!("SD/StructureDefinition/{}.json", doc.metadata.name),
                        format: "sd".to_string(),
                        content,
                        is_base64: false,
                    });
                }
                Err(e) => {
                    diagnostics.push(ResourceDiagnostic {
                        resource_id: doc.metadata.id.clone(),
                        name: doc.metadata.name.clone(),
                        diagnostics: vec![Diagnostic {
                            severity: DiagnosticSeverity::Error,
                            code: "EXPORT_FAILED".to_string(),
                            message: format!("SD export failed: {}", e),
                            path: None,
                        }],
                    });
                }
            }
        }

        // Export FSH if requested
        if matches!(query.format, BulkExportFormat::Fsh | BulkExportFormat::Both) {
            match generate_fsh_via_decompiler(&project_dir, &doc).await {
                Ok(content) => {
                    files.push(ExportedFile {
                        resource_id: doc.metadata.id.clone(),
                        name: doc.metadata.name.clone(),
                        path: format!("FSH/profiles/{}.fsh", doc.metadata.name),
                        format: "fsh".to_string(),
                        content,
                        is_base64: false,
                    });
                }
                Err(e) => {
                    diagnostics.push(ResourceDiagnostic {
                        resource_id: doc.metadata.id.clone(),
                        name: doc.metadata.name.clone(),
                        diagnostics: vec![Diagnostic {
                            severity: DiagnosticSeverity::Error,
                            code: "FSH_DECOMPILE_FAILED".to_string(),
                            message: format!("FSH decompilation failed: {}", e),
                            path: None,
                        }],
                    });
                }
            }
        }

        // Export FHIR Schema if requested
        if matches!(
            query.format,
            BulkExportFormat::FhirSchema | BulkExportFormat::Both
        ) {
            match generate_fhirschema(&project_dir, &doc).await {
                Ok(content) => {
                    files.push(ExportedFile {
                        resource_id: doc.metadata.id.clone(),
                        name: doc.metadata.name.clone(),
                        path: format!("Schema/{}.schema.json", doc.metadata.name),
                        format: "fhirschema".to_string(),
                        content,
                        is_base64: false,
                    });
                }
                Err(e) => {
                    diagnostics.push(ResourceDiagnostic {
                        resource_id: doc.metadata.id.clone(),
                        name: doc.metadata.name.clone(),
                        diagnostics: vec![Diagnostic {
                            severity: DiagnosticSeverity::Error,
                            code: "SCHEMA_GEN_FAILED".to_string(),
                            message: format!("FHIR Schema generation failed: {}", e),
                            path: None,
                        }],
                    });
                }
            }
        }

        success_count += 1;
        if !validation.diagnostics.is_empty() {
            diagnostics.push(resource_diag);
        }
    }

    let formats = match query.format {
        BulkExportFormat::Sd => vec!["sd".to_string()],
        BulkExportFormat::Fsh => vec!["fsh".to_string()],
        BulkExportFormat::FhirSchema => vec!["fhirschema".to_string()],
        BulkExportFormat::Both => vec![
            "sd".to_string(),
            "fsh".to_string(),
            "fhirschema".to_string(),
        ],
    };

    let response = BulkExportResponse {
        project_id: project_id.to_string(),
        files,
        summary: ExportSummary {
            total_resources: total,
            success_count,
            failed_count,
            skipped_count: 0,
            formats,
        },
        diagnostics,
    };

    Json(ApiResponse::ok(response)).into_response()
}

/// Export profiles as packaged tarball with IG scaffold.
async fn bulk_export_packaged(
    state: &AppState,
    project_id: &str,
    profiles: Vec<ProfileDocument>,
    query: &BulkExportQuery,
) -> Response<Body> {
    let project_dir = state.project_path(project_id);
    // Create in-memory ZIP file
    let mut zip_buffer = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o644);

        let mut hydrated_profiles = Vec::new();

        for doc in profiles {
            let doc = match hydrate_profile_document(state, doc).await {
                Ok(d) => d,
                Err(_) => {
                    tracing::warn!("Failed to hydrate profile during packaged export");
                    continue;
                }
            };
            hydrated_profiles.push(doc.clone());
            let validation = validate_for_export(&doc);
            if !validation.can_export(true) {
                continue;
            }

            // Export SD if requested
            if matches!(query.format, BulkExportFormat::Sd | BulkExportFormat::Both) {
                let config = if query.pretty {
                    ExportConfig::default().pretty()
                } else {
                    ExportConfig::default()
                };
                let mut exporter = StructureDefinitionExporter::with_config(config);

                if let Ok(mut value) = exporter.export_value(&doc).await {
                    merge_original_sd_for_export(&project_dir, &doc, &mut value).await;
                    let content = if query.pretty {
                        serde_json::to_string_pretty(&value).unwrap_or_default()
                    } else {
                        serde_json::to_string(&value).unwrap_or_default()
                    };
                    let path = format!(
                        "input/resources/StructureDefinition-{}.json",
                        doc.metadata.name
                    );
                    if let Err(e) = zip.start_file(&path, options) {
                        tracing::warn!("Failed to add SD to ZIP: {}", e);
                        continue;
                    }
                    if let Err(e) = zip.write_all(content.as_bytes()) {
                        tracing::warn!("Failed to write SD content: {}", e);
                        continue;
                    }
                }
            }

            // Export FSH if requested
            if matches!(query.format, BulkExportFormat::Fsh | BulkExportFormat::Both) {
                match generate_fsh_via_decompiler(&project_dir, &doc).await {
                    Ok(fsh_content) => {
                        let path = format!("input/fsh/profiles/{}.fsh", doc.metadata.name);
                        let _ = zip.start_file(&path, options);
                        let _ = zip.write_all(fsh_content.as_bytes());
                    }
                    Err(e) => {
                        tracing::warn!("FSH decompilation failed for {}: {}", doc.metadata.name, e);
                    }
                }
            }

            // Export FHIR Schema if requested
            if matches!(
                query.format,
                BulkExportFormat::FhirSchema | BulkExportFormat::Both
            ) {
                match generate_fhirschema(&project_dir, &doc).await {
                    Ok(content) => {
                        let path = format!("input/schema/{}.schema.json", doc.metadata.name);
                        let _ = zip.start_file(&path, options);
                        let _ = zip.write_all(content.as_bytes());
                    }
                    Err(e) => {
                        tracing::warn!(
                            "FHIR Schema generation failed for {}: {}",
                            doc.metadata.name,
                            e
                        );
                    }
                }
            }
        }

        // Add IG scaffold files
        let ig_json = generate_ig_scaffold(project_id, &hydrated_profiles);
        if zip.start_file("ig.ini", options).is_ok() {
            let ig_ini = format!(
                "[IG]\nig = input/ImplementationGuide-{}.json\ntemplate = fhir.base.template",
                project_id
            );
            let _ = zip.write_all(ig_ini.as_bytes());
        }

        if zip
            .start_file(
                &format!("input/ImplementationGuide-{}.json", project_id),
                options,
            )
            .is_ok()
        {
            let _ = zip.write_all(ig_json.as_bytes());
        }

        // Add sushi-config.yaml for FSH builds
        if matches!(query.format, BulkExportFormat::Fsh | BulkExportFormat::Both) {
            if zip.start_file("sushi-config.yaml", options).is_ok() {
                let sushi_config = generate_sushi_config(project_id, &hydrated_profiles);
                let _ = zip.write_all(sushi_config.as_bytes());
            }
        }

        let _ = zip.finish();
    }

    // Return ZIP as response
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}-export.zip\"", project_id),
        )
        .body(Body::from(zip_buffer))
        .unwrap()
}

// === Preview (R5) ===

/// GET /api/projects/:projectId/profiles/:profileId/preview
///
/// Get formatted preview without downloading.
#[axum::debug_handler]
async fn preview(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Query(query): Query<PreviewQuery>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(StorageError::NotFound(_)) => {
            return ErrorResponse::not_found("Profile", &params.profile_id).into_response();
        }
        Err(e) => return ErrorResponse::internal_error(e.to_string()).into_response(),
    };

    // Validate
    let validation = validate_for_export(&doc);

    let (content, language) = match query.format {
        PreviewFormat::Sd => {
            let config = ExportConfig::default().pretty();
            let mut exporter = StructureDefinitionExporter::with_config(config);

            match exporter.export_value(&doc).await {
                Ok(mut value) => {
                    merge_original_sd_for_export(&project_dir, &doc, &mut value).await;
                    let json = serde_json::to_string_pretty(&value).unwrap_or_default();
                    (json, "json")
                }
                Err(e) => {
                    return ErrorResponse::internal_error(format!("Export failed: {}", e))
                        .into_response();
                }
            }
        }
        PreviewFormat::Fsh => match generate_fsh_via_decompiler(&project_dir, &doc).await {
            Ok(content) => (content, "fsh"),
            Err(e) => {
                return ErrorResponse::internal_error(format!("Export failed: {}", e))
                    .into_response();
            }
        },
        PreviewFormat::FhirSchema => match generate_fhirschema(&project_dir, &doc).await {
            Ok(content) => (content, "json"),
            Err(e) => {
                return ErrorResponse::internal_error(format!("Export failed: {}", e))
                    .into_response();
            }
        },
    };

    // Generate syntax highlighting if requested
    let highlighting = if query.highlight {
        Some(generate_highlighting(&content, language))
    } else {
        None
    };

    let response = PreviewResponse {
        content,
        format: query.format,
        highlighting,
        diagnostics: validation.diagnostics,
    };

    Json(ApiResponse::ok(response)).into_response()
}

// === Helper Functions ===

async fn merge_original_sd_for_export(
    project_dir: &FsPath,
    doc: &ProfileDocument,
    sd_value: &mut serde_json::Value,
) {
    let sd_dir = project_dir.join("SD").join("StructureDefinition");
    let mut paths = Vec::new();
    paths.push(sd_dir.join(format!("{}.json", doc.metadata.name)));
    if doc.metadata.id != doc.metadata.name {
        paths.push(sd_dir.join(format!("{}.json", doc.metadata.id)));
    }

    for path in paths {
        let Ok(content) = tokio::fs::read_to_string(&path).await else {
            continue;
        };
        let Ok(original) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };
        merge_original_sd_fields(sd_value, &original);
        break;
    }
}

/// Validate a profile document before export.
fn validate_for_export(doc: &ProfileDocument) -> ValidationResult {
    let mut diagnostics = Vec::new();

    // Check required metadata
    if doc.metadata.url.is_empty() {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            code: "MISSING_URL".to_string(),
            message: "Profile URL is required for export".to_string(),
            path: None,
        });
    }

    if doc.metadata.name.is_empty() {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            code: "MISSING_NAME".to_string(),
            message: "Profile name is required for export".to_string(),
            path: None,
        });
    }

    // Check name format
    if !doc.metadata.name.is_empty() {
        let first_char = doc.metadata.name.chars().next().unwrap();
        if !first_char.is_ascii_uppercase() {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "INVALID_NAME_FORMAT".to_string(),
                message: "Profile name should start with an uppercase letter".to_string(),
                path: None,
            });
        }
        if doc.metadata.name.contains(' ') {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                code: "INVALID_NAME_SPACES".to_string(),
                message: "Profile name cannot contain spaces".to_string(),
                path: None,
            });
        }
    }

    // Check base definition
    if doc.resource.base.url.is_empty() {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            code: "MISSING_BASE".to_string(),
            message: "Base definition URL is required".to_string(),
            path: None,
        });
    }

    // Validate element tree
    validate_element_tree(&doc.resource.root, &mut diagnostics);

    ValidationResult::from_diagnostics(diagnostics)
}

/// Validate element tree recursively.
fn validate_element_tree(element: &crate::ir::ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    // Check cardinality consistency
    if let Some(card) = &element.constraints.cardinality {
        if let Some(max) = card.max {
            if max < card.min {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    code: "INVALID_CARDINALITY".to_string(),
                    message: format!(
                        "Element '{}': max ({}) cannot be less than min ({})",
                        element.path, max, card.min
                    ),
                    path: Some(element.path.clone()),
                });
            }
        }
    }

    // Validate slicing
    if let Some(slicing) = &element.slicing {
        if slicing.discriminator.is_empty() {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "SLICING_NO_DISCRIMINATOR".to_string(),
                message: format!(
                    "Element '{}' has slicing defined but no discriminator",
                    element.path
                ),
                path: Some(element.path.clone()),
            });
        }
    }

    // Validate children
    for child in &element.children {
        validate_element_tree(child, diagnostics);
    }

    // Validate slices
    for slice in element.slices.values() {
        validate_element_tree(&slice.element, diagnostics);
    }
}

/// Calculate ETag (SHA-256 hash) for content.
fn calculate_etag(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..16].to_string()
}

/// Generate FSH content using maki-decompiler.
///
/// This is the preferred method as it produces high-quality FSH output
/// by first exporting to SD JSON, then decompiling using maki-decompiler.
///
/// We use `differential_only()` config because maki-decompiler reads from
/// the differential section to determine which FSH rules to generate.
/// This ensures FSH output contains only modified elements, not the full
/// snapshot. Slicing is handled by maki-decompiler's ContainsExtractor
/// which generates proper `contains` syntax.
async fn generate_fsh_via_decompiler(
    project_dir: &FsPath,
    doc: &ProfileDocument,
) -> Result<String, DecompilerError> {
    // Export with differential-only to generate minimal FSH
    // maki-decompiler reads from sd.differential to extract rules
    let config = ExportConfig::differential_only();
    let mut exporter = StructureDefinitionExporter::with_config(config);

    let mut sd_value = exporter
        .export_value(doc)
        .await
        .map_err(|e| DecompilerError::ProcessFailed(format!("SD export failed: {}", e)))?;
    merge_original_sd_for_export(project_dir, doc, &mut sd_value).await;

    // Use maki-decompiler to convert SD to FSH
    decompile_sd_value_to_fsh(&sd_value, doc.resource.fhir_version).await
}

/// Generate FHIR Schema content using octofhir-fhirschema.
async fn generate_fhirschema(
    project_dir: &FsPath,
    doc: &ProfileDocument,
) -> Result<String, String> {
    // Export with full snapshot as FHIR Schema converter usually needs it
    let config = ExportConfig::default();
    let mut exporter = StructureDefinitionExporter::with_config(config);

    let mut sd_value = exporter
        .export_value(doc)
        .await
        .map_err(|e| format!("SD export failed: {}", e))?;
    merge_original_sd_for_export(project_dir, doc, &mut sd_value).await;

    // Convert SD to FHIR Schema using translate function
    // We need to deserialize into StructureDefinition struct first
    let sd: octofhir_fhirschema::StructureDefinition = serde_json::from_value(sd_value)
        .map_err(|e| format!("Deserialization failed for Schema conversion: {}", e))?;

    let schema = octofhir_fhirschema::translate(sd, None)
        .map_err(|e| format!("FHIR Schema conversion failed: {}", e))?;

    // Return as pretty JSON
    serde_json::to_string_pretty(&schema).map_err(|e| format!("Serialization failed: {}", e))
}

/// Generate basic syntax highlighting tokens.
fn generate_highlighting(content: &str, language: &str) -> SyntaxHighlighting {
    let mut tokens = Vec::new();

    match language {
        "json" => {
            // Simple JSON tokenization
            for (line_num, line) in content.lines().enumerate() {
                let line_num = line_num as u32;

                // Find string keys
                let chars: Vec<char> = line.chars().collect();
                let mut i = 0;
                while i < chars.len() {
                    if chars[i] == '"' {
                        let start = i;
                        i += 1;
                        while i < chars.len() && chars[i] != '"' {
                            if chars[i] == '\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                        if i < chars.len() {
                            i += 1;
                        }
                        // Check if followed by colon (key) or not (value)
                        let mut j = i;
                        while j < chars.len() && chars[j].is_whitespace() {
                            j += 1;
                        }
                        let token_type = if j < chars.len() && chars[j] == ':' {
                            "key"
                        } else {
                            "string"
                        };
                        tokens.push(HighlightToken {
                            line: line_num,
                            start_column: start as u32,
                            end_column: i as u32,
                            token_type: token_type.to_string(),
                        });
                    } else if chars[i].is_ascii_digit()
                        || (chars[i] == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
                    {
                        let start = i;
                        while i < chars.len()
                            && (chars[i].is_ascii_digit()
                                || chars[i] == '.'
                                || chars[i] == '-'
                                || chars[i] == 'e'
                                || chars[i] == 'E')
                        {
                            i += 1;
                        }
                        tokens.push(HighlightToken {
                            line: line_num,
                            start_column: start as u32,
                            end_column: i as u32,
                            token_type: "number".to_string(),
                        });
                    } else if content[i..].starts_with("true")
                        || content[i..].starts_with("false")
                        || content[i..].starts_with("null")
                    {
                        let word_len = if chars[i..].starts_with(&['t', 'r', 'u', 'e']) {
                            4
                        } else if chars[i..].starts_with(&['f', 'a', 'l', 's', 'e']) {
                            5
                        } else {
                            4
                        };
                        tokens.push(HighlightToken {
                            line: line_num,
                            start_column: i as u32,
                            end_column: (i + word_len) as u32,
                            token_type: "keyword".to_string(),
                        });
                        i += word_len;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        "fsh" => {
            // Simple FSH tokenization
            let keywords = [
                "Profile",
                "Parent",
                "Id",
                "Title",
                "Description",
                "Extension",
                "ValueSet",
                "CodeSystem",
                "Instance",
                "InstanceOf",
                "Usage",
                "Alias",
                "Invariant",
                "Logical",
                "Resource",
                "RuleSet",
                "Mapping",
                "Source",
                "Target",
                "from",
                "contains",
                "and",
                "or",
                "only",
                "MS",
            ];

            for (line_num, line) in content.lines().enumerate() {
                let line_num = line_num as u32;

                // Check for keywords at start of line
                for keyword in &keywords {
                    if line.starts_with(keyword) {
                        tokens.push(HighlightToken {
                            line: line_num,
                            start_column: 0,
                            end_column: keyword.len() as u32,
                            token_type: "keyword".to_string(),
                        });
                        break;
                    }
                }

                // Find strings
                let mut i = 0;
                let chars: Vec<char> = line.chars().collect();
                while i < chars.len() {
                    if chars[i] == '"' {
                        let start = i;
                        i += 1;
                        while i < chars.len() && chars[i] != '"' {
                            if chars[i] == '\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                        if i < chars.len() {
                            i += 1;
                        }
                        tokens.push(HighlightToken {
                            line: line_num,
                            start_column: start as u32,
                            end_column: i as u32,
                            token_type: "string".to_string(),
                        });
                    } else if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                        // Comment
                        tokens.push(HighlightToken {
                            line: line_num,
                            start_column: i as u32,
                            end_column: chars.len() as u32,
                            token_type: "comment".to_string(),
                        });
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        _ => {}
    }

    SyntaxHighlighting {
        language: language.to_string(),
        tokens,
    }
}

/// Generate ImplementationGuide scaffold JSON.
fn generate_ig_scaffold(project_id: &str, profiles: &[ProfileDocument]) -> String {
    let fhir_version = profiles
        .first()
        .map(|p| p.resource.fhir_version.as_str())
        .unwrap_or("4.0.1");

    let resources: Vec<serde_json::Value> = profiles
        .iter()
        .map(|p| {
            serde_json::json!({
                "reference": {
                    "reference": format!("StructureDefinition/{}", p.metadata.name)
                },
                "name": p.metadata.title.as_deref().unwrap_or(&p.metadata.name),
                "description": p.metadata.description.as_deref().unwrap_or(""),
                "exampleBoolean": false
            })
        })
        .collect();

    serde_json::to_string_pretty(&serde_json::json!({
        "resourceType": "ImplementationGuide",
        "id": project_id,
        "url": format!("http://example.org/fhir/ImplementationGuide/{}", project_id),
        "version": "0.1.0",
        "name": project_id,
        "title": project_id,
        "status": "draft",
        "packageId": format!("org.example.{}", project_id.to_lowercase().replace('-', "")),
        "fhirVersion": [fhir_version],
        "definition": {
            "resource": resources
        }
    }))
    .unwrap_or_default()
}

/// Generate SUSHI configuration file.
fn generate_sushi_config(project_id: &str, profiles: &[ProfileDocument]) -> String {
    let fhir_version = profiles
        .first()
        .map(|p| p.resource.fhir_version.as_str())
        .unwrap_or("4.0.1");

    format!(
        r#"id: org.example.{}
canonical: http://example.org/fhir
name: {}
title: {}
status: draft
version: 0.1.0
fhirVersion: {}
copyrightYear: 2024+
releaseLabel: ci-build

FSHOnly: false
"#,
        project_id.to_lowercase().replace('-', ""),
        project_id,
        project_id,
        fhir_version
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_etag() {
        let content = r#"{"resourceType": "StructureDefinition"}"#;
        let etag1 = calculate_etag(content);
        let etag2 = calculate_etag(content);

        assert_eq!(etag1, etag2);
        assert_eq!(etag1.len(), 16);
    }

    #[test]
    fn test_validation_result() {
        let result = ValidationResult::valid();
        assert!(result.can_export(false));
        assert!(result.can_export(true));
    }
}
