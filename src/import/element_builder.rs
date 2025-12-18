//! Element Tree Builder.
//!
//! Builds the IR element tree from FHIR snapshot/differential element definitions.
//! Handles path parsing, parent-child relationships, and element ordering.

use std::collections::HashMap;

use serde_json::Value;

use super::error::{ImportError, ImportResult};
use crate::ir::{
    Binding, BindingStrength, Cardinality, ElementConstraints, ElementNode, ElementSource,
    FixedValue, TypeConstraint,
};

/// Builder for constructing an element tree from FHIR element definitions.
#[derive(Debug, Default)]
pub struct ElementTreeBuilder {
    /// Whether to preserve unknown fields.
    preserve_unknown: bool,
}

impl ElementTreeBuilder {
    /// Create a new element tree builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            preserve_unknown: true,
        }
    }

    /// Configure whether to preserve unknown fields.
    #[must_use]
    pub fn preserve_unknown(mut self, preserve: bool) -> Self {
        self.preserve_unknown = preserve;
        self
    }

    /// Build an element tree from element definitions.
    ///
    /// # Arguments
    ///
    /// * `root_type` - The root type name (e.g., "Patient")
    /// * `elements` - Snapshot elements (complete element list)
    /// * `differential` - Optional differential elements (modified elements only)
    ///
    /// # Returns
    ///
    /// The root `ElementNode` with all children built into a tree structure.
    pub fn build_tree(
        &self,
        root_type: &str,
        elements: &[Value],
        differential: Option<&[Value]>,
    ) -> ImportResult<ElementNode> {
        if elements.is_empty() {
            return Err(ImportError::element_tree("No elements provided"));
        }

        // Build differential paths set for tracking modified elements
        let diff_paths: std::collections::HashSet<String> = differential
            .map(|diff| {
                diff.iter()
                    .filter_map(|e| e.get("path").and_then(Value::as_str).map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Create root node
        let root_path = root_type.to_string();
        let mut root = self.build_element_node(&elements[0], &root_path, &diff_paths)?;

        // Build path -> node index map for parent lookup
        let mut path_to_children: HashMap<String, Vec<ElementNode>> = HashMap::new();

        // Process remaining elements
        for element in elements.iter().skip(1) {
            let path = element
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(|| ImportError::missing_field("element.path"))?;

            // Skip slice entries for now (handled by slicing importer)
            if path.contains(':') {
                continue;
            }

            let node = self.build_element_node(element, path, &diff_paths)?;
            let parent_path = self.get_parent_path(path);

            path_to_children
                .entry(parent_path.to_string())
                .or_default()
                .push(node);
        }

        // Build tree recursively
        self.attach_children(&mut root, &mut path_to_children);

        Ok(root)
    }

    /// Build a single element node from JSON.
    fn build_element_node(
        &self,
        element: &Value,
        path: &str,
        diff_paths: &std::collections::HashSet<String>,
    ) -> ImportResult<ElementNode> {
        let mut node = ElementNode::new(path.to_string());

        // Set element ID if present
        if let Some(id) = element.get("id").and_then(Value::as_str) {
            node.element_id = Some(id.to_string());
        }

        // Track if this element is in the differential
        node.source = if diff_paths.contains(path) {
            ElementSource::Modified
        } else {
            ElementSource::Inherited
        };

        // Parse constraints
        node.constraints = self.parse_constraints(element)?;

        // Preserve unknown fields
        if self.preserve_unknown {
            node.unknown_fields = self.extract_unknown_fields(element);
        }

        Ok(node)
    }

    /// Parse element constraints from JSON.
    fn parse_constraints(&self, element: &Value) -> ImportResult<ElementConstraints> {
        let mut constraints = ElementConstraints::default();

        // Cardinality
        let min = element.get("min").and_then(Value::as_u64).map(|v| v as u32);
        let max = element.get("max").and_then(Value::as_str);

        if min.is_some() || max.is_some() {
            let min_val = min.unwrap_or(0);
            let max_val = match max {
                Some("*") => None,
                Some(s) => s.parse().ok(),
                None => None,
            };
            constraints.cardinality = Some(Cardinality::new(min_val, max_val));
        }

        // Types
        if let Some(types) = element.get("type").and_then(Value::as_array) {
            for type_val in types {
                if let Some(type_constraint) = self.parse_type_constraint(type_val) {
                    constraints.types.push(type_constraint);
                }
            }
        }

        // Short description
        constraints.short = element
            .get("short")
            .and_then(Value::as_str)
            .map(String::from);

        // Definition
        constraints.definition = element
            .get("definition")
            .and_then(Value::as_str)
            .map(String::from);

        // Comment
        constraints.comment = element
            .get("comment")
            .and_then(Value::as_str)
            .map(String::from);

        // Requirements
        constraints.requirements = element
            .get("requirements")
            .and_then(Value::as_str)
            .map(String::from);

        // Aliases
        if let Some(aliases) = element.get("alias").and_then(Value::as_array) {
            constraints.alias = aliases
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
        }

        // Max length
        constraints.max_length = element
            .get("maxLength")
            .and_then(Value::as_u64)
            .map(|v| v as u32);

        // Meaning when missing
        constraints.meaning_when_missing = element
            .get("meaningWhenMissing")
            .and_then(Value::as_str)
            .map(String::from);

        // Fixed value
        constraints.fixed_value = self.parse_fixed_value(element);

        // Default value
        constraints.default_value = self.parse_default_value(element);

        // Binding
        constraints.binding = self.parse_binding(element);

        // Flags
        constraints.flags.must_support = element
            .get("mustSupport")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        constraints.flags.is_modifier = element
            .get("isModifier")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        constraints.flags.is_modifier_reason = element
            .get("isModifierReason")
            .and_then(Value::as_str)
            .map(String::from);

        constraints.flags.is_summary = element
            .get("isSummary")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        // Invariants
        if let Some(constraints_arr) = element.get("constraint").and_then(Value::as_array) {
            for inv in constraints_arr {
                if let Some((key, invariant)) = self.parse_invariant(inv) {
                    constraints.invariants.insert(key, invariant);
                }
            }
        }

        // Mappings
        if let Some(mappings_arr) = element.get("mapping").and_then(Value::as_array) {
            for mapping in mappings_arr {
                if let Some(m) = self.parse_mapping(mapping) {
                    constraints.mappings.push(m);
                }
            }
        }

        // Examples
        if let Some(examples_arr) = element.get("example").and_then(Value::as_array) {
            for example in examples_arr {
                if let Some(e) = self.parse_example(example) {
                    constraints.examples.push(e);
                }
            }
        }

        Ok(constraints)
    }

    /// Parse a type constraint from JSON.
    fn parse_type_constraint(&self, type_val: &Value) -> Option<TypeConstraint> {
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

        Some(TypeConstraint {
            code: code.to_string(),
            profile,
            target_profile,
            aggregation,
            versioning,
        })
    }

    /// Parse fixed or pattern value from element.
    fn parse_fixed_value(&self, element: &Value) -> Option<FixedValue> {
        let obj = element.as_object()?;

        // Check for fixed[x] fields
        for (key, value) in obj {
            if key.starts_with("fixed") {
                return Some(FixedValue::Fixed(value.clone()));
            }
            if key.starts_with("pattern") {
                return Some(FixedValue::Pattern(value.clone()));
            }
        }

        None
    }

    /// Parse default value from element.
    fn parse_default_value(&self, element: &Value) -> Option<serde_json::Value> {
        let obj = element.as_object()?;

        for (key, value) in obj {
            if key.starts_with("defaultValue") {
                return Some(value.clone());
            }
        }

        None
    }

    /// Parse binding from element.
    fn parse_binding(&self, element: &Value) -> Option<Binding> {
        let binding = element.get("binding")?;

        let strength_str = binding.get("strength").and_then(Value::as_str)?;
        let strength = match strength_str {
            "required" => BindingStrength::Required,
            "extensible" => BindingStrength::Extensible,
            "preferred" => BindingStrength::Preferred,
            "example" => BindingStrength::Example,
            _ => return None,
        };

        let value_set = binding.get("valueSet").and_then(Value::as_str)?;

        let description = binding
            .get("description")
            .and_then(Value::as_str)
            .map(String::from);

        Some(Binding {
            strength,
            value_set: value_set.to_string(),
            description,
        })
    }

    /// Parse invariant from JSON.
    fn parse_invariant(&self, inv: &Value) -> Option<(String, crate::ir::constraint::Invariant)> {
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

    /// Parse mapping from JSON.
    fn parse_mapping(&self, mapping: &Value) -> Option<crate::ir::constraint::Mapping> {
        let identity = mapping.get("identity").and_then(Value::as_str)?;
        let map = mapping.get("map").and_then(Value::as_str)?;

        Some(crate::ir::constraint::Mapping {
            identity: identity.to_string(),
            map: map.to_string(),
            comment: mapping
                .get("comment")
                .and_then(Value::as_str)
                .map(String::from),
            language: mapping
                .get("language")
                .and_then(Value::as_str)
                .map(String::from),
        })
    }

    /// Parse example from JSON.
    fn parse_example(&self, example: &Value) -> Option<crate::ir::constraint::Example> {
        let label = example.get("label").and_then(Value::as_str)?;

        // Find value[x] field
        let obj = example.as_object()?;
        let value = obj
            .iter()
            .find(|(k, _)| k.starts_with("value"))
            .map(|(_, v)| v.clone())?;

        Some(crate::ir::constraint::Example {
            label: label.to_string(),
            value,
        })
    }

    /// Extract unknown fields from element JSON.
    fn extract_unknown_fields(&self, element: &Value) -> serde_json::Map<String, Value> {
        let known_fields = [
            "id",
            "path",
            "sliceName",
            "sliceIsConstraining",
            "label",
            "code",
            "slicing",
            "short",
            "definition",
            "comment",
            "requirements",
            "alias",
            "min",
            "max",
            "base",
            "contentReference",
            "type",
            "defaultValue",
            "meaningWhenMissing",
            "orderMeaning",
            "fixed",
            "pattern",
            "example",
            "minValue",
            "maxValue",
            "maxLength",
            "condition",
            "constraint",
            "mustSupport",
            "isModifier",
            "isModifierReason",
            "isSummary",
            "binding",
            "mapping",
        ];

        let obj = match element.as_object() {
            Some(o) => o,
            None => return serde_json::Map::new(),
        };

        obj.iter()
            .filter(|(k, _)| {
                !known_fields.contains(&k.as_str())
                    && !k.starts_with("fixed")
                    && !k.starts_with("pattern")
                    && !k.starts_with("defaultValue")
                    && !k.starts_with("minValue")
                    && !k.starts_with("maxValue")
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Get the parent path from an element path.
    fn get_parent_path<'a>(&self, path: &'a str) -> &'a str {
        path.rsplit_once('.').map(|(parent, _)| parent).unwrap_or("")
    }

    /// Recursively attach children to nodes.
    fn attach_children(
        &self,
        node: &mut ElementNode,
        children_map: &mut HashMap<String, Vec<ElementNode>>,
    ) {
        if let Some(children) = children_map.remove(&node.path) {
            for mut child in children {
                child.parent_id = Some(node.id);
                self.attach_children(&mut child, children_map);
                node.children.push(child);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_simple_tree() {
        let elements = vec![
            serde_json::json!({
                "id": "Patient",
                "path": "Patient"
            }),
            serde_json::json!({
                "id": "Patient.id",
                "path": "Patient.id",
                "min": 0,
                "max": "1"
            }),
            serde_json::json!({
                "id": "Patient.name",
                "path": "Patient.name",
                "min": 0,
                "max": "*"
            }),
        ];

        let builder = ElementTreeBuilder::new();
        let root = builder.build_tree("Patient", &elements, None).unwrap();

        assert_eq!(root.path, "Patient");
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].path, "Patient.id");
        assert_eq!(root.children[1].path, "Patient.name");
    }

    #[test]
    fn test_cardinality_parsing() {
        let elements = vec![
            serde_json::json!({
                "path": "Patient"
            }),
            serde_json::json!({
                "path": "Patient.name",
                "min": 1,
                "max": "5"
            }),
        ];

        let builder = ElementTreeBuilder::new();
        let root = builder.build_tree("Patient", &elements, None).unwrap();

        let name = &root.children[0];
        assert_eq!(
            name.constraints.cardinality,
            Some(Cardinality::new(1, Some(5)))
        );
    }

    #[test]
    fn test_unbounded_cardinality() {
        let elements = vec![
            serde_json::json!({
                "path": "Patient"
            }),
            serde_json::json!({
                "path": "Patient.name",
                "min": 0,
                "max": "*"
            }),
        ];

        let builder = ElementTreeBuilder::new();
        let root = builder.build_tree("Patient", &elements, None).unwrap();

        let name = &root.children[0];
        assert_eq!(
            name.constraints.cardinality,
            Some(Cardinality::new(0, None))
        );
    }

    #[test]
    fn test_type_parsing() {
        let elements = vec![
            serde_json::json!({
                "path": "Patient"
            }),
            serde_json::json!({
                "path": "Patient.name",
                "type": [
                    {
                        "code": "HumanName"
                    }
                ]
            }),
        ];

        let builder = ElementTreeBuilder::new();
        let root = builder.build_tree("Patient", &elements, None).unwrap();

        let name = &root.children[0];
        assert_eq!(name.constraints.types.len(), 1);
        assert_eq!(name.constraints.types[0].code, "HumanName");
    }

    #[test]
    fn test_nested_tree() {
        let elements = vec![
            serde_json::json!({
                "path": "Patient"
            }),
            serde_json::json!({
                "path": "Patient.name"
            }),
            serde_json::json!({
                "path": "Patient.name.family"
            }),
            serde_json::json!({
                "path": "Patient.name.given"
            }),
        ];

        let builder = ElementTreeBuilder::new();
        let root = builder.build_tree("Patient", &elements, None).unwrap();

        assert_eq!(root.children.len(), 1);
        let name = &root.children[0];
        assert_eq!(name.children.len(), 2);
        assert_eq!(name.children[0].path, "Patient.name.family");
        assert_eq!(name.children[1].path, "Patient.name.given");
    }

    #[test]
    fn test_differential_tracking() {
        let snapshot = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({"path": "Patient.name"}),
            serde_json::json!({"path": "Patient.birthDate"}),
        ];

        let differential = vec![serde_json::json!({"path": "Patient.name", "min": 1})];

        let builder = ElementTreeBuilder::new();
        let root = builder
            .build_tree("Patient", &snapshot, Some(&differential))
            .unwrap();

        // Patient.name should be marked as modified
        let name = &root.children[0];
        assert_eq!(name.source, ElementSource::Modified);

        // Patient.birthDate should be inherited
        let birth_date = &root.children[1];
        assert_eq!(birth_date.source, ElementSource::Inherited);
    }

    #[test]
    fn test_binding_parsing() {
        let elements = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.gender",
                "binding": {
                    "strength": "required",
                    "valueSet": "http://hl7.org/fhir/ValueSet/administrative-gender"
                }
            }),
        ];

        let builder = ElementTreeBuilder::new();
        let root = builder.build_tree("Patient", &elements, None).unwrap();

        let gender = &root.children[0];
        assert!(gender.constraints.binding.is_some());
        let binding = gender.constraints.binding.as_ref().unwrap();
        assert_eq!(binding.strength, BindingStrength::Required);
    }

    #[test]
    fn test_must_support_flag() {
        let elements = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "mustSupport": true
            }),
        ];

        let builder = ElementTreeBuilder::new();
        let root = builder.build_tree("Patient", &elements, None).unwrap();

        let identifier = &root.children[0];
        assert!(identifier.constraints.flags.must_support);
    }
}
