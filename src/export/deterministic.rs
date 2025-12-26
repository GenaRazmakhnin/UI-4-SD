//! Deterministic JSON serialization for StructureDefinition.
//!
//! Ensures byte-identical output for the same IR state by:
//! - Stable field ordering (FHIR spec order for SD, alphabetical for elements)
//! - Consistent handling of optional fields
//! - Canonical JSON formatting

use indexmap::IndexMap;
use serde_json::{Map, Value};

/// FHIR StructureDefinition field order (canonical).
/// This ordering follows the FHIR spec element order for StructureDefinition.
const SD_FIELD_ORDER: &[&str] = &[
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

/// FHIR ElementDefinition field order (canonical).
const ELEMENT_FIELD_ORDER: &[&str] = &[
    "id",
    "extension",
    "modifierExtension",
    "path",
    "representation",
    "sliceName",
    "sliceIsConstraining",
    "label",
    "code",
    "slicing",
    "short",
    "definition",
    "comment",
    "requirements",
    "alias",
    "min",
    "max",
    "base",
    "contentReference",
    "type",
    "defaultValue",
    "meaningWhenMissing",
    "orderMeaning",
    "fixed",
    "pattern",
    "example",
    "minValue",
    "maxValue",
    "maxLength",
    "condition",
    "constraint",
    "mustHaveValue",
    "valueAlternatives",
    "mustSupport",
    "isModifier",
    "isModifierReason",
    "isSummary",
    "binding",
    "mapping",
];

/// Builder for deterministic JSON objects.
#[derive(Debug, Default)]
pub struct DeterministicJsonBuilder {
    /// The JSON object being built.
    fields: IndexMap<String, Value>,
    /// Field order to use.
    field_order: Vec<&'static str>,
}

impl DeterministicJsonBuilder {
    /// Create a new builder for StructureDefinition.
    #[must_use]
    pub fn for_structure_definition() -> Self {
        Self {
            fields: IndexMap::new(),
            field_order: SD_FIELD_ORDER.to_vec(),
        }
    }

    /// Create a new builder for ElementDefinition.
    #[must_use]
    pub fn for_element() -> Self {
        Self {
            fields: IndexMap::new(),
            field_order: ELEMENT_FIELD_ORDER.to_vec(),
        }
    }

    /// Create a new builder with custom field order.
    #[must_use]
    pub fn with_order(order: Vec<&'static str>) -> Self {
        Self {
            fields: IndexMap::new(),
            field_order: order,
        }
    }

    /// Add a field if the value is not None.
    pub fn add_optional<T: Into<Value>>(&mut self, key: &str, value: Option<T>) -> &mut Self {
        if let Some(v) = value {
            self.fields.insert(key.to_string(), v.into());
        }
        self
    }

    /// Add a field with a value.
    pub fn add<T: Into<Value>>(&mut self, key: &str, value: T) -> &mut Self {
        self.fields.insert(key.to_string(), value.into());
        self
    }

    /// Add a string field if not empty.
    pub fn add_string(&mut self, key: &str, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.fields
                .insert(key.to_string(), Value::String(value.to_string()));
        }
        self
    }

    /// Add an optional string field.
    pub fn add_optional_string(&mut self, key: &str, value: Option<&str>) -> &mut Self {
        if let Some(v) = value {
            if !v.is_empty() {
                self.fields
                    .insert(key.to_string(), Value::String(v.to_string()));
            }
        }
        self
    }

    /// Add a boolean field only if true.
    pub fn add_bool_if_true(&mut self, key: &str, value: bool) -> &mut Self {
        if value {
            self.fields.insert(key.to_string(), Value::Bool(true));
        }
        self
    }

    /// Add a boolean field.
    pub fn add_bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.fields.insert(key.to_string(), Value::Bool(value));
        self
    }

    /// Add a number field.
    pub fn add_number(&mut self, key: &str, value: impl Into<serde_json::Number>) -> &mut Self {
        self.fields
            .insert(key.to_string(), Value::Number(value.into()));
        self
    }

    /// Add an optional number field.
    pub fn add_optional_number<N: Into<serde_json::Number>>(
        &mut self,
        key: &str,
        value: Option<N>,
    ) -> &mut Self {
        if let Some(v) = value {
            self.fields.insert(key.to_string(), Value::Number(v.into()));
        }
        self
    }

    /// Add an array field if not empty.
    pub fn add_array(&mut self, key: &str, value: Vec<Value>) -> &mut Self {
        if !value.is_empty() {
            self.fields.insert(key.to_string(), Value::Array(value));
        }
        self
    }

    /// Add an object field.
    pub fn add_object(&mut self, key: &str, value: Map<String, Value>) -> &mut Self {
        if !value.is_empty() {
            self.fields.insert(key.to_string(), Value::Object(value));
        }
        self
    }

    /// Add a pre-built JSON value.
    pub fn add_value(&mut self, key: &str, value: Value) -> &mut Self {
        if !is_empty_value(&value) {
            self.fields.insert(key.to_string(), value);
        }
        self
    }

    /// Merge unknown fields into the builder.
    pub fn merge_unknown(&mut self, unknown: &Map<String, Value>) -> &mut Self {
        for (key, value) in unknown {
            if !self.fields.contains_key(key) {
                self.fields.insert(key.clone(), value.clone());
            }
        }
        self
    }

    /// Build the JSON object with deterministic field ordering.
    #[must_use]
    pub fn build(self) -> Map<String, Value> {
        let mut result = Map::new();

        // Add fields in canonical order
        for &key in &self.field_order {
            if let Some(value) = self.fields.get(key) {
                result.insert(key.to_string(), value.clone());
            }
        }

        // Add any remaining fields alphabetically
        let mut extra_fields: Vec<_> = self
            .fields
            .iter()
            .filter(|(k, _)| !self.field_order.contains(&k.as_str()))
            .collect();
        extra_fields.sort_by_key(|(k, _)| k.as_str());

        for (key, value) in extra_fields {
            result.insert(key.clone(), value.clone());
        }

        result
    }

    /// Build and convert to a JSON Value.
    #[must_use]
    pub fn build_value(self) -> Value {
        Value::Object(self.build())
    }
}

/// Check if a JSON value is "empty" (null, empty string, empty array, empty object).
fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        Value::Object(o) => o.is_empty(),
        _ => false,
    }
}

/// Sort an array of element definitions by path for deterministic ordering.
pub fn sort_elements_by_path(elements: &mut [Value]) {
    elements.sort_by(|a, b| {
        let path_a = a.get("path").and_then(Value::as_str).unwrap_or("");
        let path_b = b.get("path").and_then(Value::as_str).unwrap_or("");
        compare_element_paths(path_a, path_b)
    });
}

/// Compare two element paths for ordering.
/// Handles slice names correctly according to FHIR spec:
/// - Base element comes first
/// - Slices of an element come before children of that element
/// - E.g., "Patient.name" < "Patient.name:official" < "Patient.name.family"
fn compare_element_paths(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let parts_a: Vec<&str> = a.split('.').collect();
    let parts_b: Vec<&str> = b.split('.').collect();

    let min_len = parts_a.len().min(parts_b.len());

    for i in 0..min_len {
        let pa = parts_a[i];
        let pb = parts_b[i];

        // Split by colon for slice names
        let (base_a, slice_a) = split_slice_name(pa);
        let (base_b, slice_b) = split_slice_name(pb);

        // Compare base names first
        match base_a.cmp(base_b) {
            Ordering::Equal => {}
            other => return other,
        }

        // If base names are equal, compare slice names
        match (slice_a, slice_b) {
            (None, None) => {}
            (None, Some(_)) => {
                // No slice vs slice: check if a has more segments
                // If a has more segments (a child), slice comes first
                if parts_a.len() > i + 1 {
                    return Ordering::Greater; // Child comes after slice
                }
                return Ordering::Less; // Base comes before slice
            }
            (Some(_), None) => {
                // Slice vs no slice: check if b has more segments
                if parts_b.len() > i + 1 {
                    return Ordering::Less; // Slice comes before child
                }
                return Ordering::Greater; // Slice comes after base
            }
            (Some(sa), Some(sb)) => match sa.cmp(sb) {
                Ordering::Equal => {}
                other => return other,
            },
        }
    }

    // Shorter paths come first
    parts_a.len().cmp(&parts_b.len())
}

/// Split an element path segment into base name and optional slice name.
fn split_slice_name(segment: &str) -> (&str, Option<&str>) {
    if let Some(idx) = segment.find(':') {
        (&segment[..idx], Some(&segment[idx + 1..]))
    } else {
        (segment, None)
    }
}

/// Serialize to canonical JSON string (compact, no extra whitespace).
pub fn to_canonical_json(value: &Value) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Serialize to pretty-printed JSON with consistent formatting.
pub fn to_pretty_json(value: &Value) -> Result<String, serde_json::Error> {
    let sorted = recursively_sort_value(value);
    serde_json::to_string_pretty(&sorted)
}

/// Recursively sort keys in a JSON Value to ensure deterministic output.
///
/// This is used to ensure that diffs are clean even when fields are merged
/// from sources with different key ordering.
pub fn recursively_sort_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted_map = Map::new();
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                if let Some(val) = map.get(&key) {
                    sorted_map.insert(key, recursively_sort_value(val));
                }
            }
            Value::Object(sorted_map)
        }
        Value::Array(arr) => {
            let sorted_arr = arr.iter().map(recursively_sort_value).collect();
            Value::Array(sorted_arr)
        }
        _ => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_builder() {
        let mut builder = DeterministicJsonBuilder::for_structure_definition();
        builder
            .add_string("resourceType", "StructureDefinition")
            .add_string("url", "http://example.org/test")
            .add_string("name", "Test")
            .add_string("status", "draft");

        let result = builder.build();

        // Check order: resourceType should come before url, url before name, etc.
        let keys: Vec<&String> = result.keys().collect();
        assert!(
            keys.iter().position(|k| *k == "resourceType") < keys.iter().position(|k| *k == "url")
        );
        assert!(keys.iter().position(|k| *k == "url") < keys.iter().position(|k| *k == "name"));
    }

    #[test]
    fn test_element_path_ordering() {
        let mut paths = vec![
            "Patient.name:official",
            "Patient",
            "Patient.name.family",
            "Patient.name",
            "Patient.identifier",
        ];

        paths.sort_by(|a, b| compare_element_paths(a, b));

        assert_eq!(
            paths,
            vec![
                "Patient",
                "Patient.identifier",
                "Patient.name",
                "Patient.name:official",
                "Patient.name.family",
            ]
        );
    }

    #[test]
    fn test_optional_fields_omitted() {
        let mut builder = DeterministicJsonBuilder::for_element();
        builder
            .add_string("path", "Patient.name")
            .add_optional_string("short", None)
            .add_optional_string("definition", Some("A name"));

        let result = builder.build();

        assert!(result.contains_key("path"));
        assert!(result.contains_key("definition"));
        assert!(!result.contains_key("short"));
    }

    #[test]
    fn test_empty_values_omitted() {
        let mut builder = DeterministicJsonBuilder::for_element();
        builder
            .add_string("path", "Patient.name")
            .add_array("alias", vec![])
            .add_array("type", vec![Value::String("HumanName".to_string())]);

        let result = builder.build();

        assert!(result.contains_key("path"));
        assert!(result.contains_key("type"));
        assert!(!result.contains_key("alias")); // Empty array omitted
    }
}
