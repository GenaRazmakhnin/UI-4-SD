//! FSH Export Module
//!
//! Provides functionality to export IR ProfileDocuments to FSH format.
//! Uses maki-decompiler for SD → FSH conversion.

use tracing::{debug, info};

use crate::decompiler::{decompile_sd_to_fsh, DecompilerError};
use crate::export::StructureDefinitionExporter;
use crate::ir::ProfileDocument;

use super::error::{FshError, FshResult, FshResultWithWarnings, FshWarning, FshWarningCode};

/// Options for FSH export.
#[derive(Debug, Clone)]
pub struct FshExportOptions {
    /// Whether to include comments in output.
    pub include_comments: bool,
    /// Whether to use aliases for common URLs.
    pub use_aliases: bool,
    /// Line ending style.
    pub line_ending: LineEnding,
    /// Indentation style.
    pub indent: IndentStyle,
}

impl Default for FshExportOptions {
    fn default() -> Self {
        Self {
            include_comments: true,
            use_aliases: true,
            line_ending: LineEnding::Lf,
            indent: IndentStyle::Spaces(2),
        }
    }
}

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// Unix-style (LF).
    Lf,
    /// Windows-style (CRLF).
    CrLf,
}

/// Indentation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentStyle {
    /// Use spaces.
    Spaces(usize),
    /// Use tabs.
    Tabs,
}

/// FSH exporter.
///
/// Exports IR ProfileDocuments to FSH format using the strategy:
/// IR → StructureDefinition → FSH (via maki-decompiler)
pub struct FshExporter {
    /// SD exporter for IR → SD conversion.
    sd_exporter: StructureDefinitionExporter,
    /// Export options.
    options: FshExportOptions,
}

impl FshExporter {
    /// Create a new exporter with default options.
    pub async fn new() -> FshResult<Self> {
        Self::with_options(FshExportOptions::default()).await
    }

    /// Create a new exporter with custom options.
    pub async fn with_options(options: FshExportOptions) -> FshResult<Self> {
        Ok(Self {
            sd_exporter: StructureDefinitionExporter::new(),
            options,
        })
    }

    /// Export a ProfileDocument to FSH.
    pub async fn export(&mut self, document: &ProfileDocument) -> FshResult<String> {
        let result = self.export_with_warnings(document).await?;
        Ok(result.value)
    }

    /// Export a ProfileDocument to FSH with warnings.
    pub async fn export_with_warnings(
        &mut self,
        document: &ProfileDocument,
    ) -> FshResult<FshResultWithWarnings<String>> {
        let mut warnings = Vec::new();

        info!("Exporting profile '{}' to FSH", document.metadata.name);

        // Step 1: Export IR to StructureDefinition JSON
        debug!("Converting IR to StructureDefinition");
        let sd_json = self
            .sd_exporter
            .export(document)
            .await
            .map_err(|e| FshError::Export(e.to_string()))?;

        // Step 2: Decompile SD to FSH using maki-decompiler
        debug!("Decompiling StructureDefinition to FSH");
        let fsh = decompile_sd_to_fsh(&sd_json, document.resource.fhir_version)
            .await
            .map_err(|e| match e {
                DecompilerError::InitFailed(msg) => {
                    warnings.push(FshWarning::new(
                        FshWarningCode::PotentialDataLoss,
                        format!("Decompiler initialization warning: {}", msg),
                    ));
                    // Return a basic FSH representation as fallback
                    FshError::Decompiler(DecompilerError::InitFailed(msg))
                }
                other => FshError::Decompiler(other),
            })?;

        // Step 3: Post-process FSH (apply formatting options)
        let formatted_fsh = self.post_process_fsh(&fsh);

        info!(
            "Successfully exported '{}' to FSH ({} chars)",
            document.metadata.name,
            formatted_fsh.len()
        );

        Ok(FshResultWithWarnings::with_warnings(formatted_fsh, warnings))
    }

    /// Export multiple documents to a combined FSH file.
    pub async fn export_multiple(
        &mut self,
        documents: &[ProfileDocument],
    ) -> FshResult<FshResultWithWarnings<String>> {
        let mut all_fsh = Vec::new();
        let mut all_warnings = Vec::new();

        for doc in documents {
            match self.export_with_warnings(doc).await {
                Ok(result) => {
                    all_fsh.push(result.value);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    all_warnings.push(FshWarning::new(
                        FshWarningCode::PotentialDataLoss,
                        format!("Failed to export '{}': {}", doc.metadata.name, e),
                    ));
                }
            }
        }

        let combined = all_fsh.join(&self.section_separator());

        Ok(FshResultWithWarnings::with_warnings(combined, all_warnings))
    }

    /// Post-process FSH output to apply formatting options.
    fn post_process_fsh(&self, fsh: &str) -> String {
        let mut result = fsh.to_string();

        // Apply line ending style
        result = match self.options.line_ending {
            LineEnding::Lf => result.replace("\r\n", "\n"),
            LineEnding::CrLf => {
                let normalized = result.replace("\r\n", "\n");
                normalized.replace('\n', "\r\n")
            }
        };

        // Ensure trailing newline
        if !result.ends_with('\n') {
            result.push('\n');
        }

        result
    }

    /// Get the section separator for multiple profiles.
    fn section_separator(&self) -> String {
        match self.options.line_ending {
            LineEnding::Lf => "\n\n".to_string(),
            LineEnding::CrLf => "\r\n\r\n".to_string(),
        }
    }
}

/// Generate a minimal FSH representation directly from IR.
///
/// This is a fallback when the decompiler is not available.
/// It produces valid but basic FSH without the full optimization
/// that maki-decompiler provides.
pub fn generate_basic_fsh(document: &ProfileDocument) -> String {
    let mut lines = Vec::new();

    // Profile header
    lines.push(format!("Profile: {}", document.metadata.name));

    // Parent
    if let Some(base_name) = &document.resource.base.name {
        lines.push(format!("Parent: {}", base_name));
    } else {
        lines.push(format!("Parent: {}", document.resource.base.url));
    }

    // Id
    lines.push(format!("Id: {}", document.metadata.id));

    // Title
    if let Some(title) = &document.metadata.title {
        lines.push(format!("Title: \"{}\"", escape_fsh_string(title)));
    }

    // Description
    if let Some(desc) = &document.metadata.description {
        lines.push(format!("Description: \"{}\"", escape_fsh_string(desc)));
    }

    // Empty line before rules
    lines.push(String::new());

    // Generate rules from element tree
    generate_element_rules(&document.resource.root, &mut lines);

    lines.join("\n")
}

/// Generate FSH rules from an element node.
fn generate_element_rules(node: &crate::ir::ElementNode, lines: &mut Vec<String>) {
    // Skip root element
    if node.depth() == 0 {
        for child in &node.children {
            generate_element_rules(child, lines);
        }
        return;
    }

    // Only generate rules for modified elements
    if !node.is_modified() {
        return;
    }

    let path = &node.path;
    let mut rule_parts = Vec::new();

    // Cardinality
    if let Some(card) = &node.constraints.cardinality {
        rule_parts.push(card.to_fhir_string());
    }

    // Flags
    if node.constraints.flags.must_support {
        rule_parts.push("MS".to_string());
    }
    if node.constraints.flags.is_summary {
        rule_parts.push("SU".to_string());
    }
    if node.constraints.flags.is_modifier {
        rule_parts.push("?!".to_string());
    }

    // Generate rule line
    if !rule_parts.is_empty() {
        lines.push(format!("* {} {}", path, rule_parts.join(" ")));
    }

    // Type constraints
    if !node.constraints.types.is_empty() {
        let types: Vec<_> = node
            .constraints
            .types
            .iter()
            .map(|t| {
                if t.target_profile.is_empty() {
                    t.code.clone()
                } else {
                    format!(
                        "Reference({})",
                        t.target_profile
                            .iter()
                            .map(|p| extract_resource_name(p))
                            .collect::<Vec<_>>()
                            .join(" or ")
                    )
                }
            })
            .collect();

        if types.len() == 1 {
            lines.push(format!("* {} only {}", path, types[0]));
        } else {
            lines.push(format!("* {} only {}", path, types.join(" or ")));
        }
    }

    // Binding
    if let Some(binding) = &node.constraints.binding {
        lines.push(format!(
            "* {} from {} ({})",
            path, binding.value_set, binding.strength
        ));
    }

    // Fixed value
    if let Some(fixed) = &node.constraints.fixed_value {
        match fixed {
            crate::ir::FixedValue::Fixed(v) => {
                lines.push(format!("* {} = {}", path, format_fsh_value(v)));
            }
            crate::ir::FixedValue::Pattern(v) => {
                lines.push(format!("* {} = {}", path, format_fsh_value(v)));
            }
        }
    }

    // Short description
    if let Some(short) = &node.constraints.short {
        lines.push(format!("* {} ^short = \"{}\"", path, escape_fsh_string(short)));
    }

    // Recurse to children
    for child in &node.children {
        generate_element_rules(child, lines);
    }
}

/// Extract resource name from a URL.
fn extract_resource_name(url: &str) -> String {
    url.rsplit('/').next().unwrap_or(url).to_string()
}

/// Escape a string for FSH.
fn escape_fsh_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Format a JSON value for FSH.
fn format_fsh_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => format!("\"{}\"", escape_fsh_string(s)),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            // For complex values, use JSON syntax
            serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, DocumentMetadata, FhirVersion, ProfiledResource};

    fn create_test_document() -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            "test-patient",
            "http://example.org/fhir/StructureDefinition/TestPatient",
            "TestPatient",
        )
        .with_title("Test Patient Profile")
        .with_description("A test profile");

        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        ProfileDocument::new(metadata, resource)
    }

    #[test]
    fn test_generate_basic_fsh() {
        let doc = create_test_document();
        let fsh = generate_basic_fsh(&doc);

        assert!(fsh.contains("Profile: TestPatient"));
        assert!(fsh.contains("Parent: Patient"));
        assert!(fsh.contains("Id: test-patient"));
        assert!(fsh.contains("Title: \"Test Patient Profile\""));
    }

    #[test]
    fn test_escape_fsh_string() {
        assert_eq!(escape_fsh_string("hello"), "hello");
        assert_eq!(escape_fsh_string("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(escape_fsh_string("line1\nline2"), "line1\\nline2");
    }
}
