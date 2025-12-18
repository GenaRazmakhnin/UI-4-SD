//! Validation API Endpoints
//!
//! Provides REST API for profile validation operations.
//!
//! # Routes
//!
//! - `POST /api/projects/:projectId/profiles/:profileId/validate` - Full validation
//! - `POST /api/projects/:projectId/profiles/:profileId/validate/quick` - Quick structural validation
//! - `POST /api/projects/:projectId/profiles/:profileId/validate/element` - Validate specific element

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, Router},
};
use serde::{Deserialize, Serialize};

use super::storage::ProfileStorage;
use crate::state::AppState;
use crate::validation::{ValidationEngine, ValidationLevel, ValidationResult};

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
pub struct ValidateResponse {
    /// Whether the profile is valid.
    pub is_valid: bool,
    /// Validation level performed.
    pub level: String,
    /// Number of errors found.
    pub error_count: usize,
    /// Number of warnings found.
    pub warning_count: usize,
    /// List of diagnostics.
    pub diagnostics: Vec<DiagnosticDto>,
}

/// Diagnostic DTO for API responses.
#[derive(Debug, Serialize)]
pub struct DiagnosticDto {
    /// Severity level.
    pub severity: String,
    /// Error/warning code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Element path if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Source layer of the diagnostic.
    pub source: String,
    /// Available quick fixes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quick_fixes: Vec<QuickFixDto>,
}

/// Quick fix DTO for API responses.
#[derive(Debug, Serialize)]
pub struct QuickFixDto {
    /// Display title.
    pub title: String,
    /// The kind of fix operation.
    pub kind: serde_json::Value,
    /// Whether this is the preferred fix.
    pub is_preferred: bool,
}

/// Convert ValidationResult to ValidateResponse.
fn to_response(result: ValidationResult, level: &str) -> ValidateResponse {
    ValidateResponse {
        is_valid: result.is_valid,
        level: level.to_string(),
        error_count: result.error_count(),
        warning_count: result.warning_count(),
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
                    path: d.element_path.clone(),
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
        Some("structural") => ValidationLevel::Structural,
        Some("references") => ValidationLevel::References,
        Some("terminology") => ValidationLevel::Terminology,
        Some("full") => ValidationLevel::Full,
        _ => ValidationLevel::Structural,
    }
}

/// Path parameters for profile validation.
#[derive(Debug, Deserialize)]
struct ProfilePath {
    project_id: String,
    profile_id: String,
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

    let level_str = format!("{:?}", level).to_lowercase();
    let response = to_response(result, &level_str);

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

    let response = to_response(result, "structural");

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

    let response = to_response(result, "element");

    Json(response).into_response()
}

/// Create validation routes.
pub fn validation_routes() -> Router<AppState> {
    Router::new()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_level() {
        assert!(matches!(parse_level(None), ValidationLevel::Structural));
        assert!(matches!(parse_level(Some("structural")), ValidationLevel::Structural));
        assert!(matches!(parse_level(Some("references")), ValidationLevel::References));
        assert!(matches!(parse_level(Some("terminology")), ValidationLevel::Terminology));
        assert!(matches!(parse_level(Some("full")), ValidationLevel::Full));
        assert!(matches!(parse_level(Some("unknown")), ValidationLevel::Structural));
    }
}
