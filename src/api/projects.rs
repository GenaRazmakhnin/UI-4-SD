//! Project API route handlers.
//!
//! Implements REST endpoints for project management.
//!
//! # Routes
//!
//! - `GET    /api/projects` - List all projects
//! - `POST   /api/projects` - Create a new project
//! - `GET    /api/projects/:projectId` - Get project details
//! - `PUT    /api/projects/:projectId` - Update project configuration
//! - `DELETE /api/projects/:projectId` - Delete a project
//! - `POST   /api/projects/:projectId/resources` - Add a resource to a project
//! - `DELETE /api/projects/:projectId/resources/:resourceId` - Remove a resource
//! - `GET    /api/projects/:projectId/tree` - Get project file tree
//! - `GET    /api/projects/:projectId/dependencies` - Get dependency graph

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::project::{
    AddResourceRequest, CreateProjectRequest, FileTreeNode, Project, ProjectError,
    ProjectResource, ProjectService, ProjectStatus, ResourceKind, UpdateProjectRequest,
};
use crate::state::AppState;

use super::dto::ApiResponse;

/// Create project routes.
pub fn project_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_projects).post(create_project))
        .route(
            "/{projectId}",
            get(get_project)
                .put(update_project)
                .patch(update_project)  // Support both PUT and PATCH
                .delete(delete_project),
        )
        .route("/{projectId}/resources", post(add_resource))
        .route("/{projectId}/resources/{resourceId}", delete(remove_resource).get(get_resource))
        // Artifact endpoints (aliases to resources for frontend compatibility)
        .route("/{projectId}/artifacts", post(add_artifact))
        .route("/{projectId}/artifacts/{resourceId}", delete(remove_resource))
        .route("/{projectId}/tree", get(get_file_tree))
        .route("/{projectId}/dependencies", get(get_dependencies))
}

// === Path Parameters ===

#[derive(Debug, Deserialize)]
pub struct ProjectPath {
    #[serde(rename = "projectId")]
    pub project_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ResourcePath {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}

// === Response Types ===
// Note: ApiResponse and ErrorInfo are imported from super::dto

/// Project list response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectSummary>,
}

/// Project summary for list view.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub canonical_base: String,
    pub fhir_version: String,
    pub status: ProjectStatus,
    pub resource_count: usize,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Full project details response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDetails {
    pub project: Project,
    pub resources: Vec<ProjectResource>,
}

/// Resource creation response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCreatedResponse {
    pub resource: ProjectResource,
}

/// Artifact creation response (frontend format).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactCreatedResponse {
    pub path: String,
    pub resource_id: String,
    pub resource_type: String,
    pub resource_kind: ResourceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
}

/// Dependency graph response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyGraphResponse {
    pub resources: Vec<ResourceNode>,
    pub edges: Vec<DependencyEdge>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceNode {
    pub id: String,
    pub canonical_url: String,
    pub name: String,
    pub kind: ResourceKind,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
}

// === Error Handling ===

fn handle_error(err: ProjectError) -> (StatusCode, Json<ApiResponse<()>>) {
    let (status, code) = match &err {
        ProjectError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
        ProjectError::ResourceNotFound(_) => (StatusCode::NOT_FOUND, "RESOURCE_NOT_FOUND"),
        ProjectError::AlreadyExists(_) => (StatusCode::CONFLICT, "ALREADY_EXISTS"),
        ProjectError::ResourceAlreadyExists(_) => (StatusCode::CONFLICT, "RESOURCE_ALREADY_EXISTS"),
        ProjectError::InvalidStructure(_) => (StatusCode::BAD_REQUEST, "INVALID_STRUCTURE"),
        ProjectError::InvalidCanonicalUrl(_) => (StatusCode::BAD_REQUEST, "INVALID_CANONICAL_URL"),
        ProjectError::DependencyError(_) => (StatusCode::CONFLICT, "DEPENDENCY_ERROR"),
        ProjectError::CircularDependency(_) => (StatusCode::CONFLICT, "CIRCULAR_DEPENDENCY"),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
    };

    (status, Json(ApiResponse::<()>::error(code, err.to_string())))
}

// === Handlers ===

/// GET /api/projects
/// List all projects in the workspace.
async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ProjectListResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let projects = service.list_projects().await.map_err(handle_error)?;

    // Get resource counts for each project
    let mut summaries = Vec::with_capacity(projects.len());
    for project in projects {
        let index = service.load_index(&project.id).await.unwrap_or_default();
        summaries.push(ProjectSummary {
            id: project.id.clone(),
            name: project.name.clone(),
            canonical_base: project.canonical_base.clone(),
            fhir_version: project.fhir_version.label().to_string(),
            status: project.status,
            resource_count: index.resource_count(),
            modified_at: project.modified_at,
        });
    }

    Ok(Json(ApiResponse::ok(ProjectListResponse {
        projects: summaries,
    })))
}

/// POST /api/projects
/// Create a new project.
async fn create_project(
    State(state): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<ApiResponse<Project>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let project = service.create_project(request).await.map_err(handle_error)?;

    Ok(Json(ApiResponse::ok(project)))
}

/// GET /api/projects/:projectId
/// Get project details including resources.
async fn get_project(
    State(state): State<AppState>,
    Path(path): Path<ProjectPath>,
) -> Result<Json<ApiResponse<ProjectDetails>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let project = service
        .load_project(&path.project_id)
        .await
        .map_err(handle_error)?;

    let resources = service
        .list_resources(&path.project_id)
        .await
        .map_err(handle_error)?;

    Ok(Json(ApiResponse::ok(ProjectDetails { project, resources })))
}

/// PUT /api/projects/:projectId
/// Update project configuration.
async fn update_project(
    State(state): State<AppState>,
    Path(path): Path<ProjectPath>,
    Json(request): Json<UpdateProjectRequest>,
) -> Result<Json<ApiResponse<Project>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let project = service
        .update_project(&path.project_id, request)
        .await
        .map_err(handle_error)?;

    Ok(Json(ApiResponse::ok(project)))
}

/// DELETE /api/projects/:projectId
/// Delete a project.
async fn delete_project(
    State(state): State<AppState>,
    Path(path): Path<ProjectPath>,
) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    service
        .delete_project(&path.project_id)
        .await
        .map_err(handle_error)?;

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:projectId/resources
/// Add a resource to a project.
async fn add_resource(
    State(state): State<AppState>,
    Path(path): Path<ProjectPath>,
    Json(request): Json<AddResourceRequest>,
) -> Result<Json<ApiResponse<ResourceCreatedResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let resource = service
        .add_resource(&path.project_id, request)
        .await
        .map_err(handle_error)?;

    Ok(Json(ApiResponse::ok(ResourceCreatedResponse { resource })))
}

/// DELETE /api/projects/:projectId/resources/:resourceId
/// Remove a resource from a project.
async fn remove_resource(
    State(state): State<AppState>,
    Path(path): Path<ResourcePath>,
) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    service
        .remove_resource(&path.project_id, &path.resource_id)
        .await
        .map_err(handle_error)?;

    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/projects/:projectId/resources/:resourceId
/// Get a single resource by ID.
async fn get_resource(
    State(state): State<AppState>,
    Path(path): Path<ResourcePath>,
) -> Result<Json<ApiResponse<ProjectResource>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let resource = service
        .get_resource(&path.project_id, &path.resource_id)
        .await
        .map_err(handle_error)?;

    Ok(Json(ApiResponse::ok(resource)))
}

/// POST /api/projects/:projectId/artifacts
/// Add an artifact to a project (frontend-compatible endpoint).
async fn add_artifact(
    State(state): State<AppState>,
    Path(path): Path<ProjectPath>,
    Json(request): Json<AddResourceRequest>,
) -> Result<Json<ApiResponse<ArtifactCreatedResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let resource = service
        .add_resource(&path.project_id, request)
        .await
        .map_err(handle_error)?;

    // Convert to frontend artifact format
    let response = ArtifactCreatedResponse {
        path: resource.ir_path.to_string_lossy().to_string(),
        resource_id: resource.id,
        resource_type: resource.kind.resource_type().to_string(),
        resource_kind: resource.kind,
        canonical_url: Some(resource.canonical_url),
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/projects/:projectId/tree
/// Get the project file tree for the explorer.
async fn get_file_tree(
    State(state): State<AppState>,
    Path(path): Path<ProjectPath>,
) -> Result<Json<ApiResponse<FileTreeNode>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    let tree = service
        .build_file_tree(&path.project_id)
        .await
        .map_err(handle_error)?;

    Ok(Json(ApiResponse::ok(tree)))
}

/// GET /api/projects/:projectId/dependencies
/// Get the dependency graph for a project.
async fn get_dependencies(
    State(state): State<AppState>,
    Path(path): Path<ProjectPath>,
) -> Result<Json<ApiResponse<DependencyGraphResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let service = ProjectService::new(state.workspace_dir().clone());

    // Load the index to get resources
    let index = service
        .load_index(&path.project_id)
        .await
        .map_err(handle_error)?;

    // Build dependency graph (validates the project exists and has valid structure)
    let _graph = service
        .build_dependency_graph(&path.project_id)
        .await
        .map_err(handle_error)?;

    // Build response
    let resources: Vec<ResourceNode> = index
        .resources
        .values()
        .map(|r| ResourceNode {
            id: r.id.clone(),
            canonical_url: r.canonical_url.clone(),
            name: r.name.clone(),
            kind: r.kind,
        })
        .collect();

    let mut edges = Vec::new();
    for resource in index.resources.values() {
        for dep in &resource.dependencies {
            edges.push(DependencyEdge {
                from: resource.canonical_url.clone(),
                to: dep.clone(),
            });
        }
    }

    Ok(Json(ApiResponse::ok(DependencyGraphResponse {
        resources,
        edges,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_ok() {
        let response = ApiResponse::ok("test data");
        assert!(response.success);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response = ApiResponse::<()>::error("TEST_ERROR", "Something went wrong");
        assert!(!response.success);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.as_ref().unwrap().code, "TEST_ERROR");
    }
}
