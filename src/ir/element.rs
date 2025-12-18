//! Element node representation for profile elements.
//!
//! This module defines [`ElementNode`], the core building block of the IR element tree.
//! Each node represents a single element in a FHIR profile with its constraints and
//! child elements.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::constraint::ElementConstraints;
use super::slicing::{SliceNode, SlicingDefinition};

/// Unique identifier for an element node in the IR tree.
///
/// This ID is stable across edits and is used for UI operations like drag-and-drop,
/// selection, and undo/redo. It is never exported to StructureDefinition or FSH.
///
/// # Example
///
/// ```
/// use niten::ir::NodeId;
///
/// let id = NodeId::new();
/// println!("Created node: {}", id);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Create a new unique node ID.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a node ID from a UUID.
    #[must_use]
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Indicates whether an element's value is inherited from base or explicitly modified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ElementSource {
    /// Value is inherited from the base resource/profile (default).
    #[default]
    Inherited,
    /// Value has been explicitly set/modified in this profile.
    Modified,
    /// Element was added by this profile (e.g., extension, slice).
    Added,
}

impl ElementSource {
    /// Returns `true` if the element has been modified from base.
    #[must_use]
    pub const fn is_modified(&self) -> bool {
        matches!(self, Self::Modified | Self::Added)
    }

    /// Returns `true` if the element is inherited without changes.
    #[must_use]
    pub const fn is_inherited(&self) -> bool {
        matches!(self, Self::Inherited)
    }
}

/// A node in the profile element tree.
///
/// Each `ElementNode` represents a single element in a FHIR StructureDefinition,
/// such as `Patient.name` or `Observation.code`. The node tracks constraints,
/// child elements, and maintains a stable UI ID for editing operations.
///
/// # Element Tree Structure
///
/// ```text
/// Patient (root)
/// ├── id
/// ├── meta
/// ├── name [sliced]
/// │   ├── official (slice)
/// │   │   ├── family
/// │   │   └── given
/// │   └── nickname (slice)
/// │       └── given
/// └── birthDate
/// ```
///
/// # Example
///
/// ```
/// use niten::ir::{ElementNode, NodeId, ElementConstraints, Cardinality};
///
/// let mut node = ElementNode::new("Patient.name".to_string());
/// node.constraints_mut().cardinality = Some(Cardinality::new(1, Some(1)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementNode {
    /// Stable unique identifier for UI operations.
    pub id: NodeId,

    /// Element path (e.g., "Patient.name", "Patient.name.family").
    pub path: String,

    /// Short element ID within the path (e.g., "name" for "Patient.name").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,

    /// Constraints applied to this element.
    #[serde(default)]
    pub constraints: ElementConstraints,

    /// Whether constraints are inherited or modified.
    #[serde(default)]
    pub source: ElementSource,

    /// Slicing definition if this element introduces slicing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slicing: Option<SlicingDefinition>,

    /// Named slices if this element is sliced.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub slices: IndexMap<String, SliceNode>,

    /// Child elements (ordered).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ElementNode>,

    /// Parent node ID (None for root elements).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NodeId>,

    /// Unknown/unrecognized fields preserved for lossless round-trip.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub unknown_fields: serde_json::Map<String, serde_json::Value>,
}

impl ElementNode {
    /// Create a new element node with the given path.
    #[must_use]
    pub fn new(path: String) -> Self {
        let element_id = path.rsplit('.').next().map(ToString::to_string);
        Self {
            id: NodeId::new(),
            path,
            element_id,
            constraints: ElementConstraints::default(),
            source: ElementSource::Inherited,
            slicing: None,
            slices: IndexMap::new(),
            children: Vec::new(),
            parent_id: None,
            unknown_fields: serde_json::Map::new(),
        }
    }

    /// Create a new element node with explicit ID.
    #[must_use]
    pub fn with_id(id: NodeId, path: String) -> Self {
        let element_id = path.rsplit('.').next().map(ToString::to_string);
        Self {
            id,
            path,
            element_id,
            constraints: ElementConstraints::default(),
            source: ElementSource::Inherited,
            slicing: None,
            slices: IndexMap::new(),
            children: Vec::new(),
            parent_id: None,
            unknown_fields: serde_json::Map::new(),
        }
    }

    /// Get the element's short name (last segment of path).
    #[must_use]
    pub fn short_name(&self) -> &str {
        self.element_id
            .as_deref()
            .unwrap_or_else(|| self.path.rsplit('.').next().unwrap_or(&self.path))
    }

    /// Get mutable access to constraints.
    pub fn constraints_mut(&mut self) -> &mut ElementConstraints {
        self.source = ElementSource::Modified;
        &mut self.constraints
    }

    /// Add a child element.
    pub fn add_child(&mut self, mut child: ElementNode) {
        child.parent_id = Some(self.id);
        self.children.push(child);
    }

    /// Add a slice to this element.
    ///
    /// # Panics
    ///
    /// Panics if `slicing` is not set on this element.
    pub fn add_slice(&mut self, name: String, mut slice: SliceNode) {
        assert!(
            self.slicing.is_some(),
            "Cannot add slice without slicing definition"
        );
        slice.parent_id = Some(self.id);
        self.slices.insert(name, slice);
    }

    /// Find a child element by path segment.
    #[must_use]
    pub fn find_child(&self, segment: &str) -> Option<&ElementNode> {
        self.children.iter().find(|c| c.short_name() == segment)
    }

    /// Find a child element by path segment (mutable).
    #[must_use]
    pub fn find_child_mut(&mut self, segment: &str) -> Option<&mut ElementNode> {
        self.children.iter_mut().find(|c| c.short_name() == segment)
    }

    /// Find a descendant element by relative path.
    #[must_use]
    pub fn find_descendant(&self, relative_path: &str) -> Option<&ElementNode> {
        let segments: Vec<&str> = relative_path.split('.').collect();
        self.find_descendant_by_segments(&segments)
    }

    fn find_descendant_by_segments(&self, segments: &[&str]) -> Option<&ElementNode> {
        if segments.is_empty() {
            return Some(self);
        }

        let child = self.find_child(segments[0])?;
        if segments.len() == 1 {
            Some(child)
        } else {
            child.find_descendant_by_segments(&segments[1..])
        }
    }

    /// Find a node by its stable ID.
    #[must_use]
    pub fn find_by_id(&self, id: NodeId) -> Option<&ElementNode> {
        if self.id == id {
            return Some(self);
        }

        for child in &self.children {
            if let Some(found) = child.find_by_id(id) {
                return Some(found);
            }
        }

        for slice in self.slices.values() {
            if let Some(found) = slice.find_by_id(id) {
                return Some(found);
            }
        }

        None
    }

    /// Check if this element has been modified from base.
    #[must_use]
    pub fn is_modified(&self) -> bool {
        self.source.is_modified()
    }

    /// Check if this element is sliced.
    #[must_use]
    pub fn is_sliced(&self) -> bool {
        self.slicing.is_some()
    }

    /// Get the depth of this element in the tree (number of dots in path).
    #[must_use]
    pub fn depth(&self) -> usize {
        self.path.matches('.').count()
    }

    /// Iterate over all descendant nodes (depth-first).
    pub fn descendants(&self) -> impl Iterator<Item = &ElementNode> {
        ElementIterator::new(self)
    }
}

/// Depth-first iterator over element tree.
struct ElementIterator<'a> {
    stack: Vec<&'a ElementNode>,
}

impl<'a> ElementIterator<'a> {
    fn new(root: &'a ElementNode) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for ElementIterator<'a> {
    type Item = &'a ElementNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        // Add children in reverse order so they come out in forward order
        for child in node.children.iter().rev() {
            self.stack.push(child);
        }

        // Add slices
        for slice in node.slices.values().rev() {
            self.stack.push(&slice.element);
        }

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_creation() {
        let id1 = NodeId::new();
        let id2 = NodeId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_element_node_creation() {
        let node = ElementNode::new("Patient.name".to_string());
        assert_eq!(node.path, "Patient.name");
        assert_eq!(node.short_name(), "name");
        assert_eq!(node.depth(), 1);
        assert!(!node.is_modified());
    }

    #[test]
    fn test_child_navigation() {
        let mut parent = ElementNode::new("Patient".to_string());
        let child = ElementNode::new("Patient.name".to_string());
        parent.add_child(child);

        assert_eq!(parent.children.len(), 1);
        assert!(parent.find_child("name").is_some());
        assert!(parent.find_child("unknown").is_none());
    }

    #[test]
    fn test_descendant_search() {
        let mut root = ElementNode::new("Patient".to_string());
        let mut name = ElementNode::new("Patient.name".to_string());
        let family = ElementNode::new("Patient.name.family".to_string());
        name.add_child(family);
        root.add_child(name);

        assert!(root.find_descendant("name").is_some());
        assert!(root.find_descendant("name.family").is_some());
        assert!(root.find_descendant("unknown").is_none());
    }
}
