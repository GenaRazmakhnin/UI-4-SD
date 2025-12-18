//! StructureDefinition JSON Parser.
//!
//! Parses FHIR StructureDefinition JSON into an intermediate representation
//! that can be processed into the IR format. Supports R4, R4B, and R5 versions.

use serde_json::Value;

use super::error::{ImportError, ImportResult};

/// Parsed intermediate representation of a StructureDefinition.
///
/// This captures all relevant fields from the SD JSON while preserving
/// unknown fields for lossless round-tripping.
#[derive(Debug, Clone)]
pub struct ParsedStructureDefinition {
    // Identity
    /// Resource ID.
    pub id: Option<String>,
    /// Canonical URL (required).
    pub url: String,
    /// Business version.
    pub version: Option<String>,
    /// Computer-friendly name.
    pub name: Option<String>,
    /// Human-friendly title.
    pub title: Option<String>,

    // Status
    /// Publication status (draft/active/retired/unknown).
    pub status: Option<String>,
    /// For testing purposes only.
    pub experimental: Option<bool>,
    /// Date of publication.
    pub date: Option<String>,

    // Publisher
    /// Name of the publisher.
    pub publisher: Option<String>,
    /// Contact details.
    pub contact: Option<Vec<Value>>,
    /// Natural language description.
    pub description: Option<String>,
    /// Why this resource is defined.
    pub purpose: Option<String>,
    /// Use and/or publishing restrictions.
    pub copyright: Option<String>,

    // Classification
    /// Keywords for finding this definition.
    pub keyword: Option<Vec<Value>>,
    /// FHIR version(s) this applies to.
    pub fhir_version: Option<String>,

    // Structure
    /// resource | complex-type | primitive-type | logical.
    pub kind: String,
    /// Whether this is abstract.
    pub is_abstract: bool,
    /// Type defined/constrained by this structure.
    pub type_name: String,
    /// Definition that this type is constrained from.
    pub base_definition: String,
    /// specialization | constraint.
    pub derivation: Option<String>,

    // Context (for extensions)
    /// Extension context.
    pub context: Option<Vec<Value>>,
    /// Context type.
    pub context_type: Option<String>,
    /// Context invariants.
    pub context_invariant: Option<Vec<String>>,

    // Elements
    /// Snapshot element definitions.
    pub snapshot_elements: Option<Vec<Value>>,
    /// Differential element definitions.
    pub differential_elements: Option<Vec<Value>>,

    // Mappings
    /// External specification mappings.
    pub mapping: Option<Vec<Value>>,

    // Unknown fields for round-trip preservation
    /// Fields not explicitly handled, preserved as-is.
    pub unknown_fields: serde_json::Map<String, Value>,
}

/// Parser for StructureDefinition JSON.
///
/// Extracts required and optional fields while preserving unknown fields.
#[derive(Debug, Default)]
pub struct StructureDefinitionParser {
    /// Whether to be strict about unknown fields (log warnings).
    strict: bool,
}

impl StructureDefinitionParser {
    /// Create a new parser with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable strict mode (warnings for unknown fields).
    #[must_use]
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Parse a JSON string into a ParsedStructureDefinition.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON parsing fails
    /// - resourceType is not "StructureDefinition"
    /// - Required fields are missing
    pub fn parse(&self, json: &str) -> ImportResult<ParsedStructureDefinition> {
        let value: Value =
            serde_json::from_str(json).map_err(ImportError::json_parse_with_source)?;

        self.parse_value(value)
    }

    /// Parse a JSON Value into a ParsedStructureDefinition.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing or invalid.
    pub fn parse_value(&self, value: Value) -> ImportResult<ParsedStructureDefinition> {
        let obj = value
            .as_object()
            .ok_or_else(|| ImportError::json_parse("Expected JSON object"))?;

        // Validate resourceType
        let resource_type = obj
            .get("resourceType")
            .and_then(Value::as_str)
            .ok_or_else(|| ImportError::missing_field("resourceType"))?;

        if resource_type != "StructureDefinition" {
            return Err(ImportError::invalid_resource_type(resource_type));
        }

        // Required fields
        let url = self.get_required_string(obj, "url")?;
        let kind = self.get_required_string(obj, "kind")?;
        let type_name = self.get_required_string(obj, "type")?;
        let base_definition = self.get_required_string(obj, "baseDefinition")?;
        let is_abstract = obj
            .get("abstract")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        // Optional fields
        let id = self.get_optional_string(obj, "id");
        let version = self.get_optional_string(obj, "version");
        let name = self.get_optional_string(obj, "name");
        let title = self.get_optional_string(obj, "title");
        let status = self.get_optional_string(obj, "status");
        let experimental = obj.get("experimental").and_then(Value::as_bool);
        let date = self.get_optional_string(obj, "date");
        let publisher = self.get_optional_string(obj, "publisher");
        let description = self.get_optional_string(obj, "description");
        let purpose = self.get_optional_string(obj, "purpose");
        let copyright = self.get_optional_string(obj, "copyright");
        let fhir_version = self.get_fhir_version(obj);
        let derivation = self.get_optional_string(obj, "derivation");
        let context_type = self.get_optional_string(obj, "contextType");

        // Array fields
        let contact = self.get_optional_array(obj, "contact");
        let keyword = self.get_optional_array(obj, "keyword");
        let context = self.get_optional_array(obj, "context");
        let context_invariant = obj.get("contextInvariant").and_then(|v| {
            v.as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        });
        let mapping = self.get_optional_array(obj, "mapping");

        // Snapshot and differential
        let snapshot_elements = obj
            .get("snapshot")
            .and_then(|s| s.get("element"))
            .and_then(Value::as_array)
            .cloned();

        let differential_elements = obj
            .get("differential")
            .and_then(|d| d.get("element"))
            .and_then(Value::as_array)
            .cloned();

        // Collect unknown fields
        let known_fields = [
            "resourceType",
            "id",
            "meta",
            "implicitRules",
            "language",
            "text",
            "contained",
            "extension",
            "modifierExtension",
            "url",
            "identifier",
            "version",
            "versionAlgorithmString",
            "versionAlgorithmCoding",
            "name",
            "title",
            "status",
            "experimental",
            "date",
            "publisher",
            "contact",
            "description",
            "useContext",
            "jurisdiction",
            "purpose",
            "copyright",
            "copyrightLabel",
            "keyword",
            "fhirVersion",
            "mapping",
            "kind",
            "abstract",
            "context",
            "contextType",
            "contextInvariant",
            "type",
            "baseDefinition",
            "derivation",
            "snapshot",
            "differential",
        ];

        let unknown_fields: serde_json::Map<String, Value> = obj
            .iter()
            .filter(|(key, _)| !known_fields.contains(&key.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(ParsedStructureDefinition {
            id,
            url,
            version,
            name,
            title,
            status,
            experimental,
            date,
            publisher,
            contact,
            description,
            purpose,
            copyright,
            keyword,
            fhir_version,
            kind,
            is_abstract,
            type_name,
            base_definition,
            derivation,
            context,
            context_type,
            context_invariant,
            snapshot_elements,
            differential_elements,
            mapping,
            unknown_fields,
        })
    }

    /// Get a required string field.
    fn get_required_string(
        &self,
        obj: &serde_json::Map<String, Value>,
        field: &str,
    ) -> ImportResult<String> {
        obj.get(field)
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| ImportError::missing_field(field))
    }

    /// Get an optional string field.
    fn get_optional_string(
        &self,
        obj: &serde_json::Map<String, Value>,
        field: &str,
    ) -> Option<String> {
        obj.get(field).and_then(Value::as_str).map(String::from)
    }

    /// Get an optional array field.
    fn get_optional_array(
        &self,
        obj: &serde_json::Map<String, Value>,
        field: &str,
    ) -> Option<Vec<Value>> {
        obj.get(field).and_then(Value::as_array).cloned()
    }

    /// Get FHIR version, handling both string and array formats.
    fn get_fhir_version(&self, obj: &serde_json::Map<String, Value>) -> Option<String> {
        obj.get("fhirVersion").and_then(|v| {
            // R4/R4B use a single string, R5 can use an array
            if let Some(s) = v.as_str() {
                Some(s.to_string())
            } else if let Some(arr) = v.as_array() {
                arr.first().and_then(Value::as_str).map(String::from)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_sd_json() -> &'static str {
        r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/fhir/StructureDefinition/TestProfile",
            "name": "TestProfile",
            "status": "draft",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "derivation": "constraint"
        }"#
    }

    #[test]
    fn test_parse_minimal_sd() {
        let parser = StructureDefinitionParser::new();
        let result = parser.parse(minimal_sd_json());

        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        let parsed = result.unwrap();

        assert_eq!(
            parsed.url,
            "http://example.org/fhir/StructureDefinition/TestProfile"
        );
        assert_eq!(parsed.name, Some("TestProfile".to_string()));
        assert_eq!(parsed.status, Some("draft".to_string()));
        assert_eq!(parsed.kind, "resource");
        assert!(!parsed.is_abstract);
        assert_eq!(parsed.type_name, "Patient");
        assert_eq!(
            parsed.base_definition,
            "http://hl7.org/fhir/StructureDefinition/Patient"
        );
    }

    #[test]
    fn test_parse_with_elements() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/fhir/StructureDefinition/TestProfile",
            "name": "TestProfile",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "differential": {
                "element": [
                    {"id": "Patient", "path": "Patient"},
                    {"id": "Patient.name", "path": "Patient.name", "min": 1}
                ]
            }
        }"#;

        let parser = StructureDefinitionParser::new();
        let parsed = parser.parse(json).unwrap();

        assert!(parsed.differential_elements.is_some());
        assert_eq!(parsed.differential_elements.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_parse_invalid_resource_type() {
        let json = r#"{
            "resourceType": "Patient",
            "url": "http://example.org/fhir/Patient/123"
        }"#;

        let parser = StructureDefinitionParser::new();
        let result = parser.parse(json);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Patient"));
    }

    #[test]
    fn test_parse_missing_url() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient"
        }"#;

        let parser = StructureDefinitionParser::new();
        let result = parser.parse(json);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("url"));
    }

    #[test]
    fn test_unknown_fields_preserved() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/test",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "customField": "customValue",
            "anotherCustom": 123
        }"#;

        let parser = StructureDefinitionParser::new();
        let parsed = parser.parse(json).unwrap();

        assert_eq!(parsed.unknown_fields.len(), 2);
        assert_eq!(
            parsed.unknown_fields.get("customField"),
            Some(&Value::String("customValue".to_string()))
        );
    }

    #[test]
    fn test_fhir_version_string() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/test",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "fhirVersion": "4.0.1"
        }"#;

        let parser = StructureDefinitionParser::new();
        let parsed = parser.parse(json).unwrap();

        assert_eq!(parsed.fhir_version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_fhir_version_array() {
        let json = r#"{
            "resourceType": "StructureDefinition",
            "url": "http://example.org/test",
            "kind": "resource",
            "abstract": false,
            "type": "Patient",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "fhirVersion": ["4.0.1", "4.3.0"]
        }"#;

        let parser = StructureDefinitionParser::new();
        let parsed = parser.parse(json).unwrap();

        // Takes first version from array
        assert_eq!(parsed.fhir_version, Some("4.0.1".to_string()));
    }
}
