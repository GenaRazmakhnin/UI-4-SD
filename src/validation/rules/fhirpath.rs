//! FHIRPath Expression Validation
//!
//! Validates FHIRPath expressions used in invariants and constraints.
//! Uses the octofhir-fhirpath engine for full parsing validation.

use crate::ir::{ElementNode, ProfileDocument};
use crate::validation::diagnostic::{Diagnostic, DiagnosticSource};

/// Error codes for FHIRPath validation.
pub mod codes {
    pub const FHIRPATH_EMPTY_EXPRESSION: &str = "FP_001";
    pub const FHIRPATH_PARSE_ERROR: &str = "FP_002";
    pub const FHIRPATH_INVALID_FUNCTION: &str = "FP_003";
    pub const FHIRPATH_MISSING_KEY: &str = "FP_004";
    pub const FHIRPATH_DUPLICATE_KEY: &str = "FP_005";
}

/// Validate FHIRPath expressions in a profile.
pub fn validate_fhirpath_expressions(document: &ProfileDocument) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_element_recursive(&document.resource.root, &mut diagnostics);
    diagnostics
}

/// Validate FHIRPath expressions in an element.
pub fn validate_element_fhirpath(element: &ElementNode) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Validate invariants (IndexMap<String, Invariant>)
    for (key, invariant) in &element.constraints.invariants {
        // Check for empty key
        if key.is_empty() {
            diagnostics.push(
                Diagnostic::error(codes::FHIRPATH_MISSING_KEY, "Invariant is missing a key")
                    .with_path(&element.path)
                    .with_source(DiagnosticSource::FhirPath),
            );
        }

        // Check for empty expression
        if invariant.expression.is_empty() {
            diagnostics.push(
                Diagnostic::error(
                    codes::FHIRPATH_EMPTY_EXPRESSION,
                    format!("Invariant '{}' has empty expression", key),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::FhirPath),
            );
            continue;
        }

        // Validate expression syntax
        diagnostics.extend(validate_expression(&invariant.expression, &element.path));
    }

    diagnostics
}

/// Validate a FHIRPath expression using the fhirpath-rs engine.
fn validate_expression(expression: &str, element_path: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Use the full FHIRPath parser for validation
    use octofhir_fhirpath::{is_valid, parse_with_analysis};

    if !is_valid(expression) {
        // Get detailed parse errors using analysis mode
        let result = parse_with_analysis(expression);

        if !result.success {
            // Extract error diagnostics from ParseResult
            for diag in &result.diagnostics {
                if matches!(
                    diag.severity,
                    octofhir_fhirpath::DiagnosticSeverity::Error
                ) {
                    diagnostics.push(
                        Diagnostic::error(
                            codes::FHIRPATH_PARSE_ERROR,
                            format!("FHIRPath parse error: {}", diag.message),
                        )
                        .with_path(element_path)
                        .with_source(DiagnosticSource::FhirPath),
                    );
                }
            }
        }

        // Fallback: if parser doesn't give details, do basic check
        if diagnostics.is_empty() {
            if let Some(error) = check_basic_syntax(expression) {
                diagnostics.push(
                    Diagnostic::error(codes::FHIRPATH_PARSE_ERROR, error)
                        .with_path(element_path)
                        .with_source(DiagnosticSource::FhirPath),
                );
            } else {
                diagnostics.push(
                    Diagnostic::error(
                        codes::FHIRPATH_PARSE_ERROR,
                        "Invalid FHIRPath expression",
                    )
                    .with_path(element_path)
                    .with_source(DiagnosticSource::FhirPath),
                );
            }
        }
    }

    diagnostics
}

/// Basic syntax check without full parser.
fn check_basic_syntax(expression: &str) -> Option<String> {
    // Check for balanced parentheses
    let mut paren_depth = 0i32;
    for c in expression.chars() {
        match c {
            '(' => paren_depth += 1,
            ')' => {
                paren_depth -= 1;
                if paren_depth < 0 {
                    return Some("Unmatched closing parenthesis".to_string());
                }
            }
            _ => {}
        }
    }
    if paren_depth != 0 {
        return Some("Unmatched opening parenthesis".to_string());
    }

    // Check for balanced brackets
    let mut bracket_depth = 0i32;
    for c in expression.chars() {
        match c {
            '[' => bracket_depth += 1,
            ']' => {
                bracket_depth -= 1;
                if bracket_depth < 0 {
                    return Some("Unmatched closing bracket".to_string());
                }
            }
            _ => {}
        }
    }
    if bracket_depth != 0 {
        return Some("Unmatched opening bracket".to_string());
    }

    // Check for balanced single quotes (strings)
    let quote_count = expression.chars().filter(|&c| c == '\'').count();
    if quote_count % 2 != 0 {
        return Some("Unmatched string quote".to_string());
    }

    None
}

/// Recursively validate FHIRPath in element tree.
fn validate_element_recursive(element: &ElementNode, diagnostics: &mut Vec<Diagnostic>) {
    diagnostics.extend(validate_element_fhirpath(element));

    for child in &element.children {
        validate_element_recursive(child, diagnostics);
    }

    for slice in element.slices.values() {
        validate_element_recursive(&slice.element, diagnostics);
    }
}

/// Check for duplicate invariant keys in a document.
pub fn check_duplicate_invariant_keys(document: &ProfileDocument) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen_keys = std::collections::HashSet::new();

    collect_invariant_keys(&document.resource.root, &mut seen_keys, &mut diagnostics);

    diagnostics
}

fn collect_invariant_keys(
    element: &ElementNode,
    seen: &mut std::collections::HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Invariants are stored as IndexMap<String, Invariant> where key is the constraint key
    for (key, _invariant) in &element.constraints.invariants {
        if !key.is_empty() && !seen.insert(key.clone()) {
            diagnostics.push(
                Diagnostic::error(
                    codes::FHIRPATH_DUPLICATE_KEY,
                    format!("Duplicate invariant key: '{}'", key),
                )
                .with_path(&element.path)
                .with_source(DiagnosticSource::FhirPath),
            );
        }
    }

    for child in &element.children {
        collect_invariant_keys(child, seen, diagnostics);
    }

    for slice in element.slices.values() {
        collect_invariant_keys(&slice.element, seen, diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Invariant, InvariantSeverity};

    #[test]
    fn test_balanced_parentheses() {
        assert!(check_basic_syntax("name.exists()").is_none());
        assert!(check_basic_syntax("(a and b)").is_none());
        assert!(check_basic_syntax("((a))").is_none());

        assert!(check_basic_syntax("(a and b").is_some());
        assert!(check_basic_syntax("a and b)").is_some());
    }

    #[test]
    fn test_balanced_brackets() {
        assert!(check_basic_syntax("name[0]").is_none());
        assert!(check_basic_syntax("name[0].family[1]").is_none());

        assert!(check_basic_syntax("name[0").is_some());
        assert!(check_basic_syntax("name]").is_some());
    }

    #[test]
    fn test_balanced_quotes() {
        assert!(check_basic_syntax("name = 'test'").is_none());
        assert!(check_basic_syntax("'hello' + 'world'").is_none());

        assert!(check_basic_syntax("name = 'test").is_some());
    }

    #[test]
    fn test_validate_expression() {
        let diagnostics = validate_expression("name.exists() and active", "Patient");
        assert!(diagnostics.is_empty());

        let diagnostics = validate_expression("name.exists() and (active", "Patient");
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_empty_expression() {
        let mut element = ElementNode::new("Patient".to_string());
        element.constraints.invariants.insert(
            "test-1".to_string(),
            Invariant {
                key: "test-1".to_string(),
                severity: InvariantSeverity::Error,
                human: "Test".to_string(),
                expression: String::new(),
                xpath: None,
                source: None,
            },
        );

        let diagnostics = validate_element_fhirpath(&element);
        assert!(!diagnostics.is_empty());
        assert!(diagnostics
            .iter()
            .any(|d| d.code == codes::FHIRPATH_EMPTY_EXPRESSION));
    }

    #[test]
    fn test_missing_key() {
        let mut element = ElementNode::new("Patient".to_string());
        // Empty key to simulate missing key
        element.constraints.invariants.insert(
            String::new(),
            Invariant {
                key: String::new(),
                severity: InvariantSeverity::Error,
                human: "Test".to_string(),
                expression: "true".to_string(),
                xpath: None,
                source: None,
            },
        );

        let diagnostics = validate_element_fhirpath(&element);
        assert!(!diagnostics.is_empty());
        assert!(diagnostics
            .iter()
            .any(|d| d.code == codes::FHIRPATH_MISSING_KEY));
    }
}
