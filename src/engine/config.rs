//! Engine configuration.
//!
//! Defines the configuration options for the ProfileBuilderEngine.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ir::FhirVersion;
use crate::validation::ValidationLevel;

/// Configuration for the ProfileBuilderEngine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Default FHIR version for new profiles.
    #[serde(default = "default_fhir_version")]
    pub fhir_version: FhirVersion,

    /// Directory for package cache.
    #[serde(default = "default_package_cache_dir")]
    pub package_cache_dir: PathBuf,

    /// Root directory for workspace/project storage.
    pub workspace_dir: PathBuf,

    /// Default validation level.
    #[serde(default = "default_validation_level")]
    pub validation_level: ValidationLevel,

    /// Optional terminology service URL for terminology validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_service_url: Option<String>,

    /// Maximum depth of undo/redo history per document.
    #[serde(default = "default_max_history_depth")]
    pub max_history_depth: usize,

    /// Whether to run validation automatically after operations.
    #[serde(default = "default_auto_validate")]
    pub auto_validate: bool,

    /// Delay in milliseconds before auto-validation runs (debounce).
    #[serde(default = "default_validation_debounce_ms")]
    pub validation_debounce_ms: u64,

    /// Maximum number of documents to keep in memory.
    #[serde(default = "default_max_open_documents")]
    pub max_open_documents: usize,
}

fn default_fhir_version() -> FhirVersion {
    FhirVersion::R4
}

fn default_package_cache_dir() -> PathBuf {
    PathBuf::from(".cache").join("fhir-packages")
}

fn default_validation_level() -> ValidationLevel {
    ValidationLevel::Structural
}

fn default_max_history_depth() -> usize {
    100
}

fn default_auto_validate() -> bool {
    true
}

fn default_validation_debounce_ms() -> u64 {
    500
}

fn default_max_open_documents() -> usize {
    50
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            fhir_version: default_fhir_version(),
            package_cache_dir: default_package_cache_dir(),
            workspace_dir: PathBuf::from("./workspace"),
            validation_level: default_validation_level(),
            terminology_service_url: None,
            max_history_depth: default_max_history_depth(),
            auto_validate: default_auto_validate(),
            validation_debounce_ms: default_validation_debounce_ms(),
            max_open_documents: default_max_open_documents(),
        }
    }
}

impl EngineConfig {
    /// Create a new configuration with the given workspace directory.
    pub fn with_workspace(workspace_dir: impl Into<PathBuf>) -> Self {
        Self {
            workspace_dir: workspace_dir.into(),
            ..Default::default()
        }
    }

    /// Set the FHIR version.
    pub fn fhir_version(mut self, version: FhirVersion) -> Self {
        self.fhir_version = version;
        self
    }

    /// Set the validation level.
    pub fn validation_level(mut self, level: ValidationLevel) -> Self {
        self.validation_level = level;
        self
    }

    /// Set the terminology service URL.
    pub fn terminology_service(mut self, url: impl Into<String>) -> Self {
        self.terminology_service_url = Some(url.into());
        self
    }

    /// Disable automatic validation.
    pub fn no_auto_validate(mut self) -> Self {
        self.auto_validate = false;
        self
    }

    /// Set the maximum history depth.
    pub fn max_history(mut self, depth: usize) -> Self {
        self.max_history_depth = depth;
        self
    }

    /// Set the maximum number of open documents.
    pub fn max_open_documents(mut self, count: usize) -> Self {
        self.max_open_documents = count;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EngineConfig::default();
        assert_eq!(config.fhir_version, FhirVersion::R4);
        assert_eq!(config.max_history_depth, 100);
        assert!(config.auto_validate);
    }

    #[test]
    fn test_builder_pattern() {
        let config = EngineConfig::with_workspace("/tmp/workspace")
            .fhir_version(FhirVersion::R5)
            .validation_level(ValidationLevel::References)
            .no_auto_validate();

        assert_eq!(config.workspace_dir, PathBuf::from("/tmp/workspace"));
        assert_eq!(config.fhir_version, FhirVersion::R5);
        assert_eq!(config.validation_level, ValidationLevel::References);
        assert!(!config.auto_validate);
    }
}
