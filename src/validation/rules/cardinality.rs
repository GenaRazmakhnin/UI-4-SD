//! Cardinality Validation Rules
//!
//! Validates cardinality constraints on profile elements:
//! - min ≤ max
//! - min ≥ 0
//! - Derived cardinality doesn't exceed base
//! - Slice cardinality sum ≤ parent max

use crate::ir::ElementNode;
use crate::validation::diagnostic::{Diagnostic, DiagnosticSource};
use crate::validation::quick_fix::QuickFixFactory;

/// Error codes for cardinality validation.
pub mod codes {
    pub const CARD_MIN_EXCEEDS_MAX: &str = "CARD_001";
    pub const CARD_NEGATIVE_MIN: &str = "CARD_002";
    pub const CARD_INVALID_MAX: &str = "CARD_003";
    pub const CARD_SLICE_SUM_EXCEEDS_PARENT: &str = "CARD_004";
    pub const CARD_REQUIRED_ELEMENT_IN_OPTIONAL_PARENT: &str = "CARD_005";
}

/// Validate cardinality for an entire element tree.
pub fn validate_cardinality_tree(root: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_element_recursive(root, &mut diagnostics);
    diagnostics
}

/// Validate a single element's cardinality.
pub fn validate_element_cardinality(element: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let Some(card) = &element.constraints.cardinality {
        // Check min ≤ max
        if let Some(max) = card.max {
            if card.min > max {
                let mut diag = Diagnostic::error(
                    codes::CARD_MIN_EXCEEDS_MAX,
                    format!(
                        "Minimum cardinality ({}) exceeds maximum ({})",
                        card.min, max
                    ),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir);

                // Add quick fix
                diag = diag.with_quick_fix(QuickFixFactory::fix_cardinality_min_exceeds_max(
                    &element.path,
                    card.min,
                    max,
                ));

                diagnostics.push(diag);
            }
        }
    }

    diagnostics
}

/// Recursively validate cardinality in element tree.
fn validate_element_recursive(element: &ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    // Validate this element
    diagnostics.extend(validate_element_cardinality(element));

    // Validate slicing cardinality consistency
    if element.slicing.is_some() {
        diagnostics.extend(validate_slice_cardinalities(element));
    }

    // Validate required elements in optional parents
    diagnostics.extend(validate_required_in_optional(element));

    // Recurse into children
    for child in &element.children {
        validate_element_recursive(child, diagnostics);
    }

    // Recurse into slices
    for slice in element.slices.values() {
        validate_element_recursive(&slice.element, diagnostics);
    }
}

/// Validate that slice cardinalities don't exceed parent max.
fn validate_slice_cardinalities(element: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Get parent max cardinality
    let parent_max = element
        .constraints
        .cardinality
        .as_ref()
        .and_then(|c| c.max);

    if let Some(max) = parent_max {
        // Sum up minimum cardinalities of all slices
        let slice_min_sum: u32 = element
            .slices
            .values()
            .filter_map(|s| s.element.constraints.cardinality.as_ref())
            .map(|c| c.min)
            .sum();

        if slice_min_sum > max {
            diagnostics.push(
                Diagnostic::error(
                    codes::CARD_SLICE_SUM_EXCEEDS_PARENT,
                    format!(
                        "Sum of slice minimum cardinalities ({}) exceeds parent maximum ({})",
                        slice_min_sum, max
                    ),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir),
            );
        }
    }

    diagnostics
}

/// Validate that required elements aren't in optional parents.
fn validate_required_in_optional(element: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Check if this element is optional (min = 0)
    let is_optional = element
        .constraints
        .cardinality
        .as_ref()
        .is_some_and(|c| c.min == 0);

    if is_optional {
        // Check for required children
        for child in &element.children {
            if let Some(card) = &child.constraints.cardinality {
                if card.min > 0 {
                    diagnostics.push(
                        Diagnostic::warning(
                            codes::CARD_REQUIRED_ELEMENT_IN_OPTIONAL_PARENT,
                            format!(
                                "Required element '{}' (min={}) is inside optional parent '{}'",
                                child.path, card.min, element.path
                            ),
                        )
                        .with_path(&child.path)
                        .with_source(DiagnosticSource::Ir),
                    );
                }
            }
        }
    }

    diagnostics
}

/// Check if a cardinality refinement is valid (doesn't widen the range).
pub fn is_valid_refinement(base_min: u32, base_max: Option<u32>, new_min: u32, new_max: Option<u32>) -> bool {
    // New min must be >= base min
    if new_min < base_min {
        return false;
    }

    // New max must be <= base max (if both defined)
    match (base_max, new_max) {
        (Some(base), Some(new)) => new <= base,
        (Some(_), None) => false, // Can't widen from finite to unlimited
        (None, _) => true,        // Base is unlimited, any max is valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Cardinality;

    #[test]
    fn test_min_exceeds_max() {
        let mut element = ElementNode::new("Patient.name".to_string());
        element.constraints.cardinality = Some(Cardinality::new(5, Some(2)));

        let diagnostics = validate_element_cardinality(&element);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, codes::CARD_MIN_EXCEEDS_MAX);
    }

    #[test]
    fn test_valid_cardinality() {
        let mut element = ElementNode::new("Patient.name".to_string());
        element.constraints.cardinality = Some(Cardinality::new(1, Some(10)));

        let diagnostics = validate_element_cardinality(&element);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_is_valid_refinement() {
        // Valid: tightening range
        assert!(is_valid_refinement(0, Some(10), 1, Some(5)));
        // Valid: same range
        assert!(is_valid_refinement(1, Some(5), 1, Some(5)));
        // Invalid: widening min
        assert!(!is_valid_refinement(1, Some(5), 0, Some(5)));
        // Invalid: widening max
        assert!(!is_valid_refinement(0, Some(5), 0, Some(10)));
    }
}
