//! Invariant operations for profile elements.
//!
//! This module provides operations for managing FHIRPath invariant constraints:
//! - Add invariant
//! - Update invariant
//! - Remove invariant

use serde_json::json;

use crate::ir::{Change, Invariant, InvariantSeverity, NodeId, ProfileDocument};

use super::error::{OperationError, OperationResult};
use super::traits::Operation;

// =============================================================================
// AddInvariant
// =============================================================================

/// Add a FHIRPath invariant to an element.
#[derive(Debug, Clone)]
pub struct AddInvariant {
    /// Element path.
    pub path: String,
    /// Invariant key (e.g., "pat-1").
    pub key: String,
    /// Severity (error or warning).
    pub severity: InvariantSeverity,
    /// Human-readable description.
    pub human: String,
    /// FHIRPath expression.
    pub expression: String,
    /// Optional source reference.
    pub source: Option<String>,
}

impl AddInvariant {
    /// Create a new add invariant operation.
    pub fn new(
        path: impl Into<String>,
        key: impl Into<String>,
        severity: InvariantSeverity,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            key: key.into(),
            severity,
            human: human.into(),
            expression: expression.into(),
            source: None,
        }
    }

    /// Create an error-level invariant.
    pub fn error(
        path: impl Into<String>,
        key: impl Into<String>,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self::new(path, key, InvariantSeverity::Error, human, expression)
    }

    /// Create a warning-level invariant.
    pub fn warning(
        path: impl Into<String>,
        key: impl Into<String>,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self::new(path, key, InvariantSeverity::Warning, human, expression)
    }

    /// Set the source.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

impl Operation for AddInvariant {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Check for duplicate key
        if element.constraints.invariants.contains_key(&self.key) {
            return Err(OperationError::DuplicateInvariantKey {
                key: self.key.clone(),
            });
        }

        // Basic FHIRPath validation (check for balanced parentheses, etc.)
        if !is_valid_fhirpath(&self.expression) {
            return Err(OperationError::InvalidFhirPathExpression {
                expression: self.expression.clone(),
                reason: "Invalid FHIRPath syntax".to_string(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let invariant = Invariant {
            key: self.key.clone(),
            severity: self.severity,
            human: self.human.clone(),
            expression: self.expression.clone(),
            xpath: None,
            source: self.source.clone(),
        };

        element.constraints.invariants.insert(self.key.clone(), invariant);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.invariants.shift_remove(&self.key);

        Ok(())
    }

    fn description(&self) -> String {
        format!("Add invariant '{}' to {}", self.key, self.path)
    }

    fn as_change(&self) -> Change {
        Change::add(
            NodeId::new(),
            "constraints.invariants",
            json!({
                "key": self.key,
                "severity": match self.severity {
                    InvariantSeverity::Error => "error",
                    InvariantSeverity::Warning => "warning",
                },
                "human": self.human,
                "expression": self.expression
            }),
        )
    }
}

// =============================================================================
// UpdateInvariant
// =============================================================================

/// Update an existing invariant on an element.
#[derive(Debug, Clone)]
pub struct UpdateInvariant {
    /// Element path.
    pub path: String,
    /// Invariant key.
    pub key: String,
    /// New severity (optional).
    pub severity: Option<InvariantSeverity>,
    /// New human description (optional).
    pub human: Option<String>,
    /// New expression (optional).
    pub expression: Option<String>,
    /// Previous invariant (for undo).
    prev_invariant: Option<Invariant>,
}

impl UpdateInvariant {
    /// Create a new update invariant operation.
    pub fn new(path: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            key: key.into(),
            severity: None,
            human: None,
            expression: None,
            prev_invariant: None,
        }
    }

    /// Update the severity.
    pub fn with_severity(mut self, severity: InvariantSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Update the human description.
    pub fn with_human(mut self, human: impl Into<String>) -> Self {
        self.human = Some(human.into());
        self
    }

    /// Update the expression.
    pub fn with_expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }
}

impl Operation for UpdateInvariant {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Check invariant exists
        if !element.constraints.invariants.contains_key(&self.key) {
            return Err(OperationError::InvariantNotFound {
                key: self.key.clone(),
            });
        }

        // Validate new expression if provided
        if let Some(ref expr) = self.expression {
            if !is_valid_fhirpath(expr) {
                return Err(OperationError::InvalidFhirPathExpression {
                    expression: expr.clone(),
                    reason: "Invalid FHIRPath syntax".to_string(),
                });
            }
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let invariant = element
            .constraints
            .invariants
            .get_mut(&self.key)
            .ok_or_else(|| OperationError::InvariantNotFound {
                key: self.key.clone(),
            })?;

        if let Some(severity) = self.severity {
            invariant.severity = severity;
        }
        if let Some(ref human) = self.human {
            invariant.human = human.clone();
        }
        if let Some(ref expression) = self.expression {
            invariant.expression = expression.clone();
        }

        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        if let Some(ref prev) = self.prev_invariant {
            let element = document
                .resource
                .find_element_mut(&self.path)
                .ok_or_else(|| OperationError::element_not_found(&self.path))?;

            element.constraints.invariants.insert(self.key.clone(), prev.clone());
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Update invariant '{}' on {}", self.key, self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            format!("constraints.invariants.{}", self.key),
            self.prev_invariant.as_ref().map(|i| json!(i)),
            json!({
                "severity": self.severity.map(|s| match s {
                    InvariantSeverity::Error => "error",
                    InvariantSeverity::Warning => "warning",
                }),
                "human": self.human,
                "expression": self.expression
            }),
        )
    }
}

// =============================================================================
// RemoveInvariant
// =============================================================================

/// Remove an invariant from an element.
#[derive(Debug, Clone)]
pub struct RemoveInvariant {
    /// Element path.
    pub path: String,
    /// Invariant key to remove.
    pub key: String,
    /// Previous invariant (for undo).
    prev_invariant: Option<Invariant>,
}

impl RemoveInvariant {
    /// Create a new remove invariant operation.
    pub fn new(path: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            key: key.into(),
            prev_invariant: None,
        }
    }
}

impl Operation for RemoveInvariant {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if !element.constraints.invariants.contains_key(&self.key) {
            return Err(OperationError::InvariantNotFound {
                key: self.key.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.invariants.shift_remove(&self.key);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        if let Some(ref prev) = self.prev_invariant {
            let element = document
                .resource
                .find_element_mut(&self.path)
                .ok_or_else(|| OperationError::element_not_found(&self.path))?;

            element.constraints.invariants.insert(self.key.clone(), prev.clone());
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Remove invariant '{}' from {}", self.key, self.path)
    }

    fn as_change(&self) -> Change {
        Change::remove(
            NodeId::new(),
            "constraints.invariants",
            json!({ "key": self.key }),
        )
    }
}

/// Basic FHIRPath expression validation.
///
/// This performs simple syntax checks. Full FHIRPath parsing would require
/// a dedicated parser.
fn is_valid_fhirpath(expr: &str) -> bool {
    // Check for empty expression
    if expr.trim().is_empty() {
        return false;
    }

    // Check balanced parentheses
    let mut paren_count = 0;
    for c in expr.chars() {
        match c {
            '(' => paren_count += 1,
            ')' => {
                paren_count -= 1;
                if paren_count < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    if paren_count != 0 {
        return false;
    }

    // Check balanced brackets
    let mut bracket_count = 0;
    for c in expr.chars() {
        match c {
            '[' => bracket_count += 1,
            ']' => {
                bracket_count -= 1;
                if bracket_count < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    if bracket_count != 0 {
        return false;
    }

    // Check for balanced single quotes
    let quote_count = expr.chars().filter(|&c| c == '\'').count();
    if quote_count % 2 != 0 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, DocumentMetadata, ElementNode, FhirVersion, ProfiledResource};

    fn create_test_document() -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            "test-patient",
            "http://example.org/fhir/StructureDefinition/TestPatient",
            "TestPatient",
        );
        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        let mut doc = ProfileDocument::new(metadata, resource);

        let name = ElementNode::new("Patient.name".to_string());
        doc.resource.root.add_child(name);

        doc
    }

    #[test]
    fn test_add_invariant() {
        let mut doc = create_test_document();

        let op = AddInvariant::error(
            "Patient.name",
            "name-1",
            "Name must have family or given",
            "family.exists() or given.exists()",
        );

        assert!(op.validate(&doc).is_ok());
        op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.name").unwrap();
        assert!(element.constraints.invariants.contains_key("name-1"));

        let invariant = element.constraints.invariants.get("name-1").unwrap();
        assert_eq!(invariant.severity, InvariantSeverity::Error);
        assert_eq!(invariant.expression, "family.exists() or given.exists()");
    }

    #[test]
    fn test_duplicate_invariant_key() {
        let mut doc = create_test_document();

        let op1 = AddInvariant::error("Patient.name", "name-1", "First", "true");
        op1.apply(&mut doc).unwrap();

        let op2 = AddInvariant::error("Patient.name", "name-1", "Second", "false");
        assert!(matches!(
            op2.validate(&doc),
            Err(OperationError::DuplicateInvariantKey { .. })
        ));
    }

    #[test]
    fn test_invalid_fhirpath() {
        let doc = create_test_document();

        // Unbalanced parentheses
        let op = AddInvariant::error("Patient.name", "name-1", "Test", "family.exists((");
        assert!(matches!(
            op.validate(&doc),
            Err(OperationError::InvalidFhirPathExpression { .. })
        ));
    }

    #[test]
    fn test_remove_invariant() {
        let mut doc = create_test_document();

        // Add then remove
        let add_op = AddInvariant::error("Patient.name", "name-1", "Test", "true");
        add_op.apply(&mut doc).unwrap();

        let remove_op = RemoveInvariant::new("Patient.name", "name-1");
        assert!(remove_op.validate(&doc).is_ok());
        remove_op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.name").unwrap();
        assert!(!element.constraints.invariants.contains_key("name-1"));
    }

    #[test]
    fn test_is_valid_fhirpath() {
        assert!(is_valid_fhirpath("name.exists()"));
        assert!(is_valid_fhirpath("family.exists() or given.exists()"));
        assert!(is_valid_fhirpath("name.where(use = 'official').exists()"));
        assert!(is_valid_fhirpath("extension['http://example.org'].exists()"));

        assert!(!is_valid_fhirpath(""));
        assert!(!is_valid_fhirpath("name.exists(("));
        assert!(!is_valid_fhirpath("name[0"));
        assert!(!is_valid_fhirpath("name = 'test"));
    }
}
