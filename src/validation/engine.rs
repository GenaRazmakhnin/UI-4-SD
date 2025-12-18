//! Validation Engine
//!
//! The main validation engine that orchestrates all validation layers.

use std::sync::Arc;

use tracing::{debug, info};

use crate::ir::{ElementNode, ProfileDocument};

use super::diagnostic::{Diagnostic, DiagnosticSource, ValidationLevel, ValidationResult};
use super::rules;

/// Validation options.
#[derive(Debug, Clone)]
pub struct ValidationOptions {
    /// Maximum validation level to perform.
    pub level: ValidationLevel,
    /// Whether to include info-level diagnostics.
    pub include_info: bool,
    /// Whether to stop on first error.
    pub fail_fast: bool,
    /// Paths to validate (empty = all).
    pub paths: Vec<String>,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            level: ValidationLevel::Structural,
            include_info: true,
            fail_fast: false,
            paths: Vec::new(),
        }
    }
}

impl ValidationOptions {
    /// Set validation level.
    pub fn with_level(mut self, level: ValidationLevel) -> Self {
        self.level = level;
        self
    }

    /// Exclude info diagnostics.
    pub fn without_info(mut self) -> Self {
        self.include_info = false;
        self
    }

    /// Enable fail-fast mode.
    pub fn fail_fast(mut self) -> Self {
        self.fail_fast = true;
        self
    }

    /// Validate only specific paths.
    pub fn with_paths(mut self, paths: Vec<String>) -> Self {
        self.paths = paths;
        self
    }
}

/// Trait for validators.
pub trait Validator: Send + Sync {
    /// Validate an entire profile document.
    fn validate(&self, document: &ProfileDocument) -> ValidationResult;

    /// Validate a specific element.
    fn validate_element(&self, element: &ElementNode, document: &ProfileDocument) -> ValidationResult;

    /// Validate after incremental changes.
    fn validate_incremental(
        &self,
        document: &ProfileDocument,
        changed_paths: &[String],
    ) -> ValidationResult;
}

/// The main validation engine.
pub struct ValidationEngine {
    /// Validation options.
    options: ValidationOptions,
}

impl Default for ValidationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationEngine {
    /// Create a new validation engine with default options.
    pub fn new() -> Self {
        Self {
            options: ValidationOptions::default(),
        }
    }

    /// Create with custom options.
    pub fn with_options(options: ValidationOptions) -> Self {
        Self { options }
    }

    /// Validate a profile document.
    pub async fn validate(
        &self,
        document: &ProfileDocument,
        level: ValidationLevel,
    ) -> ValidationResult {
        info!(
            "Validating profile '{}' at level {:?}",
            document.metadata.name, level
        );

        let mut result = ValidationResult::valid(level);

        // Layer 1: Structural validation (always run)
        if level >= ValidationLevel::Structural {
            debug!("Running structural validation");
            let structural = rules::validate_structural(document);
            result.merge(structural);

            // Also validate FHIRPath expressions
            let fhirpath = rules::fhirpath::validate_fhirpath_expressions(document);
            let fhirpath_result = ValidationResult::with_diagnostics(fhirpath, ValidationLevel::Structural);
            result.merge(fhirpath_result);

            // Check for duplicate invariant keys
            let duplicate_keys = rules::fhirpath::check_duplicate_invariant_keys(document);
            let keys_result = ValidationResult::with_diagnostics(duplicate_keys, ValidationLevel::Structural);
            result.merge(keys_result);

            if self.options.fail_fast && !result.is_valid {
                return result;
            }
        }

        // Layer 2: Cross-reference validation
        if level >= ValidationLevel::References {
            debug!("Running reference validation");
            let reference_result = self.validate_references(document).await;
            result.merge(reference_result);

            if self.options.fail_fast && !result.is_valid {
                return result;
            }
        }

        // Layer 3: Terminology validation
        if level >= ValidationLevel::Terminology {
            debug!("Running terminology validation");
            let terminology_result = self.validate_terminology(document).await;
            result.merge(terminology_result);
        }

        // Filter out info if not wanted
        if !self.options.include_info {
            result.diagnostics.retain(|d| {
                d.severity != super::diagnostic::DiagnosticSeverity::Info
            });
        }

        info!(
            "Validation complete: {} errors, {} warnings",
            result.error_count(),
            result.warning_count()
        );

        result
    }

    /// Validate a single element.
    pub fn validate_element(
        &self,
        element: &ElementNode,
        document: &ProfileDocument,
    ) -> ValidationResult {
        rules::validate_element_structural(element, document)
    }

    /// Validate after incremental changes.
    pub async fn validate_incremental(
        &self,
        document: &ProfileDocument,
        changed_paths: &[String],
    ) -> ValidationResult {
        debug!(
            "Running incremental validation for {} changed paths",
            changed_paths.len()
        );

        // For now, just validate the changed elements and their dependencies
        let mut result = ValidationResult::valid(ValidationLevel::Structural);

        for path in changed_paths {
            if let Some(element) = find_element_by_path(&document.resource.root, path) {
                let element_result = self.validate_element(element, document);
                result.merge(element_result);
            }
        }

        // Also validate metadata if it might have changed
        if changed_paths.is_empty() || changed_paths.iter().any(|p| p.is_empty() || p == ".") {
            let metadata_diags = rules::metadata::validate_metadata(document);
            let metadata_result = ValidationResult::with_diagnostics(metadata_diags, ValidationLevel::Structural);
            result.merge(metadata_result);
        }

        result
    }

    /// Validate cross-references (extensions, profiles, etc.).
    async fn validate_references(&self, document: &ProfileDocument) -> ValidationResult {
        let mut diagnostics = Vec::new();

        // Validate base definition reference
        if !document.resource.base.url.is_empty() {
            // For now, just check URL format
            if !is_resolvable_url(&document.resource.base.url) {
                diagnostics.push(
                    Diagnostic::warning(
                        "REF_001",
                        format!(
                            "Base definition URL may not be resolvable: {}",
                            document.resource.base.url
                        ),
                    )
                    .with_source(DiagnosticSource::Reference),
                );
            }
        }

        // Validate type profile references
        validate_type_references(&document.resource.root, &mut diagnostics);

        ValidationResult::with_diagnostics(diagnostics, ValidationLevel::References)
    }

    /// Validate terminology (ValueSet bindings).
    async fn validate_terminology(&self, document: &ProfileDocument) -> ValidationResult {
        let mut diagnostics = Vec::new();

        // Validate ValueSet URLs in bindings
        validate_valueset_references(&document.resource.root, &mut diagnostics);

        ValidationResult::with_diagnostics(diagnostics, ValidationLevel::Terminology)
    }
}

impl Validator for ValidationEngine {
    fn validate(&self, document: &ProfileDocument) -> ValidationResult {
        // Use blocking runtime for sync interface
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(
                self.validate(document, self.options.level)
            )
        })
    }

    fn validate_element(&self, element: &ElementNode, document: &ProfileDocument) -> ValidationResult {
        ValidationEngine::validate_element(self, element, document)
    }

    fn validate_incremental(
        &self,
        document: &ProfileDocument,
        changed_paths: &[String],
    ) -> ValidationResult {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(
                ValidationEngine::validate_incremental(self, document, changed_paths)
            )
        })
    }
}

/// Find an element by path in the tree.
fn find_element_by_path<'a>(root: &'a ElementNode, path: &str) -> Option<&'a ElementNode> {
    if root.path == path {
        return Some(root);
    }

    for child in &root.children {
        if let Some(found) = find_element_by_path(child, path) {
            return Some(found);
        }
    }

    for slice in root.slices.values() {
        if let Some(found) = find_element_by_path(&slice.element, path) {
            return Some(found);
        }
    }

    None
}

/// Check if a URL is likely resolvable.
fn is_resolvable_url(url: &str) -> bool {
    // Core FHIR URLs are always resolvable
    if url.starts_with("http://hl7.org/fhir/") {
        return true;
    }

    // Standard structure URLs
    if url.contains("/StructureDefinition/") {
        return true;
    }

    // Local references (assumed resolvable)
    url.starts_with("http://") || url.starts_with("https://")
}

/// Validate type profile references in element tree.
fn validate_type_references(element: &ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    for type_constraint in &element.constraints.types {
        for profile in &type_constraint.profile {
            if !is_resolvable_url(profile) {
                diagnostics.push(
                    Diagnostic::warning(
                        "REF_002",
                        format!("Type profile URL may not be resolvable: {}", profile),
                    )
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::Reference),
                );
            }
        }

        for target in &type_constraint.target_profile {
            if !is_resolvable_url(target) {
                diagnostics.push(
                    Diagnostic::warning(
                        "REF_003",
                        format!("Target profile URL may not be resolvable: {}", target),
                    )
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::Reference),
                );
            }
        }
    }

    for child in &element.children {
        validate_type_references(child, diagnostics);
    }

    for slice in element.slices.values() {
        validate_type_references(&slice.element, diagnostics);
    }
}

/// Validate ValueSet references in element tree.
fn validate_valueset_references(element: &ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    if let Some(binding) = &element.constraints.binding {
        if !binding.value_set.is_empty() && !is_resolvable_url(&binding.value_set) {
            diagnostics.push(
                Diagnostic::warning(
                    "TERM_001",
                    format!("ValueSet URL may not be resolvable: {}", binding.value_set),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Terminology),
            );
        }
    }

    for child in &element.children {
        validate_valueset_references(child, diagnostics);
    }

    for slice in element.slices.values() {
        validate_valueset_references(&slice.element, diagnostics);
    }
}

/// Create a shared validation engine.
pub fn create_validation_engine() -> Arc<ValidationEngine> {
    Arc::new(ValidationEngine::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, DocumentMetadata, FhirVersion, ProfiledResource};

    fn create_test_document() -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            "test-profile",
            "http://example.org/fhir/StructureDefinition/TestProfile",
            "TestProfile",
        );
        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestProfile",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        ProfileDocument::new(metadata, resource)
    }

    #[tokio::test]
    async fn test_validate_empty_document() {
        let engine = ValidationEngine::new();
        let doc = create_test_document();

        let result = engine.validate(&doc, ValidationLevel::Structural).await;
        // Empty document should be mostly valid
        assert!(result.error_count() == 0);
    }

    #[tokio::test]
    async fn test_validate_with_options() {
        let options = ValidationOptions::default()
            .with_level(ValidationLevel::Full)
            .without_info();

        let engine = ValidationEngine::with_options(options);
        let doc = create_test_document();

        let result = engine.validate(&doc, ValidationLevel::Structural).await;
        // Should not include info diagnostics
        assert!(result
            .diagnostics
            .iter()
            .all(|d| d.severity != super::super::diagnostic::DiagnosticSeverity::Info));
    }

    #[test]
    fn test_find_element_by_path() {
        let mut root = ElementNode::new("Patient".to_string());
        let child = ElementNode::new("Patient.name".to_string());
        root.children.push(child);

        assert!(find_element_by_path(&root, "Patient").is_some());
        assert!(find_element_by_path(&root, "Patient.name").is_some());
        assert!(find_element_by_path(&root, "Patient.address").is_none());
    }
}
