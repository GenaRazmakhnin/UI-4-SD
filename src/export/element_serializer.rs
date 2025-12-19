//! Element serialization for StructureDefinition export.
//!
//! Converts IR [`ElementNode`] to FHIR ElementDefinition JSON format.

use serde_json::{Map, Value};

use crate::ir::{
    Binding, BindingStrength, DifferentialElement, ElementConstraints, ElementNode, ElementSource,
    FixedValue, Invariant, InvariantSeverity, SliceNode, SlicingDefinition, TypeConstraint,
};

use super::deterministic::DeterministicJsonBuilder;
use super::error::ExportResult;

/// Serializes IR elements to ElementDefinition JSON.
#[derive(Debug, Default)]
pub struct ElementSerializer {
    /// Whether to include inherited (unmodified) elements in output.
    include_inherited: bool,
}

impl ElementSerializer {
    /// Create a new element serializer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            include_inherited: true,
        }
    }

    /// Configure whether to include inherited elements.
    #[must_use]
    pub fn include_inherited(mut self, include: bool) -> Self {
        self.include_inherited = include;
        self
    }

    /// Serialize an element node to ElementDefinition JSON.
    pub fn serialize_element(&self, element: &ElementNode) -> ExportResult<Value> {
        let mut builder = DeterministicJsonBuilder::for_element();

        // Element identity
        builder.add_string("id", &self.generate_element_id(element));
        builder.add_string("path", &element.path);

        // Slicing definition (if present)
        if let Some(slicing) = &element.slicing {
            builder.add_value("slicing", self.serialize_slicing(slicing)?);
        }

        // Constraints
        self.serialize_constraints(&mut builder, &element.constraints)?;

        // Merge unknown fields
        builder.merge_unknown(&element.unknown_fields);

        Ok(builder.build_value())
    }

    /// Serialize an element for differential (only modified fields).
    pub fn serialize_element_differential(&self, element: &ElementNode) -> ExportResult<Option<Value>> {
        // Skip unmodified elements in differential
        if element.source == ElementSource::Inherited && !element.constraints.has_any() {
            return Ok(None);
        }

        let mut builder = DeterministicJsonBuilder::for_element();

        // Always include id and path
        builder.add_string("id", &self.generate_element_id(element));
        builder.add_string("path", &element.path);

        // Slicing definition (if present and modified)
        if let Some(slicing) = &element.slicing {
            builder.add_value("slicing", self.serialize_slicing(slicing)?);
        }

        // Only modified constraints
        if element.source.is_modified() || element.constraints.has_any() {
            self.serialize_constraints(&mut builder, &element.constraints)?;
        }

        Ok(Some(builder.build_value()))
    }

    /// Serialize a slice element.
    pub fn serialize_slice(&self, slice: &SliceNode, parent_path: &str) -> ExportResult<Value> {
        let mut builder = DeterministicJsonBuilder::for_element();

        // Slice identity
        let slice_path = format!("{}:{}", parent_path, slice.name);
        builder.add_string("id", &self.generate_slice_id(&slice_path));
        builder.add_string("path", parent_path);
        builder.add_string("sliceName", &slice.name);

        // Constraints from the slice element
        self.serialize_constraints(&mut builder, &slice.element.constraints)?;

        // Merge unknown fields
        builder.merge_unknown(&slice.element.unknown_fields);

        Ok(builder.build_value())
    }

    /// Serialize a differential element (flat representation).
    pub fn serialize_differential_element(
        &self,
        diff: &DifferentialElement,
    ) -> ExportResult<Value> {
        let mut builder = DeterministicJsonBuilder::for_element();

        if let Some(slice_name) = diff.slice_name.as_deref() {
            let slice_id = format!("{}:{}", diff.path, slice_name);
            builder.add_string("id", &slice_id);
            builder.add_string("path", &diff.path);
            builder.add_string("sliceName", slice_name);
        } else {
            let id = diff.element_id.clone().unwrap_or_else(|| diff.path.clone());
            builder.add_string("id", &id);
            builder.add_string("path", &diff.path);
        }

        if let Some(slicing) = &diff.slicing {
            builder.add_value("slicing", self.serialize_slicing(slicing)?);
        }

        self.serialize_constraints(&mut builder, &diff.constraints)?;

        builder.merge_unknown(&diff.unknown_fields);

        Ok(builder.build_value())
    }

    /// Serialize constraints to the builder.
    fn serialize_constraints(
        &self,
        builder: &mut DeterministicJsonBuilder,
        constraints: &ElementConstraints,
    ) -> ExportResult<()> {
        // Short description
        builder.add_optional_string("short", constraints.short.as_deref());

        // Definition
        builder.add_optional_string("definition", constraints.definition.as_deref());

        // Comment
        builder.add_optional_string("comment", constraints.comment.as_deref());

        // Requirements
        builder.add_optional_string("requirements", constraints.requirements.as_deref());

        // Aliases
        if !constraints.alias.is_empty() {
            let aliases: Vec<Value> = constraints
                .alias
                .iter()
                .map(|a| Value::String(a.clone()))
                .collect();
            builder.add_array("alias", aliases);
        }

        // Cardinality
        if let Some(card) = &constraints.cardinality {
            builder.add_number("min", card.min);
            builder.add_string("max", &self.format_max(card.max));
        }

        // Types
        if !constraints.types.is_empty() {
            let types: Vec<Value> = constraints
                .types
                .iter()
                .map(|t| self.serialize_type(t))
                .collect();
            builder.add_array("type", types);
        }

        // Fixed/Pattern values
        if let Some(fixed_value) = &constraints.fixed_value {
            self.serialize_fixed_value(builder, fixed_value);
        }

        // Default value
        if let Some(default) = &constraints.default_value {
            self.serialize_polymorphic_value(builder, "defaultValue", default);
        }

        // Meaning when missing
        builder.add_optional_string("meaningWhenMissing", constraints.meaning_when_missing.as_deref());

        // Max length
        if let Some(max_len) = constraints.max_length {
            builder.add_number("maxLength", max_len);
        }

        // Examples
        if !constraints.examples.is_empty() {
            let examples: Vec<Value> = constraints
                .examples
                .iter()
                .map(|ex| {
                    let mut obj = Map::new();
                    obj.insert("label".to_string(), Value::String(ex.label.clone()));
                    // Determine the type suffix for the example value
                    self.insert_polymorphic_value(&mut obj, "value", &ex.value);
                    Value::Object(obj)
                })
                .collect();
            builder.add_array("example", examples);
        }

        // Flags
        builder.add_bool_if_true("mustSupport", constraints.flags.must_support);
        builder.add_bool_if_true("isModifier", constraints.flags.is_modifier);
        builder.add_optional_string("isModifierReason", constraints.flags.is_modifier_reason.as_deref());
        builder.add_bool_if_true("isSummary", constraints.flags.is_summary);

        // Binding
        if let Some(binding) = &constraints.binding {
            builder.add_value("binding", self.serialize_binding(binding));
        }

        // Invariants (constraints)
        if !constraints.invariants.is_empty() {
            let constraint_array: Vec<Value> = constraints
                .invariants
                .values()
                .map(|inv| self.serialize_invariant(inv))
                .collect();
            builder.add_array("constraint", constraint_array);
        }

        // Mappings
        if !constraints.mappings.is_empty() {
            let mappings: Vec<Value> = constraints
                .mappings
                .iter()
                .map(|m| {
                    let mut obj = Map::new();
                    obj.insert("identity".to_string(), Value::String(m.identity.clone()));
                    obj.insert("map".to_string(), Value::String(m.map.clone()));
                    if let Some(comment) = &m.comment {
                        obj.insert("comment".to_string(), Value::String(comment.clone()));
                    }
                    if let Some(language) = &m.language {
                        obj.insert("language".to_string(), Value::String(language.clone()));
                    }
                    Value::Object(obj)
                })
                .collect();
            builder.add_array("mapping", mappings);
        }

        Ok(())
    }

    /// Serialize a type constraint.
    fn serialize_type(&self, type_constraint: &TypeConstraint) -> Value {
        let mut obj = Map::new();

        obj.insert("code".to_string(), Value::String(type_constraint.code.clone()));

        if !type_constraint.profile.is_empty() {
            let profiles: Vec<Value> = type_constraint
                .profile
                .iter()
                .map(|p| Value::String(p.clone()))
                .collect();
            obj.insert("profile".to_string(), Value::Array(profiles));
        }

        if !type_constraint.target_profile.is_empty() {
            let targets: Vec<Value> = type_constraint
                .target_profile
                .iter()
                .map(|p| Value::String(p.clone()))
                .collect();
            obj.insert("targetProfile".to_string(), Value::Array(targets));
        }

        if !type_constraint.aggregation.is_empty() {
            let aggregations: Vec<Value> = type_constraint
                .aggregation
                .iter()
                .map(|a| Value::String(a.clone()))
                .collect();
            obj.insert("aggregation".to_string(), Value::Array(aggregations));
        }

        if let Some(versioning) = &type_constraint.versioning {
            obj.insert("versioning".to_string(), Value::String(versioning.clone()));
        }

        Value::Object(obj)
    }

    /// Serialize slicing definition.
    fn serialize_slicing(&self, slicing: &SlicingDefinition) -> ExportResult<Value> {
        let mut obj = Map::new();

        // Discriminators
        if !slicing.discriminator.is_empty() {
            let discriminators: Vec<Value> = slicing
                .discriminator
                .iter()
                .map(|d| {
                    let mut disc_obj = Map::new();
                    disc_obj.insert(
                        "type".to_string(),
                        Value::String(d.discriminator_type.as_str().to_string()),
                    );
                    disc_obj.insert("path".to_string(), Value::String(d.path.clone()));
                    Value::Object(disc_obj)
                })
                .collect();
            obj.insert("discriminator".to_string(), Value::Array(discriminators));
        }

        // Description
        if let Some(desc) = &slicing.description {
            obj.insert("description".to_string(), Value::String(desc.clone()));
        }

        // Ordered
        if slicing.ordered {
            obj.insert("ordered".to_string(), Value::Bool(true));
        }

        // Rules
        obj.insert("rules".to_string(), Value::String(slicing.rules.as_str().to_string()));

        Ok(Value::Object(obj))
    }

    /// Serialize a binding.
    fn serialize_binding(&self, binding: &Binding) -> Value {
        let mut obj = Map::new();

        obj.insert(
            "strength".to_string(),
            Value::String(self.format_binding_strength(binding.strength)),
        );

        obj.insert("valueSet".to_string(), Value::String(binding.value_set.clone()));

        if let Some(desc) = &binding.description {
            obj.insert("description".to_string(), Value::String(desc.clone()));
        }

        Value::Object(obj)
    }

    /// Serialize an invariant.
    fn serialize_invariant(&self, invariant: &Invariant) -> Value {
        let mut obj = Map::new();

        obj.insert("key".to_string(), Value::String(invariant.key.clone()));
        obj.insert(
            "severity".to_string(),
            Value::String(self.format_severity(invariant.severity)),
        );
        obj.insert("human".to_string(), Value::String(invariant.human.clone()));
        obj.insert("expression".to_string(), Value::String(invariant.expression.clone()));

        if let Some(ref xpath) = invariant.xpath {
            obj.insert("xpath".to_string(), Value::String(xpath.to_string()));
        }

        if let Some(ref source) = invariant.source {
            obj.insert("source".to_string(), Value::String(source.to_string()));
        }

        Value::Object(obj)
    }

    /// Serialize fixed/pattern value.
    fn serialize_fixed_value(&self, builder: &mut DeterministicJsonBuilder, fixed_value: &FixedValue) {
        let (prefix, value) = match fixed_value {
            FixedValue::Fixed(v) => ("fixed", v),
            FixedValue::Pattern(v) => ("pattern", v),
        };

        self.serialize_polymorphic_value(builder, prefix, value);
    }

    /// Serialize a polymorphic value (fixedX, patternX, defaultValueX, etc.).
    fn serialize_polymorphic_value(
        &self,
        builder: &mut DeterministicJsonBuilder,
        prefix: &str,
        value: &Value,
    ) {
        let suffix = self.get_type_suffix(value);
        let key = format!("{}{}", prefix, suffix);
        builder.add_value(&key, value.clone());
    }

    /// Insert a polymorphic value into a map.
    fn insert_polymorphic_value(&self, map: &mut Map<String, Value>, prefix: &str, value: &Value) {
        let suffix = self.get_type_suffix(value);
        let key = format!("{}{}", prefix, suffix);
        map.insert(key, value.clone());
    }

    /// Get the FHIR type suffix for a JSON value.
    fn get_type_suffix(&self, value: &Value) -> &'static str {
        match value {
            Value::Bool(_) => "Boolean",
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    "Integer"
                } else {
                    "Decimal"
                }
            }
            Value::String(_) => "String",
            Value::Array(_) => "String", // Arrays typically don't have polymorphic keys
            Value::Object(obj) => {
                // Try to determine type from object structure
                if obj.contains_key("system") && obj.contains_key("code") {
                    if obj.contains_key("coding") {
                        "CodeableConcept"
                    } else {
                        "Coding"
                    }
                } else if obj.contains_key("reference") {
                    "Reference"
                } else if obj.contains_key("value") && obj.contains_key("unit") {
                    "Quantity"
                } else if obj.contains_key("start") || obj.contains_key("end") {
                    "Period"
                } else if obj.contains_key("family") || obj.contains_key("given") {
                    "HumanName"
                } else if obj.contains_key("line") || obj.contains_key("city") {
                    "Address"
                } else {
                    "String" // Default fallback
                }
            }
            Value::Null => "String",
        }
    }

    /// Generate element ID from path.
    fn generate_element_id(&self, element: &ElementNode) -> String {
        element.element_id.clone().unwrap_or_else(|| element.path.clone())
    }

    /// Generate slice element ID.
    fn generate_slice_id(&self, slice_path: &str) -> String {
        slice_path.to_string()
    }

    /// Format max cardinality for FHIR.
    fn format_max(&self, max: Option<u32>) -> String {
        match max {
            Some(n) => n.to_string(),
            None => "*".to_string(),
        }
    }

    /// Format binding strength for FHIR.
    fn format_binding_strength(&self, strength: BindingStrength) -> String {
        strength.as_str().to_string()
    }

    /// Format invariant severity for FHIR.
    fn format_severity(&self, severity: InvariantSeverity) -> String {
        match severity {
            InvariantSeverity::Error => "error".to_string(),
            InvariantSeverity::Warning => "warning".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Cardinality, ElementNode, TypeConstraint};

    #[test]
    fn test_serialize_simple_element() {
        let mut element = ElementNode::new("Patient.name".to_string());
        element.constraints.cardinality = Some(Cardinality::required());
        element.constraints.short = Some("A name for the patient".to_string());

        let serializer = ElementSerializer::new();
        let result = serializer.serialize_element(&element).unwrap();

        assert_eq!(result.get("path").unwrap(), "Patient.name");
        assert_eq!(result.get("min").unwrap(), 1);
        assert_eq!(result.get("max").unwrap(), "1");
        assert!(result.get("short").is_some());
    }

    #[test]
    fn test_serialize_type_constraint() {
        let mut element = ElementNode::new("Patient.name".to_string());
        element.constraints.types.push(TypeConstraint::simple("HumanName"));

        let serializer = ElementSerializer::new();
        let result = serializer.serialize_element(&element).unwrap();

        let types = result.get("type").unwrap().as_array().unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].get("code").unwrap(), "HumanName");
    }

    #[test]
    fn test_serialize_binding() {
        let mut element = ElementNode::new("Patient.gender".to_string());
        element.constraints.binding = Some(Binding::required(
            "http://hl7.org/fhir/ValueSet/administrative-gender",
        ));

        let serializer = ElementSerializer::new();
        let result = serializer.serialize_element(&element).unwrap();

        let binding = result.get("binding").unwrap();
        assert_eq!(binding.get("strength").unwrap(), "required");
    }
}
