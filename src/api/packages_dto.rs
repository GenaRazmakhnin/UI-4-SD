//! DTOs and types for package management API.
//!
//! Contains request/response types and SSE event types for package operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize a field that can be either a single string or a Vec<String>.
/// This handles URL query params like `?package=foo` (single) or `?package=foo&package=bar` (multiple).
fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, SeqAccess, Visitor};

    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Option<Vec<String>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a sequence of strings")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(StringOrVecInner).map(Some)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(vec![value.to_string()]))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(vec![value]))
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            StringOrVecInner.visit_seq(seq).map(Some)
        }
    }

    struct StringOrVecInner;

    impl<'de> Visitor<'de> for StringOrVecInner {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a sequence of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_string()])
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value])
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<String>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    deserializer.deserialize_option(StringOrVec)
}

/// Package information returned from the list endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageDto {
    /// Package identifier (name@version)
    pub id: String,
    /// Package name (e.g., "hl7.fhir.us.core")
    pub name: String,
    /// Package version (e.g., "6.1.0")
    pub version: String,
    /// Package description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// FHIR version compatibility
    pub fhir_version: String,
    /// Whether the package is installed
    pub installed: bool,
    /// Installation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<DateTime<Utc>>,
    /// Resource counts by type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_counts: Option<PackageResourceCountsDto>,
}

/// Resource counts in a package.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageResourceCountsDto {
    pub profiles: u32,
    pub extensions: u32,
    pub value_sets: u32,
    pub code_systems: u32,
    pub search_parameters: u32,
    pub total: u32,
}

/// Search result from registry.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchResultDto {
    /// Package identifier (name@version)
    pub id: String,
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// FHIR version compatibility
    pub fhir_version: String,
    /// Package publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Whether the package is already installed
    pub installed: bool,
    /// Installed version (if different from latest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_version: Option<String>,
    /// Download count from registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_count: Option<u64>,
}

/// Detailed package information.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageDetailsDto {
    /// Package identifier (name@version)
    pub id: String,
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// FHIR version compatibility
    pub fhir_version: String,
    /// Package publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Whether the package is installed
    pub installed: bool,
    /// Installation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Package license
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Package homepage URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// Package repository URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// Canonical URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<String>,
    /// Resource counts by type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_counts: Option<PackageResourceCountsDto>,
    /// Package dependencies
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<PackageDependencyDto>,
    /// Available versions
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<PackageVersionDto>,
}

/// Package dependency info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageDependencyDto {
    pub name: String,
    pub version: String,
}

/// Package version info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageVersionDto {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fhir_version: Option<String>,
}

/// Install job status (for polling-based progress).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallJobDto {
    /// Unique job identifier
    pub job_id: String,
    /// Package being installed (name@version)
    pub package_id: String,
    /// Current status
    pub status: InstallJobStatus,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Current phase message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Downloaded bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloaded_bytes: Option<u64>,
    /// Total bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<u64>,
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Installed package info on completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<PackageDto>,
    /// Job creation time
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update time
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Install job status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InstallJobStatus {
    /// Job is queued
    Pending,
    /// Downloading package
    Downloading,
    /// Extracting package contents
    Extracting,
    /// Indexing resources
    Indexing,
    /// Installation completed successfully
    Completed,
    /// Installation failed
    Failed,
}

/// SSE event types for installation progress.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum InstallProgressEvent {
    /// Installation started
    Start {
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "totalBytes", skip_serializing_if = "Option::is_none")]
        total_bytes: Option<u64>,
    },
    /// Download progress
    Progress {
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "downloadedBytes")]
        downloaded_bytes: u64,
        #[serde(rename = "totalBytes", skip_serializing_if = "Option::is_none")]
        total_bytes: Option<u64>,
        percentage: u8,
    },
    /// Extracting package contents
    Extracting {
        #[serde(rename = "packageId")]
        package_id: String,
    },
    /// Indexing resources
    Indexing {
        #[serde(rename = "packageId")]
        package_id: String,
    },
    /// Installation completed successfully
    Complete { package: PackageDto },
    /// Installation failed
    Error {
        #[serde(rename = "packageId")]
        package_id: String,
        message: String,
        code: String,
    },
}

/// Query parameters for package search.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchQuery {
    /// Search query text
    pub q: String,
    /// Filter by FHIR version (e.g., "4.0.1", "5.0.0", "R4", "R5")
    #[serde(default)]
    pub fhir_version: Option<String>,
    /// Sort by: "relevance", "downloads", "date"
    #[serde(default)]
    pub sort_by: Option<String>,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Option<usize>,
}

/// Query parameters for resource search.
#[derive(Debug, Deserialize)]
pub struct ResourceSearchQuery {
    /// Text query
    #[serde(default)]
    pub q: Option<String>,
    /// Resource types to filter by
    #[serde(rename = "type", default, deserialize_with = "deserialize_string_or_vec")]
    pub resource_type: Option<Vec<String>>,
    /// Packages to filter by
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub package: Option<Vec<String>>,
    /// FHIR version filter (e.g., "4.0.1", "5.0.0")
    #[serde(default)]
    pub fhir_version: Option<String>,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Option<usize>,
    /// Offset for pagination
    #[serde(default)]
    pub offset: Option<usize>,
}

/// Query parameters for extension search.
#[derive(Debug, Deserialize)]
pub struct ExtensionSearchQuery {
    /// Text query
    #[serde(default)]
    pub q: Option<String>,
    /// Packages to filter by
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub package: Option<Vec<String>>,
    /// FHIR version filter
    #[serde(default)]
    pub fhir_version: Option<String>,
    /// Extension context type filter (Resource, Element, DataType)
    #[serde(default)]
    pub context: Option<String>,
    /// Extension context path filter (e.g., "Patient", "Observation.value")
    #[serde(default)]
    pub context_path: Option<String>,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Option<usize>,
    /// Offset for pagination
    #[serde(default)]
    pub offset: Option<usize>,
}

/// Query parameters for ValueSet search.
#[derive(Debug, Deserialize)]
pub struct ValueSetSearchQuery {
    /// Text query
    #[serde(default)]
    pub q: Option<String>,
    /// Packages to filter by
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub package: Option<Vec<String>>,
    /// FHIR version filter
    #[serde(default)]
    pub fhir_version: Option<String>,
    /// Code system URL filter
    #[serde(default)]
    pub system: Option<String>,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Option<usize>,
    /// Offset for pagination
    #[serde(default)]
    pub offset: Option<usize>,
}

/// Query parameters for profile search.
#[derive(Debug, Deserialize)]
pub struct ProfileSearchQuery {
    /// Text query
    #[serde(default)]
    pub q: Option<String>,
    /// Packages to filter by
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub package: Option<Vec<String>>,
    /// FHIR version filter
    #[serde(default)]
    pub fhir_version: Option<String>,
    /// Base resource type filter (Patient, Observation, etc.)
    #[serde(default)]
    pub base_type: Option<String>,
    /// Derivation type filter (constraint, specialization)
    #[serde(default)]
    pub derivation: Option<String>,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Option<usize>,
    /// Offset for pagination
    #[serde(default)]
    pub offset: Option<usize>,
}

/// Query parameters for element search.
#[derive(Debug, Deserialize)]
pub struct ElementSearchQuery {
    /// Profile ID to search within
    pub profile_id: String,
    /// Text query for element path or description
    #[serde(default)]
    pub q: Option<String>,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Option<usize>,
}

/// Query parameters for base resource search.
#[derive(Debug, Deserialize)]
pub struct BaseResourceSearchQuery {
    /// Text query
    #[serde(default)]
    pub q: Option<String>,
    /// Packages to filter by
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub package: Option<Vec<String>>,
    /// FHIR version filter
    #[serde(default)]
    pub fhir_version: Option<String>,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Option<usize>,
}

/// Base resource type information (for profile creation).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResourceDto {
    /// Resource type name (e.g., "Patient", "Observation")
    pub name: String,
    /// Canonical URL
    pub url: String,
    /// Resource title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Resource description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Package name
    pub package_name: String,
    /// Package version
    pub package_version: String,
}

/// Extension context information.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionContextDto {
    /// Context type (e.g., "resource", "element", "datatype")
    pub context_type: String,
    /// Context expression (e.g., "Patient", "Observation.value[x]")
    pub expression: String,
}

/// Extension search result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionDto {
    pub id: String,
    pub url: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub package_name: String,
    pub package_version: String,
    /// Extension contexts
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<ExtensionContextDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// ValueSet search result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetDto {
    pub id: String,
    pub url: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub package_name: String,
    pub package_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// Profile search result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDto {
    pub id: String,
    pub url: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Base resource type this profile constrains
    pub base_type: String,
    /// Derivation type (constraint or specialization)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivation: Option<String>,
    pub package_name: String,
    pub package_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// Element search result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDto {
    /// Element path (e.g., "Patient.name")
    pub path: String,
    /// Element short description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<String>,
    /// Element definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// Element types
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
    /// Cardinality min
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u32>,
    /// Cardinality max
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
}

/// Generic resource search result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultDto {
    pub id: String,
    pub url: String,
    pub name: String,
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub package_name: String,
    pub package_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// Highlight showing match context.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightDto {
    /// Field that was matched
    pub field: String,
    /// Snippet of matched text
    pub snippet: String,
}

/// Facet counts for search results.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FacetsDto {
    /// Counts by resource type
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub resource_types: std::collections::HashMap<String, usize>,
    /// Counts by package
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub packages: std::collections::HashMap<String, usize>,
}

/// Wrapper for search results with pagination info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse<T> {
    pub results: Vec<T>,
    pub total_count: usize,
}

/// Enhanced search response with facets.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponseWithFacets<T> {
    pub results: Vec<T>,
    pub total_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<FacetsDto>,
}

/// Standard error response for package API.
#[derive(Debug, Clone, Serialize)]
pub struct PackageErrorResponse {
    pub error: String,
    pub code: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl PackageErrorResponse {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: "PACKAGE_NOT_FOUND".to_string(),
            status: 404,
            details: None,
        }
    }

    pub fn install_failed(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: "INSTALL_FAILED".to_string(),
            status: 500,
            details: None,
        }
    }

    pub fn network_error(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: "NETWORK_ERROR".to_string(),
            status: 502,
            details: None,
        }
    }

    pub fn already_installed(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: "ALREADY_INSTALLED".to_string(),
            status: 409,
            details: None,
        }
    }
}

/// Parse a package ID string into name and version.
/// Supports formats: "name@version" or just "name" (defaults to "latest").
pub fn parse_package_id(id: &str) -> (String, String) {
    if let Some((name, version)) = id.rsplit_once('@') {
        (name.to_string(), version.to_string())
    } else {
        (id.to_string(), "latest".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_id_with_version() {
        let (name, version) = parse_package_id("hl7.fhir.us.core@6.1.0");
        assert_eq!(name, "hl7.fhir.us.core");
        assert_eq!(version, "6.1.0");
    }

    #[test]
    fn test_parse_package_id_without_version() {
        let (name, version) = parse_package_id("hl7.fhir.us.core");
        assert_eq!(name, "hl7.fhir.us.core");
        assert_eq!(version, "latest");
    }

    #[test]
    fn test_install_progress_event_serialization() {
        let event = InstallProgressEvent::Progress {
            package_id: "test@1.0.0".to_string(),
            downloaded_bytes: 1000,
            total_bytes: Some(10000),
            percentage: 10,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"progress\""));
        assert!(json.contains("\"downloadedBytes\":1000"));
    }
}
