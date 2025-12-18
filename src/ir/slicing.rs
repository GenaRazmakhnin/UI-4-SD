//! Slicing definitions for FHIR profile elements.
//!
//! This module defines types for element slicing, which allows profiles to
//! define named "slices" of repeating elements with different constraints.
//!
//! # FHIR Slicing Overview
//!
//! Slicing allows a profile to say "this repeating element has these named
//! variations, each with specific constraints". For example, Patient.identifier
//! might be sliced into "ssn", "mrn", and "passport" slices.
//!
//! ```text
//! Patient.identifier [sliced]
//! ├── ssn (slice): type = SSN, cardinality 0..1
//! ├── mrn (slice): type = MRN, cardinality 1..1
//! └── passport (slice): type = Passport, cardinality 0..*
//! ```

use serde::{Deserialize, Serialize};

use super::constraint::ElementConstraints;
use super::element::{ElementNode, ElementSource, NodeId};

/// How slices are discriminated (identified).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DiscriminatorType {
    /// Slice by value of an element.
    #[default]
    Value,
    /// Slice by whether an element exists.
    Exists,
    /// Slice by pattern match.
    Pattern,
    /// Slice by type of element.
    Type,
    /// Slice by profile conformance.
    Profile,
    /// Slice by position in the instance.
    Position,
}

impl DiscriminatorType {
    /// Get the FHIR code for this discriminator type.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Exists => "exists",
            Self::Pattern => "pattern",
            Self::Type => "type",
            Self::Profile => "profile",
            Self::Position => "position",
        }
    }
}

impl std::fmt::Display for DiscriminatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A discriminator that identifies which slice an element belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Discriminator {
    /// Type of discrimination.
    #[serde(rename = "type")]
    pub discriminator_type: DiscriminatorType,

    /// FHIRPath expression to the discriminating element.
    pub path: String,
}

impl Discriminator {
    /// Create a new discriminator.
    #[must_use]
    pub fn new(discriminator_type: DiscriminatorType, path: impl Into<String>) -> Self {
        Self {
            discriminator_type,
            path: path.into(),
        }
    }

    /// Create a value discriminator.
    #[must_use]
    pub fn value(path: impl Into<String>) -> Self {
        Self::new(DiscriminatorType::Value, path)
    }

    /// Create a type discriminator.
    #[must_use]
    pub fn by_type(path: impl Into<String>) -> Self {
        Self::new(DiscriminatorType::Type, path)
    }

    /// Create a profile discriminator.
    #[must_use]
    pub fn by_profile(path: impl Into<String>) -> Self {
        Self::new(DiscriminatorType::Profile, path)
    }

    /// Create an exists discriminator.
    #[must_use]
    pub fn exists(path: impl Into<String>) -> Self {
        Self::new(DiscriminatorType::Exists, path)
    }
}

/// Rules for how slices are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SlicingRules {
    /// Only the defined slices are allowed.
    Closed,
    /// Additional slices are allowed.
    #[default]
    Open,
    /// Additional slices are allowed, but must come after defined slices.
    OpenAtEnd,
}

impl SlicingRules {
    /// Check if additional slices are allowed.
    #[must_use]
    pub const fn allows_additional(&self) -> bool {
        matches!(self, Self::Open | Self::OpenAtEnd)
    }

    /// Get the FHIR code for this slicing rule.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
            Self::OpenAtEnd => "openAtEnd",
        }
    }
}

impl std::fmt::Display for SlicingRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Slicing definition for an element.
///
/// Defines how a repeating element is divided into named slices,
/// each with its own constraints.
///
/// # Example
///
/// ```
/// use niten::ir::{SlicingDefinition, Discriminator, SlicingRules};
///
/// let slicing = SlicingDefinition::new(vec![
///     Discriminator::value("system"),
/// ])
/// .with_rules(SlicingRules::Open)
/// .with_description("Sliced by identifier system");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlicingDefinition {
    /// Discriminators that identify slices.
    pub discriminator: Vec<Discriminator>,

    /// Human-readable description of the slicing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether element order matters.
    #[serde(default)]
    pub ordered: bool,

    /// Rules for handling additional content.
    #[serde(default)]
    pub rules: SlicingRules,
}

impl SlicingDefinition {
    /// Create a new slicing definition with discriminators.
    #[must_use]
    pub fn new(discriminator: Vec<Discriminator>) -> Self {
        Self {
            discriminator,
            description: None,
            ordered: false,
            rules: SlicingRules::default(),
        }
    }

    /// Create slicing by a single value path.
    #[must_use]
    pub fn by_value(path: impl Into<String>) -> Self {
        Self::new(vec![Discriminator::value(path)])
    }

    /// Create slicing by type.
    #[must_use]
    pub fn by_type() -> Self {
        Self::new(vec![Discriminator::by_type("$this")])
    }

    /// Create slicing by profile.
    #[must_use]
    pub fn by_profile() -> Self {
        Self::new(vec![Discriminator::by_profile("$this")])
    }

    /// Set the slicing rules.
    #[must_use]
    pub const fn with_rules(mut self, rules: SlicingRules) -> Self {
        self.rules = rules;
        self
    }

    /// Set whether slices are ordered.
    #[must_use]
    pub const fn ordered(mut self, ordered: bool) -> Self {
        self.ordered = ordered;
        self
    }

    /// Add a description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Check if this slicing allows additional (undefined) slices.
    #[must_use]
    pub const fn allows_additional(&self) -> bool {
        self.rules.allows_additional()
    }
}

/// A named slice within a sliced element.
///
/// Each slice has a name, constraints, and potentially child elements.
///
/// # Example
///
/// ```
/// use niten::ir::{SliceNode, Cardinality, ElementConstraints};
///
/// let slice = SliceNode::new("official")
///     .with_cardinality(Cardinality::required());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceNode {
    /// Stable unique identifier for UI operations.
    pub id: NodeId,

    /// Slice name (e.g., "official", "ssn").
    pub name: String,

    /// The element representing this slice (with its constraints and children).
    pub element: ElementNode,

    /// Parent node ID (the sliced element).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NodeId>,

    /// Whether this slice was added by this profile or inherited.
    #[serde(default)]
    pub source: ElementSource,
}

impl SliceNode {
    /// Create a new slice with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let id = NodeId::new();
        Self {
            id,
            name: name.clone(),
            element: ElementNode::with_id(id, name),
            parent_id: None,
            source: ElementSource::Added,
        }
    }

    /// Create a slice with a specific path.
    #[must_use]
    pub fn with_path(name: impl Into<String>, path: impl Into<String>) -> Self {
        let name = name.into();
        let id = NodeId::new();
        Self {
            id,
            name,
            element: ElementNode::with_id(id, path.into()),
            parent_id: None,
            source: ElementSource::Added,
        }
    }

    /// Set cardinality for this slice.
    #[must_use]
    pub fn with_cardinality(mut self, cardinality: super::constraint::Cardinality) -> Self {
        self.element.constraints.cardinality = Some(cardinality);
        self
    }

    /// Set constraints for this slice.
    #[must_use]
    pub fn with_constraints(mut self, constraints: ElementConstraints) -> Self {
        self.element.constraints = constraints;
        self
    }

    /// Get mutable access to the element's constraints.
    pub fn constraints_mut(&mut self) -> &mut ElementConstraints {
        self.element.constraints_mut()
    }

    /// Add a child element to this slice.
    pub fn add_child(&mut self, child: ElementNode) {
        self.element.add_child(child);
    }

    /// Find a node by ID within this slice.
    #[must_use]
    pub fn find_by_id(&self, id: NodeId) -> Option<&ElementNode> {
        if self.id == id {
            return Some(&self.element);
        }
        self.element.find_by_id(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discriminator_creation() {
        let disc = Discriminator::value("system");
        assert_eq!(disc.discriminator_type, DiscriminatorType::Value);
        assert_eq!(disc.path, "system");
    }

    #[test]
    fn test_slicing_definition() {
        let slicing = SlicingDefinition::by_value("type.coding.system")
            .with_rules(SlicingRules::Closed)
            .with_description("Sliced by coding system");

        assert_eq!(slicing.discriminator.len(), 1);
        assert_eq!(slicing.rules, SlicingRules::Closed);
        assert!(!slicing.allows_additional());
    }

    #[test]
    fn test_slice_node() {
        let slice = SliceNode::new("official")
            .with_cardinality(super::super::constraint::Cardinality::required());

        assert_eq!(slice.name, "official");
        assert!(slice.element.constraints.cardinality.is_some());
    }

    #[test]
    fn test_slicing_rules() {
        assert!(SlicingRules::Open.allows_additional());
        assert!(SlicingRules::OpenAtEnd.allows_additional());
        assert!(!SlicingRules::Closed.allows_additional());
    }
}
