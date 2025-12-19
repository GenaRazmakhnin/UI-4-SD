//! Element tree merger.
//!
//! Merges differential elements onto a base element tree to produce
//! the combined view needed for UI display.

use crate::ir::{ElementConstraints, ElementNode, ElementSource, NodeId, SliceNode, SlicingDefinition};

/// A differential element representing a modification to the base.
///
/// Unlike `ElementNode`, this is a flat representation without children,
/// as the tree structure comes from the base.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferentialElement {
    /// Stable unique identifier for UI operations.
    pub id: NodeId,

    /// Element path (e.g., "Patient.name", "Patient.name.family").
    pub path: String,

    /// Element ID within the path (e.g., "name" for "Patient.name").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,

    /// Slice name if this is a slice definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice_name: Option<String>,

    /// The constraints that are modified from the base.
    #[serde(default)]
    pub constraints: ElementConstraints,

    /// Slicing definition if this element introduces slicing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slicing: Option<SlicingDefinition>,

    /// Unknown fields preserved for lossless round-trip.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub unknown_fields: serde_json::Map<String, serde_json::Value>,
}

impl DifferentialElement {
    /// Create a new differential element.
    #[must_use]
    pub fn new(path: String) -> Self {
        let element_id = path.rsplit('.').next().map(ToString::to_string);
        Self {
            id: NodeId::new(),
            path,
            element_id,
            slice_name: None,
            constraints: ElementConstraints::default(),
            slicing: None,
            unknown_fields: serde_json::Map::new(),
        }
    }

    /// Create from an ElementNode (for converting existing IR).
    #[must_use]
    pub fn from_element_node(node: &ElementNode) -> Self {
        Self {
            id: node.id,
            path: node.path.clone(),
            element_id: node.element_id.clone(),
            slice_name: None,
            constraints: node.constraints.clone(),
            slicing: node.slicing.clone(),
            unknown_fields: node.unknown_fields.clone(),
        }
    }

    /// Check if this element has any actual constraints.
    #[must_use]
    pub fn has_constraints(&self) -> bool {
        self.constraints.has_any() || self.slicing.is_some()
    }
}

/// Merges differential elements onto a base element tree.
///
/// This is used when loading profiles for UI display. The base tree
/// provides the full structure, and the differential provides the
/// modifications made by this profile.
#[derive(Debug, Default)]
pub struct ElementTreeMerger;

impl ElementTreeMerger {
    /// Create a new tree merger.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Merge differential elements onto a base tree.
    ///
    /// # Arguments
    ///
    /// * `base_tree` - The element tree from the base definition
    /// * `differential` - The differential elements from this profile
    ///
    /// # Returns
    ///
    /// A new element tree with:
    /// - All elements from the base tree
    /// - Constraints from differential applied to matching elements
    /// - `source: Modified` for elements in the differential
    /// - `source: Inherited` for elements only in base
    #[must_use]
    pub fn merge(
        &self,
        mut base_tree: ElementNode,
        differential: &[DifferentialElement],
    ) -> ElementNode {
        // Apply differential entries in order (FHIR differential order is significant)
        for diff in differential {
            self.apply_differential_entry(&mut base_tree, diff);
        }

        base_tree
    }

    /// Apply a single differential entry to the tree.
    fn apply_differential_entry(&self, root: &mut ElementNode, diff: &DifferentialElement) {
        let (path, slice_name, slice_child) = self.parse_slice_context(diff);

        let target = self.find_or_create_element(root, &path);
        let target_was_added = target.source == ElementSource::Added;

        if let Some(slice_name) = slice_name {
            // Clone children from target BEFORE mutably borrowing for slice lookup
            // to avoid borrow checker conflicts
            let children_to_clone: Vec<_> = target
                .children
                .iter()
                .map(|child| self.clone_child_for_slice(child, &path, slice_name))
                .collect();
            let target_has_children = !children_to_clone.is_empty();

            let slice = self.find_or_create_slice(target, slice_name);
            let slice_was_added = slice.source == ElementSource::Added;
            if slice_was_added && slice.element.children.is_empty() && target_has_children {
                for cloned in children_to_clone {
                    slice.element.add_child(cloned);
                }
            }
            if let Some(child_path) = slice_child {
                let child =
                    self.find_or_create_slice_child(&mut slice.element, &path, slice_name, child_path);
                let child_was_added = child.source == ElementSource::Added;
                self.apply_constraints(child, diff);
                if diff.element_id.is_some() {
                    child.element_id = diff.element_id.clone();
                }
                child.source = if child_was_added {
                    ElementSource::Added
                } else {
                    ElementSource::Modified
                };
                child.id = diff.id;
            } else {
                self.apply_constraints(&mut slice.element, diff);
                if diff.element_id.is_some() {
                    slice.element.element_id = diff.element_id.clone();
                }
                slice.element.source = if slice_was_added {
                    ElementSource::Added
                } else {
                    ElementSource::Modified
                };
                slice.element.id = diff.id;
            }
            slice.source = if slice_was_added {
                ElementSource::Added
            } else {
                ElementSource::Modified
            };
        } else {
            self.apply_constraints(target, diff);
            if diff.element_id.is_some() {
                target.element_id = diff.element_id.clone();
            }
            target.source = if target_was_added {
                ElementSource::Added
            } else {
                ElementSource::Modified
            };
            target.id = diff.id;
        }
    }

    /// Apply differential constraints to an element.
    fn apply_constraints(&self, element: &mut ElementNode, diff: &DifferentialElement) {
        let constraints = &diff.constraints;

        // Apply cardinality if set in differential
        if constraints.cardinality.is_some() {
            element.constraints.cardinality = constraints.cardinality.clone();
        }

        // Apply types if set in differential
        if !constraints.types.is_empty() {
            element.constraints.types = constraints.types.clone();
        }

        // Apply binding if set in differential
        if constraints.binding.is_some() {
            element.constraints.binding = constraints.binding.clone();
        }

        // Apply fixed value if set in differential
        if constraints.fixed_value.is_some() {
            element.constraints.fixed_value = constraints.fixed_value.clone();
        }

        // Apply default value if set
        if constraints.default_value.is_some() {
            element.constraints.default_value = constraints.default_value.clone();
        }

        // Apply meaning when missing if set
        if constraints.meaning_when_missing.is_some() {
            element.constraints.meaning_when_missing = constraints.meaning_when_missing.clone();
        }

        // Apply flags
        if constraints.flags.must_support {
            element.constraints.flags.must_support = true;
        }
        if constraints.flags.is_modifier {
            element.constraints.flags.is_modifier = true;
        }
        if constraints.flags.is_modifier_reason.is_some() {
            element.constraints.flags.is_modifier_reason =
                constraints.flags.is_modifier_reason.clone();
        }
        if constraints.flags.is_summary {
            element.constraints.flags.is_summary = true;
        }

        // Apply text fields if set
        if constraints.short.is_some() {
            element.constraints.short = constraints.short.clone();
        }
        if constraints.definition.is_some() {
            element.constraints.definition = constraints.definition.clone();
        }
        if constraints.comment.is_some() {
            element.constraints.comment = constraints.comment.clone();
        }
        if constraints.requirements.is_some() {
            element.constraints.requirements = constraints.requirements.clone();
        }

        // Apply aliases if set
        if !constraints.alias.is_empty() {
            element.constraints.alias = constraints.alias.clone();
        }

        // Apply max length if set
        if constraints.max_length.is_some() {
            element.constraints.max_length = constraints.max_length;
        }

        // Merge invariants
        if !constraints.invariants.is_empty() {
            for (key, invariant) in &constraints.invariants {
                element
                    .constraints
                    .invariants
                    .insert(key.clone(), invariant.clone());
            }
        }

        // Apply mappings if set
        if !constraints.mappings.is_empty() {
            element.constraints.mappings = constraints.mappings.clone();
        }

        // Apply examples if set
        if !constraints.examples.is_empty() {
            element.constraints.examples = constraints.examples.clone();
        }

        // Apply slicing definition if provided
        if diff.slicing.is_some() {
            element.slicing = diff.slicing.clone();
        }

        // Merge unknown fields for round-trip preservation
        for (key, value) in &diff.unknown_fields {
            element.unknown_fields.insert(key.clone(), value.clone());
        }
    }

    /// Parse slice context from a differential element.
    fn parse_slice_context<'a>(
        &self,
        diff: &'a DifferentialElement,
    ) -> (String, Option<&'a str>, Option<&'a str>) {
        if diff.slice_name.is_some() {
            if let Some(element_id) = diff.element_id.as_deref() {
                if element_id.contains(':') {
                    if let Some((base_path, slice_suffix)) = element_id.split_once(':') {
                        if let Some((slice_name, child_path)) = slice_suffix.split_once('.') {
                            return (
                                base_path.to_string(),
                                Some(slice_name),
                                Some(child_path),
                            );
                        }
                        return (base_path.to_string(), Some(slice_suffix), None);
                    }
                }
            }

            return (diff.path.clone(), diff.slice_name.as_deref(), None);
        }

        if let Some((base_path, slice_suffix)) = diff.path.split_once(':') {
            if let Some((slice_name, child_path)) = slice_suffix.split_once('.') {
                return (
                    base_path.to_string(),
                    Some(slice_name),
                    Some(child_path),
                );
            }
            return (base_path.to_string(), Some(slice_suffix), None);
        }

        (diff.path.clone(), None, None)
    }

    /// Find or create an element by absolute path.
    fn find_or_create_element<'a>(&self, root: &'a mut ElementNode, path: &str) -> &'a mut ElementNode {
        let segments: Vec<&str> = path.split('.').collect();
        if segments.is_empty() {
            return root;
        }

        let root_name = root.short_name();
        let start_idx = if segments[0] == root_name { 1 } else { 0 };
        self.navigate_or_create(root, &segments[start_idx..])
    }

    fn navigate_or_create<'a>(
        &self,
        current: &'a mut ElementNode,
        segments: &[&str],
    ) -> &'a mut ElementNode {
        if segments.is_empty() {
            return current;
        }

        let segment = segments[0];
        if let Some(idx) = current
            .children
            .iter()
            .position(|child| child.short_name() == segment)
        {
            return self.navigate_or_create(&mut current.children[idx], &segments[1..]);
        }

        let new_path = if current.path.is_empty() {
            segment.to_string()
        } else {
            format!("{}.{}", current.path, segment)
        };
        let mut new_child = ElementNode::new(new_path);
        new_child.source = ElementSource::Added;
        current.add_child(new_child);
        let last_idx = current.children.len() - 1;
        self.navigate_or_create(&mut current.children[last_idx], &segments[1..])
    }

    fn find_or_create_slice<'a>(
        &self,
        element: &'a mut ElementNode,
        slice_name: &str,
    ) -> &'a mut SliceNode {
        if element.slicing.is_none() {
            element.slicing = Some(SlicingDefinition::new(Vec::new()));
        }

        element
            .slices
            .entry(slice_name.to_string())
            .or_insert_with(|| {
                let full_path = format!("{}:{}", element.path, slice_name);
                SliceNode::with_path(slice_name, full_path)
            })
    }

    fn find_or_create_slice_child<'a>(
        &self,
        slice_root: &'a mut ElementNode,
        base_path: &str,
        slice_name: &str,
        child_path: &str,
    ) -> &'a mut ElementNode {
        let full_path = format!("{}:{}.{}", base_path, slice_name, child_path);
        let segments: Vec<&str> = child_path.split('.').collect();
        self.navigate_or_create_with_base(slice_root, &full_path, &segments)
    }

    fn navigate_or_create_with_base<'a>(
        &self,
        current: &'a mut ElementNode,
        full_path: &str,
        segments: &[&str],
    ) -> &'a mut ElementNode {
        if segments.is_empty() {
            return current;
        }

        let segment = segments[0];
        if let Some(idx) = current
            .children
            .iter()
            .position(|child| child.short_name() == segment)
        {
            return self.navigate_or_create_with_base(
                &mut current.children[idx],
                full_path,
                &segments[1..],
            );
        }

        let new_path = if current.path.is_empty() {
            full_path.to_string()
        } else {
            format!("{}.{}", current.path, segment)
        };
        let mut new_child = ElementNode::new(new_path);
        new_child.source = ElementSource::Added;
        current.add_child(new_child);
        let last_idx = current.children.len() - 1;
        self.navigate_or_create_with_base(
            &mut current.children[last_idx],
            full_path,
            &segments[1..],
        )
    }

    fn clone_child_for_slice(
        &self,
        child: &ElementNode,
        base_path: &str,
        slice_name: &str,
    ) -> ElementNode {
        let prefix = format!("{}.", base_path);
        let relative = child
            .path
            .strip_prefix(&prefix)
            .unwrap_or_else(|| child.short_name());
        let new_path = format!("{}:{}.{}", base_path, slice_name, relative);
        let mut cloned = ElementNode::new(new_path.clone());
        cloned.element_id = Some(new_path);
        cloned.constraints = child.constraints.clone();
        cloned.source = ElementSource::Inherited;
        cloned.slicing = child.slicing.clone();
        cloned.unknown_fields = child.unknown_fields.clone();

        for grandchild in &child.children {
            let nested = self.clone_child_for_slice(grandchild, base_path, slice_name);
            cloned.add_child(nested);
        }

        cloned
    }
}

/// Extract differential elements from an existing element tree.
///
/// This is used when converting an existing IR (with full tree)
/// to the new differential-only format.
pub fn extract_differential(root: &ElementNode) -> Vec<DifferentialElement> {
    let mut differential = Vec::new();
    collect_modified_elements(root, &mut differential, None);
    differential
}

/// Recursively collect modified elements from a tree.
fn collect_modified_elements(
    element: &ElementNode,
    result: &mut Vec<DifferentialElement>,
    slice_name: Option<&str>,
) {
    // Include if modified or has constraints
    if element.source.is_modified() {
        let mut diff = DifferentialElement::from_element_node(element);
        diff.slice_name = slice_name.map(String::from);

        if let Some(slice_name) = slice_name {
            if let Some((base_path, _)) = element.path.split_once(':') {
                let child_suffix = element
                    .path
                    .split_once(':')
                    .and_then(|(_, rest)| rest.split_once('.'))
                    .map(|(_, child)| child);

                diff.path = match child_suffix {
                    Some(child) => format!("{}.{}", base_path, child),
                    None => base_path.to_string(),
                };

                diff.element_id = Some(match child_suffix {
                    Some(child) => format!("{}:{}.{}", base_path, slice_name, child),
                    None => format!("{}:{}", base_path, slice_name),
                });
            }
        }

        result.push(diff);
    }

    // Process children
    for child in &element.children {
        collect_modified_elements(child, result, slice_name);
    }

    // Process slices
    for slice in element.slices.values() {
        collect_modified_elements(&slice.element, result, Some(&slice.name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Cardinality;

    fn create_base_tree() -> ElementNode {
        let mut root = ElementNode::new("Patient".to_string());
        root.source = ElementSource::Inherited;

        let mut name = ElementNode::new("Patient.name".to_string());
        name.constraints.cardinality = Some(Cardinality::new(0, None)); // 0..*
        name.source = ElementSource::Inherited;

        let mut family = ElementNode::new("Patient.name.family".to_string());
        family.constraints.cardinality = Some(Cardinality::new(0, Some(1)));
        family.source = ElementSource::Inherited;

        name.add_child(family);
        root.add_child(name);

        root
    }

    #[test]
    fn test_merge_cardinality_constraint() {
        let base = create_base_tree();

        // Create differential that requires name
        let mut diff = DifferentialElement::new("Patient.name".to_string());
        diff.constraints.cardinality = Some(Cardinality::new(1, None)); // 1..*

        let merger = ElementTreeMerger::new();
        let merged = merger.merge(base, &[diff]);

        // Root should be inherited
        assert_eq!(merged.source, ElementSource::Inherited);

        // Name should be modified with new cardinality
        let name = &merged.children[0];
        assert_eq!(name.source, ElementSource::Modified);
        assert_eq!(name.constraints.cardinality.as_ref().unwrap().min, 1);

        // Family should still be inherited
        let family = &name.children[0];
        assert_eq!(family.source, ElementSource::Inherited);
    }

    #[test]
    fn test_merge_must_support_flag() {
        let base = create_base_tree();

        let mut diff = DifferentialElement::new("Patient.name.family".to_string());
        diff.constraints.flags.must_support = true;

        let merger = ElementTreeMerger::new();
        let merged = merger.merge(base, &[diff]);

        let family = &merged.children[0].children[0];
        assert_eq!(family.source, ElementSource::Modified);
        assert!(family.constraints.flags.must_support);
    }

    #[test]
    fn test_extract_differential() {
        let mut root = ElementNode::new("Patient".to_string());
        root.source = ElementSource::Inherited;

        let mut name = ElementNode::new("Patient.name".to_string());
        name.source = ElementSource::Modified;
        name.constraints.cardinality = Some(Cardinality::new(1, None));
        root.add_child(name);

        let mut id = ElementNode::new("Patient.id".to_string());
        id.source = ElementSource::Inherited;
        root.add_child(id);

        let differential = extract_differential(&root);

        // Should only include the modified name element
        assert_eq!(differential.len(), 1);
        assert_eq!(differential[0].path, "Patient.name");
    }
}
