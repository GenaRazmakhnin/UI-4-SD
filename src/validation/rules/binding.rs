//! Binding Validation Rules
//!
//! Validates value set bindings on profile elements:
//! - Binding strength is valid (required/extensible/preferred/example)
//! - ValueSet URL is valid format
//! - Binding strength cannot be weakened from base

use crate::ir::{BindingStrength, ElementNode};
use crate::validation::diagnostic::{Diagnostic, DiagnosticSource};
use crate::validation::quick_fix::QuickFixFactory;

/// Error codes for binding validation.
pub mod codes {
    pub const BINDING_EMPTY_VALUESET: &str = "BIND_001";
    pub const BINDING_INVALID_VALUESET_URL: &str = "BIND_002";
    pub const BINDING_STRENGTH_WEAKENED: &str = "BIND_003";
    pub const BINDING_ON_NON_CODEABLE: &str = "BIND_004";
    pub const BINDING_REQUIRED_NO_CODES: &str = "BIND_005";
}

/// Types that can have bindings.
const BINDABLE_TYPES: &[&str] = &[
    "code",
    "Coding",
    "CodeableConcept",
    "CodeableReference",
    "Quantity",
    "string",
    "uri",
];

/// Validate bindings for an entire element tree.
pub fn validate_binding_tree(root: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_element_recursive(root, &mut diagnostics);
    diagnostics
}

/// Validate a single element's binding.
pub fn validate_element_binding(element: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let Some(binding) = &element.constraints.binding {
        // Check for empty ValueSet URL
        if binding.value_set.is_empty() {
            let mut diag = Diagnostic::error(
                codes::BINDING_EMPTY_VALUESET,
                "Binding has no ValueSet URL specified",
            )
            .with_path(&element.path)
            .with_source(DiagnosticSource::Ir);

            diag = diag.with_quick_fix(QuickFixFactory::remove_binding(&element.path));
            diagnostics.push(diag);
        } else if !is_valid_valueset_url(&binding.value_set) {
            diagnostics.push(
                Diagnostic::error(
                    codes::BINDING_INVALID_VALUESET_URL,
                    format!("Invalid ValueSet URL: '{}'", binding.value_set),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir),
            );
        }

        // Check if binding is on a bindable type
        let has_bindable_type = element.constraints.types.is_empty()
            || element
                .constraints
                .types
                .iter()
                .any(|t| is_bindable_type(&t.code));

        if !has_bindable_type && !element.constraints.types.is_empty() {
            let type_codes: Vec<_> = element
                .constraints
                .types
                .iter()
                .map(|t| t.code.as_str())
                .collect();

            diagnostics.push(
                Diagnostic::warning(
                    codes::BINDING_ON_NON_CODEABLE,
                    format!(
                        "Binding on element with non-codeable type(s): {}",
                        type_codes.join(", ")
                    ),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir),
            );
        }
    }

    diagnostics
}

/// Recursively validate bindings in element tree.
fn validate_element_recursive(element: &ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    diagnostics.extend(validate_element_binding(element));

    for child in &element.children {
        validate_element_recursive(child, diagnostics);
    }

    for slice in element.slices.values() {
        validate_element_recursive(&slice.element, diagnostics);
    }
}

/// Check if a ValueSet URL is valid.
pub fn is_valid_valueset_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }

    // Must start with http://, https://, or urn:
    if !url.starts_with("http://")
        && !url.starts_with("https://")
        && !url.starts_with("urn:")
    {
        return false;
    }

    // Should not contain spaces or control characters
    if url.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return false;
    }

    true
}

/// Check if a type can have a binding.
pub fn is_bindable_type(type_code: &str) -> bool {
    BINDABLE_TYPES.contains(&type_code)
}

/// Check if binding strength can be refined from base to derived.
pub fn can_refine_binding_strength(base: BindingStrength, derived: BindingStrength) -> bool {
    // Strength order: required > extensible > preferred > example
    let base_rank = strength_rank(base);
    let derived_rank = strength_rank(derived);

    // Derived must be >= base (can only strengthen, not weaken)
    derived_rank >= base_rank
}

/// Get numeric rank for binding strength (higher = stronger).
fn strength_rank(strength: BindingStrength) -> u8 {
    match strength {
        BindingStrength::Example => 0,
        BindingStrength::Preferred => 1,
        BindingStrength::Extensible => 2,
        BindingStrength::Required => 3,
    }
}

/// Validate that a derived binding doesn't weaken the base binding.
pub fn validate_binding_refinement(
    element_path: &str,
    base_strength: BindingStrength,
    derived_strength: BindingStrength,
) -> Option<Diagnostic> {
    if !can_refine_binding_strength(base_strength, derived_strength) {
        let mut diag = Diagnostic::error(
            codes::BINDING_STRENGTH_WEAKENED,
            format!(
                "Binding strength '{}' is weaker than base '{}'",
                derived_strength, base_strength
            ),
        )
        .with_path(element_path)
        .with_source(DiagnosticSource::Ir);

        diag = diag.with_quick_fix(QuickFixFactory::fix_binding_strength(
            element_path,
            base_strength.as_str(),
        ));

        Some(diag)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Binding;

    #[test]
    fn test_valid_valueset_urls() {
        assert!(is_valid_valueset_url(
            "http://hl7.org/fhir/ValueSet/administrative-gender"
        ));
        assert!(is_valid_valueset_url("https://example.org/fhir/ValueSet/test"));
        assert!(is_valid_valueset_url("urn:oid:2.16.840.1.113883.6.238"));
    }

    #[test]
    fn test_invalid_valueset_urls() {
        assert!(!is_valid_valueset_url(""));
        assert!(!is_valid_valueset_url("not a url"));
        assert!(!is_valid_valueset_url("ftp://example.org/ValueSet"));
    }

    #[test]
    fn test_bindable_types() {
        assert!(is_bindable_type("code"));
        assert!(is_bindable_type("Coding"));
        assert!(is_bindable_type("CodeableConcept"));
        assert!(is_bindable_type("string")); // string can be bound to ValueSets
        assert!(!is_bindable_type("Reference"));
        assert!(!is_bindable_type("boolean"));
    }

    #[test]
    fn test_binding_strength_refinement() {
        // Can strengthen
        assert!(can_refine_binding_strength(
            BindingStrength::Example,
            BindingStrength::Required
        ));
        assert!(can_refine_binding_strength(
            BindingStrength::Preferred,
            BindingStrength::Extensible
        ));

        // Can keep same
        assert!(can_refine_binding_strength(
            BindingStrength::Required,
            BindingStrength::Required
        ));

        // Cannot weaken
        assert!(!can_refine_binding_strength(
            BindingStrength::Required,
            BindingStrength::Preferred
        ));
        assert!(!can_refine_binding_strength(
            BindingStrength::Extensible,
            BindingStrength::Example
        ));
    }

    #[test]
    fn test_empty_valueset_error() {
        let mut element = ElementNode::new("Patient.gender".to_string());
        element.constraints.binding = Some(Binding::new(
            BindingStrength::Required,
            "".to_string(),
        ));

        let diagnostics = validate_element_binding(&element);
        assert!(!diagnostics.is_empty());
        assert!(diagnostics
            .iter()
            .any(|d| d.code == codes::BINDING_EMPTY_VALUESET));
    }
}
