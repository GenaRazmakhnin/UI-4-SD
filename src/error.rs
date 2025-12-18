//! Error types for the backend server
//!
//! Provides a unified error type with proper HTTP status code mapping.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Result type alias using our Error
pub type Result<T> = std::result::Result<T, Error>;

/// Backend error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Configuration error
    #[error("configuration error: {0}")]
    Config(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// FHIR validation error
    #[error("validation error: {0}")]
    Validation(String),

    /// Resource not found
    #[error("not found: {0}")]
    NotFound(String),

    /// Internal server error
    #[error("internal error: {0}")]
    Internal(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Error::Config(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            Error::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::Serialization(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Error::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = serde_json::json!({
            "error": message,
            "status": status.as_u16()
        });

        (status, axum::Json(body)).into_response()
    }
}
