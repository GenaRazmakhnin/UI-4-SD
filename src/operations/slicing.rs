//! Slicing operations for profile elements.
//!
//! This module provides operations for managing element slicing:
//! - Create slicing on an element
//! - Add/remove slices
//! - Configure discriminators

use serde_json::json;

use crate::ir::{
    Cardinality, Change, Discriminator, DiscriminatorType, NodeId,
    ProfileDocument, SliceNode, SlicingDefinition, SlicingRules,
};

use super::error::{OperationError, OperationResult};
use super::traits::Operation;

// =============================================================================
// CreateSlicing
// =============================================================================

/// Create slicing definition on an element.
#[derive(Debug, Clone)]
pub struct CreateSlicing {
    /// Element path.
    pub path: String,
    /// Discriminators for the slicing.
    pub discriminators: Vec<Discriminator>,
    /// Slicing rules.
    pub rules: SlicingRules,
    /// Whether slices are ordered.
    pub ordered: bool,
    /// Description of the slicing.
    pub description: Option<String>,
    /// Previous slicing (for undo).
    prev_slicing: Option<SlicingDefinition>,
}

impl CreateSlicing {
    /// Create a new create slicing operation.
    pub fn new(path: impl Into<String>, discriminators: Vec<Discriminator>) -> Self {
        Self {
            path: path.into(),
            discriminators,
            rules: SlicingRules::Open,
            ordered: false,
            description: None,
            prev_slicing: None,
        }
    }

    /// Create slicing by value.
    pub fn by_value(path: impl Into<String>, discriminator_path: impl Into<String>) -> Self {
        Self::new(path, vec![Discriminator::value(discriminator_path)])
    }

    /// Create slicing by type.
    pub fn by_type(path: impl Into<String>) -> Self {
        Self::new(path, vec![Discriminator::by_type("$this")])
    }

    /// Create slicing by profile.
    pub fn by_profile(path: impl Into<String>) -> Self {
        Self::new(path, vec![Discriminator::by_profile("$this")])
    }

    /// Set slicing rules.
    pub fn with_rules(mut self, rules: SlicingRules) -> Self {
        self.rules = rules;
        self
    }

    /// Set ordered flag.
    pub fn ordered(mut self, ordered: bool) -> Self {
        self.ordered = ordered;
        self
    }

    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

impl Operation for CreateSlicing {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Check if slicing already exists
        if element.slicing.is_some() {
            return Err(OperationError::SlicingAlreadyExists {
                path: self.path.clone(),
            });
        }

        // Validate discriminators
        if self.discriminators.is_empty() {
            return Err(OperationError::InvalidDiscriminatorPath {
                path: "empty discriminator list".to_string(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let mut slicing = SlicingDefinition::new(self.discriminators.clone())
            .with_rules(self.rules)
            .ordered(self.ordered);

        if let Some(ref desc) = self.description {
            slicing = slicing.with_description(desc);
        }

        element.slicing = Some(slicing);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.slicing = self.prev_slicing.clone();

        Ok(())
    }

    fn description(&self) -> String {
        format!("Create slicing on {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "slicing",
            self.prev_slicing.as_ref().map(|s| json!(s)),
            json!({
                "discriminators": self.discriminators,
                "rules": self.rules.as_str(),
                "ordered": self.ordered,
                "description": self.description
            }),
        )
    }
}

// =============================================================================
// RemoveSlicing
// =============================================================================

/// Remove slicing definition from an element.
#[derive(Debug, Clone)]
pub struct RemoveSlicing {
    /// Element path.
    pub path: String,
    /// Previous slicing (for undo).
    prev_slicing: Option<SlicingDefinition>,
    /// Previous slices (for undo).
    prev_slices: Vec<(String, SliceNode)>,
}

impl RemoveSlicing {
    /// Create a new remove slicing operation.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            prev_slicing: None,
            prev_slices: Vec::new(),
        }
    }
}

impl Operation for RemoveSlicing {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if element.slicing.is_none() {
            return Err(OperationError::NoSlicingDefined {
                path: self.path.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.slicing = None;
        element.slices.clear();
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.slicing = self.prev_slicing.clone();
        for (name, slice) in &self.prev_slices {
            element.slices.insert(name.clone(), slice.clone());
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Remove slicing from {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::clear(NodeId::new(), "slicing", json!(self.prev_slicing))
    }
}

// =============================================================================
// AddSlice
// =============================================================================

/// Add a named slice to a sliced element.
#[derive(Debug, Clone)]
pub struct AddSlice {
    /// Element path (must have slicing).
    pub path: String,
    /// Slice name.
    pub name: String,
    /// Minimum cardinality.
    pub min: u32,
    /// Maximum cardinality.
    pub max: Option<u32>,
}

impl AddSlice {
    /// Create a new add slice operation.
    pub fn new(
        path: impl Into<String>,
        name: impl Into<String>,
        min: u32,
        max: Option<u32>,
    ) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            min,
            max,
        }
    }

    /// Create an optional slice (0..1).
    pub fn optional(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(path, name, 0, Some(1))
    }

    /// Create a required slice (1..1).
    pub fn required(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(path, name, 1, Some(1))
    }
}

impl Operation for AddSlice {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Check slicing exists
        if element.slicing.is_none() {
            return Err(OperationError::NoSlicingDefined {
                path: self.path.clone(),
            });
        }

        // Check for duplicate slice name
        if element.slices.contains_key(&self.name) {
            return Err(OperationError::DuplicateSliceName {
                path: self.path.clone(),
                name: self.name.clone(),
            });
        }

        // Validate cardinality
        if let Some(max) = self.max {
            if self.min > max {
                return Err(OperationError::invalid_cardinality(self.min, max));
            }
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Create slice path
        let slice_path = format!("{}:{}", self.path, self.name);
        let slice = SliceNode::with_path(&self.name, &slice_path)
            .with_cardinality(Cardinality::new(self.min, self.max));

        element.slices.insert(self.name.clone(), slice);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.slices.shift_remove(&self.name);

        Ok(())
    }

    fn description(&self) -> String {
        format!("Add slice '{}' to {}", self.name, self.path)
    }

    fn as_change(&self) -> Change {
        Change::add(
            NodeId::new(),
            "slices",
            json!({
                "name": self.name,
                "min": self.min,
                "max": self.max
            }),
        )
    }
}

// =============================================================================
// RemoveSlice
// =============================================================================

/// Remove a named slice from a sliced element.
#[derive(Debug, Clone)]
pub struct RemoveSlice {
    /// Element path.
    pub path: String,
    /// Slice name to remove.
    pub name: String,
    /// Previous slice (for undo).
    prev_slice: Option<SliceNode>,
}

impl RemoveSlice {
    /// Create a new remove slice operation.
    pub fn new(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            prev_slice: None,
        }
    }
}

impl Operation for RemoveSlice {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if !element.slices.contains_key(&self.name) {
            return Err(OperationError::slice_not_found(&self.path, &self.name));
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.slices.shift_remove(&self.name);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        if let Some(ref prev) = self.prev_slice {
            let element = document
                .resource
                .find_element_mut(&self.path)
                .ok_or_else(|| OperationError::element_not_found(&self.path))?;

            element.slices.insert(self.name.clone(), prev.clone());
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Remove slice '{}' from {}", self.name, self.path)
    }

    fn as_change(&self) -> Change {
        Change::remove(
            NodeId::new(),
            "slices",
            json!({ "name": self.name }),
        )
    }
}

// =============================================================================
// AddDiscriminator
// =============================================================================

/// Add a discriminator to a sliced element.
#[derive(Debug, Clone)]
pub struct AddDiscriminator {
    /// Element path (must have slicing).
    pub path: String,
    /// Discriminator type.
    pub discriminator_type: DiscriminatorType,
    /// Discriminator path.
    pub discriminator_path: String,
}

impl AddDiscriminator {
    /// Create a new add discriminator operation.
    pub fn new(
        path: impl Into<String>,
        discriminator_type: DiscriminatorType,
        discriminator_path: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            discriminator_type,
            discriminator_path: discriminator_path.into(),
        }
    }

    /// Add a value discriminator.
    pub fn value(path: impl Into<String>, discriminator_path: impl Into<String>) -> Self {
        Self::new(path, DiscriminatorType::Value, discriminator_path)
    }

    /// Add a type discriminator.
    pub fn by_type(path: impl Into<String>) -> Self {
        Self::new(path, DiscriminatorType::Type, "$this")
    }
}

impl Operation for AddDiscriminator {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if element.slicing.is_none() {
            return Err(OperationError::NoSlicingDefined {
                path: self.path.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let slicing = element.slicing.as_mut().ok_or_else(|| {
            OperationError::NoSlicingDefined {
                path: self.path.clone(),
            }
        })?;

        slicing.discriminator.push(Discriminator::new(
            self.discriminator_type,
            &self.discriminator_path,
        ));
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if let Some(ref mut slicing) = element.slicing {
            slicing.discriminator.retain(|d| {
                !(d.discriminator_type == self.discriminator_type
                    && d.path == self.discriminator_path)
            });
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!(
            "Add {} discriminator '{}' to {}",
            self.discriminator_type, self.discriminator_path, self.path
        )
    }

    fn as_change(&self) -> Change {
        Change::add(
            NodeId::new(),
            "slicing.discriminator",
            json!({
                "type": self.discriminator_type.as_str(),
                "path": self.discriminator_path
            }),
        )
    }
}

// =============================================================================
// SetSlicingRules
// =============================================================================

/// Set slicing rules on an element.
#[derive(Debug, Clone)]
pub struct SetSlicingRules {
    /// Element path.
    pub path: String,
    /// New slicing rules.
    pub rules: SlicingRules,
    /// Previous rules (for undo).
    prev_rules: Option<SlicingRules>,
}

impl SetSlicingRules {
    /// Create a new set slicing rules operation.
    pub fn new(path: impl Into<String>, rules: SlicingRules) -> Self {
        Self {
            path: path.into(),
            rules,
            prev_rules: None,
        }
    }
}

impl Operation for SetSlicingRules {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if element.slicing.is_none() {
            return Err(OperationError::NoSlicingDefined {
                path: self.path.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if let Some(ref mut slicing) = element.slicing {
            slicing.rules = self.rules;
        }
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        if let Some(prev) = self.prev_rules {
            let element = document
                .resource
                .find_element_mut(&self.path)
                .ok_or_else(|| OperationError::element_not_found(&self.path))?;

            if let Some(ref mut slicing) = element.slicing {
                slicing.rules = prev;
            }
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Set slicing rules to {} on {}", self.rules, self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "slicing.rules",
            self.prev_rules.map(|r| json!(r.as_str())),
            json!(self.rules.as_str()),
        )
    }
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

        // Add test element for slicing
        let identifier = ElementNode::new("Patient.identifier".to_string());
        doc.resource.root.add_child(identifier);

        doc
    }

    #[test]
    fn test_create_slicing() {
        let mut doc = create_test_document();

        let op = CreateSlicing::by_value("Patient.identifier", "system")
            .with_rules(SlicingRules::Open)
            .with_description("Sliced by system");

        assert!(op.validate(&doc).is_ok());
        op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.identifier").unwrap();
        assert!(element.slicing.is_some());
        let slicing = element.slicing.as_ref().unwrap();
        assert_eq!(slicing.rules, SlicingRules::Open);
        assert_eq!(slicing.discriminator.len(), 1);
    }

    #[test]
    fn test_add_slice() {
        let mut doc = create_test_document();

        // First create slicing
        let create_op = CreateSlicing::by_value("Patient.identifier", "system");
        create_op.apply(&mut doc).unwrap();

        // Then add slice
        let add_op = AddSlice::required("Patient.identifier", "ssn");
        assert!(add_op.validate(&doc).is_ok());
        add_op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.identifier").unwrap();
        assert!(element.slices.contains_key("ssn"));
    }

    #[test]
    fn test_add_slice_without_slicing() {
        let doc = create_test_document();

        let op = AddSlice::required("Patient.identifier", "ssn");
        assert!(matches!(
            op.validate(&doc),
            Err(OperationError::NoSlicingDefined { .. })
        ));
    }

    #[test]
    fn test_duplicate_slice_name() {
        let mut doc = create_test_document();

        // Create slicing and add first slice
        CreateSlicing::by_value("Patient.identifier", "system")
            .apply(&mut doc)
            .unwrap();
        AddSlice::required("Patient.identifier", "ssn")
            .apply(&mut doc)
            .unwrap();

        // Try to add duplicate
        let op = AddSlice::required("Patient.identifier", "ssn");
        assert!(matches!(
            op.validate(&doc),
            Err(OperationError::DuplicateSliceName { .. })
        ));
    }
}
