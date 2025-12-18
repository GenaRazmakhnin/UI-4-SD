//! FSH to IR Mapper
//!
//! Maps maki-core's `SemanticModel` and `FhirResource` to the IR data model.
//! This is the core conversion logic for FSH import.

use std::path::Path;

use maki_core::semantic::{
    Cardinality as FshCardinality, Constraint, ConstraintType, Element as FshElement,
    ElementFlag, FhirResource, ResourceMetadata, ResourceType, SemanticModel,
};

use crate::ir::{
    BaseDefinition, Binding, BindingStrength, Cardinality, DocumentMetadata,
    ElementNode, ElementSource, FhirVersion, FixedValue, ProfileDocument, ProfileStatus,
    ProfiledResource, TypeConstraint,
};

use super::error::{FshImportError, FshWarning, FshWarningCode};

/// Maps FSH semantic model to IR.
pub struct FshToIrMapper {
    /// Base URL for canonical URLs.
    canonical_base: String,
    /// FHIR version to use.
    fhir_version: FhirVersion,
}

impl Default for FshToIrMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl FshToIrMapper {
    /// Create a new mapper with default settings.
    pub fn new() -> Self {
        Self {
            canonical_base: "http://example.org/fhir".to_string(),
            fhir_version: FhirVersion::R4,
        }
    }

    /// Set the canonical base URL.
    pub fn with_canonical_base(mut self, base: impl Into<String>) -> Self {
        self.canonical_base = base.into();
        self
    }

    /// Set the FHIR version.
    pub fn with_fhir_version(mut self, version: FhirVersion) -> Self {
        self.fhir_version = version;
        self
    }

    /// Map a single FhirResource (Profile) to ProfileDocument.
    pub fn map_profile(
        &self,
        resource: &FhirResource,
        _source_file: &Path,
    ) -> Result<(ProfileDocument, Vec<FshWarning>), FshImportError> {
        let mut warnings = Vec::new();

        // Only map Profiles
        if resource.resource_type != ResourceType::Profile {
            return Err(FshImportError::mapping(format!(
                "Expected Profile, got {:?}",
                resource.resource_type
            )));
        }

        // Build canonical URL
        let url = format!(
            "{}/StructureDefinition/{}",
            self.canonical_base, resource.id
        );

        // Map metadata
        let metadata = self.map_metadata(resource, &url)?;

        // Determine base definition
        let base = self.determine_base_definition(resource, &mut warnings)?;

        // Create the profiled resource
        let mut profiled_resource = ProfiledResource::new(&url, self.fhir_version, base);
        profiled_resource.version = resource.metadata.version.clone();

        // Build element tree from FSH elements
        profiled_resource.root = self.build_element_tree(resource)?;

        // Create the document
        let mut document = ProfileDocument::new(metadata, profiled_resource);
        document.mark_saved();

        Ok((document, warnings))
    }

    /// Map all profiles from a SemanticModel to ProfileDocuments.
    pub fn map_semantic_model(
        &self,
        model: &SemanticModel,
    ) -> Result<(Vec<ProfileDocument>, Vec<FshWarning>), FshImportError> {
        let mut documents = Vec::new();
        let mut all_warnings = Vec::new();

        // Get profiles from the model
        let profiles = model.get_resources_by_type(ResourceType::Profile);
        let profile_count = profiles.len();

        for profile in profiles {
            match self.map_profile(profile, &model.source_file) {
                Ok((doc, warnings)) => {
                    documents.push(doc);
                    all_warnings.extend(warnings);
                }
                Err(e) => {
                    // Add as warning and continue for partial import
                    all_warnings.push(FshWarning::new(
                        FshWarningCode::PotentialDataLoss,
                        format!("Failed to map profile '{}': {}", profile.id, e),
                    ));
                }
            }
        }

        if documents.is_empty() && profile_count > 0 {
            return Err(FshImportError::mapping(
                "No profiles could be successfully mapped",
            ));
        }

        Ok((documents, all_warnings))
    }

    /// Map FSH resource metadata to DocumentMetadata.
    fn map_metadata(
        &self,
        resource: &FhirResource,
        url: &str,
    ) -> Result<DocumentMetadata, FshImportError> {
        let name = resource
            .name
            .clone()
            .unwrap_or_else(|| resource.id.clone());

        let mut metadata = DocumentMetadata::new(&resource.id, url, &name);

        // Map optional metadata fields
        metadata.title = resource.title.clone().or_else(|| resource.metadata.title.clone());
        metadata.description = resource
            .description
            .clone()
            .or_else(|| resource.metadata.description.clone());
        metadata.version = resource.metadata.version.clone();
        metadata.publisher = resource.metadata.publisher.clone();
        metadata.purpose = resource.metadata.purpose.clone();
        metadata.copyright = resource.metadata.copyright.clone();

        // Map status
        metadata.status = match resource.metadata.status.as_deref() {
            Some("active") => ProfileStatus::Active,
            Some("retired") => ProfileStatus::Retired,
            Some("unknown") => ProfileStatus::Unknown,
            _ => ProfileStatus::Draft,
        };

        // Map experimental flag
        metadata.experimental = resource.metadata.experimental.unwrap_or(false);

        Ok(metadata)
    }

    /// Determine the base definition from the parent reference.
    fn determine_base_definition(
        &self,
        resource: &FhirResource,
        warnings: &mut Vec<FshWarning>,
    ) -> Result<BaseDefinition, FshImportError> {
        let parent = resource.parent.as_deref().unwrap_or("Resource");

        // Check if it's a core FHIR resource
        if is_core_resource(parent) {
            return Ok(BaseDefinition::resource(parent));
        }

        // Check if it's a URL
        if parent.starts_with("http://") || parent.starts_with("https://") {
            // Extract resource type from URL
            let resource_type = parent
                .rsplit('/')
                .next()
                .unwrap_or("Resource");
            return Ok(BaseDefinition::new(parent).with_name(resource_type.to_string()));
        }

        // Assume it's a profile name - construct the URL
        let base_url = format!("{}/StructureDefinition/{}", self.canonical_base, parent);
        warnings.push(FshWarning::new(
            FshWarningCode::UnresolvedReference,
            format!("Parent '{}' resolved to assumed URL: {}", parent, base_url),
        ));

        Ok(BaseDefinition::new(&base_url).with_name(parent.to_string()))
    }

    /// Build element tree from FSH elements.
    fn build_element_tree(&self, resource: &FhirResource) -> Result<ElementNode, FshImportError> {
        // Determine root path from parent
        let root_type = resource
            .parent
            .as_deref()
            .and_then(|p| {
                if is_core_resource(p) {
                    Some(p)
                } else {
                    // Extract from URL or use as-is
                    p.rsplit('/').next()
                }
            })
            .unwrap_or("Resource");

        let mut root = ElementNode::new(root_type.to_string());
        root.source = ElementSource::Inherited;

        // Add child elements
        for fsh_element in &resource.elements {
            let ir_element = self.map_element(fsh_element)?;
            self.insert_element(&mut root, ir_element)?;
        }

        Ok(root)
    }

    /// Map a single FSH element to IR ElementNode.
    fn map_element(&self, fsh_element: &FshElement) -> Result<ElementNode, FshImportError> {
        let mut node = ElementNode::new(fsh_element.path.clone());
        node.source = ElementSource::Modified;

        // Map cardinality
        if let Some(card) = &fsh_element.cardinality {
            node.constraints.cardinality = Some(self.map_cardinality(card));
        }

        // Map type info
        if let Some(type_info) = &fsh_element.type_info {
            let mut type_constraint = TypeConstraint::simple(&type_info.type_name);

            if let Some(profile) = &type_info.profile {
                type_constraint.profile = vec![profile.clone()];
            }

            if !type_info.target_types.is_empty() {
                type_constraint.target_profile = type_info.target_types.clone();
            }

            node.constraints.types = vec![type_constraint];
        }

        // Map flags
        for flag in &fsh_element.flags {
            match flag {
                ElementFlag::MustSupport => node.constraints.flags.must_support = true,
                ElementFlag::Summary => node.constraints.flags.is_summary = true,
                ElementFlag::Modifier => node.constraints.flags.is_modifier = true,
                _ => {}
            }
        }

        // Map constraints
        for constraint in &fsh_element.constraints {
            self.apply_constraint(&mut node, constraint)?;
        }

        Ok(node)
    }

    /// Map FSH cardinality to IR cardinality.
    fn map_cardinality(&self, fsh_card: &FshCardinality) -> Cardinality {
        Cardinality::new(fsh_card.min, fsh_card.max)
    }

    /// Apply a FSH constraint to an element node.
    fn apply_constraint(
        &self,
        node: &mut ElementNode,
        constraint: &Constraint,
    ) -> Result<(), FshImportError> {
        match constraint.constraint_type {
            ConstraintType::FixedValue => {
                // Parse the fixed value (simplified - would need JSON parsing)
                if let Ok(value) = serde_json::from_str(&constraint.value) {
                    node.constraints.fixed_value = Some(FixedValue::fixed(value));
                }
            }
            ConstraintType::Pattern => {
                if let Ok(value) = serde_json::from_str(&constraint.value) {
                    node.constraints.fixed_value = Some(FixedValue::pattern(value));
                }
            }
            ConstraintType::Binding => {
                // Parse binding (format: "ValueSetUrl (strength)")
                node.constraints.binding = Some(self.parse_binding(&constraint.value));
            }
            ConstraintType::Only => {
                // Type constraint - already handled in type_info
            }
            ConstraintType::Slice | ConstraintType::Contains => {
                // Slicing - handled separately
            }
            ConstraintType::Obeys => {
                // Invariant reference - store in constraints
                // Would need to resolve the actual invariant
            }
        }
        Ok(())
    }

    /// Parse a binding string into a Binding struct.
    fn parse_binding(&self, binding_str: &str) -> Binding {
        // Simple parsing - format varies
        let strength = if binding_str.contains("required") {
            BindingStrength::Required
        } else if binding_str.contains("extensible") {
            BindingStrength::Extensible
        } else if binding_str.contains("preferred") {
            BindingStrength::Preferred
        } else {
            BindingStrength::Example
        };

        // Extract URL (simplified)
        let url = binding_str
            .split_whitespace()
            .find(|s| s.starts_with("http"))
            .unwrap_or(binding_str)
            .to_string();

        Binding::new(strength, url)
    }

    /// Insert an element into the correct position in the tree.
    fn insert_element(
        &self,
        root: &mut ElementNode,
        element: ElementNode,
    ) -> Result<(), FshImportError> {
        let path_segments: Vec<&str> = element.path.split('.').collect();

        if path_segments.len() == 1 {
            // Root element - update constraints
            root.constraints = element.constraints;
            root.source = element.source;
            return Ok(());
        }

        // Find or create parent nodes
        let mut current = root;
        for (i, _segment) in path_segments[1..path_segments.len() - 1].iter().enumerate() {
            let parent_path = path_segments[..=i + 1].join(".");

            if current.find_child_mut(&parent_path).is_none() {
                // Create intermediate node
                let intermediate = ElementNode::new(parent_path.clone());
                current.add_child(intermediate);
            }

            current = current
                .find_child_mut(&parent_path)
                .expect("just created");
        }

        // Add the element as a child
        current.add_child(element);
        Ok(())
    }
}

/// Check if a name is a core FHIR resource type.
fn is_core_resource(name: &str) -> bool {
    matches!(
        name,
        "Patient"
            | "Observation"
            | "Condition"
            | "Encounter"
            | "Procedure"
            | "MedicationRequest"
            | "DiagnosticReport"
            | "Practitioner"
            | "Organization"
            | "Location"
            | "AllergyIntolerance"
            | "Immunization"
            | "CarePlan"
            | "Goal"
            | "ServiceRequest"
            | "Specimen"
            | "Device"
            | "Medication"
            | "DocumentReference"
            | "Composition"
            | "Bundle"
            | "Resource"
            | "DomainResource"
            | "Element"
            | "BackboneElement"
            | "Extension"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use maki_core::Location;

    fn create_test_profile() -> FhirResource {
        FhirResource {
            resource_type: ResourceType::Profile,
            id: "test-patient".to_string(),
            name: Some("TestPatient".to_string()),
            title: Some("Test Patient Profile".to_string()),
            description: Some("A test profile".to_string()),
            parent: Some("Patient".to_string()),
            elements: vec![],
            location: Location::default(),
            metadata: ResourceMetadata {
                status: Some("draft".to_string()),
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_map_profile_basic() {
        let mapper = FshToIrMapper::new();
        let profile = create_test_profile();

        let result = mapper.map_profile(&profile, Path::new("test.fsh"));
        assert!(result.is_ok());

        let (doc, warnings) = result.unwrap();
        assert_eq!(doc.metadata.name, "TestPatient");
        assert_eq!(doc.metadata.status, ProfileStatus::Draft);
        assert!(warnings.is_empty() || warnings.len() == 1); // May have parent warning
    }

    #[test]
    fn test_map_cardinality() {
        let mapper = FshToIrMapper::new();

        let fsh_card = FshCardinality { min: 1, max: Some(1) };
        let ir_card = mapper.map_cardinality(&fsh_card);

        assert_eq!(ir_card.min, 1);
        assert_eq!(ir_card.max, Some(1));
    }

    #[test]
    fn test_parse_binding() {
        let mapper = FshToIrMapper::new();

        let binding = mapper.parse_binding("http://example.org/ValueSet/test (required)");
        assert_eq!(binding.strength, BindingStrength::Required);
    }
}
