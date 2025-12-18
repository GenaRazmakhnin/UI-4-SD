//! Profile document - top-level container for editing.
//!
//! This module defines [`ProfileDocument`], which wraps a [`ProfiledResource`]
//! with editing metadata like dirty state, edit history, and document lifecycle.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::resource::ProfiledResource;
use super::tracking::EditHistory;

/// Publication status of a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProfileStatus {
    /// Work in progress.
    #[default]
    Draft,
    /// Ready for use.
    Active,
    /// No longer maintained.
    Retired,
    /// Should not be used.
    Unknown,
}

impl ProfileStatus {
    /// Get the FHIR code.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Retired => "retired",
            Self::Unknown => "unknown",
        }
    }

    /// Check if this status allows editing.
    #[must_use]
    pub const fn is_editable(&self) -> bool {
        matches!(self, Self::Draft)
    }
}

impl std::fmt::Display for ProfileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Document metadata for a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Profile ID (business identifier).
    pub id: String,

    /// Canonical URL.
    pub url: String,

    /// Version string (e.g., "1.0.0").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Human-readable name (computer-friendly).
    pub name: String,

    /// Human-readable title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Publication status.
    #[serde(default)]
    pub status: ProfileStatus,

    /// Whether this is experimental.
    #[serde(default)]
    pub experimental: bool,

    /// Publisher name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    /// Description/purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Purpose/rationale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,

    /// Copyright statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    /// Date of last change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,

    /// Use context (jurisdiction, clinical focus, etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UseContext>,

    /// Jurisdictions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,

    /// Keywords for discovery.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<Coding>,

    /// Contact information.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
}

impl DocumentMetadata {
    /// Create new metadata with minimal required fields.
    #[must_use]
    pub fn new(id: impl Into<String>, url: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            version: None,
            name: name.into(),
            title: None,
            status: ProfileStatus::Draft,
            experimental: false,
            publisher: None,
            description: None,
            purpose: None,
            copyright: None,
            date: Some(Utc::now()),
            use_context: Vec::new(),
            jurisdiction: Vec::new(),
            keyword: Vec::new(),
            contact: Vec::new(),
        }
    }

    /// Set the title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the publisher.
    #[must_use]
    pub fn with_publisher(mut self, publisher: impl Into<String>) -> Self {
        self.publisher = Some(publisher.into());
        self
    }

    /// Set the version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Update the last modified date.
    pub fn touch(&mut self) {
        self.date = Some(Utc::now());
    }
}

/// Use context for a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseContext {
    /// Type of context (workflow, task, venue, etc.).
    pub code: Coding,
    /// Value of the context.
    pub value: UseContextValue,
}

/// Value for a use context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UseContextValue {
    /// CodeableConcept value.
    CodeableConcept(CodeableConcept),
    /// Quantity value.
    Quantity(Quantity),
    /// Range value.
    Range(Range),
    /// Reference value.
    Reference(Reference),
}

/// Simple coding (system + code).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coding {
    /// Code system URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Code value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Display text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

impl Coding {
    /// Create a new coding.
    #[must_use]
    pub fn new(system: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            system: Some(system.into()),
            code: Some(code.into()),
            display: None,
        }
    }

    /// Add display text.
    #[must_use]
    pub fn with_display(mut self, display: impl Into<String>) -> Self {
        self.display = Some(display.into());
        self
    }
}

/// CodeableConcept (multiple codings + text).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeableConcept {
    /// Codings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coding: Vec<Coding>,
    /// Plain text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CodeableConcept {
    /// Create from a single coding.
    #[must_use]
    pub fn from_coding(coding: Coding) -> Self {
        Self {
            coding: vec![coding],
            text: None,
        }
    }

    /// Create with just text.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            coding: Vec::new(),
            text: Some(text.into()),
        }
    }
}

/// Simple quantity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    /// Numeric value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Unit of measure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// System for unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Coded unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Range (low..high).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    /// Low bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<Quantity>,
    /// High bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<Quantity>,
}

/// Reference to another resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reference {
    /// Reference URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Display text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

/// Contact details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactDetail {
    /// Name of contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contact points (phone, email, etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<ContactPoint>,
}

/// Contact point (phone, email, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    /// System (phone, email, url, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Use (home, work, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,
}

/// Top-level profile document for editing.
///
/// This is the main container for a profile being edited in the UI.
/// It wraps the [`ProfiledResource`] with editing state like dirty
/// tracking and undo/redo history.
///
/// # Example
///
/// ```
/// use niten::ir::{
///     ProfileDocument, DocumentMetadata, ProfiledResource,
///     FhirVersion, BaseDefinition
/// };
///
/// let metadata = DocumentMetadata::new(
///     "my-patient",
///     "http://example.org/fhir/StructureDefinition/MyPatient",
///     "MyPatient"
/// );
///
/// let resource = ProfiledResource::new(
///     "http://example.org/fhir/StructureDefinition/MyPatient",
///     FhirVersion::R4,
///     BaseDefinition::resource("Patient"),
/// );
///
/// let doc = ProfileDocument::new(metadata, resource);
/// assert!(!doc.is_dirty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDocument {
    /// Unique document ID (for multi-document editing).
    pub document_id: Uuid,

    /// Profile metadata.
    pub metadata: DocumentMetadata,

    /// The profiled resource being edited.
    pub resource: ProfiledResource,

    /// Edit history for undo/redo.
    #[serde(default)]
    pub history: EditHistory,

    /// Whether the document has unsaved changes.
    #[serde(default)]
    dirty: bool,

    /// File path if saved to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    /// When the document was created.
    pub created_at: DateTime<Utc>,

    /// When the document was last modified.
    pub modified_at: DateTime<Utc>,
}

impl ProfileDocument {
    /// Create a new profile document.
    #[must_use]
    pub fn new(metadata: DocumentMetadata, resource: ProfiledResource) -> Self {
        let now = Utc::now();
        Self {
            document_id: Uuid::new_v4(),
            metadata,
            resource,
            history: EditHistory::default(),
            dirty: false,
            file_path: None,
            created_at: now,
            modified_at: now,
        }
    }

    /// Create a new profile document for a resource type.
    #[must_use]
    pub fn for_resource(
        id: impl Into<String>,
        url: impl Into<String>,
        name: impl Into<String>,
        resource_type: impl Into<String>,
        fhir_version: super::resource::FhirVersion,
    ) -> Self {
        let url = url.into();
        let metadata = DocumentMetadata::new(id, &url, name);
        let resource = ProfiledResource::new(
            &url,
            fhir_version,
            super::resource::BaseDefinition::resource(resource_type),
        );
        Self::new(metadata, resource)
    }

    /// Check if the document has unsaved changes.
    #[must_use]
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark the document as modified.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.modified_at = Utc::now();
        self.metadata.touch();
    }

    /// Mark the document as saved (not dirty).
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Set the file path.
    pub fn set_file_path(&mut self, path: impl Into<String>) {
        self.file_path = Some(path.into());
    }

    /// Get the document title (metadata title or name).
    #[must_use]
    pub fn title(&self) -> &str {
        self.metadata.title.as_deref().unwrap_or(&self.metadata.name)
    }

    /// Check if undo is available.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Check if redo is available.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Get element count in the profile.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.resource.element_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, FhirVersion};

    #[test]
    fn test_document_creation() {
        let metadata = DocumentMetadata::new(
            "test-patient",
            "http://example.org/fhir/StructureDefinition/TestPatient",
            "TestPatient",
        );

        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );

        let doc = ProfileDocument::new(metadata, resource);

        assert!(!doc.is_dirty());
        assert_eq!(doc.title(), "TestPatient");
        assert!(!doc.can_undo());
    }

    #[test]
    fn test_dirty_tracking() {
        let doc = ProfileDocument::for_resource(
            "test",
            "http://example.org/test",
            "Test",
            "Patient",
            FhirVersion::R4,
        );

        assert!(!doc.is_dirty());

        let mut doc = doc;
        doc.mark_dirty();
        assert!(doc.is_dirty());

        doc.mark_saved();
        assert!(!doc.is_dirty());
    }

    #[test]
    fn test_profile_status() {
        assert!(ProfileStatus::Draft.is_editable());
        assert!(!ProfileStatus::Active.is_editable());
        assert_eq!(ProfileStatus::Draft.as_str(), "draft");
    }
}
