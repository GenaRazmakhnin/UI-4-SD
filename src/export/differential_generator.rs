//! Differential generation for StructureDefinition export.
//!
//! Generates minimal differential containing only modified elements.
//! The differential shows what constraints were added or changed
//! compared to the base definition.

use serde_json::Value;

use crate::ir::{ElementNode, ProfiledResource};

use super::deterministic::sort_elements_by_path;
use super::element_serializer::ElementSerializer;
use super::error::ExportResult;

/// Generates differential element definitions from IR.
#[derive(Debug, Default)]
pub struct DifferentialGenerator {
    /// Element serializer (configured for differential output).
    serializer: ElementSerializer,
}

impl DifferentialGenerator {
    /// Create a new differential generator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            serializer: ElementSerializer::new().include_inherited(false),
        }
    }

    /// Generate differential elements from a profiled resource.
    ///
    /// Returns a vector of ElementDefinition JSON values containing
    /// only modified elements in canonical order.
    pub async fn generate(&self, resource: &ProfiledResource) -> ExportResult<Vec<Value>> {
        let mut elements = Vec::new();

        // Always include root element in differential
        let root_element = self.serializer.serialize_element(&resource.root)?;
        elements.push(root_element);

        // Collect modified elements
        self.collect_modified_elements(&resource.root, &mut elements)?;

        // Sort elements by path for deterministic ordering
        sort_elements_by_path(&mut elements);

        Ok(elements)
    }

    /// Recursively collect modified elements from the tree.
    fn collect_modified_elements(
        &self,
        element: &ElementNode,
        elements: &mut Vec<Value>,
    ) -> ExportResult<()> {
        // Process children
        for child in &element.children {
            if self.should_include_element(child) {
                if let Some(serialized) = self.serializer.serialize_element_differential(child)? {
                    elements.push(serialized);
                }

                // Recursively check for modified descendants even if parent isn't modified
                self.collect_modified_elements(child, elements)?;
            } else {
                // Still check children for modifications
                self.collect_modified_elements(child, elements)?;
            }
        }

        // Process slices (always include in differential as they are modifications)
        for (_, slice) in &element.slices {
            let slice_value = self.serializer.serialize_slice(slice, &element.path)?;
            elements.push(slice_value);

            // Include slice children that are modified
            self.collect_slice_modified_children(&slice.element, &element.path, &slice.name, elements)?;
        }

        Ok(())
    }

    /// Collect modified children within a slice.
    fn collect_slice_modified_children(
        &self,
        slice_element: &ElementNode,
        parent_path: &str,
        slice_name: &str,
        elements: &mut Vec<Value>,
    ) -> ExportResult<()> {
        for child in &slice_element.children {
            if self.should_include_element(child) {
                // Adjust child path to include slice name
                let child_path = format!("{}:{}.{}", parent_path, slice_name, child.short_name());
                let mut child_serialized = self.serializer.serialize_element(child)?;

                // Update path and id to include slice context
                if let Some(obj) = child_serialized.as_object_mut() {
                    obj.insert("path".to_string(), Value::String(child_path.clone()));
                    obj.insert("id".to_string(), Value::String(child_path));
                }

                elements.push(child_serialized);
            }

            // Recursively collect nested modifications
            self.collect_slice_modified_children(child, parent_path, slice_name, elements)?;
        }

        Ok(())
    }

    /// Determine if an element should be included in the differential.
    fn should_include_element(&self, element: &ElementNode) -> bool {
        // Include if explicitly modified
        if element.source.is_modified() {
            return true;
        }

        // Include if has any constraints set
        if element.constraints.has_any() {
            return true;
        }

        // Include if has slicing
        if element.slicing.is_some() {
            return true;
        }

        // Include if any descendant is modified
        self.has_modified_descendants(element)
    }

    /// Check if any descendant is modified.
    fn has_modified_descendants(&self, element: &ElementNode) -> bool {
        for child in &element.children {
            if child.source.is_modified() || child.constraints.has_any() || child.slicing.is_some() {
                return true;
            }
            if self.has_modified_descendants(child) {
                return true;
            }
        }

        // Check slices
        !element.slices.is_empty()
    }
}

/// Analyzer for determining which elements belong in the differential.
#[derive(Debug, Default)]
pub struct DifferentialAnalyzer {
    /// Collected paths of modified elements.
    modified_paths: Vec<String>,
}

impl DifferentialAnalyzer {
    /// Create a new analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            modified_paths: Vec::new(),
        }
    }

    /// Analyze a resource and return paths of all modified elements.
    pub fn analyze(&mut self, resource: &ProfiledResource) -> &[String] {
        self.modified_paths.clear();
        self.collect_modified_paths(&resource.root);
        &self.modified_paths
    }

    /// Recursively collect paths of modified elements.
    fn collect_modified_paths(&mut self, element: &ElementNode) {
        if element.source.is_modified() || element.constraints.has_any() || element.slicing.is_some() {
            self.modified_paths.push(element.path.clone());
        }

        for child in &element.children {
            self.collect_modified_paths(child);
        }

        for (name, slice) in &element.slices {
            let slice_path = format!("{}:{}", element.path, name);
            self.modified_paths.push(slice_path);
            self.collect_modified_paths(&slice.element);
        }
    }
}

/// Statistics about the differential.
#[derive(Debug, Clone, Default)]
pub struct DifferentialStats {
    /// Total number of elements in differential.
    pub element_count: usize,
    /// Number of elements with cardinality changes.
    pub cardinality_changes: usize,
    /// Number of elements with type constraints.
    pub type_constraints: usize,
    /// Number of elements with bindings.
    pub binding_constraints: usize,
    /// Number of elements with mustSupport.
    pub must_support_count: usize,
    /// Number of sliced elements.
    pub sliced_elements: usize,
    /// Number of slice definitions.
    pub slice_count: usize,
}

impl DifferentialStats {
    /// Compute statistics from a profiled resource.
    pub fn from_resource(resource: &ProfiledResource) -> Self {
        let mut stats = Self::default();
        stats.collect_stats(&resource.root);
        stats
    }

    fn collect_stats(&mut self, element: &ElementNode) {
        if element.source.is_modified() || element.constraints.has_any() {
            self.element_count += 1;

            if element.constraints.cardinality.is_some() {
                self.cardinality_changes += 1;
            }

            if !element.constraints.types.is_empty() {
                self.type_constraints += 1;
            }

            if element.constraints.binding.is_some() {
                self.binding_constraints += 1;
            }

            if element.constraints.flags.must_support {
                self.must_support_count += 1;
            }
        }

        if element.slicing.is_some() {
            self.sliced_elements += 1;
        }

        self.slice_count += element.slices.len();

        for child in &element.children {
            self.collect_stats(child);
        }

        for slice in element.slices.values() {
            self.collect_stats(&slice.element);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, Cardinality, ElementNode, ElementSource, FhirVersion, ProfiledResource};

    fn create_test_resource() -> ProfiledResource {
        let mut resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        // Add a modified element
        let mut name = ElementNode::new("Patient.name".to_string());
        name.source = ElementSource::Modified;
        name.constraints.cardinality = Some(Cardinality::required());

        // Add an unmodified child
        let family = ElementNode::new("Patient.name.family".to_string());
        name.add_child(family);

        resource.root.add_child(name);

        // Add an unmodified element
        let identifier = ElementNode::new("Patient.identifier".to_string());
        resource.root.add_child(identifier);

        resource
    }

    #[tokio::test]
    async fn test_generate_differential() {
        let resource = create_test_resource();
        let generator = DifferentialGenerator::new();

        let elements = generator.generate(&resource).await.unwrap();

        // Should have: Patient (root), Patient.name (modified)
        // Should NOT have: Patient.identifier (not modified), Patient.name.family (not modified)
        assert!(elements.len() >= 2);

        let paths: Vec<&str> = elements
            .iter()
            .filter_map(|e| e.get("path").and_then(Value::as_str))
            .collect();

        assert!(paths.contains(&"Patient"));
        assert!(paths.contains(&"Patient.name"));
    }

    #[tokio::test]
    async fn test_differential_excludes_unmodified() {
        let mut resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        // Add only unmodified elements
        let name = ElementNode::new("Patient.name".to_string());
        resource.root.add_child(name);

        let generator = DifferentialGenerator::new();
        let elements = generator.generate(&resource).await.unwrap();

        // Should only have the root element
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].get("path").unwrap(), "Patient");
    }

    #[test]
    fn test_differential_stats() {
        let resource = create_test_resource();
        let stats = DifferentialStats::from_resource(&resource);

        assert!(stats.cardinality_changes > 0);
    }

    #[test]
    fn test_differential_analyzer() {
        let resource = create_test_resource();
        let mut analyzer = DifferentialAnalyzer::new();

        let paths = analyzer.analyze(&resource);

        assert!(paths.contains(&"Patient.name".to_string()));
        assert!(!paths.contains(&"Patient.identifier".to_string()));
    }
}
