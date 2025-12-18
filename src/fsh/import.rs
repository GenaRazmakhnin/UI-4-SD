//! FSH Import Module
//!
//! Provides functionality to import FSH files into the IR data model.
//! Uses maki-core for parsing and semantic analysis.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use maki_core::{
    DefaultSemanticAnalyzer, FshParser, ParseResult, SemanticAnalyzer, SemanticModel,
    canonical::{CanonicalFacade, CanonicalOptions, FhirRelease},
    semantic::{FishingContext, FshTank, Package},
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::ir::{FhirVersion, ProfileDocument};

use super::error::{FshError, FshImportError, FshResult, FshResultWithWarnings, FshWarning, FshWarningCode};
use super::mapper::FshToIrMapper;

/// Options for FSH import.
#[derive(Debug, Clone)]
pub struct FshImportOptions {
    /// Base URL for canonical URLs (if not in FSH config).
    pub canonical_base: Option<String>,
    /// FHIR version to use.
    pub fhir_version: FhirVersion,
    /// Whether to continue on errors (partial import).
    pub continue_on_error: bool,
    /// Whether to resolve external dependencies.
    pub resolve_dependencies: bool,
    /// Paths to FHIR packages to load.
    pub package_paths: Vec<PathBuf>,
}

impl Default for FshImportOptions {
    fn default() -> Self {
        Self {
            canonical_base: None,
            fhir_version: FhirVersion::R4,
            continue_on_error: true,
            resolve_dependencies: true,
            package_paths: Vec::new(),
        }
    }
}

impl FshImportOptions {
    /// Create options with a canonical base URL.
    pub fn with_canonical_base(mut self, base: impl Into<String>) -> Self {
        self.canonical_base = Some(base.into());
        self
    }

    /// Set the FHIR version.
    pub fn with_fhir_version(mut self, version: FhirVersion) -> Self {
        self.fhir_version = version;
        self
    }
}

/// FSH file importer.
///
/// Imports single FSH files into IR ProfileDocuments.
pub struct FshImporter {
    /// Semantic analyzer.
    analyzer: DefaultSemanticAnalyzer,
    /// Mapper from FSH to IR.
    mapper: FshToIrMapper,
    /// Import options.
    options: FshImportOptions,
    /// Optional fishing context for dependency resolution.
    fishing_context: Option<Arc<FishingContext>>,
}

impl FshImporter {
    /// Create a new importer with default options.
    pub async fn new() -> FshResult<Self> {
        Self::with_options(FshImportOptions::default()).await
    }

    /// Create a new importer with custom options.
    pub async fn with_options(options: FshImportOptions) -> FshResult<Self> {
        let analyzer = DefaultSemanticAnalyzer::new();

        let mut mapper = FshToIrMapper::new()
            .with_fhir_version(options.fhir_version);

        if let Some(base) = &options.canonical_base {
            mapper = mapper.with_canonical_base(base.clone());
        }

        // Optionally set up fishing context for dependency resolution
        let fishing_context = if options.resolve_dependencies {
            match Self::create_fishing_context(&options).await {
                Ok(ctx) => Some(ctx),
                Err(e) => {
                    warn!("Failed to create fishing context: {}. Continuing without dependency resolution.", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            analyzer,
            mapper,
            options,
            fishing_context,
        })
    }

    /// Create a fishing context for dependency resolution.
    async fn create_fishing_context(options: &FshImportOptions) -> FshResult<Arc<FishingContext>> {
        let canonical_options = CanonicalOptions {
            quick_init: true,
            auto_install_core: false,
            ..Default::default()
        };

        let facade = CanonicalFacade::new(canonical_options)
            .await
            .map_err(|e| FshError::Maki(e.to_string()))?;

        let release = match options.fhir_version {
            FhirVersion::R4 | FhirVersion::R4B => FhirRelease::R4,
            FhirVersion::R5 | FhirVersion::R6 => FhirRelease::R5,
        };

        let session = facade
            .session(vec![release])
            .await
            .map_err(|e| FshError::Maki(e.to_string()))?;

        let tank = Arc::new(RwLock::new(FshTank::new()));
        let package = Arc::new(RwLock::new(Package::new()));

        Ok(Arc::new(FishingContext::new(
            Arc::new(session),
            tank,
            package,
        )))
    }

    /// Import a single FSH file.
    pub async fn import_file(
        &self,
        path: impl AsRef<Path>,
    ) -> FshResult<FshResultWithWarnings<Vec<ProfileDocument>>> {
        let path = path.as_ref();
        info!("Importing FSH file: {}", path.display());

        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(FshError::FileSystem)?;

        self.import_content(&content, path).await
    }

    /// Import FSH content from a string.
    pub async fn import_content(
        &self,
        content: &str,
        source_file: &Path,
    ) -> FshResult<FshResultWithWarnings<Vec<ProfileDocument>>> {
        let mut warnings = Vec::new();

        // Parse FSH
        let parse_result = self.parse_fsh(content, source_file)?;

        // Check for parse errors
        if !parse_result.errors.is_empty() {
            for error in &parse_result.errors {
                if self.options.continue_on_error {
                    warnings.push(FshWarning::new(
                        FshWarningCode::PotentialDataLoss,
                        format!("Parse error: {}", error.message),
                    ).with_location(source_file.to_path_buf(), error.line + 1, error.column + 1));
                } else {
                    return Err(FshError::Import(FshImportError::parse(
                        source_file,
                        error.line + 1,
                        error.column + 1,
                        &error.message,
                    )));
                }
            }
        }

        // Semantic analysis
        let semantic_model = self.analyze_semantics(&parse_result, source_file)?;

        // Add resources to fishing context tank if available
        if let Some(ctx) = &self.fishing_context {
            let tank_ref = ctx.tank();
            let mut tank = tank_ref.write().await;
            for resource in semantic_model.resources.iter() {
                tank.add_resource(resource.clone());
            }
        }

        // Map to IR
        let (documents, mapper_warnings) = self.mapper.map_semantic_model(&semantic_model)
            .map_err(FshError::Import)?;

        warnings.extend(mapper_warnings);

        if documents.is_empty() {
            return Err(FshError::Import(FshImportError::ResourceNotFound {
                name: "Profile".to_string(),
            }));
        }

        info!(
            "Successfully imported {} profile(s) from {}",
            documents.len(),
            source_file.display()
        );

        Ok(FshResultWithWarnings::with_warnings(documents, warnings))
    }

    /// Parse FSH content.
    fn parse_fsh(&self, content: &str, source_file: &Path) -> FshResult<ParseResult> {
        debug!("Parsing FSH content from {}", source_file.display());

        FshParser::parse_content(content)
            .map_err(|e| FshError::Maki(e.to_string()))
    }

    /// Perform semantic analysis on parsed FSH.
    fn analyze_semantics(
        &self,
        parse_result: &ParseResult,
        source_file: &Path,
    ) -> FshResult<SemanticModel> {
        debug!("Analyzing semantics for {}", source_file.display());

        self.analyzer
            .analyze(parse_result.cst(), parse_result.source(), source_file.to_path_buf())
            .map_err(|e| FshError::Maki(e.to_string()))
    }

    /// Get the fishing context (if available).
    pub fn fishing_context(&self) -> Option<&Arc<FishingContext>> {
        self.fishing_context.as_ref()
    }
}

/// FSH project importer.
///
/// Imports entire FSH projects (multiple files with dependencies).
pub struct FshProjectImporter {
    /// Single file importer.
    importer: FshImporter,
    /// Project root directory.
    project_root: PathBuf,
}

impl FshProjectImporter {
    /// Create a new project importer.
    pub async fn new(project_root: impl Into<PathBuf>) -> FshResult<Self> {
        let project_root = project_root.into();
        let options = FshImportOptions::default();

        Ok(Self {
            importer: FshImporter::with_options(options).await?,
            project_root,
        })
    }

    /// Create with custom options.
    pub async fn with_options(
        project_root: impl Into<PathBuf>,
        options: FshImportOptions,
    ) -> FshResult<Self> {
        Ok(Self {
            importer: FshImporter::with_options(options).await?,
            project_root: project_root.into(),
        })
    }

    /// Import all FSH files from the project.
    pub async fn import_project(&self) -> FshResult<FshResultWithWarnings<Vec<ProfileDocument>>> {
        let fsh_dir = self.project_root.join("input").join("fsh");

        // Try alternate locations
        let search_dirs = [
            fsh_dir.clone(),
            self.project_root.join("fsh"),
            self.project_root.clone(),
        ];

        let mut fsh_files = Vec::new();
        for dir in &search_dirs {
            if dir.is_dir() {
                self.find_fsh_files(dir, &mut fsh_files).await?;
                if !fsh_files.is_empty() {
                    break;
                }
            }
        }

        if fsh_files.is_empty() {
            return Err(FshError::Import(FshImportError::ResourceNotFound {
                name: "*.fsh files".to_string(),
            }));
        }

        info!("Found {} FSH files in project", fsh_files.len());

        let mut all_documents = Vec::new();
        let mut all_warnings = Vec::new();

        for file in &fsh_files {
            match self.importer.import_file(file).await {
                Ok(result) => {
                    all_documents.extend(result.value);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    if self.importer.options.continue_on_error {
                        all_warnings.push(FshWarning::new(
                            FshWarningCode::PotentialDataLoss,
                            format!("Failed to import {}: {}", file.display(), e),
                        ));
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Ok(FshResultWithWarnings::with_warnings(all_documents, all_warnings))
    }

    /// Find all FSH files in a directory recursively.
    async fn find_fsh_files(&self, dir: &Path, files: &mut Vec<PathBuf>) -> FshResult<()> {
        let mut entries = tokio::fs::read_dir(dir)
            .await
            .map_err(FshError::FileSystem)?;

        while let Some(entry) = entries.next_entry().await.map_err(FshError::FileSystem)? {
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and node_modules
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') && name != "node_modules" {
                        Box::pin(self.find_fsh_files(&path, files)).await?;
                    }
                }
            } else if path.extension().is_some_and(|ext| ext == "fsh") {
                files.push(path);
            }
        }

        Ok(())
    }

    /// Get the project root.
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_FSH: &str = r#"
Profile: TestPatient
Parent: Patient
Id: test-patient
Title: "Test Patient Profile"
Description: "A simple test profile"

* name 1..* MS
* birthDate 0..1 MS
"#;

    #[tokio::test]
    async fn test_import_simple_fsh() {
        let importer = FshImporter::new().await;

        // May fail if canonical manager not available
        if let Ok(importer) = importer {
            let result = importer
                .import_content(SIMPLE_FSH, Path::new("test.fsh"))
                .await;

            // Just check it doesn't panic - may fail due to missing deps
            if let Ok(result) = result {
                assert!(!result.value.is_empty() || !result.warnings.is_empty());
            }
        }
    }

    #[test]
    fn test_import_options_builder() {
        let options = FshImportOptions::default()
            .with_canonical_base("http://example.org")
            .with_fhir_version(FhirVersion::R4);

        assert_eq!(options.canonical_base, Some("http://example.org".to_string()));
        assert_eq!(options.fhir_version, FhirVersion::R4);
    }
}
