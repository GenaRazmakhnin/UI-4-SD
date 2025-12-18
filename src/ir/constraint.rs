//! Element constraints for FHIR profile elements.
//!
//! This module defines the constraint types that can be applied to elements
//! in a FHIR profile, including cardinality, type constraints, terminology
//! bindings, and fixed/pattern values.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Cardinality constraint for an element (min..max).
///
/// # Example
///
/// ```
/// use niten::ir::Cardinality;
///
/// // Required single element: 1..1
/// let required = Cardinality::new(1, Some(1));
///
/// // Optional with unbounded: 0..*
/// let unbounded = Cardinality::new(0, None);
///
/// // Required with at least one: 1..*
/// let at_least_one = Cardinality::new(1, None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cardinality {
    /// Minimum cardinality (0 or more).
    pub min: u32,
    /// Maximum cardinality (None = unbounded "*").
    pub max: Option<u32>,
}

impl Cardinality {
    /// Create a new cardinality constraint.
    #[must_use]
    pub const fn new(min: u32, max: Option<u32>) -> Self {
        Self { min, max }
    }

    /// Create 0..1 (optional single).
    #[must_use]
    pub const fn optional() -> Self {
        Self { min: 0, max: Some(1) }
    }

    /// Create 1..1 (required single).
    #[must_use]
    pub const fn required() -> Self {
        Self { min: 1, max: Some(1) }
    }

    /// Create 0..* (optional unbounded).
    #[must_use]
    pub const fn unbounded() -> Self {
        Self { min: 0, max: None }
    }

    /// Create 1..* (required unbounded).
    #[must_use]
    pub const fn required_unbounded() -> Self {
        Self { min: 1, max: None }
    }

    /// Check if this cardinality is more restrictive than another.
    #[must_use]
    pub fn is_more_restrictive_than(&self, other: &Self) -> bool {
        let min_ok = self.min >= other.min;
        let max_ok = match (self.max, other.max) {
            (Some(s), Some(o)) => s <= o,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => true,
        };
        min_ok && max_ok
    }

    /// Check if this allows the element to be absent.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.min == 0
    }

    /// Check if this allows multiple values.
    #[must_use]
    pub const fn is_repeating(&self) -> bool {
        match self.max {
            Some(max) => max > 1,
            None => true,
        }
    }

    /// Format as FHIR cardinality string (e.g., "0..1", "1..*").
    #[must_use]
    pub fn to_fhir_string(&self) -> String {
        match self.max {
            Some(max) => format!("{}..{}", self.min, max),
            None => format!("{}..*", self.min),
        }
    }
}

impl Default for Cardinality {
    fn default() -> Self {
        Self::unbounded()
    }
}

impl std::fmt::Display for Cardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fhir_string())
    }
}

/// Type constraint for an element.
///
/// Specifies allowed types for a polymorphic element or profiles that
/// the type must conform to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeConstraint {
    /// FHIR type code (e.g., "string", "Reference", "CodeableConcept").
    pub code: String,

    /// Profile URLs that the type must conform to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<String>,

    /// Target profiles for Reference types.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_profile: Vec<String>,

    /// Aggregation modes for Reference types.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aggregation: Vec<String>,

    /// Version handling for Reference types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning: Option<String>,
}

impl TypeConstraint {
    /// Create a simple type constraint with just a code.
    #[must_use]
    pub fn simple(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            profile: Vec::new(),
            target_profile: Vec::new(),
            aggregation: Vec::new(),
            versioning: None,
        }
    }

    /// Create a type constraint with a profile.
    #[must_use]
    pub fn with_profile(code: impl Into<String>, profile: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            profile: vec![profile.into()],
            target_profile: Vec::new(),
            aggregation: Vec::new(),
            versioning: None,
        }
    }

    /// Create a Reference type constraint with target profiles.
    #[must_use]
    pub fn reference(targets: Vec<String>) -> Self {
        Self {
            code: "Reference".to_string(),
            profile: Vec::new(),
            target_profile: targets,
            aggregation: Vec::new(),
            versioning: None,
        }
    }
}

/// Terminology binding strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BindingStrength {
    /// Required: must be from the specified value set.
    Required,
    /// Extensible: should be from value set, but can use others with good reason.
    #[default]
    Extensible,
    /// Preferred: recommended to use value set.
    Preferred,
    /// Example: value set is just an example.
    Example,
}

impl BindingStrength {
    /// Check if this binding requires validation.
    #[must_use]
    pub const fn requires_validation(&self) -> bool {
        matches!(self, Self::Required | Self::Extensible)
    }

    /// Get the FHIR code for this strength.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Extensible => "extensible",
            Self::Preferred => "preferred",
            Self::Example => "example",
        }
    }
}

impl std::fmt::Display for BindingStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Terminology binding for coded elements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binding {
    /// Binding strength.
    pub strength: BindingStrength,

    /// Canonical URL of the value set.
    pub value_set: String,

    /// Human-readable description of the binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Binding {
    /// Create a new binding.
    #[must_use]
    pub fn new(strength: BindingStrength, value_set: impl Into<String>) -> Self {
        Self {
            strength,
            value_set: value_set.into(),
            description: None,
        }
    }

    /// Create a required binding.
    #[must_use]
    pub fn required(value_set: impl Into<String>) -> Self {
        Self::new(BindingStrength::Required, value_set)
    }

    /// Create an extensible binding.
    #[must_use]
    pub fn extensible(value_set: impl Into<String>) -> Self {
        Self::new(BindingStrength::Extensible, value_set)
    }

    /// Add a description.
    #[must_use]
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Fixed or pattern value for an element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FixedValue {
    /// Exact fixed value (must match exactly).
    Fixed(serde_json::Value),
    /// Pattern value (must contain at least these fields).
    Pattern(serde_json::Value),
}

impl FixedValue {
    /// Create a fixed value.
    #[must_use]
    pub fn fixed(value: serde_json::Value) -> Self {
        Self::Fixed(value)
    }

    /// Create a pattern value.
    #[must_use]
    pub fn pattern(value: serde_json::Value) -> Self {
        Self::Pattern(value)
    }

    /// Check if this is a fixed (exact match) value.
    #[must_use]
    pub const fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    /// Check if this is a pattern value.
    #[must_use]
    pub const fn is_pattern(&self) -> bool {
        matches!(self, Self::Pattern(_))
    }

    /// Get the underlying JSON value.
    #[must_use]
    pub const fn value(&self) -> &serde_json::Value {
        match self {
            Self::Fixed(v) | Self::Pattern(v) => v,
        }
    }
}

/// Element flags (mustSupport, isModifier, etc.).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementFlags {
    /// Element must be supported by implementations.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub must_support: bool,

    /// Element is a modifier element.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_modifier: bool,

    /// Reason why this is a modifier (required if is_modifier is true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifier_reason: Option<String>,

    /// Element is included in summary view.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_summary: bool,
}

impl ElementFlags {
    /// Check if any flags are set.
    #[must_use]
    pub fn has_any(&self) -> bool {
        self.must_support || self.is_modifier || self.is_summary
    }
}

/// Complete set of constraints for an element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ElementConstraints {
    /// Cardinality constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<Cardinality>,

    /// Allowed types (for choice elements or type restrictions).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<TypeConstraint>,

    /// Short description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<String>,

    /// Full definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,

    /// Comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<String>,

    /// Alternate names/aliases.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<String>,

    /// Fixed/pattern value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_value: Option<FixedValue>,

    /// Default value (for when element is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,

    /// Meaning when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meaning_when_missing: Option<String>,

    /// Terminology binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding: Option<Binding>,

    /// Element flags.
    #[serde(default, skip_serializing_if = "is_default_flags")]
    pub flags: ElementFlags,

    /// FHIRPath invariants (constraint key -> expression).
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub invariants: IndexMap<String, Invariant>,

    /// Mapping to other specifications.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<Mapping>,

    /// Maximum length (for string types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,

    /// Example values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Example>,
}

fn is_default_flags(flags: &ElementFlags) -> bool {
    !flags.has_any()
}

impl ElementConstraints {
    /// Check if any constraints are set.
    #[must_use]
    pub fn has_any(&self) -> bool {
        self.cardinality.is_some()
            || !self.types.is_empty()
            || self.short.is_some()
            || self.definition.is_some()
            || self.fixed_value.is_some()
            || self.binding.is_some()
            || self.flags.has_any()
            || !self.invariants.is_empty()
    }

    /// Set cardinality.
    pub fn with_cardinality(mut self, cardinality: Cardinality) -> Self {
        self.cardinality = Some(cardinality);
        self
    }

    /// Add a type constraint.
    pub fn with_type(mut self, type_constraint: TypeConstraint) -> Self {
        self.types.push(type_constraint);
        self
    }

    /// Set binding.
    pub fn with_binding(mut self, binding: Binding) -> Self {
        self.binding = Some(binding);
        self
    }

    /// Set must support flag.
    pub fn must_support(mut self) -> Self {
        self.flags.must_support = true;
        self
    }
}

/// FHIRPath invariant constraint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invariant {
    /// Invariant key (e.g., "ele-1").
    pub key: String,

    /// Severity (error or warning).
    pub severity: InvariantSeverity,

    /// Human-readable description.
    pub human: String,

    /// FHIRPath expression.
    pub expression: String,

    /// XPath expression (legacy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpath: Option<String>,

    /// Source of the invariant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Invariant severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum InvariantSeverity {
    /// Violation is an error.
    #[default]
    Error,
    /// Violation is a warning.
    Warning,
}

/// Mapping to external specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mapping {
    /// Mapping identity (e.g., "rim", "v2").
    pub identity: String,

    /// Target element in external spec.
    pub map: String,

    /// Human-readable comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Language of the mapping (e.g., "text/fhirpath").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Example value for an element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Example {
    /// Label describing the example.
    pub label: String,

    /// The example value.
    pub value: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardinality_formatting() {
        assert_eq!(Cardinality::required().to_string(), "1..1");
        assert_eq!(Cardinality::optional().to_string(), "0..1");
        assert_eq!(Cardinality::unbounded().to_string(), "0..*");
        assert_eq!(Cardinality::new(2, Some(5)).to_string(), "2..5");
    }

    #[test]
    fn test_cardinality_comparison() {
        let base = Cardinality::unbounded(); // 0..*
        let restricted = Cardinality::required(); // 1..1

        assert!(restricted.is_more_restrictive_than(&base));
        assert!(!base.is_more_restrictive_than(&restricted));
    }

    #[test]
    fn test_binding_creation() {
        let binding = Binding::required("http://example.org/ValueSet/test")
            .with_description("Test binding");

        assert_eq!(binding.strength, BindingStrength::Required);
        assert_eq!(binding.value_set, "http://example.org/ValueSet/test");
        assert_eq!(binding.description.as_deref(), Some("Test binding"));
    }

    #[test]
    fn test_type_constraint() {
        let reference = TypeConstraint::reference(vec![
            "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        ]);

        assert_eq!(reference.code, "Reference");
        assert_eq!(reference.target_profile.len(), 1);
    }
}
