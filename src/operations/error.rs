//! Operation error types.

use thiserror::Error;

/// Format optional max cardinality for display.
fn max_str(max: &Option<u32>) -> String {
    match max {
        Some(m) => m.to_string(),
        None => "*".to_string(),
    }
}

/// Result type for operations.
pub type OperationResult<T> = Result<T, OperationError>;

/// Errors that can occur during operation execution.
#[derive(Debug, Error)]
pub enum OperationError {
    /// Element not found at the specified path.
    #[error("Element not found: {path}")]
    ElementNotFound { path: String },

    /// Slice not found.
    #[error("Slice not found: {name} at {path}")]
    SliceNotFound { path: String, name: String },

    /// Invalid cardinality values.
    #[error("Invalid cardinality: min ({min}) exceeds max ({max})")]
    InvalidCardinality { min: u32, max: u32 },

    /// Cardinality exceeds base constraint.
    #[error("Cardinality {min}..{} exceeds base constraint {base_min}..{}", max_str(.max), max_str(.base_max))]
    CardinalityExceedsBase {
        min: u32,
        max: Option<u32>,
        base_min: u32,
        base_max: Option<u32>,
    },

    /// Invalid type constraint.
    #[error("Invalid type constraint: {type_code} is not a subtype of base types")]
    InvalidTypeConstraint { type_code: String },

    /// Type not found in allowed types.
    #[error("Type not found: {type_code} is not in allowed types")]
    TypeNotFound { type_code: String },

    /// Binding strength cannot be weakened.
    #[error("Cannot weaken binding strength from {from} to {to}")]
    BindingStrengthWeakened { from: String, to: String },

    /// Invalid binding value set URL.
    #[error("Invalid value set URL: {url}")]
    InvalidValueSetUrl { url: String },

    /// Slicing already exists on element.
    #[error("Element already has slicing: {path}")]
    SlicingAlreadyExists { path: String },

    /// Slicing not defined on element.
    #[error("No slicing defined on element: {path}")]
    NoSlicingDefined { path: String },

    /// Duplicate slice name.
    #[error("Slice name already exists: {name} at {path}")]
    DuplicateSliceName { path: String, name: String },

    /// Invalid discriminator path.
    #[error("Invalid discriminator path: {path}")]
    InvalidDiscriminatorPath { path: String },

    /// Extension not found.
    #[error("Extension not found: {url} at {path}")]
    ExtensionNotFound { path: String, url: String },

    /// Invalid extension context.
    #[error("Extension {url} cannot be used at {path}: {reason}")]
    InvalidExtensionContext {
        url: String,
        path: String,
        reason: String,
    },

    /// Invariant key already exists.
    #[error("Invariant key already exists: {key}")]
    DuplicateInvariantKey { key: String },

    /// Invariant not found.
    #[error("Invariant not found: {key}")]
    InvariantNotFound { key: String },

    /// Invalid FHIRPath expression.
    #[error("Invalid FHIRPath expression: {expression} - {reason}")]
    InvalidFhirPathExpression { expression: String, reason: String },

    /// Invalid fixed/pattern value type.
    #[error("Value type mismatch: expected {expected}, got {actual}")]
    ValueTypeMismatch { expected: String, actual: String },

    /// Document is read-only (non-draft status).
    #[error("Document is read-only (status: {status})")]
    DocumentReadOnly { status: String },

    /// Operation cannot be undone (no previous state).
    #[error("Operation cannot be undone: no previous state recorded")]
    CannotUndo,

    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl OperationError {
    /// Create an element not found error.
    pub fn element_not_found(path: impl Into<String>) -> Self {
        Self::ElementNotFound { path: path.into() }
    }

    /// Create a slice not found error.
    pub fn slice_not_found(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self::SliceNotFound {
            path: path.into(),
            name: name.into(),
        }
    }

    /// Create an invalid cardinality error.
    pub fn invalid_cardinality(min: u32, max: u32) -> Self {
        Self::InvalidCardinality { min, max }
    }

    /// Create an internal error.
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
