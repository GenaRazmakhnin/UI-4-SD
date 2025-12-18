//! Slicing Importer.
//!
//! Imports slicing definitions and slice elements from FHIR StructureDefinition
//! into the IR element tree. Handles discriminators, slice names, and slice
//! element children.

use serde_json::Value;

use super::element_builder::ElementTreeBuilder;
use super::error::{ImportError, ImportResult};
use crate::ir::{
    Discriminator, DiscriminatorType, ElementNode, ElementSource, SliceNode, SlicingDefinition,
    SlicingRules,
};

/// Importer for slicing definitions and slices.
#[derive(Debug, Default)]
pub struct SlicingImporter {
    /// Element builder for parsing slice elements.
    element_builder: ElementTreeBuilder,
}

impl SlicingImporter {
    /// Create a new slicing importer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            element_builder: ElementTreeBuilder::new(),
        }
    }

    /// Import slicing definitions and slices into the element tree.
    ///
    /// This:
    /// 1. Finds elements with slicing definitions and adds them to nodes
    /// 2. Finds slice elements (paths with ':') and creates SliceNodes
    /// 3. Attaches slice children to their slice nodes
    pub fn import_slicing(
        &self,
        root: &mut ElementNode,
        elements: &[Value],
    ) -> ImportResult<()> {
        // First pass: import slicing definitions
        for element in elements {
            if let Some(slicing) = element.get("slicing") {
                let path = element
                    .get("path")
                    .and_then(Value::as_str)
                    .ok_or_else(|| ImportError::missing_field("element.path"))?;

                // Skip slice paths - they don't define slicing, they ARE slices
                if path.contains(':') {
                    continue;
                }

                let slicing_def = self.parse_slicing_definition(slicing)?;

                // Find the node and attach slicing
                if let Some(node) = self.find_node_by_path(root, path) {
                    node.slicing = Some(slicing_def);
                }
            }
        }

        // Second pass: import slice elements
        for element in elements {
            let path = element
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(|| ImportError::missing_field("element.path"))?;

            // Check if this is a slice (path contains ':')
            if let Some((base_path, slice_suffix)) = path.split_once(':') {
                // Extract slice name (might have more path after the slice name)
                let (slice_name, child_path) = match slice_suffix.split_once('.') {
                    Some((name, rest)) => (name, Some(rest)),
                    None => (slice_suffix, None),
                };

                // If this is the slice root element (no child path)
                if child_path.is_none() {
                    self.import_slice_element(root, base_path, slice_name, element)?;
                } else {
                    // This is a child of a slice - will be handled in slice tree building
                    // For now, skip - we'll do a third pass for children
                }
            }
        }

        // Third pass: import slice children
        self.import_slice_children(root, elements)?;

        Ok(())
    }

    /// Parse a slicing definition from JSON.
    fn parse_slicing_definition(&self, slicing: &Value) -> ImportResult<SlicingDefinition> {
        let discriminators = slicing
            .get("discriminator")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|d| self.parse_discriminator(d))
                    .collect()
            })
            .unwrap_or_default();

        let description = slicing
            .get("description")
            .and_then(Value::as_str)
            .map(String::from);

        let ordered = slicing.get("ordered").and_then(Value::as_bool).unwrap_or(false);

        let rules = match slicing.get("rules").and_then(Value::as_str) {
            Some("closed") => SlicingRules::Closed,
            Some("open") => SlicingRules::Open,
            Some("openAtEnd") => SlicingRules::OpenAtEnd,
            _ => SlicingRules::Open,
        };

        Ok(SlicingDefinition {
            discriminator: discriminators,
            description,
            ordered,
            rules,
        })
    }

    /// Parse a discriminator from JSON.
    fn parse_discriminator(&self, disc: &Value) -> Option<Discriminator> {
        let type_str = disc.get("type").and_then(Value::as_str)?;
        let path = disc.get("path").and_then(Value::as_str)?;

        let disc_type = match type_str {
            "value" => DiscriminatorType::Value,
            "exists" => DiscriminatorType::Exists,
            "pattern" => DiscriminatorType::Pattern,
            "type" => DiscriminatorType::Type,
            "profile" => DiscriminatorType::Profile,
            "position" => DiscriminatorType::Position,
            _ => return None,
        };

        Some(Discriminator {
            discriminator_type: disc_type,
            path: path.to_string(),
        })
    }

    /// Import a slice element into the tree.
    fn import_slice_element(
        &self,
        root: &mut ElementNode,
        base_path: &str,
        slice_name: &str,
        element: &Value,
    ) -> ImportResult<()> {
        // Find the sliced element
        let sliced_node = self.find_node_by_path(root, base_path).ok_or_else(|| {
            ImportError::slicing_error(
                base_path,
                format!("Cannot find element for slice '{}'", slice_name),
            )
        })?;

        // Ensure slicing is defined
        if sliced_node.slicing.is_none() {
            // Some profiles define slices without explicit slicing definition
            // Create a default one
            sliced_node.slicing = Some(SlicingDefinition::new(vec![]));
        }

        // Create the slice node
        let full_path = format!("{}:{}", base_path, slice_name);
        let mut slice = SliceNode::with_path(slice_name, &full_path);
        slice.source = ElementSource::Added;

        // Parse constraints for the slice element
        slice.element.constraints = self.parse_slice_constraints(element)?;

        // Set element ID if present
        if let Some(id) = element.get("id").and_then(Value::as_str) {
            slice.element.element_id = Some(id.to_string());
        }

        // Add to sliced element
        sliced_node.slices.insert(slice_name.to_string(), slice);

        Ok(())
    }

    /// Import children of slice elements.
    fn import_slice_children(
        &self,
        root: &mut ElementNode,
        elements: &[Value],
    ) -> ImportResult<()> {
        // Collect slice children
        let mut slice_children: std::collections::HashMap<String, Vec<(&Value, String)>> =
            std::collections::HashMap::new();

        for element in elements {
            let path = match element.get("path").and_then(Value::as_str) {
                Some(p) => p,
                None => continue,
            };

            // Check if this is a slice child (path contains ':' and has more segments after)
            if let Some((base_with_slice, child_suffix)) = path.split_once(':') {
                if let Some((slice_name, child_path)) = child_suffix.split_once('.') {
                    let slice_key = format!("{}:{}", base_with_slice, slice_name);
                    slice_children
                        .entry(slice_key)
                        .or_default()
                        .push((element, child_path.to_string()));
                }
            }
        }

        // Process collected children
        for (slice_key, children) in slice_children {
            if let Some((base_path, slice_name)) = slice_key.split_once(':') {
                self.add_slice_children(root, base_path, slice_name, &children)?;
            }
        }

        Ok(())
    }

    /// Add children to a slice.
    fn add_slice_children(
        &self,
        root: &mut ElementNode,
        base_path: &str,
        slice_name: &str,
        children: &[(&Value, String)],
    ) -> ImportResult<()> {
        // Find the sliced element and then the slice
        let sliced_node = match self.find_node_by_path(root, base_path) {
            Some(n) => n,
            None => return Ok(()), // Slice parent not found, skip
        };

        let slice = match sliced_node.slices.get_mut(slice_name) {
            Some(s) => s,
            None => return Ok(()), // Slice not found, skip
        };

        // Build children hierarchy
        let mut child_nodes: std::collections::HashMap<String, ElementNode> =
            std::collections::HashMap::new();

        for (element, relative_path) in children {
            let full_path = format!("{}:{}.{}", base_path, slice_name, relative_path);
            let mut node = ElementNode::new(full_path);
            node.constraints = self.parse_slice_constraints(element)?;
            node.source = ElementSource::Added;

            if let Some(id) = element.get("id").and_then(Value::as_str) {
                node.element_id = Some(id.to_string());
            }

            // Determine parent path
            let parent_relative = relative_path
                .rsplit_once('.')
                .map(|(p, _)| p.to_string())
                .unwrap_or_default();

            child_nodes.insert(relative_path.to_string(), node);

            // If there's a parent that's not the slice root, make sure it exists
            if !parent_relative.is_empty() && !child_nodes.contains_key(&parent_relative) {
                // Create placeholder parent if needed (will be filled in if element exists)
                let parent_full_path = format!("{}:{}.{}", base_path, slice_name, parent_relative);
                let parent_node = ElementNode::new(parent_full_path);
                child_nodes.insert(parent_relative, parent_node);
            }
        }

        // Now build the tree structure
        let mut roots: Vec<ElementNode> = Vec::new();

        // Sort by path depth (shorter paths first)
        let mut paths: Vec<_> = child_nodes.keys().cloned().collect();
        paths.sort_by_key(|p| p.matches('.').count());

        for path in paths {
            let node = child_nodes.remove(&path).unwrap();
            let parent_relative = path.rsplit_once('.').map(|(p, _)| p.to_string());

            if let Some(parent_path) = parent_relative {
                // Find parent in existing nodes
                if let Some(parent) = self.find_child_mut(&mut roots, &parent_path) {
                    parent.add_child(node);
                } else {
                    // Parent not in roots yet, this shouldn't happen with sorted paths
                    roots.push(node);
                }
            } else {
                roots.push(node);
            }
        }

        // Add roots to slice
        for child in roots {
            slice.element.add_child(child);
        }

        Ok(())
    }

    /// Find a child node by relative path in a list of nodes.
    fn find_child_mut<'a>(
        &self,
        nodes: &'a mut [ElementNode],
        relative_path: &str,
    ) -> Option<&'a mut ElementNode> {
        let (first, rest) = match relative_path.split_once('.') {
            Some((f, r)) => (f, Some(r)),
            None => (relative_path, None),
        };

        for node in nodes {
            if node.short_name() == first {
                return match rest {
                    Some(remaining) => self.find_child_mut(&mut node.children, remaining),
                    None => Some(node),
                };
            }
        }

        None
    }

    /// Parse constraints for a slice element.
    fn parse_slice_constraints(
        &self,
        element: &Value,
    ) -> ImportResult<crate::ir::ElementConstraints> {
        // Reuse element builder's constraint parsing
        let mut constraints = crate::ir::ElementConstraints::default();

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
            constraints.cardinality = Some(crate::ir::Cardinality::new(min_val, max_val));
        }

        // Types
        if let Some(types) = element.get("type").and_then(Value::as_array) {
            for type_val in types {
                if let Some(tc) = self.parse_type_constraint(type_val) {
                    constraints.types.push(tc);
                }
            }
        }

        // Text fields
        constraints.short = element
            .get("short")
            .and_then(Value::as_str)
            .map(String::from);

        constraints.definition = element
            .get("definition")
            .and_then(Value::as_str)
            .map(String::from);

        // Flags
        constraints.flags.must_support = element
            .get("mustSupport")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        // Binding
        if let Some(binding) = element.get("binding") {
            constraints.binding = self.parse_binding(binding);
        }

        // Fixed/pattern
        if let Some(obj) = element.as_object() {
            for (key, value) in obj {
                if key.starts_with("fixed") {
                    constraints.fixed_value = Some(crate::ir::FixedValue::Fixed(value.clone()));
                    break;
                }
                if key.starts_with("pattern") {
                    constraints.fixed_value = Some(crate::ir::FixedValue::Pattern(value.clone()));
                    break;
                }
            }
        }

        Ok(constraints)
    }

    /// Parse type constraint.
    fn parse_type_constraint(&self, type_val: &Value) -> Option<crate::ir::TypeConstraint> {
        let code = type_val.get("code").and_then(Value::as_str)?;

        Some(crate::ir::TypeConstraint {
            code: code.to_string(),
            profile: type_val
                .get("profile")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            target_profile: type_val
                .get("targetProfile")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            aggregation: Vec::new(),
            versioning: None,
        })
    }

    /// Parse binding.
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::ElementTreeBuilder;

    fn build_test_tree_with_identifier() -> ElementNode {
        let elements = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "min": 0,
                "max": "*"
            }),
            serde_json::json!({
                "path": "Patient.identifier.system",
                "min": 0,
                "max": "1"
            }),
            serde_json::json!({
                "path": "Patient.identifier.value",
                "min": 0,
                "max": "1"
            }),
        ];

        ElementTreeBuilder::new()
            .build_tree("Patient", &elements, None)
            .unwrap()
    }

    #[test]
    fn test_import_slicing_definition() {
        let mut root = build_test_tree_with_identifier();
        let elements = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "slicing": {
                    "discriminator": [
                        {"type": "value", "path": "system"}
                    ],
                    "rules": "open"
                }
            }),
        ];

        let importer = SlicingImporter::new();
        importer.import_slicing(&mut root, &elements).unwrap();

        let identifier = &root.children[0];
        assert!(identifier.slicing.is_some());
        let slicing = identifier.slicing.as_ref().unwrap();
        assert_eq!(slicing.discriminator.len(), 1);
        assert_eq!(slicing.discriminator[0].discriminator_type, DiscriminatorType::Value);
        assert_eq!(slicing.discriminator[0].path, "system");
        assert_eq!(slicing.rules, SlicingRules::Open);
    }

    #[test]
    fn test_import_slice() {
        let mut root = build_test_tree_with_identifier();
        let elements = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "slicing": {
                    "discriminator": [{"type": "value", "path": "system"}],
                    "rules": "open"
                }
            }),
            serde_json::json!({
                "id": "Patient.identifier:ssn",
                "path": "Patient.identifier",
                "sliceName": "ssn",
                "min": 0,
                "max": "1"
            }),
        ];

        // The slice path format varies - some use "path" with sliceName,
        // others embed in path as "Patient.identifier:ssn"
        // Let's test the embedded format
        let elements_embedded = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "slicing": {
                    "discriminator": [{"type": "value", "path": "system"}],
                    "rules": "open"
                }
            }),
            serde_json::json!({
                "id": "Patient.identifier:ssn",
                "path": "Patient.identifier:ssn",
                "min": 0,
                "max": "1"
            }),
        ];

        let importer = SlicingImporter::new();
        importer.import_slicing(&mut root, &elements_embedded).unwrap();

        let identifier = &root.children[0];
        assert!(identifier.slicing.is_some());
        assert!(identifier.slices.contains_key("ssn"));

        let ssn_slice = identifier.slices.get("ssn").unwrap();
        assert_eq!(ssn_slice.name, "ssn");
    }

    #[test]
    fn test_import_multiple_slices() {
        let mut root = build_test_tree_with_identifier();
        let elements = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "slicing": {
                    "discriminator": [{"type": "value", "path": "system"}],
                    "rules": "closed"
                }
            }),
            serde_json::json!({
                "path": "Patient.identifier:ssn",
                "min": 0,
                "max": "1"
            }),
            serde_json::json!({
                "path": "Patient.identifier:mrn",
                "min": 1,
                "max": "1"
            }),
        ];

        let importer = SlicingImporter::new();
        importer.import_slicing(&mut root, &elements).unwrap();

        let identifier = &root.children[0];
        assert_eq!(identifier.slices.len(), 2);
        assert!(identifier.slices.contains_key("ssn"));
        assert!(identifier.slices.contains_key("mrn"));

        let mrn = identifier.slices.get("mrn").unwrap();
        assert_eq!(
            mrn.element.constraints.cardinality,
            Some(crate::ir::Cardinality::new(1, Some(1)))
        );
    }

    #[test]
    fn test_slicing_rules_parsing() {
        let mut root = build_test_tree_with_identifier();

        // Test closed
        let elements_closed = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "slicing": {"rules": "closed", "discriminator": []}
            }),
        ];
        let importer = SlicingImporter::new();
        importer.import_slicing(&mut root, &elements_closed).unwrap();
        assert_eq!(
            root.children[0].slicing.as_ref().unwrap().rules,
            SlicingRules::Closed
        );

        // Reset and test openAtEnd
        let mut root2 = build_test_tree_with_identifier();
        let elements_open_at_end = vec![
            serde_json::json!({"path": "Patient"}),
            serde_json::json!({
                "path": "Patient.identifier",
                "slicing": {"rules": "openAtEnd", "discriminator": []}
            }),
        ];
        importer.import_slicing(&mut root2, &elements_open_at_end).unwrap();
        assert_eq!(
            root2.children[0].slicing.as_ref().unwrap().rules,
            SlicingRules::OpenAtEnd
        );
    }

    #[test]
    fn test_discriminator_types() {
        let importer = SlicingImporter::new();

        let disc_json = serde_json::json!({"type": "pattern", "path": "code"});
        let disc = importer.parse_discriminator(&disc_json).unwrap();
        assert_eq!(disc.discriminator_type, DiscriminatorType::Pattern);

        let disc_json = serde_json::json!({"type": "type", "path": "$this"});
        let disc = importer.parse_discriminator(&disc_json).unwrap();
        assert_eq!(disc.discriminator_type, DiscriminatorType::Type);

        let disc_json = serde_json::json!({"type": "profile", "path": "$this"});
        let disc = importer.parse_discriminator(&disc_json).unwrap();
        assert_eq!(disc.discriminator_type, DiscriminatorType::Profile);
    }
}
