//! Profiled FHIR resource representation.
//!
//! This module defines [`ProfiledResource`], the IR representation of a
//! profiled FHIR resource. It contains the element tree and metadata
//! needed for editing and export.

use serde::{Deserialize, Serialize};

use super::element::ElementNode;

/// FHIR version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum FhirVersion {
    /// FHIR R4 (4.0.1)
    #[default]
    #[serde(rename = "4.0.1")]
    R4,
    /// FHIR R4B (4.3.0)
    #[serde(rename = "4.3.0")]
    R4B,
    /// FHIR R5 (5.0.0)
    #[serde(rename = "5.0.0")]
    R5,
    /// FHIR R6 (6.0.0-ballot1) - experimental
    #[serde(rename = "6.0.0")]
    R6,
}

impl FhirVersion {
    /// Get the version string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::R4 => "4.0.1",
            Self::R4B => "4.3.0",
            Self::R5 => "5.0.0",
            Self::R6 => "6.0.0",
        }
    }

    /// Get the short label (R4, R5, etc.).
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::R4 => "R4",
            Self::R4B => "R4B",
            Self::R5 => "R5",
            Self::R6 => "R6",
        }
    }

    /// Parse from a version string.
    #[must_use]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "4.0.1" | "4.0" | "R4" => Some(Self::R4),
            "4.3.0" | "4.3" | "R4B" => Some(Self::R4B),
            "5.0.0" | "5.0" | "R5" => Some(Self::R5),
            "6.0.0" | "6.0" | "R6" => Some(Self::R6),
            _ => None,
        }
    }

    /// Get the base package name for this version.
    #[must_use]
    pub const fn base_package(&self) -> &'static str {
        match self {
            Self::R4 => "hl7.fhir.r4.core",
            Self::R4B => "hl7.fhir.r4b.core",
            Self::R5 => "hl7.fhir.r5.core",
            Self::R6 => "hl7.fhir.r6.core",
        }
    }
}

impl std::fmt::Display for FhirVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Reference to a base definition (resource or profile).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseDefinition {
    /// Canonical URL of the base.
    pub url: String,

    /// Version of the base (if specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Human-readable name/title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BaseDefinition {
    /// Create a new base definition reference.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            version: None,
            name: None,
        }
    }

    /// Create a reference to a core FHIR resource.
    #[must_use]
    pub fn resource(resource_type: impl Into<String>) -> Self {
        let resource_type = resource_type.into();
        Self {
            url: format!("http://hl7.org/fhir/StructureDefinition/{resource_type}"),
            version: None,
            name: Some(resource_type),
        }
    }

    /// Set the version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the display name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Check if this is a core FHIR resource (not a profile).
    #[must_use]
    pub fn is_core_resource(&self) -> bool {
        self.url.starts_with("http://hl7.org/fhir/StructureDefinition/")
            && !self.url.contains('-')
    }
}

/// IR representation of a profiled FHIR resource.
///
/// This is the main editable representation of a FHIR profile's resource
/// constraints. It contains the element tree with all modifications from
/// the base resource.
///
/// # Example
///
/// ```
/// use niten::ir::{ProfiledResource, FhirVersion, BaseDefinition, ElementNode};
///
/// let mut resource = ProfiledResource::new(
///     "http://example.org/fhir/StructureDefinition/MyPatient",
///     FhirVersion::R4,
///     BaseDefinition::resource("Patient"),
/// );
///
/// // Add element constraints
/// let name_element = ElementNode::new("Patient.name".to_string());
/// resource.root.add_child(name_element);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfiledResource {
    /// Canonical URL of this profile.
    pub url: String,

    /// Version of this profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// FHIR version this profile is based on.
    pub fhir_version: FhirVersion,

    /// Base resource or profile being profiled.
    pub base: BaseDefinition,

    /// Kind of structure (resource, complex-type, logical, etc.).
    #[serde(default = "default_kind")]
    pub kind: StructureKind,

    /// Root element node (represents the resource type).
    pub root: ElementNode,

    /// Extension definitions included in this profile.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<ExtensionDefinition>,

    /// Unknown fields preserved for lossless round-trip.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub unknown_fields: serde_json::Map<String, serde_json::Value>,
}

fn default_kind() -> StructureKind {
    StructureKind::Resource
}

impl ProfiledResource {
    /// Create a new profiled resource.
    #[must_use]
    pub fn new(url: impl Into<String>, fhir_version: FhirVersion, base: BaseDefinition) -> Self {
        let url = url.into();
        let root_path = base
            .url
            .rsplit('/')
            .next()
            .unwrap_or("Resource")
            .to_string();

        Self {
            url,
            version: None,
            fhir_version,
            base,
            kind: StructureKind::Resource,
            root: ElementNode::new(root_path),
            extensions: Vec::new(),
            unknown_fields: serde_json::Map::new(),
        }
    }

    /// Set the version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Get the resource type name.
    #[must_use]
    pub fn resource_type(&self) -> &str {
        self.base
            .url
            .rsplit('/')
            .next()
            .unwrap_or("Resource")
    }

    /// Find an element by path.
    #[must_use]
    pub fn find_element(&self, path: &str) -> Option<&ElementNode> {
        if path == self.root.path {
            return Some(&self.root);
        }

        // Strip resource type prefix if present
        let relative = path
            .strip_prefix(&self.root.path)
            .and_then(|s| s.strip_prefix('.'))
            .unwrap_or(path);

        self.root.find_descendant(relative)
    }

    /// Find an element by path (mutable).
    pub fn find_element_mut(&mut self, path: &str) -> Option<&mut ElementNode> {
        if path == self.root.path {
            return Some(&mut self.root);
        }

        // Strip resource type prefix if present
        let relative = path
            .strip_prefix(&self.root.path)
            .and_then(|s| s.strip_prefix('.'))
            .unwrap_or(path);

        self.root.find_descendant_mut(relative)
    }

    /// Find an element by its stable ID.
    #[must_use]
    pub fn find_by_id(&self, id: super::element::NodeId) -> Option<&ElementNode> {
        self.root.find_by_id(id)
    }

    /// Iterate over all elements in the tree.
    pub fn elements(&self) -> impl Iterator<Item = &ElementNode> {
        self.root.descendants()
    }

    /// Count total number of elements.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.elements().count()
    }

    /// Check if this profile has any modifications from base.
    #[must_use]
    pub fn has_modifications(&self) -> bool {
        self.elements().any(|e| e.is_modified())
    }
}

/// Kind of structure definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum StructureKind {
    /// FHIR resource type.
    #[default]
    Resource,
    /// Complex data type.
    ComplexType,
    /// Primitive data type.
    PrimitiveType,
    /// Logical model.
    Logical,
}

impl StructureKind {
    /// Get the FHIR code.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Resource => "resource",
            Self::ComplexType => "complex-type",
            Self::PrimitiveType => "primitive-type",
            Self::Logical => "logical",
        }
    }
}

/// Extension definition included in a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionDefinition {
    /// Canonical URL of the extension.
    pub url: String,

    /// Human-readable name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether this extension is defined inline vs. referenced.
    #[serde(default)]
    pub inline: bool,

    /// Element tree for inline extension definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<ElementNode>,
}

impl ExtensionDefinition {
    /// Create a reference to an external extension.
    #[must_use]
    pub fn reference(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            name: None,
            inline: false,
            element: None,
        }
    }

    /// Create an inline extension definition.
    #[must_use]
    pub fn inline(url: impl Into<String>, element: ElementNode) -> Self {
        Self {
            url: url.into(),
            name: None,
            inline: true,
            element: Some(element),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fhir_version() {
        assert_eq!(FhirVersion::R4.as_str(), "4.0.1");
        assert_eq!(FhirVersion::R5.label(), "R5");
        assert_eq!(FhirVersion::from_str("4.0.1"), Some(FhirVersion::R4));
    }

    #[test]
    fn test_base_definition() {
        let base = BaseDefinition::resource("Patient");
        assert_eq!(base.url, "http://hl7.org/fhir/StructureDefinition/Patient");
        assert!(base.is_core_resource());

        let profile = BaseDefinition::new("http://example.org/fhir/StructureDefinition/MyPatient");
        assert!(!profile.is_core_resource());
    }

    #[test]
    fn test_profiled_resource_creation() {
        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/MyPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        assert_eq!(resource.resource_type(), "Patient");
        assert_eq!(resource.root.path, "Patient");
        assert!(!resource.has_modifications());
    }

    #[test]
    fn test_element_navigation() {
        let mut resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/MyPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        let name = ElementNode::new("Patient.name".to_string());
        resource.root.add_child(name);

        assert!(resource.find_element("Patient.name").is_some());
        assert!(resource.find_element("Patient.unknown").is_none());
    }
}
