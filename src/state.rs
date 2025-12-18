//! Application state shared across handlers.
//!
//! Provides thread-safe access to shared resources including
//! package management and configuration.

use std::path::PathBuf;
use std::sync::Arc;

use dashmap::DashMap;
use octofhir_canonical_manager::CanonicalManager;
use tokio::sync::{OnceCell, RwLock};

use crate::Config;

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
                let manager = CanonicalManager::with_default_config().await?;
                Ok(Arc::new(manager))
            })
            .await
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
