//! Import error types.
//!
//! Provides detailed error types for StructureDefinition import operations
//! with clear messages and context.

use thiserror::Error;

/// Result type for import operations.
pub type ImportResult<T> = Result<T, ImportError>;

/// Errors that can occur during StructureDefinition import.
#[derive(Debug, Error)]
pub enum ImportError {
    /// JSON parsing failed.
    #[error("JSON parse error: {message}")]
    JsonParse {
        message: String,
        #[source]
        source: Option<serde_json::Error>,
    },

    /// Required field is missing.
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Invalid field value.
    #[error("Invalid value for field '{field}': {message}")]
    InvalidValue { field: String, message: String },

    /// Invalid resource type.
    #[error("Invalid resourceType: expected 'StructureDefinition', got '{actual}'")]
    InvalidResourceType { actual: String },

    /// Invalid element path.
    #[error("Invalid element path '{path}': {message}")]
    InvalidPath { path: String, message: String },

    /// Base definition not found.
    #[error("Base definition not found: {url}")]
    BaseNotFound { url: String },

    /// Slicing error.
    #[error("Slicing error at '{path}': {message}")]
    SlicingError { path: String, message: String },

    /// Element tree construction error.
    #[error("Element tree error: {message}")]
    ElementTreeError { message: String },

    /// IO error during import.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// General import error.
    #[error("Import error: {0}")]
    Other(String),
}

impl ImportError {
    /// Create a JSON parse error.
    pub fn json_parse(message: impl Into<String>) -> Self {
        Self::JsonParse {
            message: message.into(),
            source: None,
        }
    }

    /// Create a JSON parse error with source.
    pub fn json_parse_with_source(err: serde_json::Error) -> Self {
        Self::JsonParse {
            message: err.to_string(),
            source: Some(err),
        }
    }

    /// Create a missing field error.
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create an invalid value error.
    pub fn invalid_value(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create an invalid resource type error.
    pub fn invalid_resource_type(actual: impl Into<String>) -> Self {
        Self::InvalidResourceType {
            actual: actual.into(),
        }
    }

    /// Create an invalid path error.
    pub fn invalid_path(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidPath {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a base not found error.
    pub fn base_not_found(url: impl Into<String>) -> Self {
        Self::BaseNotFound { url: url.into() }
    }

    /// Create a slicing error.
    pub fn slicing_error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::SlicingError {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create an element tree error.
    pub fn element_tree(message: impl Into<String>) -> Self {
        Self::ElementTreeError {
            message: message.into(),
        }
    }

    /// Create a general error.
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

/// Warning generated during import (non-fatal issues).
#[derive(Debug, Clone)]
pub struct ImportWarning {
    /// Warning message.
    pub message: String,
    /// Element path where warning occurred.
    pub path: Option<String>,
    /// Warning code for programmatic handling.
    pub code: ImportWarningCode,
}

impl ImportWarning {
    /// Create a new warning.
    pub fn new(code: ImportWarningCode, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            path: None,
            code,
        }
    }

    /// Set the path for this warning.
    #[must_use]
    pub fn at_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl std::fmt::Display for ImportWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "[{}] {}", path, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

/// Warning codes for programmatic handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportWarningCode {
    /// Unknown field was preserved.
    UnknownFieldPreserved,
    /// Missing snapshot, generated from differential.
    MissingSnapshot,
    /// Deprecated field used.
    DeprecatedField,
    /// Base definition could not be resolved for full validation.
    BaseUnresolved,
    /// Slicing discriminator may be incomplete.
    IncompleteSlicing,
    /// Element has no constraints defined.
    NoConstraints,
}

/// Import result with warnings.
#[derive(Debug)]
pub struct ImportResultWithWarnings<T> {
    /// The imported value.
    pub value: T,
    /// Warnings generated during import.
    pub warnings: Vec<ImportWarning>,
}

impl<T> ImportResultWithWarnings<T> {
    /// Create a result with no warnings.
    pub fn ok(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
        }
    }

    /// Create a result with warnings.
    pub fn with_warnings(value: T, warnings: Vec<ImportWarning>) -> Self {
        Self { value, warnings }
    }

    /// Check if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get the value, discarding warnings.
    pub fn into_value(self) -> T {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let err = ImportError::missing_field("url");
        assert!(err.to_string().contains("url"));

        let err = ImportError::invalid_value("status", "unknown value");
        assert!(err.to_string().contains("status"));
        assert!(err.to_string().contains("unknown value"));
    }

    #[test]
    fn test_warning_creation() {
        let warning = ImportWarning::new(
            ImportWarningCode::UnknownFieldPreserved,
            "Unknown field 'x' preserved",
        )
        .at_path("Patient.name");

        assert_eq!(warning.path.as_deref(), Some("Patient.name"));
        assert!(warning.to_string().contains("Patient.name"));
    }
}
