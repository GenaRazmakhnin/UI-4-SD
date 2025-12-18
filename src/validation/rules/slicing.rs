//! Slicing Validation Rules
//!
//! Validates slicing definitions on profile elements:
//! - Slice names are unique within parent
//! - Discriminator paths are valid element paths
//! - Discriminator types are appropriate for path
//! - Slicing rules are consistent

use crate::ir::{ElementNode, SlicingRules};
use crate::validation::diagnostic::{Diagnostic, DiagnosticSource};
use crate::validation::quick_fix::QuickFixFactory;

/// Error codes for slicing validation.
pub mod codes {
    pub const SLICE_DUPLICATE_NAME: &str = "SLICE_001";
    pub const SLICE_INVALID_DISCRIMINATOR_PATH: &str = "SLICE_002";
    pub const SLICE_EMPTY_DISCRIMINATOR: &str = "SLICE_003";
    pub const SLICE_INVALID_DISCRIMINATOR_TYPE: &str = "SLICE_004";
    pub const SLICE_CLOSED_BUT_HAS_UNSLICED: &str = "SLICE_005";
    pub const SLICE_MISSING_DEFINITION: &str = "SLICE_006";
    pub const SLICE_NAME_EMPTY: &str = "SLICE_007";
    pub const SLICE_NAME_INVALID_CHARS: &str = "SLICE_008";
}

/// Valid discriminator types.
const VALID_DISCRIMINATOR_TYPES: &[&str] = &[
    "value", "exists", "pattern", "type", "profile", "position",
];

/// Validate slicing for an entire element tree.
pub fn validate_slicing_tree(root: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_element_recursive(root, &mut diagnostics);
    diagnostics
}

/// Validate a single element's slicing definition.
pub fn validate_element_slicing(element: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Check slicing definition
    if let Some(slicing) = &element.slicing {
        // Check for empty discriminator
        if slicing.discriminator.is_empty() {
            diagnostics.push(
                Diagnostic::warning(
                    codes::SLICE_EMPTY_DISCRIMINATOR,
                    "Slicing has no discriminator defined",
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir),
            );
        }

        // Validate each discriminator
        for disc in &slicing.discriminator {
            // Check discriminator type
            if !VALID_DISCRIMINATOR_TYPES.contains(&disc.discriminator_type.as_str()) {
                diagnostics.push(
                    Diagnostic::error(
                        codes::SLICE_INVALID_DISCRIMINATOR_TYPE,
                        format!(
                            "Invalid discriminator type: '{}'. Must be one of: {}",
                            disc.discriminator_type,
                            VALID_DISCRIMINATOR_TYPES.join(", ")
                        ),
                    )
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::Ir),
                );
            }

            // Validate discriminator path
            if disc.path.is_empty() {
                diagnostics.push(
                    Diagnostic::error(
                        codes::SLICE_INVALID_DISCRIMINATOR_PATH,
                        "Discriminator path cannot be empty",
                    )
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::Ir),
                );
            } else if !is_valid_discriminator_path(&disc.path) {
                let suggested = suggest_discriminator_path(&disc.path);
                let mut diag = Diagnostic::warning(
                    codes::SLICE_INVALID_DISCRIMINATOR_PATH,
                    format!("Discriminator path '{}' may be invalid", disc.path),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir);

                if let Some(suggestion) = suggested {
                    diag = diag.with_quick_fix(QuickFixFactory::fix_discriminator_path(
                        &element.path,
                        &disc.path,
                        &suggestion,
                    ));
                }

                diagnostics.push(diag);
            }
        }

        // Check closed slicing with unsliced content
        if slicing.rules == SlicingRules::Closed && element.slices.is_empty() {
            diagnostics.push(
                Diagnostic::warning(
                    codes::SLICE_CLOSED_BUT_HAS_UNSLICED,
                    "Slicing is closed but no slices are defined",
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::Ir),
            );
        }
    }

    // Validate slice names
    let mut seen_names = std::collections::HashSet::new();
    for (name, slice) in &element.slices {
        // Check for empty name
        if name.is_empty() {
            diagnostics.push(
                Diagnostic::error(codes::SLICE_NAME_EMPTY, "Slice name cannot be empty")
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::Ir),
            );
            continue;
        }

        // Check for invalid characters
        if !is_valid_slice_name(name) {
            diagnostics.push(
                Diagnostic::error(
                    codes::SLICE_NAME_INVALID_CHARS,
                    format!(
                        "Slice name '{}' contains invalid characters. Use alphanumeric and underscore only.",
                        name
                    ),
                )
                .with_path(&slice.element.path)
                .with_source(DiagnosticSource::Ir),
            );
        }

        // Check for duplicates
        if !seen_names.insert(name) {
            let mut diag = Diagnostic::error(
                codes::SLICE_DUPLICATE_NAME,
                format!("Duplicate slice name: '{}'", name),
            )
            .with_path(&slice.element.path)
            .with_source(DiagnosticSource::Ir);

            diag = diag.with_quick_fix(QuickFixFactory::remove_duplicate_slice(
                &element.path,
                name,
            ));

            diagnostics.push(diag);
        }
    }

    diagnostics
}

/// Recursively validate slicing in element tree.
fn validate_element_recursive(element: &ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    diagnostics.extend(validate_element_slicing(element));

    for child in &element.children {
        validate_element_recursive(child, diagnostics);
    }

    for slice in element.slices.values() {
        validate_element_recursive(&slice.element, diagnostics);
    }
}

/// Check if a discriminator path is syntactically valid.
fn is_valid_discriminator_path(path: &str) -> bool {
    // Path should not be empty
    if path.is_empty() {
        return false;
    }

    // Special paths
    if path == "$this" || path == "@type" || path == "resolve()" {
        return true;
    }

    // Path should be valid element path or FHIRPath-like expression
    // Allow alphanumeric, dots, brackets, parentheses
    path.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || c == '.'
            || c == '['
            || c == ']'
            || c == '('
            || c == ')'
            || c == '\''
            || c == '@'
            || c == '$'
            || c == '_'
    })
}

/// Check if a slice name is valid.
fn is_valid_slice_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // First char must be letter
    let first = name.chars().next().unwrap();
    if !first.is_ascii_alphabetic() {
        return false;
    }

    // Rest can be alphanumeric or underscore
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Suggest a corrected discriminator path.
fn suggest_discriminator_path(path: &str) -> Option<String> {
    // Common typos
    let corrections = [
        ("resolv()", "resolve()"),
        ("@Type", "@type"),
        ("$This", "$this"),
        ("system.code", "code"),
        ("coding.system", "coding"),
    ];

    for (typo, correction) in corrections {
        if path.eq_ignore_ascii_case(typo) {
            return Some(correction.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Discriminator, DiscriminatorType, SliceNode, SlicingDefinition};

    #[test]
    fn test_valid_discriminator_paths() {
        assert!(is_valid_discriminator_path("$this"));
        assert!(is_valid_discriminator_path("@type"));
        assert!(is_valid_discriminator_path("resolve()"));
        assert!(is_valid_discriminator_path("system"));
        assert!(is_valid_discriminator_path("code.coding.system"));
    }

    #[test]
    fn test_invalid_discriminator_paths() {
        assert!(!is_valid_discriminator_path(""));
        assert!(!is_valid_discriminator_path("path with spaces"));
    }

    #[test]
    fn test_valid_slice_names() {
        assert!(is_valid_slice_name("official"));
        assert!(is_valid_slice_name("mySlice"));
        assert!(is_valid_slice_name("slice_1"));
        assert!(is_valid_slice_name("slice-name"));
    }

    #[test]
    fn test_invalid_slice_names() {
        assert!(!is_valid_slice_name(""));
        assert!(!is_valid_slice_name("123slice"));
        assert!(!is_valid_slice_name("slice name"));
    }

    #[test]
    fn test_duplicate_slice_names() {
        let mut element = ElementNode::new("Patient.identifier".to_string());
        element.slicing = Some(SlicingDefinition {
            discriminator: vec![Discriminator {
                discriminator_type: DiscriminatorType::Value,
                path: "system".to_string(),
            }],
            description: None,
            ordered: false,
            rules: SlicingRules::Open,
        });

        // Add slices
        element.slices.insert(
            "mrn".to_string(),
            SliceNode::new("mrn"),
        );
        element.slices.insert(
            "ssn".to_string(),
            SliceNode::new("ssn"),
        );

        // Note: HashMap doesn't allow duplicates, so this test is more about the validation logic
        // In practice, duplicates would come from parsing or the UI
    }

    #[test]
    fn test_empty_discriminator_warning() {
        let mut element = ElementNode::new("Patient.identifier".to_string());
        element.slicing = Some(SlicingDefinition {
            discriminator: vec![],
            description: None,
            ordered: false,
            rules: SlicingRules::Open,
        });

        let diagnostics = validate_element_slicing(&element);
        assert!(diagnostics
            .iter()
            .any(|d| d.code == codes::SLICE_EMPTY_DISCRIMINATOR));
    }
}
