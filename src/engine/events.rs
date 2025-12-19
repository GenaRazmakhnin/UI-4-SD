//! Engine events for UI updates.
//!
//! Events are emitted by the engine to notify listeners of state changes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ir::HistoryState;
use crate::validation::ValidationResult;

/// Unique identifier for a document within the engine.
pub type DocumentId = String;

/// Events emitted by the ProfileBuilderEngine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineEvent {
    /// A document was opened.
    DocumentOpened(DocumentOpenedEvent),

    /// A document was modified.
    DocumentModified(DocumentModifiedEvent),

    /// A document was saved.
    DocumentSaved(DocumentSavedEvent),

    /// A document was closed.
    DocumentClosed(DocumentClosedEvent),

    /// An operation was applied.
    OperationApplied(OperationAppliedEvent),

    /// An operation was undone.
    OperationUndone(OperationUndoneEvent),

    /// An operation was redone.
    OperationRedone(OperationRedoneEvent),

    /// Validation completed.
    ValidationCompleted(ValidationCompletedEvent),

    /// An error occurred.
    Error(ErrorEvent),
}

/// Event emitted when a document is opened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentOpenedEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Project ID.
    pub project_id: String,
    /// Profile name.
    pub name: String,
    /// Canonical URL.
    pub url: String,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when a document is modified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentModifiedEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Changed element paths.
    pub changed_paths: Vec<String>,
    /// Whether the document has unsaved changes.
    pub is_dirty: bool,
    /// Current history state.
    pub history_state: HistoryState,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when a document is saved.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSavedEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Formats saved to (ir, sd, fsh).
    pub formats: Vec<String>,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when a document is closed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentClosedEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Whether changes were saved before closing.
    pub saved: bool,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when an operation is applied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationAppliedEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Operation description.
    pub description: String,
    /// Affected element path.
    pub path: Option<String>,
    /// Updated history state.
    pub history_state: HistoryState,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when an operation is undone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationUndoneEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Description of undone operation.
    pub description: String,
    /// Updated history state.
    pub history_state: HistoryState,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when an operation is redone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRedoneEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Description of redone operation.
    pub description: String,
    /// Updated history state.
    pub history_state: HistoryState,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when validation completes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCompletedEvent {
    /// Document ID.
    pub document_id: DocumentId,
    /// Validation result.
    pub result: ValidationResultSummary,
    /// Whether this was incremental validation.
    pub incremental: bool,
    /// Validation duration in milliseconds.
    pub duration_ms: u64,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Summary of validation results (for serialization).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResultSummary {
    /// Whether the document is valid.
    pub is_valid: bool,
    /// Number of errors.
    pub error_count: usize,
    /// Number of warnings.
    pub warning_count: usize,
    /// Number of info messages.
    pub info_count: usize,
}

impl From<&ValidationResult> for ValidationResultSummary {
    fn from(result: &ValidationResult) -> Self {
        Self {
            is_valid: result.is_valid,
            error_count: result.error_count(),
            warning_count: result.warning_count(),
            info_count: result.info_count(),
        }
    }
}

/// Event emitted when an error occurs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    /// Document ID (if applicable).
    pub document_id: Option<DocumentId>,
    /// Error code.
    pub code: String,
    /// Error message.
    pub message: String,
    /// Additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

impl ErrorEvent {
    /// Create a new error event.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            document_id: None,
            code: code.into(),
            message: message.into(),
            details: None,
            timestamp: Utc::now(),
        }
    }

    /// Set the document ID.
    pub fn for_document(mut self, doc_id: DocumentId) -> Self {
        self.document_id = Some(doc_id);
        self
    }

    /// Set additional details.
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

/// Event listener trait.
pub trait EventListener: Send + Sync {
    /// Handle an engine event.
    fn on_event(&self, event: &EngineEvent);
}

/// A simple callback-based event listener.
pub struct CallbackListener<F>
where
    F: Fn(&EngineEvent) + Send + Sync,
{
    callback: F,
}

impl<F> CallbackListener<F>
where
    F: Fn(&EngineEvent) + Send + Sync,
{
    /// Create a new callback listener.
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

impl<F> EventListener for CallbackListener<F>
where
    F: Fn(&EngineEvent) + Send + Sync,
{
    fn on_event(&self, event: &EngineEvent) {
        (self.callback)(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_event_builder() {
        let event = ErrorEvent::new("TEST_ERROR", "Something went wrong")
            .for_document("doc-123".to_string())
            .with_details("More info");

        assert_eq!(event.code, "TEST_ERROR");
        assert_eq!(event.document_id, Some("doc-123".to_string()));
        assert_eq!(event.details, Some("More info".to_string()));
    }

    #[test]
    fn test_validation_result_summary() {
        let result = ValidationResult::valid(crate::validation::ValidationLevel::Structural);
        let summary = ValidationResultSummary::from(&result);

        assert!(summary.is_valid);
        assert_eq!(summary.error_count, 0);
    }
}
