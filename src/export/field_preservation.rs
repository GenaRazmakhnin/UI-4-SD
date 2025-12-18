//! Unknown field preservation for lossless round-trip.
//!
//! Preserves unknown fields from import and re-inserts them at export time
//! to ensure byte-identical output for unchanged data.

use serde_json::{Map, Value};

use crate::ir::{ElementNode, ProfiledResource};

/// Handles preservation and restoration of unknown fields.
#[derive(Debug, Default)]
pub struct FieldPreserver {
    /// Track which fields were preserved.
    preserved_count: usize,
}

impl FieldPreserver {
    /// Create a new field preserver.
    #[must_use]
    pub fn new() -> Self {
        Self { preserved_count: 0 }
    }

    /// Merge unknown fields from the resource into the SD object.
    ///
    /// Places fields at their correct JSON paths while maintaining
    /// original ordering where possible.
    pub fn merge_resource_unknown_fields(
        &mut self,
        sd_object: &mut Map<String, Value>,
        resource: &ProfiledResource,
    ) {
        // Merge top-level unknown fields
        for (key, value) in &resource.unknown_fields {
            if !sd_object.contains_key(key) {
                sd_object.insert(key.clone(), value.clone());
                self.preserved_count += 1;
            }
        }
    }

    /// Merge unknown fields from an element into an element definition object.
    pub fn merge_element_unknown_fields(
        &mut self,
        element_object: &mut Map<String, Value>,
        element: &ElementNode,
    ) {
        for (key, value) in &element.unknown_fields {
            if !element_object.contains_key(key) {
                element_object.insert(key.clone(), value.clone());
                self.preserved_count += 1;
            }
        }
    }

    /// Get the count of preserved fields.
    #[must_use]
    pub fn preserved_count(&self) -> usize {
        self.preserved_count
    }

    /// Reset the preserved count.
    pub fn reset_count(&mut self) {
        self.preserved_count = 0;
    }
}

/// Collector for unknown fields during export.
///
/// Tracks all unknown fields found during export for reporting purposes.
#[derive(Debug, Default)]
pub struct UnknownFieldCollector {
    /// Map of path -> list of unknown field names.
    unknown_fields: Vec<UnknownFieldInfo>,
}

/// Information about an unknown field.
#[derive(Debug, Clone)]
pub struct UnknownFieldInfo {
    /// JSON path where the field was found.
    pub path: String,
    /// Field name.
    pub field_name: String,
    /// Field value type (for debugging).
    pub value_type: String,
}

impl UnknownFieldCollector {
    /// Create a new collector.
    #[must_use]
    pub fn new() -> Self {
        Self {
            unknown_fields: Vec::new(),
        }
    }

    /// Record an unknown field.
    pub fn record(&mut self, path: impl Into<String>, field_name: impl Into<String>, value: &Value) {
        self.unknown_fields.push(UnknownFieldInfo {
            path: path.into(),
            field_name: field_name.into(),
            value_type: value_type_name(value).to_string(),
        });
    }

    /// Collect unknown fields from a resource.
    pub fn collect_from_resource(&mut self, resource: &ProfiledResource) {
        // Top-level unknown fields
        for (key, value) in &resource.unknown_fields {
            self.record("StructureDefinition", key, value);
        }

        // Element unknown fields
        self.collect_from_element(&resource.root, "");
    }

    /// Recursively collect unknown fields from an element.
    fn collect_from_element(&mut self, element: &ElementNode, parent_path: &str) {
        let current_path = if parent_path.is_empty() {
            element.path.clone()
        } else {
            format!("{}.{}", parent_path, element.short_name())
        };

        for (key, value) in &element.unknown_fields {
            self.record(&current_path, key, value);
        }

        for child in &element.children {
            self.collect_from_element(child, &current_path);
        }

        for slice in element.slices.values() {
            let slice_path = format!("{}:{}", current_path, slice.name);
            self.collect_from_element(&slice.element, &slice_path);
        }
    }

    /// Get all collected unknown fields.
    #[must_use]
    pub fn fields(&self) -> &[UnknownFieldInfo] {
        &self.unknown_fields
    }

    /// Check if any unknown fields were collected.
    #[must_use]
    pub fn has_unknown_fields(&self) -> bool {
        !self.unknown_fields.is_empty()
    }

    /// Get count of unknown fields.
    #[must_use]
    pub fn count(&self) -> usize {
        self.unknown_fields.len()
    }
}

/// Get a human-readable name for a JSON value type.
fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Deep merge two JSON objects, preserving unknown fields.
///
/// Fields in `source` are copied to `target` if they don't exist in `target`.
pub fn deep_merge_objects(target: &mut Map<String, Value>, source: &Map<String, Value>) {
    for (key, value) in source {
        match target.get_mut(key) {
            Some(Value::Object(target_obj)) => {
                if let Value::Object(source_obj) = value {
                    deep_merge_objects(target_obj, source_obj);
                }
            }
            None => {
                target.insert(key.clone(), value.clone());
            }
            _ => {
                // Don't override existing non-object values
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, FhirVersion, ProfiledResource};

    #[test]
    fn test_field_preserver() {
        let mut preserver = FieldPreserver::new();
        let mut sd_object = Map::new();
        sd_object.insert("url".to_string(), Value::String("http://example.org".to_string()));

        let mut resource = ProfiledResource::new(
            "http://example.org",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        resource.unknown_fields.insert(
            "customField".to_string(),
            Value::String("customValue".to_string()),
        );

        preserver.merge_resource_unknown_fields(&mut sd_object, &resource);

        assert!(sd_object.contains_key("customField"));
        assert_eq!(preserver.preserved_count(), 1);
    }

    #[test]
    fn test_unknown_field_collector() {
        let mut resource = ProfiledResource::new(
            "http://example.org",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        resource.unknown_fields.insert(
            "customField".to_string(),
            Value::String("customValue".to_string()),
        );

        let mut collector = UnknownFieldCollector::new();
        collector.collect_from_resource(&resource);

        assert!(collector.has_unknown_fields());
        assert_eq!(collector.count(), 1);
        assert_eq!(collector.fields()[0].field_name, "customField");
    }

    #[test]
    fn test_deep_merge() {
        let mut target = Map::new();
        target.insert("a".to_string(), Value::String("original".to_string()));
        target.insert("b".to_string(), Value::Object({
            let mut inner = Map::new();
            inner.insert("x".to_string(), Value::Number(1.into()));
            inner
        }));

        let mut source = Map::new();
        source.insert("c".to_string(), Value::String("new".to_string()));
        source.insert("b".to_string(), Value::Object({
            let mut inner = Map::new();
            inner.insert("y".to_string(), Value::Number(2.into()));
            inner
        }));

        deep_merge_objects(&mut target, &source);

        assert!(target.contains_key("a"));
        assert!(target.contains_key("b"));
        assert!(target.contains_key("c"));

        // Check nested merge
        let b = target.get("b").unwrap().as_object().unwrap();
        assert!(b.contains_key("x"));
        assert!(b.contains_key("y"));
    }
}
