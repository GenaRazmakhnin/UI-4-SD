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

mod constraint_extractor;
mod element_builder;
mod error;
mod sd_parser;
mod slicing_importer;

pub use constraint_extractor::ConstraintExtractor;
pub use element_builder::ElementTreeBuilder;
pub use error::{ImportError, ImportResult, ImportWarning};
pub use sd_parser::{ParsedStructureDefinition, StructureDefinitionParser};
pub use slicing_importer::SlicingImporter;

use crate::ir::{
    BaseDefinition, DocumentMetadata, FhirVersion, ProfileDocument, ProfileStatus,
    ProfiledResource,
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
    /// Extractor for constraints.
    constraint_extractor: ConstraintExtractor,
    /// Importer for slicing.
    slicing_importer: SlicingImporter,
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
            constraint_extractor: ConstraintExtractor::new(),
            slicing_importer: SlicingImporter::new(),
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

        // Build element tree from snapshot (or differential if no snapshot)
        let elements = parsed
            .snapshot_elements
            .as_ref()
            .or(parsed.differential_elements.as_ref())
            .ok_or_else(|| {
                ImportError::missing_field("snapshot.element or differential.element")
            })?;

        // Build the element tree
        resource.root = self.element_builder.build_tree(
            &parsed.type_name,
            elements,
            parsed.differential_elements.as_deref(),
        )?;

        // Extract constraints from differential
        if let Some(diff_elements) = &parsed.differential_elements {
            self.constraint_extractor
                .apply_differential(&mut resource.root, diff_elements)?;
        }

        // Import slicing definitions
        self.slicing_importer.import_slicing(&mut resource.root, elements)?;

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
}
