//! Constraint Extractor.
//!
//! Extracts and applies constraints from differential elements to the element tree.
//! The differential contains only modified elements, so this module merges those
//! constraints into the snapshot-based tree.

use serde_json::Value;

use super::element_builder::ElementTreeBuilder;
use super::error::{ImportError, ImportResult};
use crate::ir::{ElementNode, ElementSource};

/// Extracts constraints from differential elements and applies them to the element tree.
#[derive(Debug, Default)]
pub struct ConstraintExtractor {
    /// Element builder for parsing constraints.
    element_builder: ElementTreeBuilder,
}

impl ConstraintExtractor {
    /// Create a new constraint extractor.
    #[must_use]
    pub fn new() -> Self {
        Self {
            element_builder: ElementTreeBuilder::new(),
        }
    }

    /// Apply differential constraints to an element tree.
    ///
    /// This finds each differential element in the tree and marks it as modified,
    /// applying any constraint changes.
    ///
    /// # Arguments
    ///
    /// * `root` - The root element node (built from snapshot)
    /// * `differential` - The differential elements from the SD
    pub fn apply_differential(
        &self,
        root: &mut ElementNode,
        differential: &[Value],
    ) -> ImportResult<()> {
        for diff_element in differential {
            let path = diff_element
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(|| ImportError::missing_field("differential.element.path"))?;

            // Skip slice entries (handled by slicing importer)
            if path.contains(':') {
                continue;
            }

            // Find the node in the tree
            if let Some(node) = self.find_node_by_path(root, path) {
                self.apply_element_constraints(node, diff_element)?;
            }
            // Note: We don't error on missing nodes since the differential might
            // include slices or extensions not yet in the tree
        }

        Ok(())
    }

    /// Find a node by its full path.
    fn find_node_by_path<'a>(
        &self,
        root: &'a mut ElementNode,
        path: &str,
    ) -> Option<&'a mut ElementNode> {
        if root.path == path {
            return Some(root);
        }

        // Get relative path after root
        let relative = path.strip_prefix(&root.path)?.strip_prefix('.')?;
        self.find_node_recursive(root, relative)
    }

    fn find_node_recursive<'a>(
        &self,
        node: &'a mut ElementNode,
        relative_path: &str,
    ) -> Option<&'a mut ElementNode> {
        let (segment, rest) = match relative_path.split_once('.') {
            Some((s, r)) => (s, Some(r)),
            None => (relative_path, None),
        };

        let child = node.children.iter_mut().find(|c| c.short_name() == segment)?;

        match rest {
            Some(remaining) => self.find_node_recursive(child, remaining),
            None => Some(child),
        }
    }

    /// Apply constraints from a differential element to a node.
    fn apply_element_constraints(
        &self,
        node: &mut ElementNode,
        diff_element: &Value,
    ) -> ImportResult<()> {
        // Mark as modified
        node.source = ElementSource::Modified;

        // Apply cardinality if specified
        if let Some(min) = diff_element.get("min").and_then(Value::as_u64) {
            if let Some(ref mut card) = node.constraints.cardinality {
                card.min = min as u32;
            } else {
                node.constraints.cardinality = Some(crate::ir::Cardinality::new(min as u32, None));
            }
        }

        if let Some(max) = diff_element.get("max").and_then(Value::as_str) {
            let max_val = if max == "*" { None } else { max.parse().ok() };
            if let Some(ref mut card) = node.constraints.cardinality {
                card.max = max_val;
            } else {
                node.constraints.cardinality = Some(crate::ir::Cardinality::new(0, max_val));
            }
        }

        // Apply type constraints if specified
        if let Some(types) = diff_element.get("type").and_then(Value::as_array) {
            node.constraints.types.clear();
            for type_val in types {
                if let Some(tc) = self.parse_type_constraint(type_val) {
                    node.constraints.types.push(tc);
                }
            }
        }

        // Apply text fields if specified
        if let Some(short) = diff_element.get("short").and_then(Value::as_str) {
            node.constraints.short = Some(short.to_string());
        }

        if let Some(definition) = diff_element.get("definition").and_then(Value::as_str) {
            node.constraints.definition = Some(definition.to_string());
        }

        if let Some(comment) = diff_element.get("comment").and_then(Value::as_str) {
            node.constraints.comment = Some(comment.to_string());
        }

        if let Some(requirements) = diff_element.get("requirements").and_then(Value::as_str) {
            node.constraints.requirements = Some(requirements.to_string());
        }

        // Apply flags
        if let Some(must_support) = diff_element.get("mustSupport").and_then(Value::as_bool) {
            node.constraints.flags.must_support = must_support;
        }

        if let Some(is_modifier) = diff_element.get("isModifier").and_then(Value::as_bool) {
            node.constraints.flags.is_modifier = is_modifier;
        }

        if let Some(reason) = diff_element.get("isModifierReason").and_then(Value::as_str) {
            node.constraints.flags.is_modifier_reason = Some(reason.to_string());
        }

        if let Some(is_summary) = diff_element.get("isSummary").and_then(Value::as_bool) {
            node.constraints.flags.is_summary = is_summary;
        }

        // Apply binding if specified
        if let Some(binding) = diff_element.get("binding") {
            node.constraints.binding = self.parse_binding(binding);
        }

        // Apply fixed/pattern values
        self.apply_fixed_pattern(node, diff_element);

        // Apply invariants
        if let Some(constraints) = diff_element.get("constraint").and_then(Value::as_array) {
            for inv in constraints {
                if let Some((key, invariant)) = self.parse_invariant(inv) {
                    node.constraints.invariants.insert(key, invariant);
                }
            }
        }

        // Max length
        if let Some(max_length) = diff_element.get("maxLength").and_then(Value::as_u64) {
            node.constraints.max_length = Some(max_length as u32);
        }

        Ok(())
    }

    /// Parse type constraint from JSON.
    fn parse_type_constraint(&self, type_val: &Value) -> Option<crate::ir::TypeConstraint> {
        let code = type_val.get("code").and_then(Value::as_str)?;

        let profile = type_val
            .get("profile")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let target_profile = type_val
            .get("targetProfile")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let aggregation = type_val
            .get("aggregation")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let versioning = type_val
            .get("versioning")
            .and_then(Value::as_str)
            .map(String::from);

        Some(crate::ir::TypeConstraint {
            code: code.to_string(),
            profile,
            target_profile,
            aggregation,
            versioning,
        })
    }

    /// Parse binding from JSON.
    fn parse_binding(&self, binding: &Value) -> Option<crate::ir::Binding> {
        let strength_str = binding.get("strength").and_then(Value::as_str)?;
        let strength = match strength_str {
            "required" => crate::ir::BindingStrength::Required,
            "extensible" => crate::ir::BindingStrength::Extensible,
            "preferred" => crate::ir::BindingStrength::Preferred,
            "example" => crate::ir::BindingStrength::Example,
            _ => return None,
        };

        let value_set = binding.get("valueSet").and_then(Value::as_str)?;

        Some(crate::ir::Binding {
            strength,
            value_set: value_set.to_string(),
            description: binding
                .get("description")
                .and_then(Value::as_str)
                .map(String::from),
        })
    }

    /// Parse invariant from JSON.
    fn parse_invariant(
        &self,
        inv: &Value,
    ) -> Option<(String, crate::ir::constraint::Invariant)> {
        let key = inv.get("key").and_then(Value::as_str)?;
        let human = inv.get("human").and_then(Value::as_str)?;
        let expression = inv.get("expression").and_then(Value::as_str).unwrap_or("");

        let severity = match inv.get("severity").and_then(Value::as_str) {
            Some("error") => crate::ir::constraint::InvariantSeverity::Error,
            Some("warning") => crate::ir::constraint::InvariantSeverity::Warning,
            _ => crate::ir::constraint::InvariantSeverity::Error,
        };

        Some((
            key.to_string(),
            crate::ir::constraint::Invariant {
                key: key.to_string(),
                severity,
                human: human.to_string(),
                expression: expression.to_string(),
                xpath: inv.get("xpath").and_then(Value::as_str).map(String::from),
                source: inv.get("source").and_then(Value::as_str).map(String::from),
            },
        ))
    }

    /// Apply fixed or pattern values from differential.
    fn apply_fixed_pattern(&self, node: &mut ElementNode, element: &Value) {
        let obj = match element.as_object() {
            Some(o) => o,
            None => return,
        };

        for (key, value) in obj {
            if key.starts_with("fixed") {
                node.constraints.fixed_value = Some(crate::ir::FixedValue::Fixed(value.clone()));
                return;
            }
            if key.starts_with("pattern") {
                node.constraints.fixed_value = Some(crate::ir::FixedValue::Pattern(value.clone()));
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::ElementTreeBuilder;

    fn build_test_tree() -> ElementNode {
        let elements = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.name",
                "min": 0,
                "max": "*"
            }),
            serde_json::json!({
                "path": "Patient.name.family",
                "min": 0,
                "max": "1"
            }),
            serde_json::json!({
                "path": "Patient.birthDate",
                "min": 0,
                "max": "1"
            }),
        ];

        ElementTreeBuilder::new()
            .build_tree("Patient", &elements, None)
            .unwrap()
    }

    #[test]
    fn test_apply_cardinality_constraint() {
        let mut root = build_test_tree();
        let differential = vec![serde_json::json!({
            "path": "Patient.name",
            "min": 1
        })];

        let extractor = ConstraintExtractor::new();
        extractor.apply_differential(&mut root, &differential).unwrap();

        let name = &root.children[0];
        assert_eq!(name.source, ElementSource::Modified);
        assert_eq!(name.constraints.cardinality.as_ref().unwrap().min, 1);
    }

    #[test]
    fn test_apply_must_support() {
        let mut root = build_test_tree();
        let differential = vec![serde_json::json!({
            "path": "Patient.name",
            "mustSupport": true
        })];

        let extractor = ConstraintExtractor::new();
        extractor.apply_differential(&mut root, &differential).unwrap();

        let name = &root.children[0];
        assert!(name.constraints.flags.must_support);
    }

    #[test]
    fn test_apply_nested_constraint() {
        let mut root = build_test_tree();
        let differential = vec![serde_json::json!({
            "path": "Patient.name.family",
            "min": 1,
            "mustSupport": true
        })];

        let extractor = ConstraintExtractor::new();
        extractor.apply_differential(&mut root, &differential).unwrap();

        let name = &root.children[0];
        let family = &name.children[0];
        assert_eq!(family.source, ElementSource::Modified);
        assert_eq!(family.constraints.cardinality.as_ref().unwrap().min, 1);
        assert!(family.constraints.flags.must_support);
    }

    #[test]
    fn test_apply_binding() {
        let mut root = build_test_tree();
        let differential = vec![serde_json::json!({
            "path": "Patient.name",
            "binding": {
                "strength": "extensible",
                "valueSet": "http://example.org/ValueSet/names"
            }
        })];

        let extractor = ConstraintExtractor::new();
        extractor.apply_differential(&mut root, &differential).unwrap();

        let name = &root.children[0];
        assert!(name.constraints.binding.is_some());
        let binding = name.constraints.binding.as_ref().unwrap();
        assert_eq!(binding.strength, crate::ir::BindingStrength::Extensible);
    }

    #[test]
    fn test_apply_fixed_value() {
        let mut root = build_test_tree();
        let differential = vec![serde_json::json!({
            "path": "Patient.name.family",
            "fixedString": "Smith"
        })];

        let extractor = ConstraintExtractor::new();
        extractor.apply_differential(&mut root, &differential).unwrap();

        let name = &root.children[0];
        let family = &name.children[0];
        assert!(family.constraints.fixed_value.is_some());
        assert!(family.constraints.fixed_value.as_ref().unwrap().is_fixed());
    }

    #[test]
    fn test_apply_short_definition() {
        let mut root = build_test_tree();
        let differential = vec![serde_json::json!({
            "path": "Patient.name",
            "short": "Patient's legal name",
            "definition": "The name by which the patient is legally known."
        })];

        let extractor = ConstraintExtractor::new();
        extractor.apply_differential(&mut root, &differential).unwrap();

        let name = &root.children[0];
        assert_eq!(
            name.constraints.short.as_deref(),
            Some("Patient's legal name")
        );
        assert!(name.constraints.definition.is_some());
    }

    #[test]
    fn test_skip_slice_paths() {
        let mut root = build_test_tree();
        let differential = vec![
            serde_json::json!({
                "path": "Patient.name",
                "min": 1
            }),
            serde_json::json!({
                "path": "Patient.name:official",
                "min": 1
            }),
        ];

        let extractor = ConstraintExtractor::new();
        // Should not error on slice path
        let result = extractor.apply_differential(&mut root, &differential);
        assert!(result.is_ok());
    }
}
