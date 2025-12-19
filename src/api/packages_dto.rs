//! DTOs and types for package management API.
//!
//! Contains request/response types and SSE event types for package operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

/// Search result from registry (not installed).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageSearchResultDto {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub fhir_version: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
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
pub struct PackageSearchQuery {
    pub q: String,
}

/// Query parameters for resource search.
#[derive(Debug, Deserialize)]
pub struct ResourceSearchQuery {
    /// Text query
    #[serde(default)]
    pub q: Option<String>,
    /// Resource types to filter by
    #[serde(rename = "type", default)]
    pub resource_type: Option<Vec<String>>,
    /// Packages to filter by
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
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
