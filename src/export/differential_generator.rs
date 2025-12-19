//! Differential generation for StructureDefinition export.
//!
//! Generates minimal differential containing only modified elements.
//! The differential shows what constraints were added or changed
//! compared to the base definition.

use serde_json::Value;

use crate::ir::ProfiledResource;

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

        let root_path = resource.resource_type();
        let has_root = resource
            .differential
            .iter()
            .any(|diff| diff.path == root_path);
        if !has_root {
            let root_diff = crate::merge::DifferentialElement::new(root_path.to_string());
            elements.push(self.serializer.serialize_differential_element(&root_diff)?);
        }

        for diff in &resource.differential {
            elements.push(self.serializer.serialize_differential_element(diff)?);
        }

        // Sort elements by path for deterministic ordering
        sort_elements_by_path(&mut elements);

        Ok(elements)
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
        for diff in &resource.differential {
            if let Some(slice_name) = diff.slice_name.as_deref() {
                self.modified_paths
                    .push(format!("{}:{}", diff.path, slice_name));
            } else {
                self.modified_paths.push(diff.path.clone());
            }
        }
        &self.modified_paths
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
        stats.collect_stats(resource);
        stats
    }

    fn collect_stats(&mut self, resource: &ProfiledResource) {
        for diff in &resource.differential {
            self.element_count += 1;

            if diff.constraints.cardinality.is_some() {
                self.cardinality_changes += 1;
            }

            if !diff.constraints.types.is_empty() {
                self.type_constraints += 1;
            }

            if diff.constraints.binding.is_some() {
                self.binding_constraints += 1;
            }

            if diff.constraints.flags.must_support {
                self.must_support_count += 1;
            }

            if diff.slicing.is_some() {
                self.sliced_elements += 1;
            }
            if diff.slice_name.is_some() {
                self.slice_count += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, Cardinality, DifferentialElement, FhirVersion, ProfiledResource};

    fn create_test_resource() -> ProfiledResource {
        let mut resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        let mut diff = DifferentialElement::new("Patient.name".to_string());
        diff.constraints.cardinality = Some(Cardinality::required());
        diff.constraints.flags.must_support = true;

        resource.differential.push(diff);

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
    }
}
