//! Data Transfer Objects for the Export API.
//!
//! Defines request and response types for export-related endpoints.

use serde::{Deserialize, Serialize};

use super::dto::{Diagnostic, DiagnosticSeverity};

// === Query Parameters ===

/// Query parameters for SD export endpoint.
#[derive(Debug, Deserialize)]
pub struct SdExportQuery {
    /// Export format: "differential", "snapshot", or "both" (default)
    #[serde(default = "default_sd_format")]
    pub format: SdExportFormat,
    /// Pretty print JSON (default: false for deterministic output)
    #[serde(default)]
    pub pretty: bool,
    /// Persist exported file to SD/ directory (default: false)
    #[serde(default)]
    pub persist: bool,
    /// Force export even with validation warnings (default: false)
    #[serde(default)]
    pub force: bool,
}

fn default_sd_format() -> SdExportFormat {
    SdExportFormat::Both
}

/// SD export format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SdExportFormat {
    /// Export only differential
    Differential,
    /// Export only snapshot
    Snapshot,
    /// Export both differential and snapshot (default)
    Both,
}

/// Query parameters for FSH export endpoint.
#[derive(Debug, Deserialize)]
pub struct FshExportQuery {
    /// Persist exported file to FSH/ directory (default: false)
    #[serde(default)]
    pub persist: bool,
    /// Force export even with validation warnings (default: false)
    #[serde(default)]
    pub force: bool,
}

/// Query parameters for bulk export endpoint.
#[derive(Debug, Deserialize)]
pub struct BulkExportQuery {
    /// Export format: "sd", "fsh", or "both"
    #[serde(default = "default_bulk_format")]
    pub format: BulkExportFormat,
    /// Structure: "flat" for individual files, "packaged" for tarball
    #[serde(default = "default_structure")]
    pub structure: ExportStructure,
    /// Pretty print JSON (for SD exports)
    #[serde(default)]
    pub pretty: bool,
}

fn default_bulk_format() -> BulkExportFormat {
    BulkExportFormat::Both
}

fn default_structure() -> ExportStructure {
    ExportStructure::Flat
}

/// Bulk export format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BulkExportFormat {
    /// Export as StructureDefinition JSON only
    Sd,
    /// Export as FSH only
    Fsh,
    /// Export as FHIR Schema only
    FhirSchema,
    /// Export all formats
    Both,
}

/// Export structure options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportStructure {
    /// Return individual files (JSON response or ZIP)
    Flat,
    /// Return as packaged tarball with IG scaffold
    Packaged,
}

/// Query parameters for preview endpoint.
#[derive(Debug, Deserialize)]
pub struct PreviewQuery {
    /// Preview format: "sd" or "fsh"
    #[serde(default = "default_preview_format")]
    pub format: PreviewFormat,
    /// Include syntax highlighting metadata
    #[serde(default = "default_highlight")]
    pub highlight: bool,
}

fn default_preview_format() -> PreviewFormat {
    PreviewFormat::Sd
}

fn default_highlight() -> bool {
    true
}

/// Preview format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewFormat {
    /// Preview as StructureDefinition JSON
    Sd,
    /// Preview as FSH
    Fsh,
    /// Preview as FHIR Schema
    FhirSchema,
}

// === Response Types ===

/// Response for single resource SD export.
#[derive(Debug, Serialize)]
pub struct SdExportResponse {
    /// The exported StructureDefinition JSON
    pub data: serde_json::Value,
    /// Export metadata
    pub metadata: ExportMetadata,
    /// Validation diagnostics (if any)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}

/// Response for single resource FSH export.
#[derive(Debug, Serialize)]
pub struct FshExportResponse {
    /// The exported FSH content
    pub data: String,
    /// Export metadata
    pub metadata: ExportMetadata,
    /// Validation diagnostics (if any)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}

/// Metadata about the export operation.
#[derive(Debug, Serialize)]
pub struct ExportMetadata {
    /// Resource ID
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Resource name
    pub name: String,
    /// Canonical URL
    pub url: String,
    /// FHIR version
    #[serde(rename = "fhirVersion")]
    pub fhir_version: String,
    /// Suggested filename for download
    pub filename: String,
    /// MIME type
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// Content hash for ETag (SHA-256 hex)
    pub etag: String,
    /// File path if persisted
    #[serde(rename = "persistedPath", skip_serializing_if = "Option::is_none")]
    pub persisted_path: Option<String>,
}

/// Response for preview endpoint.
#[derive(Debug, Serialize)]
pub struct PreviewResponse {
    /// The formatted content
    pub content: String,
    /// Format of the preview
    pub format: PreviewFormat,
    /// Syntax highlighting tokens (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighting: Option<SyntaxHighlighting>,
    /// Validation diagnostics
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}

/// Syntax highlighting metadata for preview.
#[derive(Debug, Serialize)]
pub struct SyntaxHighlighting {
    /// Language identifier
    pub language: String,
    /// Token ranges for highlighting
    pub tokens: Vec<HighlightToken>,
}

/// A syntax highlighting token.
#[derive(Debug, Serialize)]
pub struct HighlightToken {
    /// Start line (0-based)
    pub line: u32,
    /// Start column (0-based)
    #[serde(rename = "startColumn")]
    pub start_column: u32,
    /// End column (0-based)
    #[serde(rename = "endColumn")]
    pub end_column: u32,
    /// Token type (keyword, string, number, etc.)
    #[serde(rename = "tokenType")]
    pub token_type: String,
}

/// Response for bulk export endpoint (flat structure).
#[derive(Debug, Serialize)]
pub struct BulkExportResponse {
    /// Project ID
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Exported files
    pub files: Vec<ExportedFile>,
    /// Export summary
    pub summary: ExportSummary,
    /// Validation diagnostics across all resources
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<ResourceDiagnostic>,
}

/// A single exported file in bulk export.
#[derive(Debug, Serialize)]
pub struct ExportedFile {
    /// Resource ID
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Resource name
    pub name: String,
    /// Relative path in the export structure
    pub path: String,
    /// Format of this file
    pub format: String,
    /// Content (base64 encoded for binary, plain for text)
    pub content: String,
    /// Whether content is base64 encoded
    #[serde(rename = "isBase64")]
    pub is_base64: bool,
}

/// Summary of bulk export operation.
#[derive(Debug, Serialize)]
pub struct ExportSummary {
    /// Total resources processed
    #[serde(rename = "totalResources")]
    pub total_resources: u32,
    /// Successfully exported count
    #[serde(rename = "successCount")]
    pub success_count: u32,
    /// Failed export count
    #[serde(rename = "failedCount")]
    pub failed_count: u32,
    /// Skipped count (due to errors)
    #[serde(rename = "skippedCount")]
    pub skipped_count: u32,
    /// Formats included
    pub formats: Vec<String>,
}

/// Diagnostic tied to a specific resource.
#[derive(Debug, Serialize)]
pub struct ResourceDiagnostic {
    /// Resource ID
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Resource name
    pub name: String,
    /// Diagnostics for this resource
    pub diagnostics: Vec<Diagnostic>,
}

// === Validation Result ===

/// Pre-export validation result.
#[derive(Debug, Serialize)]
pub struct ValidationResult {
    /// Whether the profile is valid for export
    pub valid: bool,
    /// Error count (prevents export)
    #[serde(rename = "errorCount")]
    pub error_count: u32,
    /// Warning count (can be forced)
    #[serde(rename = "warningCount")]
    pub warning_count: u32,
    /// All diagnostics
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationResult {
    /// Create a valid result with no issues.
    pub fn valid() -> Self {
        Self {
            valid: true,
            error_count: 0,
            warning_count: 0,
            diagnostics: Vec::new(),
        }
    }

    /// Create from a list of diagnostics.
    pub fn from_diagnostics(diagnostics: Vec<Diagnostic>) -> Self {
        let error_count = diagnostics
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Error)
            .count() as u32;
        let warning_count = diagnostics
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Warning)
            .count() as u32;

        Self {
            valid: error_count == 0,
            error_count,
            warning_count,
            diagnostics,
        }
    }

    /// Check if export can proceed (valid or only warnings with force).
    pub fn can_export(&self, force: bool) -> bool {
        self.error_count == 0 && (self.warning_count == 0 || force)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_valid() {
        let result = ValidationResult::valid();
        assert!(result.valid);
        assert!(result.can_export(false));
    }

    #[test]
    fn test_validation_result_with_warnings() {
        let diagnostics = vec![Diagnostic {
            severity: DiagnosticSeverity::Warning,
            code: "TEST".to_string(),
            message: "Test warning".to_string(),
            path: None,
        }];
        let result = ValidationResult::from_diagnostics(diagnostics);

        assert!(result.valid);
        assert!(!result.can_export(false));
        assert!(result.can_export(true));
    }

    #[test]
    fn test_validation_result_with_errors() {
        let diagnostics = vec![Diagnostic {
            severity: DiagnosticSeverity::Error,
            code: "TEST".to_string(),
            message: "Test error".to_string(),
            path: None,
        }];
        let result = ValidationResult::from_diagnostics(diagnostics);

        assert!(!result.valid);
        assert!(!result.can_export(false));
        assert!(!result.can_export(true));
    }
}
