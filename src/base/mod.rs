//! Base definition resolver.
//!
//! Loads and parses base FHIR resource/profile definitions from the canonical manager.
//! Used to build the complete element tree by merging differential with base.

use std::sync::Arc;

use octofhir_canonical_manager::CanonicalManager;
use thiserror::Error;

use crate::import::ElementTreeBuilder;
use crate::ir::{ElementNode, FhirVersion};

/// Errors that can occur when resolving base definitions.
#[derive(Debug, Error)]
pub enum BaseResolverError {
    /// Failed to resolve the base definition URL.
    #[error("Failed to resolve base definition '{0}': {1}")]
    ResolutionFailed(String, String),

    /// The resolved resource has no snapshot or differential.
    #[error("Base definition '{0}' has no snapshot or differential elements")]
    NoElements(String),

    /// Failed to parse the base definition elements.
    #[error("Failed to parse base definition elements: {0}")]
    ParseFailed(String),

    /// Canonical manager not available.
    #[error("Canonical manager not available: {0}")]
    ManagerUnavailable(String),
}

/// Resolves base FHIR definitions from the canonical manager.
///
/// This resolver loads base StructureDefinitions (like Patient, Observation)
/// from installed FHIR packages and parses them into the IR element tree format.
pub struct BaseResolver {
    /// Reference to the canonical manager.
    canonical_manager: Arc<CanonicalManager>,
}

impl BaseResolver {
    /// Create a new base resolver with the given canonical manager.
    #[must_use]
    pub fn new(canonical_manager: Arc<CanonicalManager>) -> Self {
        Self { canonical_manager }
    }

    /// Load a base definition and parse it into an element tree.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Canonical URL of the base resource/profile
    ///   (e.g., "http://hl7.org/fhir/StructureDefinition/Patient")
    /// * `_fhir_version` - FHIR version for context (currently unused but may be needed for version-specific parsing)
    ///
    /// # Returns
    ///
    /// The root `ElementNode` of the parsed base definition tree.
    ///
    /// # Errors
    ///
    /// Returns an error if the base definition cannot be resolved or parsed.
    pub async fn load_base_tree(
        &self,
        base_url: &str,
        _fhir_version: FhirVersion,
    ) -> Result<ElementNode, BaseResolverError> {
        // Resolve the base definition from packages
        let resolved = self
            .canonical_manager
            .resolve(base_url)
            .await
            .map_err(|e| BaseResolverError::ResolutionFailed(base_url.to_string(), e.to_string()))?;

        let content = &resolved.resource.content;

        // Extract elements from snapshot (preferred) or differential
        let elements = self.extract_elements(content, base_url)?;

        // Get the resource type from the content
        let type_name = content
            .get("type")
            .and_then(|v| v.as_str())
            .or_else(|| content.get("name").and_then(|v| v.as_str()))
            .unwrap_or("Resource");

        // Build the element tree
        let builder = ElementTreeBuilder::new();
        let root = builder
            .build_tree(type_name, &elements, None)
            .map_err(|e| BaseResolverError::ParseFailed(e.to_string()))?;

        Ok(root)
    }

    /// Load the raw StructureDefinition JSON for a base definition.
    ///
    /// This is useful for the diff preview to show the base SD alongside
    /// the profiled SD.
    pub async fn load_base_sd_json(
        &self,
        base_url: &str,
    ) -> Result<serde_json::Value, BaseResolverError> {
        let resolved = self
            .canonical_manager
            .resolve(base_url)
            .await
            .map_err(|e| BaseResolverError::ResolutionFailed(base_url.to_string(), e.to_string()))?;

        Ok(resolved.resource.content.clone())
    }

    /// Extract elements array from StructureDefinition content.
    fn extract_elements(
        &self,
        content: &serde_json::Value,
        base_url: &str,
    ) -> Result<Vec<serde_json::Value>, BaseResolverError> {
        // Prefer snapshot, fall back to differential
        let element_source = content
            .get("snapshot")
            .and_then(|s| s.get("element"))
            .or_else(|| content.get("differential").and_then(|d| d.get("element")));

        match element_source.and_then(|e| e.as_array()) {
            Some(arr) => Ok(arr.clone()),
            None => Err(BaseResolverError::NoElements(base_url.to_string())),
        }
    }

    /// Check if a base definition is available in the package cache.
    pub async fn is_available(&self, base_url: &str) -> bool {
        self.canonical_manager.resolve(base_url).await.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a configured canonical manager with packages installed.
    // They are marked as ignore by default and can be run with `cargo test -- --ignored`

    #[tokio::test]
    #[ignore = "Requires package installation"]
    async fn test_load_patient_base() {
        let manager = CanonicalManager::with_default_config()
            .await
            .expect("Failed to create canonical manager");
        let resolver = BaseResolver::new(Arc::new(manager));

        let result = resolver
            .load_base_tree(
                "http://hl7.org/fhir/StructureDefinition/Patient",
                FhirVersion::R4,
            )
            .await;

        assert!(result.is_ok(), "Failed to load Patient: {:?}", result.err());

        let root = result.unwrap();
        assert_eq!(root.path, "Patient");
        assert!(!root.children.is_empty(), "Patient should have children");
    }
}
