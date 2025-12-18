//! Validation Diagnostics
//!
//! Types for representing validation results, errors, warnings, and suggestions.

use serde::{Deserialize, Serialize};

use super::quick_fix::QuickFix;

/// Validation result containing all diagnostics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationResult {
    /// All diagnostics (errors, warnings, info).
    pub diagnostics: Vec<Diagnostic>,
    /// Whether the profile is valid (no errors).
    pub is_valid: bool,
    /// The validation level that was performed.
    pub validation_level: ValidationLevel,
    /// Timestamp of validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ValidationResult {
    /// Create an empty valid result.
    pub fn valid(level: ValidationLevel) -> Self {
        Self {
            diagnostics: Vec::new(),
            is_valid: true,
            validation_level: level,
            validated_at: Some(chrono::Utc::now()),
        }
    }

    /// Create a result with diagnostics.
    pub fn with_diagnostics(diagnostics: Vec<Diagnostic>, level: ValidationLevel) -> Self {
        let is_valid = !diagnostics
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error);

        Self {
            diagnostics,
            is_valid,
            validation_level: level,
            validated_at: Some(chrono::Utc::now()),
        }
    }

    /// Merge another result into this one.
    pub fn merge(&mut self, other: ValidationResult) {
        self.diagnostics.extend(other.diagnostics);
        self.is_valid = self.is_valid && other.is_valid;
        self.validation_level = self.validation_level.max(other.validation_level);
    }

    /// Get error count.
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Error)
            .count()
    }

    /// Get warning count.
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Warning)
            .count()
    }

    /// Get diagnostics for a specific path.
    pub fn diagnostics_for_path(&self, path: &str) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.element_path.as_deref() == Some(path))
            .collect()
    }

    /// Check if can proceed with export (only warnings, no errors).
    pub fn can_export(&self) -> bool {
        self.is_valid
    }
}

/// Validation level indicating depth of validation performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationLevel {
    /// No validation performed.
    #[default]
    None,
    /// Structural validation only (IR layer).
    Structural,
    /// Cross-reference validation included.
    References,
    /// Terminology validation included.
    Terminology,
    /// Full validation including external validators.
    Full,
}

impl ValidationLevel {
    /// Get display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Structural => "structural",
            Self::References => "references",
            Self::Terminology => "terminology",
            Self::Full => "full",
        }
    }
}

/// A single diagnostic message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Severity of the diagnostic.
    pub severity: DiagnosticSeverity,
    /// Unique code for this type of diagnostic.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Element path where the issue occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_path: Option<String>,
    /// Source of the diagnostic.
    pub source: DiagnosticSource,
    /// Optional quick fix suggestion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_fix: Option<QuickFix>,
    /// Additional context or details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl Diagnostic {
    /// Create a new error diagnostic.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            code: code.into(),
            message: message.into(),
            element_path: None,
            source: DiagnosticSource::Ir,
            quick_fix: None,
            details: None,
        }
    }

    /// Create a new warning diagnostic.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            code: code.into(),
            message: message.into(),
            element_path: None,
            source: DiagnosticSource::Ir,
            quick_fix: None,
            details: None,
        }
    }

    /// Create a new info diagnostic.
    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Info,
            code: code.into(),
            message: message.into(),
            element_path: None,
            source: DiagnosticSource::Ir,
            quick_fix: None,
            details: None,
        }
    }

    /// Set the element path.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.element_path = Some(path.into());
        self
    }

    /// Set the source.
    pub fn with_source(mut self, source: DiagnosticSource) -> Self {
        self.source = source;
        self
    }

    /// Set a quick fix.
    pub fn with_quick_fix(mut self, fix: QuickFix) -> Self {
        self.quick_fix = Some(fix);
        self
    }

    /// Set additional details.
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Severity level of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    /// Fatal error - profile cannot be exported.
    Error,
    /// Warning - profile can be exported but has issues.
    Warning,
    /// Informational - suggestion or hint.
    Info,
}

impl std::fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
        }
    }
}

/// Source of the diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSource {
    /// IR structural validation.
    Ir,
    /// FHIRPath expression validation.
    FhirPath,
    /// Terminology validation.
    Terminology,
    /// Cross-reference validation.
    Reference,
    /// External validator (e.g., HL7 Validator).
    External,
}

impl std::fmt::Display for DiagnosticSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ir => write!(f, "IR"),
            Self::FhirPath => write!(f, "FHIRPath"),
            Self::Terminology => write!(f, "Terminology"),
            Self::Reference => write!(f, "Reference"),
            Self::External => write!(f, "External"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_valid() {
        let result = ValidationResult::valid(ValidationLevel::Structural);
        assert!(result.is_valid);
        assert_eq!(result.error_count(), 0);
    }

    #[test]
    fn test_validation_result_with_errors() {
        let diagnostics = vec![
            Diagnostic::error("TEST_ERROR", "Test error message"),
            Diagnostic::warning("TEST_WARNING", "Test warning message"),
        ];

        let result = ValidationResult::with_diagnostics(diagnostics, ValidationLevel::Structural);
        assert!(!result.is_valid);
        assert_eq!(result.error_count(), 1);
        assert_eq!(result.warning_count(), 1);
    }

    #[test]
    fn test_diagnostic_builder() {
        let diagnostic = Diagnostic::error("CARD_001", "Invalid cardinality")
            .with_path("Patient.name")
            .with_source(DiagnosticSource::Ir);

        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.code, "CARD_001");
        assert_eq!(diagnostic.element_path, Some("Patient.name".to_string()));
    }
}
