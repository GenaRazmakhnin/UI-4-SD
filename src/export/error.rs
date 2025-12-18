//! Export error types.
//!
//! Provides detailed error types for StructureDefinition export operations
//! with clear messages and context.

use thiserror::Error;

/// Result type for export operations.
pub type ExportResult<T> = Result<T, ExportError>;

/// Errors that can occur during StructureDefinition export.
#[derive(Debug, Error)]
pub enum ExportError {
    /// Invalid IR state for export.
    #[error("Invalid IR state: {message}")]
    InvalidState { message: String },

    /// Missing required metadata.
    #[error("Missing required metadata field: {field}")]
    MissingMetadata { field: String },

    /// Invalid element structure.
    #[error("Invalid element at '{path}': {message}")]
    InvalidElement { path: String, message: String },

    /// Slicing export error.
    #[error("Slicing error at '{path}': {message}")]
    SlicingError { path: String, message: String },

    /// Serialization error.
    #[error("Serialization error: {message}")]
    Serialization {
        message: String,
        #[source]
        source: Option<serde_json::Error>,
    },

    /// Snapshot generation error.
    #[error("Snapshot generation error at '{path}': {message}")]
    SnapshotGeneration { path: String, message: String },

    /// Differential generation error.
    #[error("Differential generation error: {message}")]
    DifferentialGeneration { message: String },

    /// Validation error.
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// General export error.
    #[error("Export error: {0}")]
    Other(String),
}

impl ExportError {
    /// Create an invalid state error.
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState {
            message: message.into(),
        }
    }

    /// Create a missing metadata error.
    pub fn missing_metadata(field: impl Into<String>) -> Self {
        Self::MissingMetadata {
            field: field.into(),
        }
    }

    /// Create an invalid element error.
    pub fn invalid_element(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidElement {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a slicing error.
    pub fn slicing_error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::SlicingError {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a serialization error.
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::Serialization {
            message: message.into(),
            source: None,
        }
    }

    /// Create a serialization error with source.
    pub fn serialization_with_source(err: serde_json::Error) -> Self {
        Self::Serialization {
            message: err.to_string(),
            source: Some(err),
        }
    }

    /// Create a snapshot generation error.
    pub fn snapshot_generation(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::SnapshotGeneration {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a differential generation error.
    pub fn differential_generation(message: impl Into<String>) -> Self {
        Self::DifferentialGeneration {
            message: message.into(),
        }
    }

    /// Create a validation error.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create a general error.
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

/// Warning generated during export (non-fatal issues).
#[derive(Debug, Clone)]
pub struct ExportWarning {
    /// Warning message.
    pub message: String,
    /// Element path where warning occurred.
    pub path: Option<String>,
    /// Warning code for programmatic handling.
    pub code: ExportWarningCode,
}

impl ExportWarning {
    /// Create a new warning.
    pub fn new(code: ExportWarningCode, message: impl Into<String>) -> Self {
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

impl std::fmt::Display for ExportWarning {
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
pub enum ExportWarningCode {
    /// Unknown field will be included in output.
    UnknownFieldIncluded,
    /// Element has no modifications (inherited only).
    NoModifications,
    /// Potential validation issue.
    ValidationHint,
    /// Deprecated pattern used.
    DeprecatedPattern,
    /// Slicing may be incomplete.
    IncompleteSlicing,
}

/// Export result with warnings.
#[derive(Debug)]
pub struct ExportResultWithWarnings<T> {
    /// The exported value.
    pub value: T,
    /// Warnings generated during export.
    pub warnings: Vec<ExportWarning>,
}

impl<T> ExportResultWithWarnings<T> {
    /// Create a result with no warnings.
    pub fn ok(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
        }
    }

    /// Create a result with warnings.
    pub fn with_warnings(value: T, warnings: Vec<ExportWarning>) -> Self {
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

    /// Map the inner value.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ExportResultWithWarnings<U> {
        ExportResultWithWarnings {
            value: f(self.value),
            warnings: self.warnings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let err = ExportError::missing_metadata("url");
        assert!(err.to_string().contains("url"));

        let err = ExportError::invalid_element("Patient.name", "invalid cardinality");
        assert!(err.to_string().contains("Patient.name"));
        assert!(err.to_string().contains("invalid cardinality"));
    }

    #[test]
    fn test_warning_creation() {
        let warning = ExportWarning::new(
            ExportWarningCode::UnknownFieldIncluded,
            "Unknown field 'x' included",
        )
        .at_path("Patient.name");

        assert_eq!(warning.path.as_deref(), Some("Patient.name"));
        assert!(warning.to_string().contains("Patient.name"));
    }
}
