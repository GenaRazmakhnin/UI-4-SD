//! Operation trait definition.

use crate::ir::{Change, ProfileDocument};

use super::error::OperationResult;

/// Context for operation execution.
///
/// Provides additional information that may be needed during
/// operation validation or application.
#[derive(Debug, Clone, Default)]
pub struct OperationContext {
    /// Whether to validate against base definition constraints.
    pub validate_against_base: bool,

    /// Whether to allow weakening of constraints.
    pub allow_constraint_weakening: bool,

    /// Whether this is a dry-run (validate only, don't apply).
    pub dry_run: bool,
}

impl OperationContext {
    /// Create a new context with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable base validation.
    pub fn with_base_validation(mut self) -> Self {
        self.validate_against_base = true;
        self
    }

    /// Allow constraint weakening.
    pub fn allow_weakening(mut self) -> Self {
        self.allow_constraint_weakening = true;
        self
    }

    /// Set as dry-run.
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }
}

/// A reversible operation that can be applied to a profile document.
///
/// Operations follow the Command pattern:
/// - `validate` checks if the operation can be applied
/// - `apply` performs the operation
/// - `undo` reverses the operation
/// - `description` provides a human-readable description
///
/// # Implementation Notes
///
/// Operations must:
/// - Be atomic (all-or-nothing)
/// - Record state needed for undo
/// - Not modify document during validation
/// - Provide clear error messages
pub trait Operation: Send + Sync {
    /// Validate that this operation can be applied to the document.
    ///
    /// This method must not modify the document. It should check:
    /// - Target element exists
    /// - Operation is valid for the element type
    /// - Constraints are satisfied
    ///
    /// # Errors
    ///
    /// Returns an error if the operation cannot be applied.
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()>;

    /// Apply this operation to the document.
    ///
    /// This method modifies the document to reflect the operation.
    /// It should only be called after `validate` succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails (should be rare
    /// if validation passed).
    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()>;

    /// Undo this operation, restoring the previous state.
    ///
    /// This method reverses the changes made by `apply`.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation cannot be undone.
    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()>;

    /// Get a human-readable description of this operation.
    ///
    /// This is used for undo/redo history display.
    fn description(&self) -> String;

    /// Convert this operation to a Change for history tracking.
    fn as_change(&self) -> Change;
}

/// Macro for implementing common operation boilerplate.
#[macro_export]
macro_rules! impl_operation_common {
    ($type:ty, $desc:expr) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.description())
            }
        }
    };
}
