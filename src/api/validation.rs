//! Validation API Endpoints
//!
//! Provides REST API for profile validation operations.
//!
//! # Routes
//!
//! - `POST /api/projects/:projectId/profiles/:profileId/validate` - Full validation
//! - `POST /api/projects/:projectId/profiles/:profileId/validate/quick` - Quick structural validation
//! - `POST /api/projects/:projectId/profiles/:profileId/validate/element` - Validate specific element
//! - `GET /api/projects/:projectId/profiles/:profileId/validation` - Get cached validation results
//! - `POST /api/projects/:projectId/validate/batch` - Batch validate multiple profiles
//! - `POST /api/projects/:projectId/profiles/:profileId/apply-fix` - Apply a quick fix
//! - `GET /api/validation/config` - Get validation configuration
//! - `PUT /api/validation/config` - Update validation configuration

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::storage::ProfileStorage;
use crate::state::{AppState, ValidationConfig};
use crate::validation::{QuickFixKind, ValidationEngine, ValidationLevel, ValidationResult};

/// Validation request options.
#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    /// Validation level to perform.
    #[serde(default)]
    pub level: Option<String>,
    /// Include info-level diagnostics.
    #[serde(default = "default_true")]
    pub include_info: bool,
    /// Specific element paths to validate (empty = all).
    #[serde(default)]
    pub paths: Vec<String>,
}

fn default_true() -> bool {
    true
}

/// Validation response with diagnostics.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateResponse {
    /// Profile ID.
    pub profile_id: String,
    /// Whether the profile is valid.
    pub is_valid: bool,
    /// Validation level performed.
    pub level: String,
    /// Validation timestamp.
    pub timestamp: DateTime<Utc>,
    /// List of diagnostics.
    pub diagnostics: Vec<DiagnosticDto>,
    /// Validation statistics.
    pub stats: ValidationStats,
}

/// Validation statistics.
#[derive(Debug, Serialize)]
pub struct ValidationStats {
    /// Number of errors found.
    pub errors: usize,
    /// Number of warnings found.
    pub warnings: usize,
    /// Number of info diagnostics.
    pub info: usize,
}

/// Diagnostic DTO for API responses.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticDto {
    /// Severity level.
    pub severity: String,
    /// Error/warning code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Element path if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_path: Option<String>,
    /// Source layer of the diagnostic.
    pub source: String,
    /// Available quick fixes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quick_fixes: Vec<QuickFixDto>,
}

/// Quick fix DTO for API responses.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickFixDto {
    /// Display title.
    pub title: String,
    /// The kind of fix operation.
    pub kind: serde_json::Value,
    /// Whether this is the preferred fix.
    pub is_preferred: bool,
}

/// Convert ValidationResult to ValidateResponse.
fn to_response(result: ValidationResult, profile_id: &str, level: &str) -> ValidateResponse {
    let info_count = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == crate::validation::DiagnosticSeverity::Info)
        .count();

    ValidateResponse {
        profile_id: profile_id.to_string(),
        is_valid: result.is_valid,
        level: level.to_string(),
        timestamp: result.validated_at.unwrap_or_else(Utc::now),
        stats: ValidationStats {
            errors: result.error_count(),
            warnings: result.warning_count(),
            info: info_count,
        },
        diagnostics: result
            .diagnostics
            .into_iter()
            .map(|d| {
                let quick_fixes = d
                    .quick_fix
                    .map(|qf| {
                        vec![QuickFixDto {
                            title: qf.title.clone(),
                            kind: serde_json::to_value(&qf.kind).unwrap_or_default(),
                            is_preferred: qf.is_preferred,
                        }]
                    })
                    .unwrap_or_default();

                DiagnosticDto {
                    severity: format!("{:?}", d.severity).to_lowercase(),
                    code: d.code.clone(),
                    message: d.message.clone(),
                    element_path: d.element_path.clone(),
                    source: format!("{:?}", d.source).to_lowercase(),
                    quick_fixes,
                }
            })
            .collect(),
    }
}

/// Parse validation level from string.
fn parse_level(level: Option<&str>) -> ValidationLevel {
    match level {
        Some("structural") | Some("fast") => ValidationLevel::Structural,
        Some("references") => ValidationLevel::References,
        Some("terminology") => ValidationLevel::Terminology,
        Some("full") | Some("parity") => ValidationLevel::Full,
        _ => ValidationLevel::Structural,
    }
}

/// Path parameters for profile validation.
#[derive(Debug, Deserialize)]
struct ProfilePath {
    project_id: String,
    profile_id: String,
}

/// Path parameters for project-level operations.
#[derive(Debug, Deserialize)]
struct ProjectPath {
    project_id: String,
}

/// Validate a profile with full validation.
async fn validate_profile(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Json(request): Json<ValidateRequest>,
) -> impl IntoResponse {
    // Get project directory and create storage
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load the profile
    let document = match storage.load_profile(&params.profile_id).await {
        Ok(doc) => doc,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response();
        }
    };

    // Perform validation
    let level = parse_level(request.level.as_deref());
    let engine = ValidationEngine::new();
    let result = engine.validate(&document, level).await;

    // Cache the result
    state.cache_validation(
        &params.project_id,
        &params.profile_id,
        result.clone(),
        document.modified_at,
    );

    let level_str = level.as_str();
    let response = to_response(result, &params.profile_id, level_str);

    Json(response).into_response()
}

/// Quick structural validation (fast, synchronous).
async fn validate_quick(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
) -> impl IntoResponse {
    // Get project directory and create storage
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load the profile
    let document = match storage.load_profile(&params.profile_id).await {
        Ok(doc) => doc,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response();
        }
    };

    // Perform quick structural validation only
    let engine = ValidationEngine::new();
    let result = engine.validate(&document, ValidationLevel::Structural).await;

    // Cache the result
    state.cache_validation(
        &params.project_id,
        &params.profile_id,
        result.clone(),
        document.modified_at,
    );

    let response = to_response(result, &params.profile_id, "structural");

    Json(response).into_response()
}

/// Validate specific element request.
#[derive(Debug, Deserialize)]
pub struct ValidateElementRequest {
    /// Element path to validate.
    pub path: String,
}

/// Validate a specific element.
async fn validate_element(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Json(request): Json<ValidateElementRequest>,
) -> impl IntoResponse {
    // Get project directory and create storage
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load the profile
    let document = match storage.load_profile(&params.profile_id).await {
        Ok(doc) => doc,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response();
        }
    };

    // Perform incremental validation for specific path
    let engine = ValidationEngine::new();
    let result = engine
        .validate_incremental(&document, &[request.path])
        .await;

    let response = to_response(result, &params.profile_id, "element");

    Json(response).into_response()
}

/// Get cached validation results.
async fn get_validation(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
) -> impl IntoResponse {
    // Check cache
    match state.get_cached_validation(&params.project_id, &params.profile_id) {
        Some(cached) => {
            let level_str = cached.result.validation_level.as_str();
            let response = to_response(cached.result, &params.profile_id, level_str);
            Json(response).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "No validation results found",
                "message": "Run validation first using POST /validate"
            })),
        )
            .into_response(),
    }
}

/// Batch validation request.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchValidateRequest {
    /// Profile IDs to validate.
    pub profile_ids: Vec<String>,
    /// Validation level.
    #[serde(default)]
    pub level: Option<String>,
}

/// Batch validation response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchValidateResponse {
    /// Results for each profile.
    pub results: Vec<BatchValidationItem>,
    /// Total profiles validated.
    pub total: usize,
    /// Number of valid profiles.
    pub valid_count: usize,
    /// Number of invalid profiles.
    pub invalid_count: usize,
}

/// Single item in batch validation results.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchValidationItem {
    /// Profile ID.
    pub profile_id: String,
    /// Whether the profile is valid.
    pub is_valid: bool,
    /// Error count.
    pub error_count: usize,
    /// Warning count.
    pub warning_count: usize,
    /// Error message if profile couldn't be loaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Batch validate multiple profiles.
async fn validate_batch(
    State(state): State<AppState>,
    Path(params): Path<ProjectPath>,
    Json(request): Json<BatchValidateRequest>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);
    let level = parse_level(request.level.as_deref());
    let engine = ValidationEngine::new();

    let mut results = Vec::new();
    let mut valid_count = 0;
    let mut invalid_count = 0;

    // Validate each profile
    for profile_id in &request.profile_ids {
        match storage.load_profile(profile_id).await {
            Ok(document) => {
                let result = engine.validate(&document, level).await;

                // Cache result
                state.cache_validation(
                    &params.project_id,
                    profile_id,
                    result.clone(),
                    document.modified_at,
                );

                if result.is_valid {
                    valid_count += 1;
                } else {
                    invalid_count += 1;
                }

                results.push(BatchValidationItem {
                    profile_id: profile_id.clone(),
                    is_valid: result.is_valid,
                    error_count: result.error_count(),
                    warning_count: result.warning_count(),
                    error: None,
                });
            }
            Err(e) => {
                invalid_count += 1;
                results.push(BatchValidationItem {
                    profile_id: profile_id.clone(),
                    is_valid: false,
                    error_count: 1,
                    warning_count: 0,
                    error: Some(format!("{}", e)),
                });
            }
        }
    }

    Json(BatchValidateResponse {
        total: results.len(),
        valid_count,
        invalid_count,
        results,
    })
    .into_response()
}

/// Apply quick fix request.
#[derive(Debug, Deserialize)]
pub struct ApplyFixRequest {
    /// The quick fix operation to apply.
    pub fix: QuickFixKind,
}

/// Apply quick fix response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyFixResponse {
    /// Whether the fix was applied successfully.
    pub success: bool,
    /// Message describing the result.
    pub message: String,
    /// Updated element path (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_path: Option<String>,
    /// New validation result for the affected element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidateResponse>,
}

/// Apply a quick fix to a profile.
async fn apply_fix(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Json(request): Json<ApplyFixRequest>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load the profile
    let mut document = match storage.load_profile(&params.profile_id).await {
        Ok(doc) => doc,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response();
        }
    };

    // Apply the fix based on kind
    let (success, message, path) = apply_quick_fix_to_document(&mut document, &request.fix);

    if success {
        // Save the updated document
        if let Err(e) = storage.save_profile(&document).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Failed to save: {}", e) })),
            )
                .into_response();
        }

        // Invalidate cache and re-validate
        state.invalidate_validation(&params.project_id, &params.profile_id);

        // Re-validate if we have a path
        let validation = if let Some(ref p) = path {
            let engine = ValidationEngine::new();
            let result = engine.validate_incremental(&document, &[p.clone()]).await;
            Some(to_response(result, &params.profile_id, "element"))
        } else {
            None
        };

        Json(ApplyFixResponse {
            success: true,
            message,
            updated_path: path,
            validation,
        })
        .into_response()
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(ApplyFixResponse {
                success: false,
                message,
                updated_path: None,
                validation: None,
            }),
        )
            .into_response()
    }
}

/// Apply a quick fix to a profile document.
fn apply_quick_fix_to_document(
    document: &mut crate::ir::ProfileDocument,
    fix: &QuickFixKind,
) -> (bool, String, Option<String>) {
    match fix {
        QuickFixKind::SetCardinality { path, min, max } => {
            if let Some(element) = document.resource.find_element_mut(path) {
                // Create or update cardinality
                let cardinality = crate::ir::Cardinality { min: *min, max: *max };
                element.constraints.cardinality = Some(cardinality);
                (
                    true,
                    format!("Set cardinality to {}..{}", min, max.map(|m| m.to_string()).unwrap_or_else(|| "*".to_string())),
                    Some(path.clone()),
                )
            } else {
                (false, format!("Element not found: {}", path), None)
            }
        }

        QuickFixKind::SetBindingStrength { path, strength } => {
            if let Some(element) = document.resource.find_element_mut(path) {
                if let Some(ref mut binding) = element.constraints.binding {
                    binding.strength = crate::ir::BindingStrength::from_str(strength);
                    (
                        true,
                        format!("Set binding strength to '{}'", strength),
                        Some(path.clone()),
                    )
                } else {
                    (false, "Element has no binding".to_string(), None)
                }
            } else {
                (false, format!("Element not found: {}", path), None)
            }
        }

        QuickFixKind::RemoveBinding { path } => {
            if let Some(element) = document.resource.find_element_mut(path) {
                element.constraints.binding = None;
                (true, "Removed binding".to_string(), Some(path.clone()))
            } else {
                (false, format!("Element not found: {}", path), None)
            }
        }

        QuickFixKind::SetFlag { path, flag, value } => {
            if let Some(element) = document.resource.find_element_mut(path) {
                match flag.as_str() {
                    "must_support" => element.constraints.flags.must_support = *value,
                    "is_modifier" => element.constraints.flags.is_modifier = *value,
                    "is_summary" => element.constraints.flags.is_summary = *value,
                    _ => return (false, format!("Unknown flag: {}", flag), None),
                }
                (
                    true,
                    format!("Set {} to {}", flag, value),
                    Some(path.clone()),
                )
            } else {
                (false, format!("Element not found: {}", path), None)
            }
        }

        QuickFixKind::AddMetadata { field, suggested_value } => {
            match field.as_str() {
                "id" => document.metadata.id = suggested_value.clone(),
                "url" => document.metadata.url = suggested_value.clone(),
                "name" => document.metadata.name = suggested_value.clone(),
                "title" => document.metadata.title = Some(suggested_value.clone()),
                "description" => document.metadata.description = Some(suggested_value.clone()),
                _ => return (false, format!("Unknown metadata field: {}", field), None),
            }
            (
                true,
                format!("Set {} to '{}'", field, suggested_value),
                None,
            )
        }

        // For other fix types, we'll need more complex handling
        _ => (
            false,
            "This fix type is not yet implemented".to_string(),
            None,
        ),
    }
}

/// Get validation configuration.
async fn get_validation_config(State(state): State<AppState>) -> impl IntoResponse {
    let config = state.validation_config().await;
    Json(config).into_response()
}

/// Update validation configuration.
async fn update_validation_config(
    State(state): State<AppState>,
    Json(config): Json<ValidationConfig>,
) -> impl IntoResponse {
    state.set_validation_config(config.clone()).await;
    Json(serde_json::json!({
        "success": true,
        "config": config
    }))
    .into_response()
}

/// Create validation routes.
pub fn validation_routes() -> Router<AppState> {
    Router::new()
        // Profile-level validation
        .route(
            "/api/projects/{project_id}/profiles/{profile_id}/validate",
            post(validate_profile),
        )
        .route(
            "/api/projects/{project_id}/profiles/{profile_id}/validate/quick",
            post(validate_quick),
        )
        .route(
            "/api/projects/{project_id}/profiles/{profile_id}/validate/element",
            post(validate_element),
        )
        .route(
            "/api/projects/{project_id}/profiles/{profile_id}/validation",
            get(get_validation),
        )
        .route(
            "/api/projects/{project_id}/profiles/{profile_id}/apply-fix",
            post(apply_fix),
        )
        // Project-level validation
        .route(
            "/api/projects/{project_id}/validate/batch",
            post(validate_batch),
        )
        // Global validation config
        .route("/api/validation/config", get(get_validation_config))
        .route("/api/validation/config", put(update_validation_config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_level() {
        assert!(matches!(parse_level(None), ValidationLevel::Structural));
        assert!(matches!(
            parse_level(Some("structural")),
            ValidationLevel::Structural
        ));
        assert!(matches!(parse_level(Some("fast")), ValidationLevel::Structural));
        assert!(matches!(
            parse_level(Some("references")),
            ValidationLevel::References
        ));
        assert!(matches!(
            parse_level(Some("terminology")),
            ValidationLevel::Terminology
        ));
        assert!(matches!(parse_level(Some("full")), ValidationLevel::Full));
        assert!(matches!(parse_level(Some("parity")), ValidationLevel::Full));
        assert!(matches!(
            parse_level(Some("unknown")),
            ValidationLevel::Structural
        ));
    }
}
