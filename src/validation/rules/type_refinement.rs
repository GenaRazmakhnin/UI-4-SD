//! Type Refinement Validation Rules
//!
//! Validates type constraints on profile elements:
//! - Constrained types are subtypes of base
//! - Profile references are valid URLs
//! - Type cardinality matches element cardinality

use crate::ir::ElementNode;
use crate::validation::diagnostic::{Diagnostic, DiagnosticSource};
use crate::validation::quick_fix::QuickFixFactory;

/// Error codes for type validation.
pub mod codes {
    pub const TYPE_EMPTY_CONSTRAINT: &str = "TYPE_001";
    pub const TYPE_INVALID_CODE: &str = "TYPE_002";
    pub const TYPE_INVALID_PROFILE_URL: &str = "TYPE_003";
    pub const TYPE_DUPLICATE: &str = "TYPE_004";
    pub const TYPE_REFERENCE_NO_TARGET: &str = "TYPE_005";
    pub const TYPE_REFERENCE_INVALID_TARGET: &str = "TYPE_006";
}

/// Known FHIR primitive types.
const PRIMITIVE_TYPES: &[&str] = &[
    "boolean",
    "integer",
    "integer64",
    "string",
    "decimal",
    "uri",
    "url",
    "canonical",
    "base64Binary",
    "instant",
    "date",
    "dateTime",
    "time",
    "code",
    "oid",
    "id",
    "markdown",
    "unsignedInt",
    "positiveInt",
    "uuid",
    "xhtml",
];

/// Known FHIR complex types.
const COMPLEX_TYPES: &[&str] = &[
    "Address",
    "Age",
    "Annotation",
    "Attachment",
    "CodeableConcept",
    "CodeableReference",
    "Coding",
    "ContactDetail",
    "ContactPoint",
    "Contributor",
    "Count",
    "DataRequirement",
    "Distance",
    "Dosage",
    "Duration",
    "Expression",
    "Extension",
    "HumanName",
    "Identifier",
    "Meta",
    "Money",
    "Narrative",
    "ParameterDefinition",
    "Period",
    "Quantity",
    "Range",
    "Ratio",
    "RatioRange",
    "Reference",
    "RelatedArtifact",
    "SampledData",
    "Signature",
    "Timing",
    "TriggerDefinition",
    "UsageContext",
];

/// Validate type constraints for an entire element tree.
pub fn validate_type_tree(root: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_element_recursive(root, &mut diagnostics);
    diagnostics
}

/// Validate a single element's type constraints.
pub fn validate_element_types(element: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Skip if no type constraints
    if element.constraints.types.is_empty() {
        return diagnostics;
    }

    // Check for duplicate type codes
    let mut seen_types = std::collections::HashSet::new();
    for type_constraint in &element.constraints.types {
        if !seen_types.insert(&type_constraint.code) {
            diagnostics.push(
                Diagnostic::warning(
                    codes::TYPE_DUPLICATE,
                    format!("Duplicate type constraint: '{}'", type_constraint.code),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir),
            );
        }

        // Validate type code
        if !is_valid_type_code(&type_constraint.code) {
            let mut diag = Diagnostic::error(
                codes::TYPE_INVALID_CODE,
                format!("Invalid type code: '{}'", type_constraint.code),
            )
            .with_path(&element.path)
            .with_source(DiagnosticSource::Ir);

            // Suggest removal
            diag = diag.with_quick_fix(QuickFixFactory::remove_invalid_type(
                &element.path,
                &type_constraint.code,
            ));

            diagnostics.push(diag);
        }

        // Validate profile URLs
        for profile in &type_constraint.profile {
            if !is_valid_canonical_url(profile) {
                diagnostics.push(
                    Diagnostic::error(
                        codes::TYPE_INVALID_PROFILE_URL,
                        format!("Invalid profile URL: '{}'", profile),
                    )
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::Ir),
                );
            }
        }

        // Validate Reference target profiles
        if type_constraint.code == "Reference" {
            if type_constraint.target_profile.is_empty() {
                diagnostics.push(
                    Diagnostic::info(
                        codes::TYPE_REFERENCE_NO_TARGET,
                        "Reference type has no target profile (allows any resource)",
                    )
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::Ir),
                );
            } else {
                for target in &type_constraint.target_profile {
                    if !is_valid_canonical_url(target) {
                        diagnostics.push(
                            Diagnostic::error(
                                codes::TYPE_REFERENCE_INVALID_TARGET,
                                format!("Invalid target profile URL: '{}'", target),
                            )
                            .with_path(&element.path)
                            .with_source(DiagnosticSource::Ir),
                        );
                    }
                }
            }
        }
    }

    diagnostics
}

/// Recursively validate types in element tree.
fn validate_element_recursive(element: &ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    diagnostics.extend(validate_element_types(element));

    for child in &element.children {
        validate_element_recursive(child, diagnostics);
    }

    for slice in element.slices.values() {
        validate_element_recursive(&slice.element, diagnostics);
    }
}

/// Check if a type code is valid.
pub fn is_valid_type_code(code: &str) -> bool {
    // Check primitives
    if PRIMITIVE_TYPES.contains(&code) {
        return true;
    }

    // Check complex types
    if COMPLEX_TYPES.contains(&code) {
        return true;
    }

    // Check if it's a resource type (starts with uppercase, alphanumeric)
    if code.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
        return code.chars().all(|c| c.is_ascii_alphanumeric());
    }

    false
}

/// Check if a URL is a valid canonical URL.
pub fn is_valid_canonical_url(url: &str) -> bool {
    // Basic URL validation
    if url.is_empty() {
        return false;
    }

    // Must start with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        // Allow urn: URIs
        if !url.starts_with("urn:") {
            return false;
        }
    }

    // Should not contain spaces or control characters
    if url.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return false;
    }

    true
}

/// Check if one type is a valid subtype of another.
pub fn is_subtype_of(subtype: &str, supertype: &str) -> bool {
    // Same type is always valid
    if subtype == supertype {
        return true;
    }

    // Element is supertype of all types
    if supertype == "Element" {
        return true;
    }

    // Resource is supertype of all resources
    if supertype == "Resource" && is_resource_type(subtype) {
        return true;
    }

    // DomainResource is supertype of domain resources
    if supertype == "DomainResource" && is_domain_resource(subtype) {
        return true;
    }

    // Quantity subtypes
    if supertype == "Quantity" {
        return matches!(subtype, "Age" | "Count" | "Distance" | "Duration" | "Money");
    }

    false
}

/// Check if a type is a FHIR resource type.
fn is_resource_type(type_code: &str) -> bool {
    // Resource types start with uppercase and are not complex types
    type_code
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_uppercase())
        && !COMPLEX_TYPES.contains(&type_code)
}

/// Check if a type is a DomainResource.
fn is_domain_resource(type_code: &str) -> bool {
    // Most resources are DomainResources except a few
    is_resource_type(type_code)
        && !matches!(
            type_code,
            "Resource" | "Bundle" | "Binary" | "Parameters"
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::TypeConstraint;

    #[test]
    fn test_valid_primitive_types() {
        assert!(is_valid_type_code("string"));
        assert!(is_valid_type_code("boolean"));
        assert!(is_valid_type_code("dateTime"));
    }

    #[test]
    fn test_valid_complex_types() {
        assert!(is_valid_type_code("CodeableConcept"));
        assert!(is_valid_type_code("Reference"));
        assert!(is_valid_type_code("Identifier"));
    }

    #[test]
    fn test_valid_resource_types() {
        assert!(is_valid_type_code("Patient"));
        assert!(is_valid_type_code("Observation"));
        assert!(is_valid_type_code("MedicationRequest"));
    }

    #[test]
    fn test_invalid_types() {
        assert!(!is_valid_type_code(""));
        assert!(!is_valid_type_code("invalid-type"));
        assert!(!is_valid_type_code("123"));
    }

    #[test]
    fn test_valid_canonical_urls() {
        assert!(is_valid_canonical_url(
            "http://hl7.org/fhir/StructureDefinition/Patient"
        ));
        assert!(is_valid_canonical_url("https://example.org/fhir/Profile"));
        assert!(is_valid_canonical_url("urn:oid:2.16.840.1.113883"));
    }

    #[test]
    fn test_invalid_canonical_urls() {
        assert!(!is_valid_canonical_url(""));
        assert!(!is_valid_canonical_url("not a url"));
        assert!(!is_valid_canonical_url("ftp://example.org"));
    }

    #[test]
    fn test_subtype_relationships() {
        assert!(is_subtype_of("Patient", "Resource"));
        assert!(is_subtype_of("Age", "Quantity"));
        assert!(is_subtype_of("string", "Element"));
        assert!(!is_subtype_of("string", "Quantity"));
    }

    #[test]
    fn test_duplicate_types() {
        let mut element = ElementNode::new("Patient.extension".to_string());
        element.constraints.types = vec![
            TypeConstraint::simple("Extension"),
            TypeConstraint::simple("Extension"),
        ];

        let diagnostics = validate_element_types(&element);
        assert!(!diagnostics.is_empty());
        assert!(diagnostics.iter().any(|d| d.code == codes::TYPE_DUPLICATE));
    }
}
