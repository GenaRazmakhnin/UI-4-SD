//! Profile storage service for disk persistence.
//!
//! Handles reading and writing profile documents to the project directory structure:
//! ```text
//! <workspace>/<projectId>/
//! ├── IR/
//! │   ├── index.json          # Index of all profiles
//! │   └── resources/
//! │       └── <profileId>.json # Profile IR documents
//! ├── SD/
//! │   └── StructureDefinition/
//! │       └── <name>.json      # Exported SD JSON files
//! └── FSH/
//!     └── profiles/
//!         └── <name>.fsh       # FSH source files
//! ```

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::ir::ProfileDocument;

/// Profile storage service.
#[derive(Debug, Clone)]
pub struct ProfileStorage {
    /// Project root directory.
    project_dir: PathBuf,
}

/// Project configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Version of the config format.
    pub version: u32,
    /// Project canonical base URL.
    pub canonical: String,
    /// FHIR version (e.g., "4.0.1", "5.0.0").
    pub fhir_version: String,
    /// Project name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Project description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            version: 1,
            canonical: "http://example.org/fhir".to_string(),
            fhir_version: "4.0.1".to_string(),
            name: None,
            description: None,
            publisher: None,
        }
    }
}

/// Index of all profiles in a project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileIndex {
    /// Version of the index format.
    #[serde(default = "default_index_version")]
    pub version: u32,
    /// Last modified timestamp.
    #[serde(default = "default_index_modified_at")]
    pub modified_at: DateTime<Utc>,
    /// List of profile entries.
    #[serde(default)]
    pub profiles: Vec<ProfileIndexEntry>,
}

fn default_index_version() -> u32 {
    1
}

fn default_index_modified_at() -> DateTime<Utc> {
    Utc::now()
}

/// Entry in the profile index.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileIndexEntry {
    /// Profile ID.
    pub id: String,
    /// Canonical URL.
    pub url: String,
    /// Profile name.
    pub name: String,
    /// File path relative to IR/resources/.
    pub file: String,
    /// Last modified timestamp.
    pub modified_at: DateTime<Utc>,
}

/// Storage error type.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Profile not found.
    #[error("Profile not found: {0}")]
    NotFound(String),

    /// Invalid project structure.
    #[error("Invalid project structure: {0}")]
    InvalidStructure(String),

    /// Concurrent modification detected.
    #[error("Concurrent modification detected for profile: {0}")]
    ConcurrentModification(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

impl ProfileStorage {
    /// Create a new storage instance for a project.
    pub fn new(project_dir: impl Into<PathBuf>) -> Self {
        Self {
            project_dir: project_dir.into(),
        }
    }

    /// Get the project directory path.
    pub fn project_dir(&self) -> &Path {
        &self.project_dir
    }

    /// Get the IR directory path.
    fn ir_dir(&self) -> PathBuf {
        self.project_dir.join("IR")
    }

    /// Get the IR resources directory path.
    fn ir_resources_dir(&self) -> PathBuf {
        self.ir_dir().join("resources")
    }

    /// Get the index file path.
    fn index_path(&self) -> PathBuf {
        self.ir_dir().join("index.json")
    }

    /// Get the profile file path.
    fn profile_path(&self, profile_id: &str) -> PathBuf {
        self.ir_resources_dir().join(format!("{}.json", profile_id))
    }

    /// Get the SD directory path.
    fn sd_dir(&self) -> PathBuf {
        self.project_dir.join("SD").join("StructureDefinition")
    }

    /// Get the FSH directory path.
    fn fsh_dir(&self) -> PathBuf {
        self.project_dir.join("FSH").join("profiles")
    }

    /// Get the project config file path.
    fn config_path(&self) -> PathBuf {
        self.project_dir.join("project.json")
    }

    /// Load project configuration.
    pub async fn load_config(&self) -> StorageResult<ProjectConfig> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(ProjectConfig::default());
        }

        let content = fs::read_to_string(&path).await?;
        let config: ProjectConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save project configuration.
    pub async fn save_config(&self, config: &ProjectConfig) -> StorageResult<()> {
        let path = self.config_path();
        let content = serde_json::to_string_pretty(config)?;

        // Write atomically
        let temp_path = path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path).await?;
        file.write_all(content.as_bytes()).await?;
        file.sync_all().await?;
        fs::rename(&temp_path, &path).await?;

        Ok(())
    }

    /// Initialize the project directory structure.
    pub async fn init(&self) -> StorageResult<()> {
        // Create directories
        fs::create_dir_all(self.ir_resources_dir()).await?;
        fs::create_dir_all(self.sd_dir()).await?;
        fs::create_dir_all(self.fsh_dir()).await?;

        // Create empty index if it doesn't exist
        if !self.index_path().exists() {
            let index = ProfileIndex {
                version: 1,
                modified_at: Utc::now(),
                profiles: Vec::new(),
            };
            self.write_index(&index).await?;
        }

        Ok(())
    }

    /// Read the profile index.
    pub async fn read_index(&self) -> StorageResult<ProfileIndex> {
        let path = self.index_path();
        if !path.exists() {
            return Ok(ProfileIndex::default());
        }

        let content = fs::read_to_string(&path).await?;
        let index: ProfileIndex = serde_json::from_str(&content)?;
        Ok(index)
    }

    /// Write the profile index.
    async fn write_index(&self, index: &ProfileIndex) -> StorageResult<()> {
        let path = self.index_path();
        let content = serde_json::to_string_pretty(index)?;

        // Write atomically using temp file
        let temp_path = path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path).await?;
        file.write_all(content.as_bytes()).await?;
        file.sync_all().await?;
        fs::rename(&temp_path, &path).await?;

        Ok(())
    }

    /// List all profiles in the project.
    pub async fn list_profiles(&self) -> StorageResult<Vec<ProfileDocument>> {
        let index = self.read_index().await?;
        let mut profiles = Vec::with_capacity(index.profiles.len());

        for entry in index.profiles {
            match self.load_profile(&entry.id).await {
                Ok(doc) => profiles.push(doc),
                Err(StorageError::NotFound(_)) => {
                    // Skip missing profiles (could be deleted externally)
                    tracing::warn!("Profile {} not found, skipping", entry.id);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(profiles)
    }

    /// Load a profile by ID.
    pub async fn load_profile(&self, profile_id: &str) -> StorageResult<ProfileDocument> {
        let path = self.profile_path(profile_id);
        if !path.exists() {
            return Err(StorageError::NotFound(profile_id.to_string()));
        }

        let content = fs::read_to_string(&path).await?;
        let doc: ProfileDocument = serde_json::from_str(&content)?;
        Ok(doc)
    }

    /// Save a profile to disk.
    pub async fn save_profile(&self, doc: &ProfileDocument) -> StorageResult<()> {
        // Ensure directories exist
        fs::create_dir_all(self.ir_resources_dir()).await?;

        // Save profile document (persist differential-only IR)
        let path = self.profile_path(&doc.metadata.id);
        let mut doc_to_save = doc.clone();
        if doc_to_save.resource.differential.is_empty() && !doc_to_save.resource.root.is_empty() {
            doc_to_save.resource.extract_differential();
        }
        doc_to_save.resource.root = crate::ir::ElementNode::default();
        let content = serde_json::to_string_pretty(&doc_to_save)?;

        // Write atomically
        let temp_path = path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path).await?;
        file.write_all(content.as_bytes()).await?;
        file.sync_all().await?;
        fs::rename(&temp_path, &path).await?;

        // Update index
        self.update_index_entry(doc).await?;

        Ok(())
    }

    /// Update or add an entry in the index.
    async fn update_index_entry(&self, doc: &ProfileDocument) -> StorageResult<()> {
        let mut index = self.read_index().await?;

        let entry = ProfileIndexEntry {
            id: doc.metadata.id.clone(),
            url: doc.metadata.url.clone(),
            name: doc.metadata.name.clone(),
            file: format!("{}.json", doc.metadata.id),
            modified_at: doc.modified_at,
        };

        // Find and update existing entry, or add new
        if let Some(existing) = index.profiles.iter_mut().find(|e| e.id == doc.metadata.id) {
            *existing = entry;
        } else {
            index.profiles.push(entry);
        }

        index.modified_at = Utc::now();
        self.write_index(&index).await?;

        Ok(())
    }

    /// Delete a profile.
    pub async fn delete_profile(&self, profile_id: &str) -> StorageResult<()> {
        // Remove profile file
        let path = self.profile_path(profile_id);
        if path.exists() {
            fs::remove_file(&path).await?;
        }

        // Update index
        let mut index = self.read_index().await?;
        index.profiles.retain(|e| e.id != profile_id);
        index.modified_at = Utc::now();
        self.write_index(&index).await?;

        Ok(())
    }

    /// Check if a profile exists.
    pub async fn profile_exists(&self, profile_id: &str) -> bool {
        self.profile_path(profile_id).exists()
    }

    /// Save an imported SD JSON file.
    pub async fn save_sd_json(&self, name: &str, content: &str) -> StorageResult<PathBuf> {
        fs::create_dir_all(self.sd_dir()).await?;

        let path = self.sd_dir().join(format!("{}.json", name));
        fs::write(&path, content).await?;

        Ok(path)
    }

    /// Save an imported FSH file.
    pub async fn save_fsh(&self, name: &str, content: &str) -> StorageResult<PathBuf> {
        fs::create_dir_all(self.fsh_dir()).await?;

        let path = self.fsh_dir().join(format!("{}.fsh", name));
        fs::write(&path, content).await?;

        Ok(path)
    }

    /// Get all SD JSON files in the project.
    pub async fn list_sd_files(&self) -> StorageResult<Vec<PathBuf>> {
        let dir = self.sd_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        let mut entries = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                files.push(path);
            }
        }

        Ok(files)
    }

    /// Delete all files associated with a profile.
    pub async fn delete_profile_files(&self, profile_id: &str, name: &str) -> StorageResult<()> {
        // Delete IR file
        self.delete_profile(profile_id).await?;

        // Delete SD file if exists
        let sd_path = self.sd_dir().join(format!("{}.json", name));
        if sd_path.exists() {
            fs::remove_file(&sd_path).await?;
        }

        // Delete FSH file if exists
        let fsh_path = self.fsh_dir().join(format!("{}.fsh", name));
        if fsh_path.exists() {
            fs::remove_file(&fsh_path).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, DocumentMetadata, FhirVersion, ProfiledResource};
    use tempfile::TempDir;

    async fn create_test_storage() -> (ProfileStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = ProfileStorage::new(temp_dir.path());
        let _ = storage.init().await;
        (storage, temp_dir)
    }

    fn create_test_document(id: &str) -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            id,
            format!("http://example.org/fhir/StructureDefinition/{}", id),
            format!("Test{}", id),
        );
        let resource = ProfiledResource::new(
            &metadata.url,
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        ProfileDocument::new(metadata, resource)
    }

    #[tokio::test]
    async fn test_init_creates_directories() {
        let (storage, _temp_dir): (ProfileStorage, TempDir) = create_test_storage().await;

        assert!(storage.ir_resources_dir().exists());
        assert!(storage.sd_dir().exists());
        assert!(storage.fsh_dir().exists());
        assert!(storage.index_path().exists());
    }

    #[tokio::test]
    async fn test_save_and_load_profile() {
        let (storage, _temp_dir): (ProfileStorage, TempDir) = create_test_storage().await;

        let doc = create_test_document("test-profile");
        let _: () = storage.save_profile(&doc).await.unwrap();

        let loaded: ProfileDocument = storage.load_profile("test-profile").await.unwrap();
        assert_eq!(loaded.metadata.id, "test-profile");
    }

    #[tokio::test]
    async fn test_list_profiles() {
        let (storage, _temp_dir): (ProfileStorage, TempDir) = create_test_storage().await;

        let doc1 = create_test_document("profile-1");
        let doc2 = create_test_document("profile-2");

        let _: () = storage.save_profile(&doc1).await.unwrap();
        let _: () = storage.save_profile(&doc2).await.unwrap();

        let profiles: Vec<ProfileDocument> = storage.list_profiles().await.unwrap();
        assert_eq!(profiles.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_profile() {
        let (storage, _temp_dir): (ProfileStorage, TempDir) = create_test_storage().await;

        let doc = create_test_document("to-delete");
        let _: () = storage.save_profile(&doc).await.unwrap();

        assert!(storage.profile_exists("to-delete").await);

        let _: () = storage.delete_profile("to-delete").await.unwrap();

        assert!(!storage.profile_exists("to-delete").await);
    }

    #[tokio::test]
    async fn test_load_nonexistent_profile() {
        let (storage, _temp_dir): (ProfileStorage, TempDir) = create_test_storage().await;

        let result: Result<ProfileDocument, StorageError> =
            storage.load_profile("nonexistent").await;
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }
}
