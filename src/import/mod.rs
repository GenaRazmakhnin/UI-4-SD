//! StructureDefinition Import Module
//!
//! This module provides functionality to import FHIR StructureDefinition JSON
//! into the internal IR format. It supports lossless round-tripping by preserving
//! all fields, including unknown extensions.
//!
//! # Architecture
//!
//! ```text
//! StructureDefinition JSON
//!         │
//!         ▼
//! ┌───────────────────┐
//! │    SD Parser      │  Parse JSON, validate structure
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │  Element Builder  │  Build tree from snapshot/differential
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │ Constraint Extract│  Extract cardinality, types, bindings
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │  Slicing Import   │  Handle discriminators, slices
//! └─────────┬─────────┘
//!           │
//!           ▼
//!    ProfileDocument (IR)
//! ```
//!
//! # Example
//!
//! ```no_run
//! use niten::import::StructureDefinitionImporter;
//!
//! async fn import_profile() -> anyhow::Result<()> {
//!     let json = r#"{"resourceType": "StructureDefinition", ...}"#;
//!     let importer = StructureDefinitionImporter::new();
//!     let document = importer.import_json(json).await?;
//!     Ok(())
//! }
//! ```

mod element_builder;
mod error;
mod sd_parser;

pub use element_builder::ElementTreeBuilder;
pub use error::{ImportError, ImportResult, ImportWarning};
pub use sd_parser::{ParsedStructureDefinition, StructureDefinitionParser};

use std::collections::HashMap;

use serde_json::Value;

use crate::ir::{
    BaseDefinition, Discriminator, DiscriminatorType, DocumentMetadata, FhirVersion,
    ProfileDocument, ProfileStatus, ProfiledResource, SlicingDefinition, SlicingRules,
};

/// Main importer for StructureDefinition JSON.
///
/// Coordinates the import process from JSON to IR, including:
/// - Parsing and validating JSON
/// - Building the element tree
/// - Extracting constraints
/// - Handling slicing
/// - Preserving unknown fields
pub struct StructureDefinitionImporter {
    /// Parser for SD JSON.
    parser: StructureDefinitionParser,
    /// Builder for element tree.
    element_builder: ElementTreeBuilder,
}

impl Default for StructureDefinitionImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl StructureDefinitionImporter {
    /// Create a new importer with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: StructureDefinitionParser::new(),
            element_builder: ElementTreeBuilder::new(),
        }
    }

    /// Import a StructureDefinition from JSON string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON parsing fails
    /// - Required fields are missing
    /// - The structure is invalid
    pub async fn import_json(&self, json: &str) -> ImportResult<ProfileDocument> {
        // Parse JSON into intermediate representation
        let parsed = self.parser.parse(json)?;

        // Build the IR document
        self.build_document(parsed).await
    }

    /// Import a StructureDefinition from a JSON value.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing or invalid.
    pub async fn import_value(
        &self,
        value: serde_json::Value,
    ) -> ImportResult<ProfileDocument> {
        let parsed = self.parser.parse_value(value)?;
        self.build_document(parsed).await
    }

    /// Build a ProfileDocument from parsed SD.
    async fn build_document(&self, parsed: ParsedStructureDefinition) -> ImportResult<ProfileDocument> {
        // Extract metadata
        let metadata = self.extract_metadata(&parsed)?;

        // Determine FHIR version
        let fhir_version = self.determine_fhir_version(&parsed);

        // Build base definition reference
        let base = BaseDefinition::new(&parsed.base_definition)
            .with_name(parsed.type_name.clone());

        // Create the profiled resource
        let mut resource = ProfiledResource::new(&parsed.url, fhir_version, base);
        resource.version = parsed.version.clone();

        // Set kind from parsed SD
        resource.kind = match parsed.kind.as_str() {
            "resource" => crate::ir::StructureKind::Resource,
            "complex-type" => crate::ir::StructureKind::ComplexType,
            "primitive-type" => crate::ir::StructureKind::PrimitiveType,
            "logical" => crate::ir::StructureKind::Logical,
            _ => crate::ir::StructureKind::Resource,
        };

        // Build differential-only representation
        let mut differential = if let Some(diff_elements) = &parsed.differential_elements {
            self.element_builder.build_differential_elements(diff_elements)?
        } else {
            Vec::new()
        };

        if let Some(snapshot_elements) = &parsed.snapshot_elements {
            self.merge_snapshot_slicing(&mut differential, snapshot_elements)?;
        }

        resource.differential = differential;

        // Preserve unknown fields
        resource.unknown_fields = parsed.unknown_fields;

        // Create the document
        let mut document = ProfileDocument::new(metadata, resource);

        // Mark as not dirty (freshly imported)
        document.mark_saved();

        Ok(document)
    }

    /// Extract document metadata from parsed SD.
    fn extract_metadata(&self, parsed: &ParsedStructureDefinition) -> ImportResult<DocumentMetadata> {
        let id = parsed
            .id
            .clone()
            .unwrap_or_else(|| self.generate_id_from_url(&parsed.url));

        let name = parsed
            .name
            .clone()
            .ok_or_else(|| ImportError::missing_field("name"))?;

        let mut metadata = DocumentMetadata::new(&id, &parsed.url, &name);

        metadata.version = parsed.version.clone();
        metadata.title = parsed.title.clone();
        metadata.status = self.parse_status(&parsed.status);
        metadata.experimental = parsed.experimental.unwrap_or(false);
        metadata.publisher = parsed.publisher.clone();
        metadata.description = parsed.description.clone();
        metadata.purpose = parsed.purpose.clone();
        metadata.copyright = parsed.copyright.clone();

        if let Some(date_str) = &parsed.date {
            if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                metadata.date = Some(date.with_timezone(&chrono::Utc));
            }
        }

        Ok(metadata)
    }

    /// Determine FHIR version from SD metadata.
    fn determine_fhir_version(&self, parsed: &ParsedStructureDefinition) -> FhirVersion {
        parsed
            .fhir_version
            .as_deref()
            .and_then(FhirVersion::from_str)
            .unwrap_or(FhirVersion::R4)
    }

    /// Parse status string to enum.
    fn parse_status(&self, status: &Option<String>) -> ProfileStatus {
        match status.as_deref() {
            Some("draft") => ProfileStatus::Draft,
            Some("active") => ProfileStatus::Active,
            Some("retired") => ProfileStatus::Retired,
            _ => ProfileStatus::Unknown,
        }
    }

    /// Generate an ID from URL if not provided.
    fn generate_id_from_url(&self, url: &str) -> String {
        url.rsplit('/')
            .next()
            .unwrap_or("unknown")
            .to_string()
    }

    fn merge_snapshot_slicing(
        &self,
        differential: &mut Vec<crate::merge::DifferentialElement>,
        snapshot: &[Value],
    ) -> ImportResult<()> {
        let mut by_path: HashMap<String, usize> = differential
            .iter()
            .enumerate()
            .filter(|(_, diff)| diff.slice_name.is_none())
            .map(|(idx, diff)| (diff.path.clone(), idx))
            .collect();

        for element in snapshot {
            let path = element
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(|| ImportError::missing_field("snapshot.element.path"))?;

            if path.contains(':') {
                continue;
            }

            let slicing = match element.get("slicing") {
                Some(slicing) => self.parse_slicing_definition(slicing)?,
                None => continue,
            };

            if let Some(index) = by_path.get(path).copied() {
                if differential[index].slicing.is_none() {
                    differential[index].slicing = Some(slicing);
                }
                continue;
            }

            let mut diff = crate::merge::DifferentialElement::new(path.to_string());
            if let Some(id) = element.get("id").and_then(Value::as_str) {
                diff.element_id = Some(id.to_string());
            }
            diff.slicing = Some(slicing);
            differential.push(diff);
            by_path.insert(path.to_string(), differential.len() - 1);
        }

        Ok(())
    }

    fn parse_slicing_definition(&self, slicing: &Value) -> ImportResult<SlicingDefinition> {
        let discriminators = slicing
            .get("discriminator")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|d| {
                        let disc_type = d.get("type").and_then(Value::as_str)?;
                        let path = d.get("path").and_then(Value::as_str)?;
                        let disc_type = match disc_type {
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
                    })
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_import_minimal_sd() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/fhir/StructureDefinition/TestProfile",
            "name": "TestProfile",
            "status": "draft",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "derivation": "constraint",
            "differential": {
                "element": [
                    {
                        "id": "Patient",
                        "path": "Patient"
                    }
                ]
            }
        }"#;

        let importer = StructureDefinitionImporter::new();
        let result = importer.import_json(json).await;

        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let doc = result.unwrap();
        assert_eq!(doc.metadata.name, "TestProfile");
        assert_eq!(doc.metadata.status, ProfileStatus::Draft);
    }

    #[tokio::test]
    async fn test_import_snapshot_slicing_fallback() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/fhir/StructureDefinition/TestSlicingProfile",
            "name": "TestSlicingProfile",
            "status": "draft",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "derivation": "constraint",
            "snapshot": {
                "element": [
                    {
                        "id": "Patient",
                        "path": "Patient"
                    },
                    {
                        "id": "Patient.identifier.type.extension",
                        "path": "Patient.identifier.type.extension",
                        "min": 0,
                        "max": "*",
                        "slicing": {
                            "rules": "open",
                            "discriminator": [
                                {
                                    "type": "value",
                                    "path": "url"
                                }
                            ]
                        }
                    }
                ]
            },
            "differential": {
                "element": [
                    {
                        "id": "Patient",
                        "path": "Patient"
                    },
                    {
                        "id": "Patient.identifier.type.extension",
                        "path": "Patient.identifier.type.extension"
                    }
                ]
            }
        }"#;

        let importer = StructureDefinitionImporter::new();
        let result = importer.import_json(json).await;

        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let doc = result.unwrap();
        let diff = doc
            .resource
            .differential
            .iter()
            .find(|d| d.path == "Patient.identifier.type.extension")
            .expect("Missing slicing differential element");

        let slicing = diff.slicing.as_ref().expect("Missing slicing definition");
        assert_eq!(slicing.discriminator.len(), 1);
        assert_eq!(slicing.discriminator[0].path, "url");
        assert_eq!(slicing.discriminator[0].discriminator_type, DiscriminatorType::Value);
    }

    #[tokio::test]
    async fn test_snapshot_slicing_applies_to_base_element_with_slice_entries() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/fhir/StructureDefinition/TestSliceOnlyProfile",
            "name": "TestSliceOnlyProfile",
            "status": "draft",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "derivation": "constraint",
            "snapshot": {
                "element": [
                    { "id": "Patient", "path": "Patient" },
                    {
                        "id": "Patient.identifier.type.extension",
                        "path": "Patient.identifier.type.extension",
                        "slicing": {
                            "rules": "open",
                            "discriminator": [
                                { "type": "value", "path": "url" }
                            ]
                        }
                    }
                ]
            },
            "differential": {
                "element": [
                    { "id": "Patient", "path": "Patient" },
                    {
                        "id": "Patient.identifier.type.extension:paisEmisionDocumento",
                        "path": "Patient.identifier.type.extension",
                        "sliceName": "paisEmisionDocumento"
                    }
                ]
            }
        }"#;

        let importer = StructureDefinitionImporter::new();
        let result = importer.import_json(json).await;

        assert!(result.is_ok(), "Import failed: {:?}", result.err());

        let doc = result.unwrap();
        let base_diff = doc
            .resource
            .differential
            .iter()
            .find(|d| d.path == "Patient.identifier.type.extension" && d.slice_name.is_none())
            .expect("Missing base differential element");

        let slicing = base_diff
            .slicing
            .as_ref()
            .expect("Missing slicing definition on base element");
        assert_eq!(slicing.discriminator.len(), 1);
        assert_eq!(slicing.discriminator[0].path, "url");
        assert_eq!(slicing.discriminator[0].discriminator_type, DiscriminatorType::Value);
    }
}
