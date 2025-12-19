//! Profile API route handlers.
//!
//! Implements REST endpoints for profile CRUD operations.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::ir::{
    BaseDefinition, Binding, BindingStrength, Cardinality, DocumentMetadata, ElementNode,
    FhirVersion, ProfileDocument, ProfileStatus, ProfiledResource, TypeConstraint,
};
use crate::state::AppState;

use super::dto::*;
use super::storage::{ProfileStorage, StorageError};

/// Create profile routes.
pub fn profile_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_profiles).post(create_profile))
        .route("/{profileId}", get(get_profile).delete(delete_profile))
        .route("/{profileId}/metadata", patch(update_metadata))
        .route("/{profileId}/elements/{*path}", patch(update_element))
        .route("/{profileId}/import", post(import_profile))
        .route("/{profileId}/input-it", get(get_input_it))
}

/// Path parameters for project-scoped routes.
#[derive(Debug, serde::Deserialize)]
pub struct ProjectPath {
    #[serde(rename = "projectId")]
    pub project_id: String,
}

/// Path parameters for profile-scoped routes.
#[derive(Debug, serde::Deserialize)]
pub struct ProfilePath {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "profileId")]
    pub profile_id: String,
}

/// Path parameters for element-scoped routes.
#[derive(Debug, serde::Deserialize)]
pub struct ElementPath {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    pub path: String,
}

// === Error Handling ===

/// API error response.
#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    error: String,
    code: String,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: code.into(),
            status: status.as_u16(),
            details: None,
        }
    }

    pub fn not_found(resource: &str, id: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::NOT_FOUND,
            Json(Self::new(
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                format!("{} '{}' not found", resource, id),
            )),
        )
    }

    pub fn bad_request(message: impl Into<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(Self::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", message)),
        )
    }

    pub fn internal_error(message: impl Into<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                message,
            )),
        )
    }

    pub fn validation_error(message: impl Into<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(Self::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "VALIDATION_ERROR",
                message,
            )),
        )
    }
}

impl From<StorageError> for (StatusCode, Json<ErrorResponse>) {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::NotFound(id) => ErrorResponse::not_found("Profile", &id),
            StorageError::ConcurrentModification(id) => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::new(
                    StatusCode::CONFLICT,
                    "CONCURRENT_MODIFICATION",
                    format!("Profile '{}' was modified concurrently", id),
                )),
            ),
            _ => ErrorResponse::internal_error(err.to_string()),
        }
    }
}

// === Route Handlers ===

/// GET /api/projects/:projectId/profiles
/// List all profiles in a project.
async fn list_profiles(
    State(state): State<AppState>,
    Path(params): Path<ProjectPath>,
    Query(query): Query<ListProfilesQuery>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Initialize storage if needed
    if let Err(e) = storage.init().await {
        return ErrorResponse::internal_error(e.to_string()).into_response();
    }

    // Load profiles
    let profiles = match storage.list_profiles().await {
        Ok(p) => p,
        Err(e) => return Into::<(StatusCode, Json<ErrorResponse>)>::into(e).into_response(),
    };

    // Apply FHIR version filter
    let filtered: Vec<_> = profiles
        .iter()
        .filter(|p| {
            query.fhir_version.as_ref().is_none_or(|v| {
                FhirVersion::from_str(v).is_some_and(|fv| fv == p.resource.fhir_version)
            })
        })
        .collect();

    // Pagination
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(50).min(100).max(1);
    let total_items = filtered.len() as u32;
    let total_pages = (total_items + page_size - 1) / page_size;

    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(filtered.len());

    let profile_items: Vec<ProfileListItem> = filtered[start..end]
        .iter()
        .map(|p| ProfileListItem::from(*p))
        .collect();

    let response = ProfileListResponse {
        profiles: profile_items,
        pagination: PaginationInfo {
            page,
            page_size,
            total_items,
            total_pages,
        },
    };

    Json(ApiResponse::ok(response)).into_response()
}

/// POST /api/projects/:projectId/profiles
/// Create a new profile.
async fn create_profile(
    State(state): State<AppState>,
    Path(params): Path<ProjectPath>,
    Json(req): Json<CreateProfileRequest>,
) -> impl IntoResponse {
    // Validate FHIR version
    let fhir_version = match FhirVersion::from_str(&req.fhir_version) {
        Some(v) => v,
        None => {
            return ErrorResponse::bad_request(format!(
                "Invalid FHIR version: {}. Valid values: R4, R4B, R5, R6",
                req.fhir_version
            ))
            .into_response()
        }
    };

    // Validate resource type (basic check)
    if req.resource_type.is_empty() {
        return ErrorResponse::bad_request("Resource type is required").into_response();
    }

    // Validate name
    if req.name.is_empty() {
        return ErrorResponse::bad_request("Profile name is required").into_response();
    }

    // Generate ID and URL
    let profile_id = format!("{}-{}", req.name.to_lowercase(), Uuid::new_v4().to_string()[..8].to_string());
    let url = req.url.unwrap_or_else(|| {
        format!(
            "http://example.org/fhir/StructureDefinition/{}",
            req.name
        )
    });

    // Create metadata
    let mut metadata = DocumentMetadata::new(&profile_id, &url, &req.name);
    if let Some(title) = req.title {
        metadata.title = Some(title);
    }
    if let Some(description) = req.description {
        metadata.description = Some(description);
    }
    if let Some(publisher) = req.publisher {
        metadata.publisher = Some(publisher);
    }

    // Create resource
    let resource = ProfiledResource::new(
        &url,
        fhir_version,
        BaseDefinition::resource(&req.resource_type),
    );

    // Create document
    let doc = ProfileDocument::new(metadata, resource);

    // Save to storage
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    if let Err(e) = storage.init().await {
        return ErrorResponse::internal_error(e.to_string()).into_response();
    }

    if let Err(e) = storage.save_profile(&doc).await {
        return ErrorResponse::internal_error(e.to_string()).into_response();
    }

    let response = ProfileDetailsResponse::from(&doc);
    (StatusCode::CREATED, Json(ApiResponse::ok(response))).into_response()
}

/// GET /api/projects/:projectId/profiles/:profileId
/// Get profile details.
async fn get_profile(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    match storage.load_profile(&params.profile_id).await {
        Ok(doc) => {
            let response = ProfileDetailsResponse::from(&doc);
            Json(ApiResponse::ok(response)).into_response()
        }
        Err(e) => Into::<(StatusCode, Json<ErrorResponse>)>::into(e).into_response(),
    }
}

/// GET /api/projects/:projectId/profiles/:profileId/input-it
/// Get the original input StructureDefinition resource.
async fn get_input_it(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
) -> impl IntoResponse {
    use tokio::fs;

    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // First load the profile to get its name
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(e) => return Into::<(StatusCode, Json<ErrorResponse>)>::into(e).into_response(),
    };

    // Try to find the original SD JSON file
    let sd_dir = project_dir.join("IR").join("resources");
    let sd_path = sd_dir.join(format!("{}.json", doc.metadata.name));

    match fs::read_to_string(&sd_path).await {
        Ok(content) => {
            // Parse and return as JSON
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json) => Json(json).into_response(),
                Err(_) => {
                    // Return as raw string if not valid JSON
                    content.into_response()
                }
            }
        }
        Err(_) => {
            // Try alternative path with profile ID
            let alt_path = sd_dir.join(format!("{}.json", params.profile_id));
            match fs::read_to_string(&alt_path).await {
                Ok(content) => {
                    match serde_json::from_str::<serde_json::Value>(&content) {
                        Ok(json) => Json(json).into_response(),
                        Err(_) => content.into_response(),
                    }
                }
                Err(_) => {
                    ErrorResponse::not_found("Input IT", &params.profile_id).into_response()
                }
            }
        }
    }
}

/// DELETE /api/projects/:projectId/profiles/:profileId
/// Delete a profile.
async fn delete_profile(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Check if profile exists
    let doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(StorageError::NotFound(_)) => {
            return ErrorResponse::not_found("Profile", &params.profile_id).into_response()
        }
        Err(e) => return ErrorResponse::internal_error(e.to_string()).into_response(),
    };

    // Delete all associated files
    if let Err(e) = storage
        .delete_profile_files(&params.profile_id, &doc.metadata.name)
        .await
    {
        return ErrorResponse::internal_error(e.to_string()).into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

/// PATCH /api/projects/:projectId/profiles/:profileId/metadata
/// Update profile metadata.
async fn update_metadata(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Json(req): Json<UpdateMetadataRequest>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load existing profile
    let mut doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(e) => return Into::<(StatusCode, Json<ErrorResponse>)>::into(e).into_response(),
    };

    // Update metadata fields
    if let Some(name) = req.name {
        doc.metadata.name = name;
    }
    if let Some(title) = req.title {
        doc.metadata.title = Some(title);
    }
    if let Some(description) = req.description {
        doc.metadata.description = Some(description);
    }
    if let Some(status) = req.status {
        doc.metadata.status = match status.as_str() {
            "draft" => ProfileStatus::Draft,
            "active" => ProfileStatus::Active,
            "retired" => ProfileStatus::Retired,
            _ => ProfileStatus::Unknown,
        };
    }
    if let Some(version) = req.version {
        doc.metadata.version = Some(version);
    }
    if let Some(publisher) = req.publisher {
        doc.metadata.publisher = Some(publisher);
    }
    if let Some(purpose) = req.purpose {
        doc.metadata.purpose = Some(purpose);
    }
    if let Some(copyright) = req.copyright {
        doc.metadata.copyright = Some(copyright);
    }
    if let Some(experimental) = req.experimental {
        doc.metadata.experimental = experimental;
    }

    doc.mark_dirty();

    // Save updated profile
    if let Err(e) = storage.save_profile(&doc).await {
        return ErrorResponse::internal_error(e.to_string()).into_response();
    }

    let response = ProfileDetailsResponse::from(&doc);
    Json(ApiResponse::ok(response)).into_response()
}

/// PATCH /api/projects/:projectId/profiles/:profileId/elements/:path
/// Update an element's constraints.
async fn update_element(
    State(state): State<AppState>,
    Path(params): Path<ElementPath>,
    Json(req): Json<UpdateElementRequest>,
) -> impl IntoResponse {
    let project_dir = state.project_path(&params.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load existing profile
    let mut doc = match storage.load_profile(&params.profile_id).await {
        Ok(d) => d,
        Err(e) => return Into::<(StatusCode, Json<ErrorResponse>)>::into(e).into_response(),
    };

    // Find or create element at path
    let element_path = params.path.trim_start_matches('/');

    // Apply constraint updates and collect diagnostics
    let (constraints, diagnostics) =
        apply_element_updates(&mut doc.resource.root, element_path, req);

    // Mark document as modified
    doc.mark_dirty();

    // Save updated profile
    if let Err(e) = storage.save_profile(&doc).await {
        return ErrorResponse::internal_error(e.to_string()).into_response();
    }

    let response = UpdateElementResponse {
        path: element_path.to_string(),
        constraints,
        validation: diagnostics,
    };

    Json(ApiResponse::ok(response)).into_response()
}

/// Apply updates to an element and return the updated constraints.
fn apply_element_updates(
    root: &mut ElementNode,
    element_path: &str,
    req: UpdateElementRequest,
) -> (crate::ir::ElementConstraints, Vec<Diagnostic>) {
    let element = find_or_create_element(root, element_path);
    let mut diagnostics = Vec::new();

    if let Some(cardinality) = req.cardinality {
        let min = cardinality.min.unwrap_or(0);
        let max = cardinality.max.map(|m| m.to_option()).unwrap_or(None);

        // Validate cardinality
        if let Some(max_val) = max {
            if min > max_val {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    code: "INVALID_CARDINALITY".to_string(),
                    message: format!("min ({}) cannot be greater than max ({})", min, max_val),
                    path: Some(element_path.to_string()),
                });
            }
        }

        element.constraints.cardinality = Some(Cardinality { min, max });
    }

    if let Some(flags) = req.flags {
        if let Some(must_support) = flags.must_support {
            element.constraints.flags.must_support = must_support;
        }
        if let Some(is_modifier) = flags.is_modifier {
            element.constraints.flags.is_modifier = is_modifier;
        }
        if let Some(reason) = flags.is_modifier_reason {
            element.constraints.flags.is_modifier_reason = Some(reason);
        }
        if let Some(is_summary) = flags.is_summary {
            element.constraints.flags.is_summary = is_summary;
        }
    }

    if let Some(types) = req.types {
        element.constraints.types = types
            .into_iter()
            .map(|t| {
                let mut tc = TypeConstraint::simple(&t.code);
                if let Some(profile) = t.profile {
                    tc.profile = profile;
                }
                if let Some(target) = t.target_profile {
                    tc.target_profile = target;
                }
                tc
            })
            .collect();
    }

    if let Some(binding) = req.binding {
        let strength = match binding.strength.as_str() {
            "required" => BindingStrength::Required,
            "extensible" => BindingStrength::Extensible,
            "preferred" => BindingStrength::Preferred,
            _ => BindingStrength::Example,
        };

        element.constraints.binding = Some(Binding {
            strength,
            value_set: binding.value_set,
            description: binding.description,
        });
    }

    if let Some(short) = req.short {
        element.constraints.short = Some(short);
    }
    if let Some(definition) = req.definition {
        element.constraints.definition = Some(definition);
    }
    if let Some(comment) = req.comment {
        element.constraints.comment = Some(comment);
    }

    // Mark element as modified
    element.source = crate::ir::ElementSource::Modified;

    (element.constraints.clone(), diagnostics)
}

/// Find or create an element at the given path.
fn find_or_create_element<'a>(root: &'a mut ElementNode, path: &str) -> &'a mut ElementNode {
    // Split path into segments
    let segments: Vec<&str> = path.split('.').collect();

    if segments.is_empty() || segments[0] == root.short_name() {
        // Path starts with root, navigate from there
        if segments.len() <= 1 {
            return root;
        }
        navigate_to_element(root, &segments[1..])
    } else {
        // Path doesn't include root, navigate from root
        navigate_to_element(root, &segments)
    }
}

fn navigate_to_element<'a>(current: &'a mut ElementNode, segments: &[&str]) -> &'a mut ElementNode {
    if segments.is_empty() {
        return current;
    }

    let segment = segments[0];
    let remaining = &segments[1..];

    // Find existing child or create new
    let child_idx = current
        .children
        .iter()
        .position(|c| c.short_name() == segment);

    if let Some(idx) = child_idx {
        navigate_to_element(&mut current.children[idx], remaining)
    } else {
        // Create new element
        let new_path = format!("{}.{}", current.path, segment);
        let new_element = ElementNode::new(new_path);
        current.children.push(new_element);
        let idx = current.children.len() - 1;
        navigate_to_element(&mut current.children[idx], remaining)
    }
}

/// POST /api/projects/:projectId/profiles/:profileId/import
/// Import SD or FSH content into a profile.
async fn import_profile(
    State(state): State<AppState>,
    Path(params): Path<ProfilePath>,
    Json(req): Json<ImportProfileRequest>,
) -> impl IntoResponse {
    use crate::project::{ProjectService, ResourceKind, SourceFormat};
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    let project_dir = state.project_path(&params.project_id);
    let project_service = ProjectService::new(state.workspace_dir().clone());

    // Ensure directories exist
    let ir_resources_dir = project_dir.join("IR").join("resources");
    let sd_dir = project_dir.join("SD").join("StructureDefinition");
    if let Err(e) = fs::create_dir_all(&ir_resources_dir).await {
        return ErrorResponse::internal_error(format!("Failed to create IR directory: {}", e)).into_response();
    }
    if let Err(e) = fs::create_dir_all(&sd_dir).await {
        return ErrorResponse::internal_error(format!("Failed to create SD directory: {}", e)).into_response();
    }

    let mut diagnostics = Vec::new();

    match req.format {
        ImportFormat::Json => {
            // Import using the import module
            let importer = crate::import::StructureDefinitionImporter::new();
            match importer.import_json(&req.content).await {
                Ok(imported_doc) => {
                    // Use imported document directly (no merge with existing for now)
                    let doc = imported_doc;

                    // Save raw SD JSON to SD folder
                    let sd_path = sd_dir.join(format!("{}.json", doc.metadata.name));
                    if let Err(e) = fs::write(&sd_path, &req.content).await {
                        diagnostics.push(Diagnostic {
                            severity: DiagnosticSeverity::Warning,
                            code: "SAVE_SOURCE_FAILED".to_string(),
                            message: format!("Failed to save source file: {}", e),
                            path: None,
                        });
                    }

                    // Save profile document to IR/resources
                    let ir_path = ir_resources_dir.join(format!("{}.json", doc.metadata.id));
                    let content = match serde_json::to_string_pretty(&doc) {
                        Ok(c) => c,
                        Err(e) => return ErrorResponse::internal_error(format!("Failed to serialize profile: {}", e)).into_response(),
                    };
                    if let Err(e) = fs::write(&ir_path, &content).await {
                        return ErrorResponse::internal_error(format!("Failed to save profile: {}", e)).into_response();
                    }

                    // Register in project index for tree visibility
                    let resource_kind = match doc.resource.kind {
                        crate::ir::StructureKind::Resource | crate::ir::StructureKind::ComplexType => {
                            // Check if it's an extension based on base definition
                            if doc.resource.base.url.contains("Extension") {
                                ResourceKind::Extension
                            } else {
                                ResourceKind::Profile
                            }
                        }
                        crate::ir::StructureKind::Logical => ResourceKind::Profile,
                        crate::ir::StructureKind::PrimitiveType => ResourceKind::Profile,
                    };

                    let add_request = crate::project::AddResourceRequest {
                        id: Some(doc.metadata.id.clone()),
                        name: doc.metadata.name.clone(),
                        kind: resource_kind,
                        canonical_url: Some(doc.metadata.url.clone()),
                        base: Some(doc.resource.base.url.clone()),
                        source_format: Some(SourceFormat::Sd),
                        description: doc.metadata.description.clone(),
                        context: None,
                        purpose: None,
                        content: Some(req.content.clone()),
                    };

                    // Try to add to project index, but don't fail if it already exists
                    if let Err(e) = project_service.add_resource(&params.project_id, add_request).await {
                        // If resource already exists, try to update it instead
                        if !e.to_string().contains("already exists") {
                            diagnostics.push(Diagnostic {
                                severity: DiagnosticSeverity::Warning,
                                code: "INDEX_UPDATE_FAILED".to_string(),
                                message: format!("Failed to update project index: {}", e),
                                path: None,
                            });
                        }
                    }

                    let response = ImportResponse {
                        profile: ProfileDetailsResponse::from(&doc),
                        diagnostics,
                    };

                    Json(ApiResponse::ok(response)).into_response()
                }
                Err(e) => ErrorResponse::validation_error(format!("Import failed: {}", e))
                    .into_response(),
            }
        }
        ImportFormat::Fsh => {
            // Import FSH using the fsh module
            use crate::fsh::{FshImporter, FshImportOptions};
            use crate::ir::FhirVersion;

            // Load project for canonical URL and FHIR version
            let project = match project_service.load_project(&params.project_id).await {
                Ok(p) => p,
                Err(e) => {
                    return ErrorResponse::internal_error(format!("Failed to load project: {}", e)).into_response();
                }
            };

            let options = FshImportOptions::default()
                .with_canonical_base(&project.canonical_base)
                .with_fhir_version(project.fhir_version);

            let importer = match FshImporter::with_options(options).await {
                Ok(i) => i,
                Err(e) => {
                    return ErrorResponse::internal_error(format!(
                        "Failed to initialize FSH importer: {}",
                        e
                    ))
                    .into_response();
                }
            };

            let temp_path = std::path::PathBuf::from(&params.profile_id).with_extension("fsh");
            match importer.import_content(&req.content, &temp_path).await {
                Ok(result) => {
                    // Convert FSH warnings to API diagnostics
                    for warning in &result.warnings {
                        diagnostics.push(Diagnostic {
                            severity: DiagnosticSeverity::Warning,
                            code: format!("{:?}", warning.code),
                            message: warning.message.clone(),
                            path: warning.file.as_ref().map(|p| p.display().to_string()),
                        });
                    }

                    if result.value.is_empty() {
                        return ErrorResponse::validation_error(
                            "No profiles found in FSH content",
                        )
                        .into_response();
                    }

                    // Use the first imported profile
                    let doc = result.value.into_iter().next().unwrap();

                    // Save FSH source file
                    let fsh_dir = project_dir.join("FSH");
                    if let Err(e) = fs::create_dir_all(&fsh_dir).await {
                        diagnostics.push(Diagnostic {
                            severity: DiagnosticSeverity::Warning,
                            code: "SAVE_SOURCE_FAILED".to_string(),
                            message: format!("Failed to create FSH directory: {}", e),
                            path: None,
                        });
                    } else {
                        let fsh_path = fsh_dir.join(format!("{}.fsh", doc.metadata.name));
                        if let Err(e) = fs::write(&fsh_path, &req.content).await {
                            diagnostics.push(Diagnostic {
                                severity: DiagnosticSeverity::Warning,
                                code: "SAVE_SOURCE_FAILED".to_string(),
                                message: format!("Failed to save FSH source: {}", e),
                                path: None,
                            });
                        }
                    }

                    // Save profile document to IR/resources
                    let ir_path = ir_resources_dir.join(format!("{}.json", doc.metadata.id));
                    let content = match serde_json::to_string_pretty(&doc) {
                        Ok(c) => c,
                        Err(e) => return ErrorResponse::internal_error(format!("Failed to serialize profile: {}", e)).into_response(),
                    };
                    if let Err(e) = fs::write(&ir_path, &content).await {
                        return ErrorResponse::internal_error(format!("Failed to save profile: {}", e)).into_response();
                    }

                    // Register in project index for tree visibility
                    let resource_kind = match doc.resource.kind {
                        crate::ir::StructureKind::Resource | crate::ir::StructureKind::ComplexType => {
                            if doc.resource.base.url.contains("Extension") {
                                ResourceKind::Extension
                            } else {
                                ResourceKind::Profile
                            }
                        }
                        crate::ir::StructureKind::Logical => ResourceKind::Profile,
                        crate::ir::StructureKind::PrimitiveType => ResourceKind::Profile,
                    };

                    let add_request = crate::project::AddResourceRequest {
                        id: Some(doc.metadata.id.clone()),
                        name: doc.metadata.name.clone(),
                        kind: resource_kind,
                        canonical_url: Some(doc.metadata.url.clone()),
                        base: Some(doc.resource.base.url.clone()),
                        source_format: Some(SourceFormat::Fsh),
                        description: doc.metadata.description.clone(),
                        context: None,
                        purpose: None,
                        content: None, // FSH content already saved separately
                    };

                    if let Err(e) = project_service.add_resource(&params.project_id, add_request).await {
                        if !e.to_string().contains("already exists") {
                            diagnostics.push(Diagnostic {
                                severity: DiagnosticSeverity::Warning,
                                code: "INDEX_UPDATE_FAILED".to_string(),
                                message: format!("Failed to update project index: {}", e),
                                path: None,
                            });
                        }
                    }

                    let response = ImportResponse {
                        profile: ProfileDetailsResponse::from(&doc),
                        diagnostics,
                    };

                    Json(ApiResponse::ok(response)).into_response()
                }
                Err(e) => {
                    ErrorResponse::validation_error(format!("FSH import failed: {}", e))
                        .into_response()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_creation() {
        let (status, json) = ErrorResponse::not_found("Profile", "test-id");
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(json.0.code, "NOT_FOUND");
    }

    #[test]
    fn test_find_or_create_element() {
        let mut root = ElementNode::new("Patient".to_string());

        // Find/create a nested element
        let element = find_or_create_element(&mut root, "Patient.name.family");
        assert_eq!(element.path, "Patient.name.family");

        // Should have created intermediate elements
        assert!(!root.children.is_empty());
    }
}
