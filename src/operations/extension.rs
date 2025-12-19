//! Extension operations for profile elements.
//!
//! This module provides operations for managing extensions:
//! - Add extension to an element
//! - Configure extension cardinality and values
//! - Remove extension

use serde_json::json;

use crate::ir::{
    Cardinality, Change, ElementNode, ElementSource, NodeId,
    ProfileDocument, TypeConstraint,
};

use super::error::{OperationError, OperationResult};
use super::traits::Operation;

// =============================================================================
// AddExtension
// =============================================================================

/// Add an extension to an element.
#[derive(Debug, Clone)]
pub struct AddExtension {
    /// Element path to add extension to.
    pub path: String,
    /// Extension URL.
    pub extension_url: String,
    /// Minimum cardinality.
    pub min: u32,
    /// Maximum cardinality.
    pub max: Option<u32>,
}

impl AddExtension {
    /// Create a new add extension operation.
    pub fn new(path: impl Into<String>, extension_url: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            extension_url: extension_url.into(),
            min: 0,
            max: Some(1),
        }
    }

    /// Set cardinality.
    pub fn with_cardinality(mut self, min: u32, max: Option<u32>) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Make extension required.
    pub fn required(mut self) -> Self {
        self.min = 1;
        self.max = Some(1);
        self
    }
}

impl Operation for AddExtension {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        // Check element exists
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }

        // Validate extension URL format
        if !self.extension_url.starts_with("http://")
            && !self.extension_url.starts_with("https://")
        {
            return Err(OperationError::InvalidExtensionContext {
                url: self.extension_url.clone(),
                path: self.path.clone(),
                reason: "Extension URL must start with http:// or https://".to_string(),
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

        // Create extension element path
        let ext_path = format!("{}.extension:{}", self.path, extension_slug(&self.extension_url));

        // Create extension element with type constraint
        let mut ext_element = ElementNode::new(ext_path);
        ext_element.constraints.cardinality = Some(Cardinality::new(self.min, self.max));
        ext_element.constraints.types.push(TypeConstraint::with_profile(
            "Extension",
            &self.extension_url,
        ));
        ext_element.source = ElementSource::Added;

        element.add_child(ext_element);
        element.source = ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let ext_slug = extension_slug(&self.extension_url);
        element.children.retain(|c| {
            !c.path.contains(&format!(".extension:{}", ext_slug))
        });

        Ok(())
    }

    fn description(&self) -> String {
        format!("Add extension {} to {}", self.extension_url, self.path)
    }

    fn as_change(&self) -> Change {
        Change::add(
            NodeId::new(),
            "children",
            json!({
                "extension_url": self.extension_url,
                "min": self.min,
                "max": self.max
            }),
        )
    }
}

// =============================================================================
// RemoveExtension
// =============================================================================

/// Remove an extension from an element.
#[derive(Debug, Clone)]
pub struct RemoveExtension {
    /// Element path.
    pub path: String,
    /// Extension URL to remove.
    pub extension_url: String,
    /// Previous extension element (for undo).
    prev_element: Option<ElementNode>,
}

impl RemoveExtension {
    /// Create a new remove extension operation.
    pub fn new(path: impl Into<String>, extension_url: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            extension_url: extension_url.into(),
            prev_element: None,
        }
    }

    fn find_extension_index(&self, element: &ElementNode) -> Option<usize> {
        let ext_slug = extension_slug(&self.extension_url);
        element.children.iter().position(|c| {
            c.path.contains(&format!(".extension:{}", ext_slug))
                || c.constraints
                    .types
                    .iter()
                    .any(|t| t.profile.contains(&self.extension_url))
        })
    }
}

impl Operation for RemoveExtension {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if self.find_extension_index(element).is_none() {
            return Err(OperationError::ExtensionNotFound {
                path: self.path.clone(),
                url: self.extension_url.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let ext_slug = extension_slug(&self.extension_url);
        element.children.retain(|c| {
            !c.path.contains(&format!(".extension:{}", ext_slug))
                && !c.constraints
                    .types
                    .iter()
                    .any(|t| t.profile.contains(&self.extension_url))
        });
        element.source = ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        if let Some(ref prev) = self.prev_element {
            let element = document
                .resource
                .find_element_mut(&self.path)
                .ok_or_else(|| OperationError::element_not_found(&self.path))?;

            element.children.push(prev.clone());
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Remove extension {} from {}", self.extension_url, self.path)
    }

    fn as_change(&self) -> Change {
        Change::remove(
            NodeId::new(),
            "children",
            json!({ "extension_url": self.extension_url }),
        )
    }
}

// =============================================================================
// SetExtensionCardinality
// =============================================================================

/// Set cardinality on an extension element.
#[derive(Debug, Clone)]
pub struct SetExtensionCardinality {
    /// Element path.
    pub path: String,
    /// Extension URL.
    pub extension_url: String,
    /// New minimum cardinality.
    pub min: u32,
    /// New maximum cardinality.
    pub max: Option<u32>,
    /// Previous cardinality (for undo).
    prev_cardinality: Option<Cardinality>,
}

impl SetExtensionCardinality {
    /// Create a new set extension cardinality operation.
    pub fn new(
        path: impl Into<String>,
        extension_url: impl Into<String>,
        min: u32,
        max: Option<u32>,
    ) -> Self {
        Self {
            path: path.into(),
            extension_url: extension_url.into(),
            min,
            max,
            prev_cardinality: None,
        }
    }

    fn find_extension_mut<'a>(
        &self,
        element: &'a mut ElementNode,
    ) -> Option<&'a mut ElementNode> {
        let ext_slug = extension_slug(&self.extension_url);
        element.children.iter_mut().find(|c| {
            c.path.contains(&format!(".extension:{}", ext_slug))
                || c.constraints
                    .types
                    .iter()
                    .any(|t| t.profile.contains(&self.extension_url))
        })
    }
}

impl Operation for SetExtensionCardinality {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Check extension exists
        let ext_slug = extension_slug(&self.extension_url);
        let has_ext = element.children.iter().any(|c| {
            c.path.contains(&format!(".extension:{}", ext_slug))
                || c.constraints
                    .types
                    .iter()
                    .any(|t| t.profile.contains(&self.extension_url))
        });

        if !has_ext {
            return Err(OperationError::ExtensionNotFound {
                path: self.path.clone(),
                url: self.extension_url.clone(),
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

        let ext = self.find_extension_mut(element).ok_or_else(|| {
            OperationError::ExtensionNotFound {
                path: self.path.clone(),
                url: self.extension_url.clone(),
            }
        })?;

        ext.constraints.cardinality = Some(Cardinality::new(self.min, self.max));
        ext.source = ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if let Some(ext) = self.find_extension_mut(element) {
            ext.constraints.cardinality = self.prev_cardinality.clone();
        }

        Ok(())
    }

    fn description(&self) -> String {
        let max_str = self.max.map_or("*".to_string(), |m| m.to_string());
        format!(
            "Set extension {} cardinality to {}..{} on {}",
            extension_slug(&self.extension_url),
            self.min,
            max_str,
            self.path
        )
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "extension.cardinality",
            self.prev_cardinality.as_ref().map(|c| json!(c)),
            json!({
                "min": self.min,
                "max": self.max
            }),
        )
    }
}

// =============================================================================
// SetExtensionFixedValue
// =============================================================================

/// Set a fixed value on an extension.
#[derive(Debug, Clone)]
pub struct SetExtensionFixedValue {
    /// Element path.
    pub path: String,
    /// Extension URL.
    pub extension_url: String,
    /// Fixed value (JSON).
    pub value: serde_json::Value,
    /// Previous fixed value (for undo).
    prev_value: Option<crate::ir::FixedValue>,
}

impl SetExtensionFixedValue {
    /// Create a new set extension fixed value operation.
    pub fn new(
        path: impl Into<String>,
        extension_url: impl Into<String>,
        value: serde_json::Value,
    ) -> Self {
        Self {
            path: path.into(),
            extension_url: extension_url.into(),
            value,
            prev_value: None,
        }
    }

    fn find_extension_mut<'a>(
        &self,
        element: &'a mut ElementNode,
    ) -> Option<&'a mut ElementNode> {
        let ext_slug = extension_slug(&self.extension_url);
        element.children.iter_mut().find(|c| {
            c.path.contains(&format!(".extension:{}", ext_slug))
                || c.constraints
                    .types
                    .iter()
                    .any(|t| t.profile.contains(&self.extension_url))
        })
    }
}

impl Operation for SetExtensionFixedValue {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let ext_slug = extension_slug(&self.extension_url);
        let has_ext = element.children.iter().any(|c| {
            c.path.contains(&format!(".extension:{}", ext_slug))
                || c.constraints
                    .types
                    .iter()
                    .any(|t| t.profile.contains(&self.extension_url))
        });

        if !has_ext {
            return Err(OperationError::ExtensionNotFound {
                path: self.path.clone(),
                url: self.extension_url.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let ext = self.find_extension_mut(element).ok_or_else(|| {
            OperationError::ExtensionNotFound {
                path: self.path.clone(),
                url: self.extension_url.clone(),
            }
        })?;

        ext.constraints.fixed_value = Some(crate::ir::FixedValue::fixed(self.value.clone()));
        ext.source = ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        if let Some(ext) = self.find_extension_mut(element) {
            ext.constraints.fixed_value = self.prev_value.clone();
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!(
            "Set fixed value on extension {} at {}",
            extension_slug(&self.extension_url),
            self.path
        )
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "extension.fixed_value",
            self.prev_value.as_ref().map(|v| json!(v)),
            json!({ "type": "Fixed", "value": self.value }),
        )
    }
}

/// Extract a short slug from an extension URL for use in paths.
fn extension_slug(url: &str) -> String {
    url.rsplit('/')
        .next()
        .unwrap_or("ext")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, DocumentMetadata, FhirVersion, ProfiledResource};

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
    fn test_add_extension() {
        let mut doc = create_test_document();

        let op = AddExtension::new(
            "Patient.name",
            "http://example.org/fhir/StructureDefinition/namePrefix",
        );

        assert!(op.validate(&doc).is_ok());
        op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.name").unwrap();
        assert!(element.children.iter().any(|c| c.path.contains("extension:namePrefix")));
    }

    #[test]
    fn test_extension_slug() {
        assert_eq!(
            extension_slug("http://example.org/fhir/StructureDefinition/myExtension"),
            "myExtension"
        );
        assert_eq!(
            extension_slug("http://hl7.org/fhir/StructureDefinition/patient-birthPlace"),
            "patient-birthPlace"
        );
    }
}
