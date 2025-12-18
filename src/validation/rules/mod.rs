//! Validation Rules
//!
//! Individual validation rules organized by category.
//!
//! Each rule module provides:
//! - A set of validation functions
//! - Error codes and messages
//! - Quick fix suggestions where applicable

pub mod binding;
pub mod cardinality;
pub mod fhirpath;
pub mod metadata;
pub mod slicing;
pub mod type_refinement;

use crate::ir::{ElementNode, ProfileDocument};
use crate::validation::diagnostic::{Diagnostic, ValidationResult, ValidationLevel};

/// Trait for validation rules that can be applied to a profile document.
pub trait ValidationRule: Send + Sync {
    /// The name of this rule for logging/debugging.
    fn name(&self) -> &'static str;

    /// Validate the entire profile document.
    fn validate_document(&self, document: &ProfileDocument) -> Vec<Diagnostic>;

    /// Validate a single element (for incremental validation).
    fn validate_element(&self, element: &ElementNode, document: &ProfileDocument) -> Vec<Diagnostic>;
}

/// Validate a document with all structural rules.
pub fn validate_structural(document: &ProfileDocument) -> ValidationResult {
    let mut diagnostics = Vec::new();

    // Run all structural validation rules
    diagnostics.extend(metadata::validate_metadata(document));
    diagnostics.extend(cardinality::validate_cardinality_tree(&document.resource.root));
    diagnostics.extend(type_refinement::validate_type_tree(&document.resource.root));
    diagnostics.extend(slicing::validate_slicing_tree(&document.resource.root));
    diagnostics.extend(binding::validate_binding_tree(&document.resource.root));

    ValidationResult::with_diagnostics(diagnostics, ValidationLevel::Structural)
}

/// Validate a single element with structural rules.
pub fn validate_element_structural(
    element: &ElementNode,
    _document: &ProfileDocument,
) -> ValidationResult {
    let mut diagnostics = Vec::new();

    diagnostics.extend(cardinality::validate_element_cardinality(element));
    diagnostics.extend(type_refinement::validate_element_types(element));
    diagnostics.extend(slicing::validate_element_slicing(element));
    diagnostics.extend(binding::validate_element_binding(element));

    ValidationResult::with_diagnostics(diagnostics, ValidationLevel::Structural)
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

    #[test]
    fn test_validate_structural_empty_document() {
        let doc = create_test_document();
        let result = validate_structural(&doc);
        // Empty document should be valid (no constraints to violate)
        assert!(result.is_valid || result.warning_count() > 0);
    }
}
