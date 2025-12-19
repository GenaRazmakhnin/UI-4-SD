//! Registry catalog service for searching FHIR packages.
//!
//! Queries the official FHIR package registry API for package search.

use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;

/// Registry catalog search URL (supports ?name= query parameter)
const CATALOG_URL: &str = "https://packages.fhir.org/catalog";

/// Package entry from the registry catalog API response
/// Note: The API returns PascalCase field names
#[derive(Debug, Clone, Deserialize)]
pub struct CatalogPackage {
    /// Package name (e.g., "hl7.fhir.us.core")
    #[serde(rename = "Name")]
    pub name: String,
    /// Package description
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    /// FHIR version (e.g., "R4", "STU3")
    #[serde(default, rename = "FhirVersion")]
    pub fhir_version: Option<String>,
    // Fields for compatibility with existing code
    #[serde(skip)]
    pub version: Option<String>,
    #[serde(skip)]
    pub versions: Vec<String>,
    #[serde(skip)]
    pub fhir_versions: Vec<String>,
    #[serde(skip)]
    pub author: Option<String>,
}

/// Registry catalog service
pub struct RegistryCatalog {
    client: reqwest::Client,
}

impl RegistryCatalog {
    /// Create a new registry catalog service
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("octofhir/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Search for packages matching the query
    pub async fn search(&self, query: &str) -> Result<Vec<CatalogPackage>, String> {
        let url = format!("{}?name={}", CATALOG_URL, urlencoding::encode(query));
        tracing::info!("Searching packages: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to search packages: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Registry returned status {}",
                response.status()
            ));
        }

        let packages: Vec<CatalogPackage> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse search results: {}", e))?;

        tracing::info!("Found {} packages matching '{}'", packages.len(), query);

        Ok(packages)
    }
}

impl Default for RegistryCatalog {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared registry catalog instance
pub type SharedRegistryCatalog = Arc<RegistryCatalog>;

/// Create a shared registry catalog instance
pub fn create_registry_catalog() -> SharedRegistryCatalog {
    Arc::new(RegistryCatalog::new())
}
