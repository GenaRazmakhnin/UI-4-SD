//! Data Transfer Objects for the Profile API.
//!
//! Defines request and response types for all profile-related endpoints.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ir::{
    DocumentMetadata, ElementConstraints, FhirVersion, ProfileDocument, ProfileStatus,
    ProfiledResource,
};

// === Response Wrapper ===

/// Standard API response wrapper.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// Response data.
    pub data: T,
    /// Diagnostics (warnings, errors).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    /// Response metadata.
    pub metadata: ResponseMetadata,
}

impl<T> ApiResponse<T> {
    /// Create a successful response.
    pub fn ok(data: T) -> Self {
        Self {
            data,
            diagnostics: Vec::new(),
            metadata: ResponseMetadata::now(),
        }
    }

    /// Create a response with diagnostics.
    pub fn with_diagnostics(data: T, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            data,
            diagnostics,
            metadata: ResponseMetadata::now(),
        }
    }
}

/// Response metadata.
#[derive(Debug, Serialize)]
pub struct ResponseMetadata {
    /// Timestamp of the response.
    pub timestamp: DateTime<Utc>,
    /// Version for optimistic locking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

impl ResponseMetadata {
    /// Create metadata with current timestamp.
    pub fn now() -> Self {
        Self {
            timestamp: Utc::now(),
            version: None,
        }
    }

    /// Set version for optimistic locking.
    pub fn with_version(mut self, version: u64) -> Self {
        self.version = Some(version);
        self
    }
}

/// Diagnostic message (warning or error).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Severity (error, warning, info).
    pub severity: DiagnosticSeverity,
    /// Error/warning code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Element path if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    /// Fatal error.
    Error,
    /// Warning (non-fatal).
    Warning,
    /// Informational.
    Info,
}

// === Profile Listing ===

/// Query parameters for profile listing.
#[derive(Debug, Deserialize)]
pub struct ListProfilesQuery {
    /// Filter by FHIR version.
    #[serde(rename = "fhirVersion")]
    pub fhir_version: Option<String>,
    /// Page number (1-based).
    pub page: Option<u32>,
    /// Page size (default 50, max 100).
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}

/// Profile list item (summary).
#[derive(Debug, Serialize)]
pub struct ProfileListItem {
    /// Profile ID.
    pub id: String,
    /// Canonical URL.
    pub url: String,
    /// Human-readable name.
    pub name: String,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Publication status.
    pub status: ProfileStatus,
    /// FHIR version.
    #[serde(rename = "fhirVersion")]
    pub fhir_version: FhirVersion,
    /// Base resource type.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Resource kind (Profile, Extension, etc.).
    #[serde(rename = "resourceKind")]
    pub resource_kind: ResourceKind,
    /// Last modified timestamp.
    #[serde(rename = "modifiedAt")]
    pub modified_at: DateTime<Utc>,
    /// Whether the profile has unsaved changes.
    #[serde(rename = "isDirty")]
    pub is_dirty: bool,
}

/// Kind of resource document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceKind {
    /// Profile on a resource.
    Profile,
    /// Extension definition.
    Extension,
    /// ValueSet.
    ValueSet,
    /// CodeSystem.
    CodeSystem,
    /// Other.
    Other,
}

impl From<&ProfileDocument> for ProfileListItem {
    fn from(doc: &ProfileDocument) -> Self {
        Self {
            id: doc.metadata.id.clone(),
            url: doc.metadata.url.clone(),
            name: doc.metadata.name.clone(),
            title: doc.metadata.title.clone(),
            status: doc.metadata.status,
            fhir_version: doc.resource.fhir_version,
            resource_type: doc.resource.resource_type().to_string(),
            resource_kind: ResourceKind::Profile, // TODO: detect from SD
            modified_at: doc.modified_at,
            is_dirty: doc.is_dirty(),
        }
    }
}

/// Paginated profile list response.
#[derive(Debug, Serialize)]
pub struct ProfileListResponse {
    /// List of profiles.
    pub profiles: Vec<ProfileListItem>,
    /// Pagination info.
    pub pagination: PaginationInfo,
}

/// Pagination information.
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    /// Current page (1-based).
    pub page: u32,
    /// Page size.
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    /// Total number of items.
    #[serde(rename = "totalItems")]
    pub total_items: u32,
    /// Total number of pages.
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
}

// === Create Profile ===

/// Request to create a new profile.
#[derive(Debug, Deserialize)]
pub struct CreateProfileRequest {
    /// Base resource type (e.g., "Patient", "Observation").
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// FHIR version.
    #[serde(rename = "fhirVersion")]
    pub fhir_version: String,
    /// Profile name (computer-friendly).
    pub name: String,
    /// Canonical URL (optional, will be generated if not provided).
    pub url: Option<String>,
    /// Display title.
    pub title: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Publisher name.
    pub publisher: Option<String>,
}

// === Get Profile Details ===

/// Full profile details response.
#[derive(Debug, Serialize)]
pub struct ProfileDetailsResponse {
    /// Document ID.
    #[serde(rename = "documentId")]
    pub document_id: String,
    /// Profile metadata.
    pub metadata: DocumentMetadata,
    /// Profile resource (element tree).
    pub resource: ProfiledResource,
    /// Edit history summary.
    pub history: HistorySummary,
    /// Whether the profile is dirty.
    #[serde(rename = "isDirty")]
    pub is_dirty: bool,
    /// File path if saved.
    #[serde(rename = "filePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

impl From<&ProfileDocument> for ProfileDetailsResponse {
    fn from(doc: &ProfileDocument) -> Self {
        Self {
            document_id: doc.document_id.to_string(),
            metadata: doc.metadata.clone(),
            resource: doc.resource.clone(),
            history: HistorySummary {
                can_undo: doc.can_undo(),
                can_redo: doc.can_redo(),
                undo_count: doc.history.undo_stack_size(),
                redo_count: doc.history.redo_stack_size(),
            },
            is_dirty: doc.is_dirty(),
            file_path: doc.file_path.clone(),
        }
    }
}

/// Summary of edit history.
#[derive(Debug, Serialize)]
pub struct HistorySummary {
    /// Whether undo is available.
    #[serde(rename = "canUndo")]
    pub can_undo: bool,
    /// Whether redo is available.
    #[serde(rename = "canRedo")]
    pub can_redo: bool,
    /// Number of undo operations available.
    #[serde(rename = "undoCount")]
    pub undo_count: usize,
    /// Number of redo operations available.
    #[serde(rename = "redoCount")]
    pub redo_count: usize,
}

// === Update Element ===

/// Request to update an element's constraints.
#[derive(Debug, Deserialize)]
pub struct UpdateElementRequest {
    /// Cardinality constraint.
    pub cardinality: Option<CardinalityUpdate>,
    /// Flags update.
    pub flags: Option<FlagsUpdate>,
    /// Type constraints.
    pub types: Option<Vec<TypeConstraintUpdate>>,
    /// Binding update.
    pub binding: Option<BindingUpdate>,
    /// Short description.
    pub short: Option<String>,
    /// Definition text.
    pub definition: Option<String>,
    /// Comment.
    pub comment: Option<String>,
}

/// Cardinality update.
#[derive(Debug, Deserialize)]
pub struct CardinalityUpdate {
    /// Minimum cardinality.
    pub min: Option<u32>,
    /// Maximum cardinality (null for unbounded).
    pub max: Option<MaxCardinality>,
}

/// Maximum cardinality value.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MaxCardinality {
    /// Numeric maximum.
    Bounded(u32),
    /// Unbounded ("*").
    Unbounded(String),
}

impl MaxCardinality {
    /// Convert to Option<u32> (None = unbounded).
    pub fn to_option(&self) -> Option<u32> {
        match self {
            Self::Bounded(n) => Some(*n),
            Self::Unbounded(_) => None,
        }
    }
}

/// Flags update.
#[derive(Debug, Deserialize)]
pub struct FlagsUpdate {
    /// Must support flag.
    #[serde(rename = "mustSupport")]
    pub must_support: Option<bool>,
    /// Is modifier flag.
    #[serde(rename = "isModifier")]
    pub is_modifier: Option<bool>,
    /// Is modifier reason.
    #[serde(rename = "isModifierReason")]
    pub is_modifier_reason: Option<String>,
    /// Is summary flag.
    #[serde(rename = "isSummary")]
    pub is_summary: Option<bool>,
}

/// Type constraint update.
#[derive(Debug, Deserialize)]
pub struct TypeConstraintUpdate {
    /// Type code (e.g., "Reference", "CodeableConcept").
    pub code: String,
    /// Profile URLs.
    pub profile: Option<Vec<String>>,
    /// Target profile URLs (for Reference types).
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<String>>,
}

/// Binding update.
#[derive(Debug, Deserialize)]
pub struct BindingUpdate {
    /// Binding strength.
    pub strength: String,
    /// ValueSet URL.
    #[serde(rename = "valueSet")]
    pub value_set: String,
    /// Binding description.
    pub description: Option<String>,
}

/// Response after updating an element.
#[derive(Debug, Serialize)]
pub struct UpdateElementResponse {
    /// Updated element path.
    pub path: String,
    /// Updated constraints.
    pub constraints: ElementConstraints,
    /// Validation results.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub validation: Vec<Diagnostic>,
}

// === Update Metadata ===

/// Request to update profile metadata.
#[derive(Debug, Deserialize)]
pub struct UpdateMetadataRequest {
    /// Profile name.
    pub name: Option<String>,
    /// Display title.
    pub title: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Publication status.
    pub status: Option<String>,
    /// Version string.
    pub version: Option<String>,
    /// Publisher name.
    pub publisher: Option<String>,
    /// Purpose.
    pub purpose: Option<String>,
    /// Copyright.
    pub copyright: Option<String>,
    /// Experimental flag.
    pub experimental: Option<bool>,
}

// === Import Profile ===

/// Request to import a profile from SD or FSH.
#[derive(Debug, Deserialize)]
pub struct ImportProfileRequest {
    /// Content format.
    pub format: ImportFormat,
    /// The content to import.
    pub content: String,
    /// Whether to replace existing profile or create new.
    #[serde(default)]
    pub replace: bool,
}

/// Import format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportFormat {
    /// FHIR StructureDefinition JSON.
    Json,
    /// FHIR Shorthand.
    Fsh,
}

/// Import result response.
#[derive(Debug, Serialize)]
pub struct ImportResponse {
    /// Imported profile details.
    pub profile: ProfileDetailsResponse,
    /// Import diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

// === Delete Profile ===

/// Response for delete confirmation.
#[derive(Debug, Serialize)]
pub struct DeleteConfirmation {
    /// Whether deletion requires confirmation.
    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,
    /// Reason for requiring confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_creation() {
        let response = ApiResponse::ok("test data");
        assert_eq!(response.data, "test data");
        assert!(response.diagnostics.is_empty());
    }

    #[test]
    fn test_max_cardinality_conversion() {
        let bounded = MaxCardinality::Bounded(5);
        assert_eq!(bounded.to_option(), Some(5));

        let unbounded = MaxCardinality::Unbounded("*".to_string());
        assert_eq!(unbounded.to_option(), None);
    }
}
