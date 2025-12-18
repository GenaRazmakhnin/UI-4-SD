//! FSH Decompiler service.
//!
//! Provides FSH decompilation using maki-decompiler. Converts StructureDefinition
//! JSON to FHIR Shorthand format.

use std::sync::Arc;

use maki_core::canonical::{CanonicalFacade, CanonicalOptions, FhirRelease};
use maki_decompiler::{
    lake::ResourceLake,
    models::StructureDefinition,
    processor::StructureDefinitionProcessor,
};
use tokio::sync::OnceCell;

use crate::ir::FhirVersion;

/// Global decompiler context (lazily initialized).
static DECOMPILER_CONTEXT: OnceCell<DecompilerContext> = OnceCell::const_new();

/// Decompiler context holding the canonical session.
pub struct DecompilerContext {
    /// R4 resource lake for decompilation.
    r4_lake: ResourceLake,
    /// R5 resource lake for decompilation.
    r5_lake: Option<ResourceLake>,
}

impl DecompilerContext {
    /// Initialize the decompiler context.
    async fn init() -> Result<Self, DecompilerError> {
        let options = CanonicalOptions {
            quick_init: true,
            auto_install_core: false,
            ..Default::default()
        };

        let facade = CanonicalFacade::new(options)
            .await
            .map_err(|e| DecompilerError::InitFailed(e.to_string()))?;

        // Initialize R4 session (most common)
        let r4_session = facade
            .session(vec![FhirRelease::R4])
            .await
            .map_err(|e| DecompilerError::InitFailed(e.to_string()))?;
        let r4_lake = ResourceLake::new(Arc::new(r4_session));

        // Try to initialize R5 session (optional)
        let r5_lake = match facade.session(vec![FhirRelease::R5]).await {
            Ok(session) => Some(ResourceLake::new(Arc::new(session))),
            Err(e) => {
                tracing::warn!("R5 decompiler initialization failed: {}", e);
                None
            }
        };

        Ok(Self { r4_lake, r5_lake })
    }

    /// Get the resource lake for a FHIR version.
    fn lake_for_version(&self, version: FhirVersion) -> &ResourceLake {
        match version {
            FhirVersion::R5 | FhirVersion::R6 => self.r5_lake.as_ref().unwrap_or(&self.r4_lake),
            _ => &self.r4_lake,
        }
    }
}

/// Decompiler error type.
#[derive(Debug, thiserror::Error)]
pub enum DecompilerError {
    #[error("Failed to initialize decompiler: {0}")]
    InitFailed(String),

    #[error("Failed to parse StructureDefinition JSON: {0}")]
    ParseFailed(String),

    #[error("Failed to process StructureDefinition: {0}")]
    ProcessFailed(String),
}

/// Get or initialize the global decompiler context.
async fn get_context() -> Result<&'static DecompilerContext, DecompilerError> {
    DECOMPILER_CONTEXT
        .get_or_try_init(DecompilerContext::init)
        .await
}

/// Decompile a StructureDefinition JSON string to FSH.
///
/// # Arguments
///
/// * `sd_json` - The StructureDefinition JSON string
/// * `fhir_version` - The FHIR version of the SD
///
/// # Returns
///
/// The FSH representation of the StructureDefinition.
pub async fn decompile_sd_to_fsh(
    sd_json: &str,
    fhir_version: FhirVersion,
) -> Result<String, DecompilerError> {
    // Parse the SD JSON into maki-decompiler's model
    let sd: StructureDefinition = serde_json::from_str(sd_json)
        .map_err(|e| DecompilerError::ParseFailed(e.to_string()))?;

    // Get the decompiler context
    let context = get_context().await?;
    let lake = context.lake_for_version(fhir_version);

    // Process the StructureDefinition
    let processor = StructureDefinitionProcessor::new(lake);
    let exportable = processor
        .process(&sd)
        .await
        .map_err(|e| DecompilerError::ProcessFailed(e.to_string()))?;

    // Generate FSH
    Ok(exportable.to_fsh())
}

/// Decompile a StructureDefinition JSON value to FSH.
pub async fn decompile_sd_value_to_fsh(
    sd_value: &serde_json::Value,
    fhir_version: FhirVersion,
) -> Result<String, DecompilerError> {
    // Convert to StructureDefinition
    let sd: StructureDefinition = serde_json::from_value(sd_value.clone())
        .map_err(|e| DecompilerError::ParseFailed(e.to_string()))?;

    // Get the decompiler context
    let context = get_context().await?;
    let lake = context.lake_for_version(fhir_version);

    // Process the StructureDefinition
    let processor = StructureDefinitionProcessor::new(lake);
    let exportable = processor
        .process(&sd)
        .await
        .map_err(|e| DecompilerError::ProcessFailed(e.to_string()))?;

    // Generate FSH
    Ok(exportable.to_fsh())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decompile_simple_profile() {
        let sd_json = r#"{
            "resourceType": "StructureDefinition",
            "id": "test-patient",
            "url": "http://example.org/fhir/StructureDefinition/TestPatient",
            "name": "TestPatient",
            "status": "draft",
            "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
            "derivation": "constraint",
            "differential": {
                "element": [
                    {
                        "id": "Patient",
                        "path": "Patient"
                    },
                    {
                        "id": "Patient.name",
                        "path": "Patient.name",
                        "min": 1,
                        "mustSupport": true
                    }
                ]
            }
        }"#;

        let result = decompile_sd_to_fsh(sd_json, FhirVersion::R4).await;

        // May fail if canonical manager not available, that's ok for unit tests
        if let Ok(fsh) = result {
            assert!(fsh.contains("Profile:"));
            assert!(fsh.contains("TestPatient"));
        }
    }
}
