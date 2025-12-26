//! Application state shared across handlers.
//!
//! Provides thread-safe access to shared resources including
//! package management and configuration.

use std::path::PathBuf;
use std::sync::Arc;

use dashmap::DashMap;
use octofhir_canonical_manager::CanonicalManager;
use serde::{Deserialize, Serialize};
use tokio::sync::{OnceCell, RwLock};

use crate::Config;
use crate::api::registry_catalog::{SharedRegistryCatalog, create_registry_catalog};
use crate::validation::ValidationResult;

/// Shared application state accessible from all request handlers.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

/// Inner state wrapped in Arc for thread-safe sharing.
struct AppStateInner {
    /// Server configuration.
    config: Config,
    /// Workspace directory for project storage.
    workspace_dir: PathBuf,
    /// Active sessions (project_id -> session data).
    sessions: DashMap<String, SessionData>,
    /// Startup timestamp.
    started_at: chrono::DateTime<chrono::Utc>,
    /// Request counter for diagnostics.
    request_count: RwLock<u64>,
    /// FHIR package manager (lazy-initialized).
    canonical_manager: OnceCell<Arc<CanonicalManager>>,
    /// Cached validation results (key: "project_id/profile_id").
    validation_cache: DashMap<String, CachedValidation>,
    /// Validation configuration.
    validation_config: RwLock<ValidationConfig>,
    /// Registry catalog for package search.
    registry_catalog: SharedRegistryCatalog,
}

/// Cached validation result with metadata.
#[derive(Debug, Clone)]
pub struct CachedValidation {
    /// The validation result.
    pub result: ValidationResult,
    /// Profile modification timestamp when validated.
    pub profile_modified_at: chrono::DateTime<chrono::Utc>,
}

/// Validation configuration settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Default validation level.
    #[serde(default = "default_validation_level")]
    pub default_level: String,
    /// Terminology service URL (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_service_url: Option<String>,
    /// HL7 Validator path for parity checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hl7_validator_path: Option<String>,
    /// Cache validation results.
    #[serde(default = "default_true")]
    pub cache_enabled: bool,
    /// Cache TTL in seconds.
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_seconds: u64,
}

fn default_validation_level() -> String {
    "structural".to_string()
}

fn default_true() -> bool {
    true
}

fn default_cache_ttl() -> u64 {
    300 // 5 minutes
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            default_level: default_validation_level(),
            terminology_service_url: None,
            hl7_validator_path: None,
            cache_enabled: true,
            cache_ttl_seconds: default_cache_ttl(),
        }
    }
}

/// Session data for a project.
#[derive(Debug, Clone)]
pub struct SessionData {
    /// Project ID.
    pub project_id: String,
    /// Last accessed timestamp.
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    /// Whether the project has unsaved changes.
    pub has_unsaved_changes: bool,
}

impl AppState {
    /// Create new application state.
    pub fn new(config: Config, workspace_dir: PathBuf) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                config,
                workspace_dir,
                sessions: DashMap::new(),
                started_at: chrono::Utc::now(),
                request_count: RwLock::new(0),
                canonical_manager: OnceCell::new(),
                validation_cache: DashMap::new(),
                validation_config: RwLock::new(ValidationConfig::default()),
                registry_catalog: create_registry_catalog(),
            }),
        }
    }

    /// Get the canonical manager, initializing it if needed.
    ///
    /// The canonical manager is lazily initialized on first access
    /// using the default configuration.
    pub async fn canonical_manager(
        &self,
    ) -> Result<&Arc<CanonicalManager>, octofhir_canonical_manager::FcmError> {
        self.inner
            .canonical_manager
            .get_or_try_init(|| async {
                // Initialize with parallel processing enabled
                let mut config = octofhir_canonical_manager::config::FcmConfig::default();

                // Set parallel workers to a reasonable default if not already set
                // The library handles num_cpus internally, so this ensures it's explicitly active.
                config.optimization.parallel_workers = 8;

                let manager = CanonicalManager::new(config).await?;
                Ok(Arc::new(manager))
            })
            .await
    }

    /// Get the registry catalog for package search.
    #[must_use]
    pub fn registry_catalog(&self) -> &SharedRegistryCatalog {
        &self.inner.registry_catalog
    }

    /// Get a reference to the configuration.
    #[must_use]
    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    /// Get the workspace directory.
    #[must_use]
    pub fn workspace_dir(&self) -> &PathBuf {
        &self.inner.workspace_dir
    }

    /// Get the server startup time.
    #[must_use]
    pub fn started_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.inner.started_at
    }

    /// Increment request counter and return new count.
    pub async fn increment_requests(&self) -> u64 {
        let mut count = self.inner.request_count.write().await;
        *count += 1;
        *count
    }

    /// Get current request count.
    pub async fn request_count(&self) -> u64 {
        *self.inner.request_count.read().await
    }

    /// Check if a project session exists.
    pub fn has_session(&self, project_id: &str) -> bool {
        self.inner.sessions.contains_key(project_id)
    }

    /// Get or create a session for a project.
    pub fn get_or_create_session(&self, project_id: &str) -> SessionData {
        self.inner
            .sessions
            .entry(project_id.to_string())
            .or_insert_with(|| SessionData {
                project_id: project_id.to_string(),
                last_accessed: chrono::Utc::now(),
                has_unsaved_changes: false,
            })
            .clone()
    }

    /// Update session last accessed time.
    pub fn touch_session(&self, project_id: &str) {
        if let Some(mut session) = self.inner.sessions.get_mut(project_id) {
            session.last_accessed = chrono::Utc::now();
        }
    }

    /// Mark session as having unsaved changes.
    pub fn mark_session_dirty(&self, project_id: &str) {
        if let Some(mut session) = self.inner.sessions.get_mut(project_id) {
            session.has_unsaved_changes = true;
            session.last_accessed = chrono::Utc::now();
        }
    }

    /// Mark session as saved.
    pub fn mark_session_clean(&self, project_id: &str) {
        if let Some(mut session) = self.inner.sessions.get_mut(project_id) {
            session.has_unsaved_changes = false;
            session.last_accessed = chrono::Utc::now();
        }
    }

    /// Remove a session.
    pub fn remove_session(&self, project_id: &str) -> Option<SessionData> {
        self.inner.sessions.remove(project_id).map(|(_, v)| v)
    }

    /// Get all active session IDs.
    pub fn active_sessions(&self) -> Vec<String> {
        self.inner
            .sessions
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }

    /// Get path to a project directory.
    #[must_use]
    pub fn project_path(&self, project_id: &str) -> PathBuf {
        self.inner.workspace_dir.join(project_id)
    }

    /// Calculate server uptime in seconds.
    #[must_use]
    pub fn uptime_seconds(&self) -> i64 {
        (chrono::Utc::now() - self.inner.started_at).num_seconds()
    }

    // === Validation Cache Methods ===

    /// Get cached validation result for a profile.
    pub fn get_cached_validation(
        &self,
        project_id: &str,
        profile_id: &str,
    ) -> Option<CachedValidation> {
        let key = format!("{}/{}", project_id, profile_id);
        self.inner.validation_cache.get(&key).map(|v| v.clone())
    }

    /// Store validation result in cache.
    pub fn cache_validation(
        &self,
        project_id: &str,
        profile_id: &str,
        result: ValidationResult,
        profile_modified_at: chrono::DateTime<chrono::Utc>,
    ) {
        let key = format!("{}/{}", project_id, profile_id);
        self.inner.validation_cache.insert(
            key,
            CachedValidation {
                result,
                profile_modified_at,
            },
        );
    }

    /// Invalidate cached validation for a profile.
    pub fn invalidate_validation(&self, project_id: &str, profile_id: &str) {
        let key = format!("{}/{}", project_id, profile_id);
        self.inner.validation_cache.remove(&key);
    }

    /// Invalidate all cached validations for a project.
    pub fn invalidate_project_validations(&self, project_id: &str) {
        let prefix = format!("{}/", project_id);
        self.inner
            .validation_cache
            .retain(|k, _| !k.starts_with(&prefix));
    }

    /// Get validation configuration.
    pub async fn validation_config(&self) -> ValidationConfig {
        self.inner.validation_config.read().await.clone()
    }

    /// Update validation configuration.
    pub async fn set_validation_config(&self, config: ValidationConfig) {
        let mut current = self.inner.validation_config.write().await;
        *current = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_state() -> AppState {
        let config = Config::default();
        let workspace_dir = PathBuf::from("/tmp/test-workspace");
        AppState::new(config, workspace_dir)
    }

    #[test]
    fn test_session_management() {
        let state = create_test_state();

        assert!(!state.has_session("project-1"));

        let session = state.get_or_create_session("project-1");
        assert_eq!(session.project_id, "project-1");
        assert!(!session.has_unsaved_changes);

        assert!(state.has_session("project-1"));

        state.mark_session_dirty("project-1");
        let session = state.get_or_create_session("project-1");
        assert!(session.has_unsaved_changes);

        state.mark_session_clean("project-1");
        let session = state.get_or_create_session("project-1");
        assert!(!session.has_unsaved_changes);

        let removed = state.remove_session("project-1");
        assert!(removed.is_some());
        assert!(!state.has_session("project-1"));
    }

    #[test]
    fn test_project_path() {
        let state = create_test_state();
        let path = state.project_path("my-project");
        assert_eq!(path, PathBuf::from("/tmp/test-workspace/my-project"));
    }

    #[tokio::test]
    async fn test_request_counter() {
        let state = create_test_state();

        assert_eq!(state.request_count().await, 0);

        let count = state.increment_requests().await;
        assert_eq!(count, 1);

        let count = state.increment_requests().await;
        assert_eq!(count, 2);

        assert_eq!(state.request_count().await, 2);
    }
}
