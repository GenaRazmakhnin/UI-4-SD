//! Main StructureDefinition exporter.
//!
//! Coordinates the export of IR [`ProfileDocument`] to valid FHIR
//! StructureDefinition JSON. Produces deterministic output that passes
//! IG Publisher validation.

use serde_json::{Map, Value};

use crate::ir::{ProfileDocument, ProfileStatus, StructureKind};

use super::deterministic::{to_canonical_json, to_pretty_json, DeterministicJsonBuilder};
use super::differential_generator::DifferentialGenerator;
use super::error::{ExportError, ExportResult, ExportResultWithWarnings, ExportWarning, ExportWarningCode};
use super::field_preservation::FieldPreserver;
use super::snapshot_generator::SnapshotGenerator;

/// Configuration for SD export.
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Whether to include snapshot in output.
    pub include_snapshot: bool,
    /// Whether to include differential in output.
    pub include_differential: bool,
    /// Whether to validate before export.
    pub validate: bool,
    /// Whether to use pretty-printed JSON.
    pub pretty_print: bool,
    /// Whether to preserve unknown fields.
    pub preserve_unknown_fields: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            include_snapshot: true,
            include_differential: true,
            validate: true,
            pretty_print: false,
            preserve_unknown_fields: true,
        }
    }
}

impl ExportConfig {
    /// Create config for differential-only export.
    #[must_use]
    pub fn differential_only() -> Self {
        Self {
            include_snapshot: false,
            include_differential: true,
            ..Self::default()
        }
    }

    /// Create config for snapshot-only export.
    #[must_use]
    pub fn snapshot_only() -> Self {
        Self {
            include_snapshot: true,
            include_differential: false,
            ..Self::default()
        }
    }

    /// Enable pretty printing.
    #[must_use]
    pub fn pretty(mut self) -> Self {
        self.pretty_print = true;
        self
    }

    /// Disable validation.
    #[must_use]
    pub fn skip_validation(mut self) -> Self {
        self.validate = false;
        self
    }
}

/// Main exporter for StructureDefinition.
///
/// Converts IR [`ProfileDocument`] to valid FHIR StructureDefinition JSON.
#[derive(Debug)]
pub struct StructureDefinitionExporter {
    /// Export configuration.
    config: ExportConfig,
    /// Snapshot generator.
    snapshot_generator: SnapshotGenerator,
    /// Differential generator.
    differential_generator: DifferentialGenerator,
    /// Field preserver.
    field_preserver: FieldPreserver,
}

impl Default for StructureDefinitionExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl StructureDefinitionExporter {
    /// Create a new exporter with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(ExportConfig::default())
    }

    /// Create an exporter with custom configuration.
    #[must_use]
    pub fn with_config(config: ExportConfig) -> Self {
        Self {
            config,
            snapshot_generator: SnapshotGenerator::new(),
            differential_generator: DifferentialGenerator::new(),
            field_preserver: FieldPreserver::new(),
        }
    }

    /// Export a profile document to JSON string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Validation fails (if enabled)
    /// - Required fields are missing
    /// - Element tree is invalid
    pub async fn export(&mut self, document: &ProfileDocument) -> ExportResult<String> {
        let result = self.export_with_warnings(document).await?;
        Ok(result.into_value())
    }

    /// Export a profile document to JSON string with warnings.
    pub async fn export_with_warnings(
        &mut self,
        document: &ProfileDocument,
    ) -> ExportResult<ExportResultWithWarnings<String>> {
        let mut warnings = Vec::new();

        // Validate if configured
        if self.config.validate {
            self.validate_document(document)?;
        }

        // Build the SD JSON object
        let sd_value = self.build_structure_definition(document, &mut warnings).await?;

        // Serialize to string
        let json_string = if self.config.pretty_print {
            to_pretty_json(&sd_value).map_err(ExportError::serialization_with_source)?
        } else {
            to_canonical_json(&sd_value).map_err(ExportError::serialization_with_source)?
        };

        Ok(ExportResultWithWarnings::with_warnings(json_string, warnings))
    }

    /// Export to a JSON Value.
    pub async fn export_value(&mut self, document: &ProfileDocument) -> ExportResult<Value> {
        let mut warnings = Vec::new();

        if self.config.validate {
            self.validate_document(document)?;
        }

        self.build_structure_definition(document, &mut warnings).await
    }

    /// Build the complete StructureDefinition JSON object.
    async fn build_structure_definition(
        &mut self,
        document: &ProfileDocument,
        warnings: &mut Vec<ExportWarning>,
    ) -> ExportResult<Value> {
        let resource = &document.resource;
        let metadata = &document.metadata;

        let mut builder = DeterministicJsonBuilder::for_structure_definition();

        // Resource type
        builder.add_string("resourceType", "StructureDefinition");

        // Identity
        builder.add_string("id", &metadata.id);
        builder.add_string("url", &metadata.url);
        builder.add_optional_string("version", metadata.version.as_deref());
        builder.add_string("name", &metadata.name);
        builder.add_optional_string("title", metadata.title.as_deref());

        // Status
        builder.add_string("status", metadata.status.as_str());
        builder.add_bool_if_true("experimental", metadata.experimental);

        // Date
        if let Some(date) = &metadata.date {
            builder.add_string("date", &date.to_rfc3339());
        }

        // Publisher and contact
        builder.add_optional_string("publisher", metadata.publisher.as_deref());

        if !metadata.contact.is_empty() {
            let contacts: Vec<Value> = metadata
                .contact
                .iter()
                .map(|c| self.serialize_contact(c))
                .collect();
            builder.add_array("contact", contacts);
        }

        // Description and purpose
        builder.add_optional_string("description", metadata.description.as_deref());
        builder.add_optional_string("purpose", metadata.purpose.as_deref());
        builder.add_optional_string("copyright", metadata.copyright.as_deref());

        // Use context
        if !metadata.use_context.is_empty() {
            let contexts: Vec<Value> = metadata
                .use_context
                .iter()
                .map(|uc| self.serialize_use_context(uc))
                .collect();
            builder.add_array("useContext", contexts);
        }

        // Jurisdiction
        if !metadata.jurisdiction.is_empty() {
            let jurisdictions: Vec<Value> = metadata
                .jurisdiction
                .iter()
                .map(|j| self.serialize_codeable_concept(j))
                .collect();
            builder.add_array("jurisdiction", jurisdictions);
        }

        // Keywords
        if !metadata.keyword.is_empty() {
            let keywords: Vec<Value> = metadata
                .keyword
                .iter()
                .map(|k| self.serialize_coding(k))
                .collect();
            builder.add_array("keyword", keywords);
        }

        // FHIR version
        builder.add_string("fhirVersion", resource.fhir_version.as_str());

        // Structure metadata
        builder.add_string("kind", self.format_kind(resource.kind));
        builder.add_bool("abstract", false);
        builder.add_string("type", resource.resource_type());
        builder.add_string("baseDefinition", &resource.base.url);
        builder.add_string("derivation", "constraint");

        // Generate snapshot if configured
        if self.config.include_snapshot {
            let snapshot_elements = self.snapshot_generator.generate(resource).await?;
            let snapshot_obj = self.build_element_array("snapshot", snapshot_elements);
            builder.add_value("snapshot", snapshot_obj);
        }

        // Generate differential if configured
        if self.config.include_differential {
            let diff_elements = self.differential_generator.generate(resource).await?;
            let diff_obj = self.build_element_array("differential", diff_elements);
            builder.add_value("differential", diff_obj);
        }

        // Preserve unknown fields
        if self.config.preserve_unknown_fields {
            let mut result = builder.build();
            self.field_preserver
                .merge_resource_unknown_fields(&mut result, resource);

            if self.field_preserver.preserved_count() > 0 {
                warnings.push(ExportWarning::new(
                    ExportWarningCode::UnknownFieldIncluded,
                    format!(
                        "{} unknown field(s) preserved in output",
                        self.field_preserver.preserved_count()
                    ),
                ));
            }

            Ok(Value::Object(result))
        } else {
            Ok(builder.build_value())
        }
    }

    /// Build an element array container (snapshot or differential).
    fn build_element_array(&self, _name: &str, elements: Vec<Value>) -> Value {
        let mut obj = Map::new();
        obj.insert("element".to_string(), Value::Array(elements));
        Value::Object(obj)
    }

    /// Validate document before export.
    fn validate_document(&self, document: &ProfileDocument) -> ExportResult<()> {
        let metadata = &document.metadata;

        // Required fields
        if metadata.url.is_empty() {
            return Err(ExportError::missing_metadata("url"));
        }

        if metadata.name.is_empty() {
            return Err(ExportError::missing_metadata("name"));
        }

        // Name must be computer-friendly (no spaces, starts with uppercase)
        if !is_valid_name(&metadata.name) {
            return Err(ExportError::validation(format!(
                "name '{}' is not a valid FHIR name (must start with uppercase, no spaces)",
                metadata.name
            )));
        }

        // Status must be valid
        if matches!(metadata.status, ProfileStatus::Unknown) {
            return Err(ExportError::validation("status is required"));
        }

        // Validate resource
        let resource = &document.resource;

        if resource.base.url.is_empty() {
            return Err(ExportError::missing_metadata("baseDefinition"));
        }

        // Validate element tree
        self.validate_element_tree(&resource.root)?;

        Ok(())
    }

    /// Validate the element tree.
    fn validate_element_tree(&self, root: &crate::ir::ElementNode) -> ExportResult<()> {
        // Root path must match resource type
        if root.path.is_empty() {
            return Err(ExportError::invalid_element("root", "root element path is empty"));
        }

        // Validate all elements recursively
        self.validate_element_recursive(root)?;

        Ok(())
    }

    /// Recursively validate elements.
    fn validate_element_recursive(&self, element: &crate::ir::ElementNode) -> ExportResult<()> {
        // Validate path format
        if element.path.contains(' ') {
            return Err(ExportError::invalid_element(
                &element.path,
                "element path cannot contain spaces",
            ));
        }

        // Validate cardinality consistency
        if let Some(card) = &element.constraints.cardinality {
            if let Some(max) = card.max {
                if max < card.min {
                    return Err(ExportError::invalid_element(
                        &element.path,
                        format!("max ({}) cannot be less than min ({})", max, card.min),
                    ));
                }
            }
        }

        // Validate slicing
        if let Some(slicing) = &element.slicing {
            if slicing.discriminator.is_empty() {
                return Err(ExportError::slicing_error(
                    &element.path,
                    "slicing must have at least one discriminator",
                ));
            }
        }

        // Validate children
        for child in &element.children {
            self.validate_element_recursive(child)?;
        }

        // Validate slices
        for slice in element.slices.values() {
            self.validate_element_recursive(&slice.element)?;
        }

        Ok(())
    }

    /// Format structure kind for FHIR.
    fn format_kind(&self, kind: StructureKind) -> &'static str {
        kind.as_str()
    }

    /// Serialize contact detail.
    fn serialize_contact(&self, contact: &crate::ir::document::ContactDetail) -> Value {
        let mut obj = Map::new();

        if let Some(name) = &contact.name {
            obj.insert("name".to_string(), Value::String(name.clone()));
        }

        if !contact.telecom.is_empty() {
            let telecoms: Vec<Value> = contact
                .telecom
                .iter()
                .map(|t| {
                    let mut t_obj = Map::new();
                    if let Some(system) = &t.system {
                        t_obj.insert("system".to_string(), Value::String(system.clone()));
                    }
                    if let Some(value) = &t.value {
                        t_obj.insert("value".to_string(), Value::String(value.clone()));
                    }
                    if let Some(use_) = &t.r#use {
                        t_obj.insert("use".to_string(), Value::String(use_.clone()));
                    }
                    Value::Object(t_obj)
                })
                .collect();
            obj.insert("telecom".to_string(), Value::Array(telecoms));
        }

        Value::Object(obj)
    }

    /// Serialize use context.
    fn serialize_use_context(&self, context: &crate::ir::document::UseContext) -> Value {
        let mut obj = Map::new();
        obj.insert("code".to_string(), self.serialize_coding(&context.code));

        match &context.value {
            crate::ir::document::UseContextValue::CodeableConcept(cc) => {
                obj.insert("valueCodeableConcept".to_string(), self.serialize_codeable_concept(cc));
            }
            crate::ir::document::UseContextValue::Quantity(q) => {
                obj.insert("valueQuantity".to_string(), self.serialize_quantity(q));
            }
            crate::ir::document::UseContextValue::Range(r) => {
                obj.insert("valueRange".to_string(), self.serialize_range(r));
            }
            crate::ir::document::UseContextValue::Reference(r) => {
                obj.insert("valueReference".to_string(), self.serialize_reference(r));
            }
        }

        Value::Object(obj)
    }

    /// Serialize coding.
    fn serialize_coding(&self, coding: &crate::ir::document::Coding) -> Value {
        let mut obj = Map::new();

        if let Some(system) = &coding.system {
            obj.insert("system".to_string(), Value::String(system.clone()));
        }
        if let Some(code) = &coding.code {
            obj.insert("code".to_string(), Value::String(code.clone()));
        }
        if let Some(display) = &coding.display {
            obj.insert("display".to_string(), Value::String(display.clone()));
        }

        Value::Object(obj)
    }

    /// Serialize codeable concept.
    fn serialize_codeable_concept(&self, cc: &crate::ir::document::CodeableConcept) -> Value {
        let mut obj = Map::new();

        if !cc.coding.is_empty() {
            let codings: Vec<Value> = cc.coding.iter().map(|c| self.serialize_coding(c)).collect();
            obj.insert("coding".to_string(), Value::Array(codings));
        }

        if let Some(text) = &cc.text {
            obj.insert("text".to_string(), Value::String(text.clone()));
        }

        Value::Object(obj)
    }

    /// Serialize quantity.
    fn serialize_quantity(&self, q: &crate::ir::document::Quantity) -> Value {
        let mut obj = Map::new();

        if let Some(value) = q.value {
            obj.insert("value".to_string(), Value::Number(serde_json::Number::from_f64(value).unwrap_or_else(|| 0.into())));
        }
        if let Some(unit) = &q.unit {
            obj.insert("unit".to_string(), Value::String(unit.clone()));
        }
        if let Some(system) = &q.system {
            obj.insert("system".to_string(), Value::String(system.clone()));
        }
        if let Some(code) = &q.code {
            obj.insert("code".to_string(), Value::String(code.clone()));
        }

        Value::Object(obj)
    }

    /// Serialize range.
    fn serialize_range(&self, r: &crate::ir::document::Range) -> Value {
        let mut obj = Map::new();

        if let Some(low) = &r.low {
            obj.insert("low".to_string(), self.serialize_quantity(low));
        }
        if let Some(high) = &r.high {
            obj.insert("high".to_string(), self.serialize_quantity(high));
        }

        Value::Object(obj)
    }

    /// Serialize reference.
    fn serialize_reference(&self, r: &crate::ir::document::Reference) -> Value {
        let mut obj = Map::new();

        if let Some(reference) = &r.reference {
            obj.insert("reference".to_string(), Value::String(reference.clone()));
        }
        if let Some(display) = &r.display {
            obj.insert("display".to_string(), Value::String(display.clone()));
        }

        Value::Object(obj)
    }
}

/// Check if a name is valid for FHIR (computer-friendly).
fn is_valid_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Must start with uppercase letter
    let first_char = name.chars().next().unwrap();
    if !first_char.is_ascii_uppercase() {
        return false;
    }

    // No spaces allowed
    if name.contains(' ') {
        return false;
    }

    // Only alphanumeric and limited special chars
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{
        BaseDefinition, Cardinality, DocumentMetadata, ElementNode, ElementSource,
        FhirVersion, ProfiledResource,
    };

    fn create_test_document() -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            "test-patient",
            "http://example.org/fhir/StructureDefinition/TestPatient",
            "TestPatient",
        );

        let mut resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        // Add a modified element
        let mut name = ElementNode::new("Patient.name".to_string());
        name.source = ElementSource::Modified;
        name.constraints.cardinality = Some(Cardinality::required());
        resource.root.add_child(name);

        ProfileDocument::new(metadata, resource)
    }

    #[tokio::test]
    async fn test_export_basic() {
        let document = create_test_document();
        let mut exporter = StructureDefinitionExporter::new();

        let result = exporter.export(&document).await;
        assert!(result.is_ok(), "Export failed: {:?}", result.err());

        let json_string = result.unwrap();
        let parsed: Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(parsed.get("resourceType").unwrap(), "StructureDefinition");
        assert_eq!(parsed.get("url").unwrap(), "http://example.org/fhir/StructureDefinition/TestPatient");
        assert_eq!(parsed.get("name").unwrap(), "TestPatient");
        assert_eq!(parsed.get("status").unwrap(), "draft");
        assert_eq!(parsed.get("kind").unwrap(), "resource");
        assert_eq!(parsed.get("derivation").unwrap(), "constraint");
    }

    #[tokio::test]
    async fn test_export_with_snapshot() {
        let document = create_test_document();
        let mut exporter = StructureDefinitionExporter::new();

        let result = exporter.export(&document).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert!(parsed.get("snapshot").is_some());
        let snapshot = parsed.get("snapshot").unwrap();
        let elements = snapshot.get("element").unwrap().as_array().unwrap();

        assert!(!elements.is_empty());
    }

    #[tokio::test]
    async fn test_export_with_differential() {
        let document = create_test_document();
        let mut exporter = StructureDefinitionExporter::new();

        let result = exporter.export(&document).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert!(parsed.get("differential").is_some());
        let differential = parsed.get("differential").unwrap();
        let elements = differential.get("element").unwrap().as_array().unwrap();

        assert!(!elements.is_empty());
    }

    #[tokio::test]
    async fn test_validation_missing_url() {
        let mut metadata = DocumentMetadata::new("test", "", "TestPatient");
        metadata.url = String::new(); // Empty URL

        let resource = ProfiledResource::new(
            "http://example.org/test",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        let document = ProfileDocument::new(metadata, resource);
        let mut exporter = StructureDefinitionExporter::new();

        let result = exporter.export(&document).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("url"));
    }

    #[tokio::test]
    async fn test_validation_invalid_name() {
        let metadata = DocumentMetadata::new(
            "test",
            "http://example.org/test",
            "invalid name with spaces", // Invalid name
        );

        let resource = ProfiledResource::new(
            "http://example.org/test",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        let document = ProfileDocument::new(metadata, resource);
        let mut exporter = StructureDefinitionExporter::new();

        let result = exporter.export(&document).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_name() {
        assert!(is_valid_name("TestPatient"));
        assert!(is_valid_name("MyProfile"));
        assert!(is_valid_name("US-Core-Patient"));
        assert!(!is_valid_name("test patient")); // Has space
        assert!(!is_valid_name("testPatient")); // Starts lowercase
        assert!(!is_valid_name("")); // Empty
    }

    #[tokio::test]
    async fn test_differential_only_export() {
        let document = create_test_document();
        let mut exporter = StructureDefinitionExporter::with_config(ExportConfig::differential_only());

        let result = exporter.export(&document).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert!(parsed.get("differential").is_some());
        assert!(parsed.get("snapshot").is_none());
    }
}
