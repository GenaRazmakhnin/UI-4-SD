//! Profile Builder Engine - Main orchestration layer.
//!
//! The ProfileBuilderEngine is the central coordinator that ties together
//! all subsystems for profile editing.

use std::sync::{Arc, RwLock};
use std::time::Instant;

use chrono::Utc;
use octofhir_canonical_manager::CanonicalManager;
use thiserror::Error;
use tokio::sync::OnceCell;

use crate::ir::{HistoryState, Operation, ProfileDocument};
use crate::operations::{self, OperationError};
use crate::validation::{ValidationEngine, ValidationResult};

use super::config::EngineConfig;
use super::document_manager::{DocumentError, DocumentManager};
use super::events::*;

/// Engine error type.
#[derive(Debug, Error)]
pub enum EngineError {
    /// Document error.
    #[error("Document error: {0}")]
    Document(#[from] DocumentError),

    /// Operation error.
    #[error("Operation error: {0}")]
    Operation(#[from] OperationError),

    /// Validation error.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Package manager error.
    #[error("Package manager error: {0}")]
    PackageManager(String),

    /// Import error.
    #[error("Import error: {0}")]
    Import(String),

    /// Export error.
    #[error("Export error: {0}")]
    Export(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type EngineResult<T> = Result<T, EngineError>;

/// The main Profile Builder Engine.
///
/// Orchestrates all subsystems including:
/// - Document lifecycle management
/// - Operations with undo/redo
/// - Validation
/// - Package management
pub struct ProfileBuilderEngine {
    /// Engine configuration.
    config: EngineConfig,
    /// Document manager.
    document_manager: DocumentManager,
    /// Validation engine.
    validation_engine: ValidationEngine,
    /// Canonical manager for package resolution (lazy initialized).
    canonical_manager: OnceCell<Arc<CanonicalManager>>,
    /// Event listeners.
    listeners: RwLock<Vec<Arc<dyn EventListener>>>,
    /// Cached validation results.
    validation_cache: RwLock<std::collections::HashMap<DocumentId, ValidationResult>>,
}

impl ProfileBuilderEngine {
    /// Create a new engine with the given configuration.
    pub async fn new(config: EngineConfig) -> EngineResult<Self> {
        let validation_engine = ValidationEngine::new();

        Ok(Self {
            document_manager: DocumentManager::new(config.clone()),
            config,
            validation_engine,
            canonical_manager: OnceCell::new(),
            listeners: RwLock::new(Vec::new()),
            validation_cache: RwLock::new(std::collections::HashMap::new()),
        })
    }

    /// Get the engine configuration.
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Get the canonical manager, initializing if needed.
    pub async fn canonical_manager(
        &self,
    ) -> Result<&Arc<CanonicalManager>, octofhir_canonical_manager::FcmError> {
        self.canonical_manager
            .get_or_try_init(|| async {
                let manager = CanonicalManager::with_default_config().await?;
                Ok(Arc::new(manager))
            })
            .await
    }

    // === Event Management ===

    /// Add an event listener.
    pub fn add_listener(&self, listener: Arc<dyn EventListener>) {
        self.listeners.write().unwrap().push(listener);
    }

    /// Remove all listeners.
    pub fn clear_listeners(&self) {
        self.listeners.write().unwrap().clear();
    }

    /// Emit an event to all listeners.
    fn emit(&self, event: EngineEvent) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_event(&event);
        }
    }

    // === Document Lifecycle ===

    /// Create a new profile document.
    pub fn create_profile(
        &self,
        project_id: &str,
        name: &str,
        base_type: &str,
        canonical_base: &str,
    ) -> EngineResult<DocumentId> {
        let doc_id =
            self.document_manager
                .create_profile(project_id, name, base_type, canonical_base)?;

        // Get document for event
        let doc = self.document_manager.get_document(&doc_id)?;

        // Emit event
        self.emit(EngineEvent::DocumentOpened(DocumentOpenedEvent {
            document_id: doc_id.clone(),
            project_id: project_id.to_string(),
            name: doc.metadata.name.clone(),
            url: doc.metadata.url.clone(),
            timestamp: Utc::now(),
        }));

        // Run initial validation if auto-validate is enabled
        if self.config.auto_validate {
            // Note: In production, this would be async/background
            let _ = self.validate_document_sync(&doc_id);
        }

        Ok(doc_id)
    }

    /// Open an existing document.
    pub async fn open_document(
        &self,
        project_id: &str,
        doc_id: &str,
    ) -> EngineResult<DocumentId> {
        let opened_id = self
            .document_manager
            .open_document(project_id, doc_id)
            .await?;

        // Get document for event
        let doc = self.document_manager.get_document(&opened_id)?;

        // Emit event
        self.emit(EngineEvent::DocumentOpened(DocumentOpenedEvent {
            document_id: opened_id.clone(),
            project_id: project_id.to_string(),
            name: doc.metadata.name.clone(),
            url: doc.metadata.url.clone(),
            timestamp: Utc::now(),
        }));

        // Run initial validation
        if self.config.auto_validate {
            let _ = self.validate_document_sync(&opened_id);
        }

        Ok(opened_id)
    }

    /// Get a document by ID.
    pub fn get_document(&self, doc_id: &DocumentId) -> EngineResult<ProfileDocument> {
        Ok(self.document_manager.get_document(doc_id)?)
    }

    /// Get document history state.
    pub fn get_history_state(&self, doc_id: &DocumentId) -> EngineResult<HistoryState> {
        Ok(self.document_manager.get_history_state(doc_id)?)
    }

    /// Save a document.
    pub async fn save_document(&self, doc_id: &DocumentId) -> EngineResult<()> {
        self.document_manager.save_document(doc_id).await?;

        // Emit event
        self.emit(EngineEvent::DocumentSaved(DocumentSavedEvent {
            document_id: doc_id.clone(),
            formats: vec!["ir".to_string()],
            timestamp: Utc::now(),
        }));

        Ok(())
    }

    /// Close a document.
    pub fn close_document(&self, doc_id: &DocumentId, force: bool) -> EngineResult<bool> {
        let was_dirty = if force {
            self.document_manager.close_document(doc_id)?
        } else {
            self.document_manager.close_if_clean(doc_id)?;
            false
        };

        // Clear validation cache
        self.validation_cache.write().unwrap().remove(doc_id);

        // Emit event
        self.emit(EngineEvent::DocumentClosed(DocumentClosedEvent {
            document_id: doc_id.clone(),
            saved: !was_dirty,
            timestamp: Utc::now(),
        }));

        Ok(was_dirty)
    }

    /// Get all open document IDs.
    pub fn open_documents(&self) -> Vec<DocumentId> {
        self.document_manager.open_documents()
    }

    /// Check if a document is open.
    pub fn is_document_open(&self, doc_id: &DocumentId) -> bool {
        self.document_manager.is_open(doc_id)
    }

    // === Operations ===

    /// Apply an operation to a document.
    pub fn apply_operation<O: operations::Operation>(
        &self,
        doc_id: &DocumentId,
        operation: &O,
    ) -> EngineResult<()> {
        // Get the change for recording
        let change = operation.as_change();
        let description = operation.description();

        // Apply operation
        self.document_manager.with_document_mut(doc_id, |doc| {
            // Validate operation
            operation.validate(doc)?;

            // Apply operation
            operation.apply(doc)?;

            // Record in history
            let op = Operation::single(&description, change);
            doc.history.push(op);

            Ok::<(), OperationError>(())
        })??;

        // Invalidate validation cache
        self.validation_cache.write().unwrap().remove(doc_id);

        // Get updated history state
        let history_state = self.document_manager.get_history_state(doc_id)?;

        // Emit event
        self.emit(EngineEvent::OperationApplied(OperationAppliedEvent {
            document_id: doc_id.clone(),
            description,
            path: None, // TODO: Extract from change
            history_state,
            timestamp: Utc::now(),
        }));

        // Run incremental validation if auto-validate is enabled
        if self.config.auto_validate {
            let _ = self.validate_document_sync(doc_id);
        }

        Ok(())
    }

    /// Undo the last operation.
    pub fn undo(&self, doc_id: &DocumentId) -> EngineResult<Option<String>> {
        let description = self.document_manager.undo(doc_id)?;

        if let Some(ref desc) = description {
            // Invalidate validation cache
            self.validation_cache.write().unwrap().remove(doc_id);

            // Get updated history state
            let history_state = self.document_manager.get_history_state(doc_id)?;

            // Emit event
            self.emit(EngineEvent::OperationUndone(OperationUndoneEvent {
                document_id: doc_id.clone(),
                description: desc.clone(),
                history_state,
                timestamp: Utc::now(),
            }));

            // Run validation
            if self.config.auto_validate {
                let _ = self.validate_document_sync(doc_id);
            }
        }

        Ok(description)
    }

    /// Redo the next operation.
    pub fn redo(&self, doc_id: &DocumentId) -> EngineResult<Option<String>> {
        let description = self.document_manager.redo(doc_id)?;

        if let Some(ref desc) = description {
            // Invalidate validation cache
            self.validation_cache.write().unwrap().remove(doc_id);

            // Get updated history state
            let history_state = self.document_manager.get_history_state(doc_id)?;

            // Emit event
            self.emit(EngineEvent::OperationRedone(OperationRedoneEvent {
                document_id: doc_id.clone(),
                description: desc.clone(),
                history_state,
                timestamp: Utc::now(),
            }));

            // Run validation
            if self.config.auto_validate {
                let _ = self.validate_document_sync(doc_id);
            }
        }

        Ok(description)
    }

    // === Validation ===

    /// Validate a document synchronously.
    fn validate_document_sync(&self, doc_id: &DocumentId) -> EngineResult<ValidationResult> {
        let start = Instant::now();

        let doc = self.document_manager.get_document(doc_id)?;

        // Run validation (would be async in real implementation)
        let result = futures::executor::block_on(
            self.validation_engine
                .validate(&doc, self.config.validation_level),
        );

        let duration_ms = start.elapsed().as_millis() as u64;

        // Cache result
        self.validation_cache
            .write()
            .unwrap()
            .insert(doc_id.clone(), result.clone());

        // Emit event
        self.emit(EngineEvent::ValidationCompleted(ValidationCompletedEvent {
            document_id: doc_id.clone(),
            result: ValidationResultSummary::from(&result),
            incremental: false,
            duration_ms,
            timestamp: Utc::now(),
        }));

        Ok(result)
    }

    /// Validate a document asynchronously.
    pub async fn validate_document(&self, doc_id: &DocumentId) -> EngineResult<ValidationResult> {
        let start = Instant::now();

        let doc = self.document_manager.get_document(doc_id)?;

        // Run validation
        let result = self
            .validation_engine
            .validate(&doc, self.config.validation_level)
            .await;

        let duration_ms = start.elapsed().as_millis() as u64;

        // Cache result
        self.validation_cache
            .write()
            .unwrap()
            .insert(doc_id.clone(), result.clone());

        // Emit event
        self.emit(EngineEvent::ValidationCompleted(ValidationCompletedEvent {
            document_id: doc_id.clone(),
            result: ValidationResultSummary::from(&result),
            incremental: false,
            duration_ms,
            timestamp: Utc::now(),
        }));

        Ok(result)
    }

    /// Get cached validation result.
    pub fn get_cached_validation(&self, doc_id: &DocumentId) -> Option<ValidationResult> {
        self.validation_cache.read().unwrap().get(doc_id).cloned()
    }

    /// Invalidate validation cache for a document.
    pub fn invalidate_validation(&self, doc_id: &DocumentId) {
        self.validation_cache.write().unwrap().remove(doc_id);
    }

    // === Utility ===

    /// Emit an error event.
    pub fn emit_error(&self, code: &str, message: &str, doc_id: Option<DocumentId>) {
        let mut event = ErrorEvent::new(code, message);
        if let Some(id) = doc_id {
            event = event.for_document(id);
        }
        self.emit(EngineEvent::Error(event));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn test_config() -> EngineConfig {
        EngineConfig::with_workspace("/tmp/test-engine").no_auto_validate()
    }

    #[tokio::test]
    async fn test_create_profile() {
        let engine = ProfileBuilderEngine::new(test_config()).await.unwrap();

        let doc_id = engine
            .create_profile("my-project", "MyPatient", "Patient", "http://example.org/fhir")
            .unwrap();

        assert!(engine.is_document_open(&doc_id));

        let doc = engine.get_document(&doc_id).unwrap();
        assert_eq!(doc.metadata.name, "MyPatient");
    }

    #[tokio::test]
    async fn test_undo_redo() {
        let engine = ProfileBuilderEngine::new(test_config()).await.unwrap();

        let doc_id = engine
            .create_profile("my-project", "MyPatient", "Patient", "http://example.org/fhir")
            .unwrap();

        // Get initial history state
        let state = engine.get_history_state(&doc_id).unwrap();
        assert!(!state.can_undo);
        assert!(!state.can_redo);

        // Try undo (should return None)
        let result = engine.undo(&doc_id).unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_event_emission() {
        let engine = ProfileBuilderEngine::new(test_config()).await.unwrap();

        // Track events
        let event_count = Arc::new(AtomicUsize::new(0));
        let event_count_clone = event_count.clone();

        let listener = CallbackListener::new(move |_event| {
            event_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        engine.add_listener(Arc::new(listener));

        // Create profile (should emit DocumentOpened)
        let _doc_id = engine
            .create_profile("my-project", "MyPatient", "Patient", "http://example.org/fhir")
            .unwrap();

        assert!(event_count.load(Ordering::SeqCst) >= 1);
    }

    #[tokio::test]
    async fn test_close_document() {
        let engine = ProfileBuilderEngine::new(test_config()).await.unwrap();

        let doc_id = engine
            .create_profile("my-project", "MyPatient", "Patient", "http://example.org/fhir")
            .unwrap();

        assert!(engine.is_document_open(&doc_id));

        // Force close (document is dirty from creation)
        let was_dirty = engine.close_document(&doc_id, true).unwrap();
        assert!(was_dirty);
        assert!(!engine.is_document_open(&doc_id));
    }
}
