//! Snapshot generation for StructureDefinition export.
//!
//! Generates complete snapshot from the IR element tree.
//! The snapshot contains all elements with their full constraints,
//! including inherited values from the base definition.

use serde_json::Value;

use crate::ir::{ElementNode, ProfiledResource};

use super::deterministic::sort_elements_by_path;
use super::element_serializer::ElementSerializer;
use super::error::{ExportError, ExportResult};

/// Generates snapshot element definitions from IR.
#[derive(Debug, Default)]
pub struct SnapshotGenerator {
    /// Element serializer.
    serializer: ElementSerializer,
}

impl SnapshotGenerator {
    /// Create a new snapshot generator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            serializer: ElementSerializer::new().include_inherited(true),
        }
    }

    /// Generate snapshot elements from a profiled resource.
    ///
    /// Returns a vector of ElementDefinition JSON values in canonical order.
    pub async fn generate(&self, resource: &ProfiledResource) -> ExportResult<Vec<Value>> {
        let mut elements = Vec::new();

        // Recursively collect all elements from the tree
        self.collect_elements(&resource.root, &mut elements)?;

        // Sort elements by path for deterministic ordering
        sort_elements_by_path(&mut elements);

        // Validate snapshot element count
        let expected_count = count_all_elements(&resource.root);
        if elements.len() != expected_count {
            return Err(ExportError::snapshot_generation(
                &resource.root.path,
                format!(
                    "Element count mismatch: expected {}, got {}",
                    expected_count,
                    elements.len()
                ),
            ));
        }

        Ok(elements)
    }

    /// Recursively collect elements from the tree.
    fn collect_elements(
        &self,
        element: &ElementNode,
        elements: &mut Vec<Value>,
    ) -> ExportResult<()> {
        // Serialize the current element
        let serialized = self.serializer.serialize_element(element)?;
        elements.push(serialized);

        // Collect children
        for child in &element.children {
            self.collect_elements(child, elements)?;
        }

        // Collect slices (if any)
        for (name, slice) in &element.slices {
            // Serialize the slice element
            let slice_value = self.serializer.serialize_slice(slice, &element.path)?;
            elements.push(slice_value);

            // Collect slice children
            self.collect_slice_children(&slice.element, &element.path, name, elements)?;
        }

        Ok(())
    }

    /// Collect children of a slice element.
    fn collect_slice_children(
        &self,
        slice_element: &ElementNode,
        parent_path: &str,
        slice_name: &str,
        elements: &mut Vec<Value>,
    ) -> ExportResult<()> {
        for child in &slice_element.children {
            // Adjust child path to include slice name
            let child_path = format!("{}:{}.{}", parent_path, slice_name, child.short_name());
            let mut child_serialized = self.serializer.serialize_element(child)?;

            // Update path and id to include slice context
            if let Some(obj) = child_serialized.as_object_mut() {
                obj.insert("path".to_string(), Value::String(child_path.clone()));
                obj.insert("id".to_string(), Value::String(child_path));
            }

            elements.push(child_serialized);

            // Recursively collect nested children
            self.collect_nested_slice_children(child, parent_path, slice_name, elements)?;
        }

        Ok(())
    }

    /// Collect nested children within a slice.
    fn collect_nested_slice_children(
        &self,
        element: &ElementNode,
        parent_path: &str,
        slice_name: &str,
        elements: &mut Vec<Value>,
    ) -> ExportResult<()> {
        for child in &element.children {
            let relative_path = child.path.strip_prefix(&element.path)
                .and_then(|s| s.strip_prefix('.'))
                .unwrap_or(child.short_name());

            let child_path = format!("{}:{}.{}", parent_path, slice_name, relative_path);
            let mut child_serialized = self.serializer.serialize_element(child)?;

            if let Some(obj) = child_serialized.as_object_mut() {
                obj.insert("path".to_string(), Value::String(child_path.clone()));
                obj.insert("id".to_string(), Value::String(child_path));
            }

            elements.push(child_serialized);

            self.collect_nested_slice_children(child, parent_path, slice_name, elements)?;
        }

        Ok(())
    }
}

/// Count all elements in a tree (including slices).
fn count_all_elements(root: &ElementNode) -> usize {
    let mut count = 1; // Count this element

    for child in &root.children {
        count += count_all_elements(child);
    }

    for slice in root.slices.values() {
        count += 1; // The slice element itself
        count += count_slice_children(&slice.element);
    }

    count
}

/// Count children within a slice.
fn count_slice_children(element: &ElementNode) -> usize {
    let mut count = 0;

    for child in &element.children {
        count += 1;
        count += count_slice_children(child);
    }

    count
}

/// Snapshot generator configuration.
#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    /// Whether to validate element IDs.
    pub validate_ids: bool,
    /// Whether to include base element info.
    pub include_base: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            validate_ids: true,
            include_base: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, Cardinality, ElementNode, FhirVersion, ProfiledResource};

    fn create_test_resource() -> ProfiledResource {
        let mut resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        // Add some child elements
        let mut name = ElementNode::new("Patient.name".to_string());
        name.constraints.cardinality = Some(Cardinality::required());

        let family = ElementNode::new("Patient.name.family".to_string());
        name.add_child(family);

        resource.root.add_child(name);

        resource
    }

    #[tokio::test]
    async fn test_generate_snapshot() {
        let resource = create_test_resource();
        let generator = SnapshotGenerator::new();

        let elements = generator.generate(&resource).await.unwrap();

        // Should have: Patient, Patient.name, Patient.name.family
        assert_eq!(elements.len(), 3);

        // First element should be root
        assert_eq!(elements[0].get("path").unwrap(), "Patient");
    }

    #[tokio::test]
    async fn test_element_ordering() {
        let mut resource = create_test_resource();

        // Add more elements
        let identifier = ElementNode::new("Patient.identifier".to_string());
        let gender = ElementNode::new("Patient.gender".to_string());

        resource.root.add_child(identifier);
        resource.root.add_child(gender);

        let generator = SnapshotGenerator::new();
        let elements = generator.generate(&resource).await.unwrap();

        // Check ordering
        let paths: Vec<&str> = elements
            .iter()
            .filter_map(|e| e.get("path").and_then(Value::as_str))
            .collect();

        // Should be alphabetically ordered by path after root
        assert_eq!(paths[0], "Patient");
        assert!(paths.contains(&"Patient.gender"));
        assert!(paths.contains(&"Patient.identifier"));
        assert!(paths.contains(&"Patient.name"));
    }
}
