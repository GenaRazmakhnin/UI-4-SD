//! Validation support for IR types.
//!
//! This module provides validation error types and results for validating
//! profile IR structures. It supports incremental validation (only changed
//! elements) and provides clear error paths for UI display.

use serde::{Deserialize, Serialize};

use super::element::NodeId;

/// Severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ValidationSeverity {
    /// Informational message (not an issue).
    Information,
    /// Warning (valid but potentially problematic).
    Warning,
    /// Error (invalid, must be fixed).
    #[default]
    Error,
    /// Fatal error (cannot proceed).
    Fatal,
}

impl ValidationSeverity {
    /// Check if this is an error or worse.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }

    /// Get the severity level for sorting (higher = more severe).
    #[must_use]
    pub const fn level(&self) -> u8 {
        match self {
            Self::Information => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Fatal => 3,
        }
    }
}

impl std::fmt::Display for ValidationSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Information => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
        }
    }
}

/// Category of validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCategory {
    /// Structural issue (missing required field, invalid tree).
    Structure,
    /// Cardinality violation.
    Cardinality,
    /// Type constraint violation.
    Type,
    /// Terminology binding violation.
    Terminology,
    /// FHIRPath invariant violation.
    Invariant,
    /// Slicing rule violation.
    Slicing,
    /// Reference target violation.
    Reference,
    /// Value constraint violation (fixed/pattern).
    Value,
    /// General constraint violation.
    Constraint,
}

impl std::fmt::Display for ValidationCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Structure => write!(f, "structure"),
            Self::Cardinality => write!(f, "cardinality"),
            Self::Type => write!(f, "type"),
            Self::Terminology => write!(f, "terminology"),
            Self::Invariant => write!(f, "invariant"),
            Self::Slicing => write!(f, "slicing"),
            Self::Reference => write!(f, "reference"),
            Self::Value => write!(f, "value"),
            Self::Constraint => write!(f, "constraint"),
        }
    }
}

/// A validation error with location and context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Severity of the error.
    pub severity: ValidationSeverity,

    /// Category of the error.
    pub category: ValidationCategory,

    /// Element path where the error occurred.
    pub path: String,

    /// Stable node ID (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<NodeId>,

    /// Human-readable error message.
    pub message: String,

    /// Additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// Suggested fix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,

    /// Error code (for i18n/documentation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ValidationError {
    /// Create a new validation error.
    #[must_use]
    pub fn new(
        severity: ValidationSeverity,
        category: ValidationCategory,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            category,
            path: path.into(),
            node_id: None,
            message: message.into(),
            details: None,
            suggestion: None,
            code: None,
        }
    }

    /// Create an error-level validation error.
    #[must_use]
    pub fn error(
        category: ValidationCategory,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ValidationSeverity::Error, category, path, message)
    }

    /// Create a warning-level validation error.
    #[must_use]
    pub fn warning(
        category: ValidationCategory,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(ValidationSeverity::Warning, category, path, message)
    }

    /// Set the node ID.
    #[must_use]
    pub fn with_node_id(mut self, node_id: NodeId) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Set additional details.
    #[must_use]
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Set a suggested fix.
    #[must_use]
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Set an error code.
    #[must_use]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Check if this is an error or worse.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.path, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Result of validation containing errors and warnings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationResult {
    /// All validation issues found.
    pub issues: Vec<ValidationError>,
}

impl ValidationResult {
    /// Create an empty validation result.
    #[must_use]
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    /// Create a validation result with a single error.
    #[must_use]
    pub fn with_error(error: ValidationError) -> Self {
        Self {
            issues: vec![error],
        }
    }

    /// Add an issue.
    pub fn add(&mut self, error: ValidationError) {
        self.issues.push(error);
    }

    /// Add an error.
    pub fn add_error(
        &mut self,
        category: ValidationCategory,
        path: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.add(ValidationError::error(category, path, message));
    }

    /// Add a warning.
    pub fn add_warning(
        &mut self,
        category: ValidationCategory,
        path: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.add(ValidationError::warning(category, path, message));
    }

    /// Merge another result into this one.
    pub fn merge(&mut self, other: ValidationResult) {
        self.issues.extend(other.issues);
    }

    /// Check if validation passed (no errors).
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.issues.iter().any(|e| e.is_error())
    }

    /// Check if there are any issues.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Get only errors (not warnings).
    pub fn errors(&self) -> impl Iterator<Item = &ValidationError> {
        self.issues.iter().filter(|e| e.is_error())
    }

    /// Get only warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationError> {
        self.issues
            .iter()
            .filter(|e| e.severity == ValidationSeverity::Warning)
    }

    /// Count errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    /// Count warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }

    /// Get issues for a specific path.
    pub fn issues_at_path(&self, path: &str) -> impl Iterator<Item = &ValidationError> {
        self.issues.iter().filter(move |e| e.path == path)
    }

    /// Get issues for a specific node.
    pub fn issues_for_node(&self, node_id: NodeId) -> impl Iterator<Item = &ValidationError> {
        self.issues
            .iter()
            .filter(move |e| e.node_id == Some(node_id))
    }

    /// Sort issues by severity (most severe first).
    pub fn sort_by_severity(&mut self) {
        self.issues
            .sort_by(|a, b| b.severity.level().cmp(&a.severity.level()));
    }
}

/// Trait for types that can be validated.
pub trait Validate {
    /// Validate this item, returning any errors/warnings.
    fn validate(&self) -> ValidationResult;

    /// Check if this item is valid.
    fn is_valid(&self) -> bool {
        self.validate().is_valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::error(
            ValidationCategory::Cardinality,
            "Patient.name",
            "Minimum cardinality must be >= 0",
        )
        .with_suggestion("Set min to 0 or higher");

        assert!(error.is_error());
        assert_eq!(error.category, ValidationCategory::Cardinality);
        assert!(error.suggestion.is_some());
    }

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();

        result.add_error(
            ValidationCategory::Structure,
            "Patient",
            "Missing required element",
        );
        result.add_warning(
            ValidationCategory::Terminology,
            "Patient.gender",
            "Consider using preferred binding",
        );

        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1);
        assert_eq!(result.warning_count(), 1);
    }

    #[test]
    fn test_validation_severity() {
        assert!(ValidationSeverity::Error.is_error());
        assert!(ValidationSeverity::Fatal.is_error());
        assert!(!ValidationSeverity::Warning.is_error());
        assert!(!ValidationSeverity::Information.is_error());
    }

    #[test]
    fn test_result_merge() {
        let mut result1 = ValidationResult::new();
        result1.add_error(ValidationCategory::Structure, "path1", "error1");

        let mut result2 = ValidationResult::new();
        result2.add_warning(ValidationCategory::Type, "path2", "warning1");

        result1.merge(result2);

        assert_eq!(result1.issues.len(), 2);
    }
}
