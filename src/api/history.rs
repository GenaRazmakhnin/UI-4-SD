//! History API route handlers.
//!
//! Implements REST endpoints for undo/redo and history navigation.
//!
//! # Routes
//!
//! - `POST /api/projects/:projectId/profiles/:profileId/undo` - Undo last operation
//! - `POST /api/projects/:projectId/profiles/:profileId/redo` - Redo next operation
//! - `GET  /api/projects/:projectId/profiles/:profileId/history` - Get history list
//! - `POST /api/projects/:projectId/profiles/:profileId/history/goto` - Jump to index

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::ir::{HistoryState, OperationSummary, ProfileDocument};
use crate::state::AppState;

use super::profile_merge::hydrate_profile_document;
use super::profiles::{ErrorResponse, ProfilePath};
use super::storage::{ProfileStorage, StorageError};

/// Create history routes.
pub fn history_routes() -> Router<AppState> {
    Router::new()
        .route("/{profileId}/undo", post(undo))
        .route("/{profileId}/redo", post(redo))
        .route("/{profileId}/history", get(get_history))
        .route("/{profileId}/history/goto", post(goto_history))
}

// === Response Types ===

/// Response for undo/redo operations.
#[derive(Debug, Serialize)]
pub struct UndoRedoResponse {
    /// Whether the operation succeeded.
    pub success: bool,
    /// Description of what was undone/redone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// Current history state.
    pub history: HistoryState,
    /// Updated profile (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileSummary>,
}

/// Minimal profile summary for undo/redo responses.
#[derive(Debug, Serialize)]
pub struct ProfileSummary {
    pub id: String,
    pub name: String,
    pub is_dirty: bool,
    pub element_count: usize,
}

impl From<&ProfileDocument> for ProfileSummary {
    fn from(doc: &ProfileDocument) -> Self {
        Self {
            id: doc.metadata.id.clone(),
            name: doc.metadata.name.clone(),
            is_dirty: doc.is_dirty(),
            element_count: doc.element_count(),
        }
    }
}

/// Response for history list.
#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    /// Current history state.
    pub state: HistoryState,
    /// List of all operations.
    pub operations: Vec<OperationSummary>,
}

/// Request for goto operation.
#[derive(Debug, Deserialize)]
pub struct GotoRequest {
    /// Target index to jump to.
    pub index: usize,
}

/// Response for goto operation.
#[derive(Debug, Serialize)]
pub struct GotoResponse {
    /// Whether the operation succeeded.
    pub success: bool,
    /// Number of operations applied (undo or redo).
    pub operations_applied: usize,
    /// Direction moved (undo or redo).
    pub direction: String,
    /// Current history state.
    pub history: HistoryState,
    /// Updated profile summary.
    pub profile: ProfileSummary,
}

// === Handlers ===

/// Undo the last operation.
///
/// POST /api/projects/:projectId/profiles/:profileId/undo
async fn undo(
    State(state): State<AppState>,
    Path(path): Path<ProfilePath>,
) -> Result<Json<UndoRedoResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let project_dir = state.project_path(&path.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let mut doc = storage
        .load_profile(&path.profile_id)
        .await
        .map_err(|e: StorageError| -> (axum::http::StatusCode, Json<ErrorResponse>) { e.into() })?;
    let mut doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(e))),
    };

    // Check if undo is available
    if !doc.history.can_undo() {
        return Ok(Json(UndoRedoResponse {
            success: false,
            operation: None,
            history: doc.history.state(),
            profile: Some(ProfileSummary::from(&doc)),
        }));
    }

    // Get description before undo
    let description = doc.history.undo_description().map(String::from);

    // Perform undo
    if let Some(_inverse_op) = doc.history.undo() {
        // Note: The actual operation would need to be applied to the document
        // For now we just update the history state
        doc.mark_dirty();

        // Save the updated document
        storage
            .save_profile(&doc)
            .await
            .map_err(|e: StorageError| -> (axum::http::StatusCode, Json<ErrorResponse>) { e.into() })?;
    }

    Ok(Json(UndoRedoResponse {
        success: true,
        operation: description,
        history: doc.history.state(),
        profile: Some(ProfileSummary::from(&doc)),
    }))
}

/// Redo the next operation.
///
/// POST /api/projects/:projectId/profiles/:profileId/redo
async fn redo(
    State(state): State<AppState>,
    Path(path): Path<ProfilePath>,
) -> Result<Json<UndoRedoResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let project_dir = state.project_path(&path.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let mut doc = storage
        .load_profile(&path.profile_id)
        .await
        .map_err(|e: StorageError| -> (axum::http::StatusCode, Json<ErrorResponse>) { e.into() })?;
    let mut doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(e))),
    };

    // Check if redo is available
    if !doc.history.can_redo() {
        return Ok(Json(UndoRedoResponse {
            success: false,
            operation: None,
            history: doc.history.state(),
            profile: Some(ProfileSummary::from(&doc)),
        }));
    }

    // Get description before redo
    let description = doc.history.redo_description().map(String::from);

    // Perform redo
    if let Some(_op) = doc.history.redo() {
        // Note: The actual operation would need to be applied to the document
        doc.mark_dirty();

        // Save the updated document
        storage
            .save_profile(&doc)
            .await
            .map_err(|e: StorageError| -> (axum::http::StatusCode, Json<ErrorResponse>) { e.into() })?;
    }

    Ok(Json(UndoRedoResponse {
        success: true,
        operation: description,
        history: doc.history.state(),
        profile: Some(ProfileSummary::from(&doc)),
    }))
}

/// Get the operation history for a profile.
///
/// GET /api/projects/:projectId/profiles/:profileId/history
async fn get_history(
    State(state): State<AppState>,
    Path(path): Path<ProfilePath>,
) -> Result<Json<HistoryResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let project_dir = state.project_path(&path.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let doc = storage
        .load_profile(&path.profile_id)
        .await
        .map_err(|e: StorageError| -> (axum::http::StatusCode, Json<ErrorResponse>) { e.into() })?;
    let doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(e))),
    };

    Ok(Json(HistoryResponse {
        state: doc.history.state(),
        operations: doc.history.get_operations(),
    }))
}

/// Jump to a specific point in history.
///
/// POST /api/projects/:projectId/profiles/:profileId/history/goto
async fn goto_history(
    State(state): State<AppState>,
    Path(path): Path<ProfilePath>,
    Json(request): Json<GotoRequest>,
) -> Result<Json<GotoResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let project_dir = state.project_path(&path.project_id);
    let storage = ProfileStorage::new(&project_dir);

    // Load profile
    let mut doc = storage
        .load_profile(&path.profile_id)
        .await
        .map_err(|e: StorageError| -> (axum::http::StatusCode, Json<ErrorResponse>) { e.into() })?;
    let mut doc = match hydrate_profile_document(&state, doc).await {
        Ok(d) => d,
        Err(e) => return Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(e))),
    };

    let current_index = doc.history.current_index();
    let target_index = request.index;

    // Determine direction
    let (direction, operations_applied) = if target_index < current_index {
        let ops = doc.history.goto(target_index);
        ("undo".to_string(), ops.len())
    } else if target_index > current_index {
        let ops = doc.history.goto(target_index);
        ("redo".to_string(), ops.len())
    } else {
        ("none".to_string(), 0)
    };

    // Mark dirty if we moved
    if operations_applied > 0 {
        doc.mark_dirty();

        // Save the updated document
        storage
            .save_profile(&doc)
            .await
            .map_err(|e: StorageError| -> (axum::http::StatusCode, Json<ErrorResponse>) { e.into() })?;
    }

    Ok(Json(GotoResponse {
        success: true,
        operations_applied,
        direction,
        history: doc.history.state(),
        profile: ProfileSummary::from(&doc),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_summary() {
        use crate::ir::{BaseDefinition, DocumentMetadata, FhirVersion, ProfiledResource};

        let metadata = DocumentMetadata::new(
            "test-patient",
            "http://example.org/fhir/StructureDefinition/TestPatient",
            "TestPatient",
        );
        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        let doc = crate::ir::ProfileDocument::new(metadata, resource);

        let summary = ProfileSummary::from(&doc);
        assert_eq!(summary.id, "test-patient");
        assert_eq!(summary.name, "TestPatient");
        assert!(!summary.is_dirty);
    }
}
