//! StructureDefinition Export Module
//!
//! This module provides functionality to export the internal IR format
//! to valid FHIR StructureDefinition JSON. It supports lossless round-tripping
//! by preserving unknown fields and producing deterministic output.
//!
//! # Architecture
//!
//! ```text
//!    ProfileDocument (IR)
//!         │
//!         ▼
//! ┌───────────────────┐
//! │   SD Exporter     │  Coordinates export process
//! └─────────┬─────────┘
//!           │
//!     ┌─────┴─────┐
//!     ▼           ▼
//! ┌─────────┐ ┌─────────────┐
//! │Snapshot │ │ Differential│
//! │Generator│ │  Generator  │
//! └────┬────┘ └──────┬──────┘
//!      │             │
//!      └──────┬──────┘
//!             ▼
//! ┌───────────────────┐
//! │Element Serializer │  Converts IR elements to JSON
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │  Deterministic    │  Ensures stable field ordering
//! │  JSON Builder     │
//! └─────────┬─────────┘
//!           │
//!           ▼
//!   StructureDefinition JSON
//! ```
//!
//! # Features
//!
//! - **Deterministic Output**: Same IR state produces byte-identical JSON
//! - **Snapshot Generation**: Complete element tree with inherited values
//! - **Differential Generation**: Minimal diff with only modified elements
//! - **Field Preservation**: Unknown fields from import are preserved
//! - **Validation**: Ensures exported SD has all required metadata
//!
//! # Example
//!
//! ```no_run
//! use niten::export::StructureDefinitionExporter;
//! use niten::ir::ProfileDocument;
//!
//! async fn export_profile(document: &ProfileDocument) -> anyhow::Result<String> {
//!     let mut exporter = StructureDefinitionExporter::new();
//!     let json = exporter.export(document).await?;
//!     Ok(json)
//! }
//! ```
//!
//! # Configuration
//!
//! ```no_run
//! use niten::export::{StructureDefinitionExporter, ExportConfig};
//!
//! // Export differential only (no snapshot)
//! let config = ExportConfig::differential_only();
//! let exporter = StructureDefinitionExporter::with_config(config);
//!
//! // Pretty-printed output
//! let config = ExportConfig::default().pretty();
//! let exporter = StructureDefinitionExporter::with_config(config);
//! ```

mod deterministic;
mod differential_generator;
mod element_serializer;
mod error;
mod field_preservation;
mod sd_exporter;
mod snapshot_generator;

// Re-export main types
pub use deterministic::{
    sort_elements_by_path, to_canonical_json, to_pretty_json, DeterministicJsonBuilder,
};
pub use differential_generator::{DifferentialAnalyzer, DifferentialGenerator, DifferentialStats};
pub use element_serializer::ElementSerializer;
pub use error::{
    ExportError, ExportResult, ExportResultWithWarnings, ExportWarning, ExportWarningCode,
};
pub use field_preservation::{FieldPreserver, UnknownFieldCollector, UnknownFieldInfo};
pub use sd_exporter::{ExportConfig, StructureDefinitionExporter};
pub use snapshot_generator::{SnapshotConfig, SnapshotGenerator};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{
        BaseDefinition, Cardinality, DocumentMetadata, ElementNode, ElementSource,
        FhirVersion, ProfileDocument, ProfiledResource,
    };

    /// Create a test document for round-trip testing.
    fn create_comprehensive_test_document() -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            "comprehensive-patient",
            "http://example.org/fhir/StructureDefinition/ComprehensivePatient",
            "ComprehensivePatient",
        )
        .with_title("Comprehensive Patient Profile")
        .with_description("A test profile with various constraints")
        .with_publisher("Test Organization")
        .with_version("1.0.0");

        let mut resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/ComprehensivePatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        // Add identifier with cardinality
        let mut identifier = ElementNode::new("Patient.identifier".to_string());
        identifier.source = ElementSource::Modified;
        identifier.constraints.cardinality = Some(Cardinality::required_unbounded());
        identifier.constraints.flags.must_support = true;
        resource.root.add_child(identifier);

        // Add name with cardinality
        let mut name = ElementNode::new("Patient.name".to_string());
        name.source = ElementSource::Modified;
        name.constraints.cardinality = Some(Cardinality::required());
        name.constraints.short = Some("Patient's official name".to_string());

        // Add family under name
        let mut family = ElementNode::new("Patient.name.family".to_string());
        family.source = ElementSource::Modified;
        family.constraints.cardinality = Some(Cardinality::required());
        name.add_child(family);

        resource.root.add_child(name);

        // Add birthDate
        let mut birth_date = ElementNode::new("Patient.birthDate".to_string());
        birth_date.source = ElementSource::Modified;
        birth_date.constraints.cardinality = Some(Cardinality::optional());
        birth_date.constraints.flags.must_support = true;
        resource.root.add_child(birth_date);

        ProfileDocument::new(metadata, resource)
    }

    #[tokio::test]
    async fn test_full_export_workflow() {
        let document = create_comprehensive_test_document();
        let mut exporter = StructureDefinitionExporter::new();

        let result = exporter.export(&document).await;
        assert!(result.is_ok(), "Export failed: {:?}", result.err());

        let json_string = result.unwrap();

        // Parse and validate structure
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        // Check required fields
        assert_eq!(parsed.get("resourceType").unwrap(), "StructureDefinition");
        assert!(parsed.get("url").is_some());
        assert!(parsed.get("name").is_some());
        assert!(parsed.get("status").is_some());
        assert!(parsed.get("fhirVersion").is_some());
        assert!(parsed.get("kind").is_some());
        assert!(parsed.get("type").is_some());
        assert!(parsed.get("baseDefinition").is_some());
        assert!(parsed.get("derivation").is_some());

        // Check snapshot and differential
        assert!(parsed.get("snapshot").is_some());
        assert!(parsed.get("differential").is_some());
    }

    #[tokio::test]
    async fn test_deterministic_output() {
        let document = create_comprehensive_test_document();

        // Export twice
        let mut exporter1 = StructureDefinitionExporter::new();
        let mut exporter2 = StructureDefinitionExporter::new();

        let json1 = exporter1.export(&document).await.unwrap();
        let json2 = exporter2.export(&document).await.unwrap();

        // Should be byte-identical
        assert_eq!(json1, json2, "Export is not deterministic");
    }

    #[tokio::test]
    async fn test_differential_only() {
        let document = create_comprehensive_test_document();
        let config = ExportConfig::differential_only();
        let mut exporter = StructureDefinitionExporter::with_config(config);

        let json = exporter.export(&document).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("differential").is_some());
        assert!(parsed.get("snapshot").is_none());
    }

    #[tokio::test]
    async fn test_snapshot_only() {
        let document = create_comprehensive_test_document();
        let config = ExportConfig::snapshot_only();
        let mut exporter = StructureDefinitionExporter::with_config(config);

        let json = exporter.export(&document).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("snapshot").is_some());
        assert!(parsed.get("differential").is_none());
    }

    #[tokio::test]
    async fn test_pretty_print() {
        let document = create_comprehensive_test_document();
        let config = ExportConfig::default().pretty();
        let mut exporter = StructureDefinitionExporter::with_config(config);

        let json = exporter.export(&document).await.unwrap();

        // Pretty printed JSON should contain newlines
        assert!(json.contains('\n'), "Pretty print should contain newlines");
    }

    #[tokio::test]
    async fn test_export_with_warnings() {
        let document = create_comprehensive_test_document();
        let mut exporter = StructureDefinitionExporter::new();

        let result = exporter.export_with_warnings(&document).await.unwrap();

        // Should succeed with or without warnings
        assert!(!result.value.is_empty());
    }

    #[test]
    fn test_differential_stats() {
        let document = create_comprehensive_test_document();
        let stats = DifferentialStats::from_resource(&document.resource);

        assert!(stats.cardinality_changes > 0);
        assert!(stats.must_support_count > 0);
    }
}
