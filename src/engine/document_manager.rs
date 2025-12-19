//! Document lifecycle management.
//!
//! Manages the lifecycle of profile documents including:
//! - Opening and loading documents
//! - Saving documents to disk
//! - Closing documents
//! - Thread-safe concurrent access

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use chrono::Utc;
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::ir::{
    BaseDefinition, DocumentMetadata, HistoryState, ProfileDocument, ProfiledResource,
};

use super::events::DocumentId;
use super::EngineConfig;

/// Document manager error type.
#[derive(Debug, Error)]
pub enum DocumentError {
    /// Document not found.
    #[error("Document not found: {0}")]
    NotFound(DocumentId),

    /// Document already exists.
    #[error("Document already exists: {0}")]
    AlreadyExists(DocumentId),

    /// Document is locked.
    #[error("Document is locked: {0}")]
    Locked(DocumentId),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid document state.
    #[error("Invalid document state: {0}")]
    InvalidState(String),

    /// Maximum documents exceeded.
    #[error("Maximum open documents exceeded ({0})")]
    MaxDocumentsExceeded(usize),
}

pub type DocumentResult<T> = Result<T, DocumentError>;

/// Metadata about an open document.
#[derive(Debug, Clone)]
pub struct OpenDocument {
    /// Document ID.
    pub id: DocumentId,
    /// Project ID this document belongs to.
    pub project_id: String,
    /// Path to the IR file.
    pub ir_path: PathBuf,
    /// Whether the document has unsaved changes.
    pub is_dirty: bool,
    /// When the document was opened.
    pub opened_at: chrono::DateTime<Utc>,
    /// When the document was last accessed.
    pub last_accessed: chrono::DateTime<Utc>,
}

/// Thread-safe document manager.
#[derive(Debug)]
pub struct DocumentManager {
    /// Open documents.
    documents: RwLock<HashMap<DocumentId, ProfileDocument>>,
    /// Document metadata.
    metadata: RwLock<HashMap<DocumentId, OpenDocument>>,
    /// Configuration.
    config: EngineConfig,
}

impl DocumentManager {
    /// Create a new document manager.
    pub fn new(config: EngineConfig) -> Self {
        Self {
            documents: RwLock::new(HashMap::new()),
            metadata: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// Get the number of open documents.
    pub fn document_count(&self) -> usize {
        self.documents.read().unwrap().len()
    }

    /// Check if a document is open.
    pub fn is_open(&self, doc_id: &str) -> bool {
        self.documents.read().unwrap().contains_key(doc_id)
    }

    /// Get all open document IDs.
    pub fn open_documents(&self) -> Vec<DocumentId> {
        self.documents.read().unwrap().keys().cloned().collect()
    }

    /// Create a new profile document.
    pub fn create_profile(
        &self,
        project_id: &str,
        name: &str,
        base_type: &str,
        canonical_base: &str,
    ) -> DocumentResult<DocumentId> {
        // Check document limit
        if self.document_count() >= self.config.max_open_documents {
            return Err(DocumentError::MaxDocumentsExceeded(
                self.config.max_open_documents,
            ));
        }

        // Generate document ID
        let doc_id = format!("{}-{}", project_id, slug::slugify(name));

        // Check if already exists
        if self.is_open(&doc_id) {
            return Err(DocumentError::AlreadyExists(doc_id));
        }

        // Create canonical URL
        let canonical_url = format!("{}/StructureDefinition/{}", canonical_base, name);

        // Create metadata
        let metadata = DocumentMetadata::new(&doc_id, &canonical_url, name);

        // Create base definition
        let base_def = BaseDefinition::resource(base_type);

        // Create profiled resource
        let resource = ProfiledResource::new(&canonical_url, self.config.fhir_version, base_def);

        // Create document
        let document = ProfileDocument::new(metadata, resource);

        // Create open document metadata
        let now = Utc::now();
        let open_meta = OpenDocument {
            id: doc_id.clone(),
            project_id: project_id.to_string(),
            ir_path: self
                .config
                .workspace_dir
                .join(project_id)
                .join("IR")
                .join("resources")
                .join(format!("{}.json", doc_id)),
            is_dirty: true, // New document is dirty
            opened_at: now,
            last_accessed: now,
        };

        // Store document
        self.documents
            .write()
            .unwrap()
            .insert(doc_id.clone(), document);
        self.metadata
            .write()
            .unwrap()
            .insert(doc_id.clone(), open_meta);

        Ok(doc_id)
    }

    /// Open a document from disk.
    pub async fn open_document(
        &self,
        project_id: &str,
        doc_id: &str,
    ) -> DocumentResult<DocumentId> {
        // Check document limit
        if self.document_count() >= self.config.max_open_documents {
            return Err(DocumentError::MaxDocumentsExceeded(
                self.config.max_open_documents,
            ));
        }

        // Check if already open
        if self.is_open(doc_id) {
            // Just update last accessed and return
            if let Some(meta) = self.metadata.write().unwrap().get_mut(doc_id) {
                meta.last_accessed = Utc::now();
            }
            return Ok(doc_id.to_string());
        }

        // Build path to IR file
        let ir_path = self
            .config
            .workspace_dir
            .join(project_id)
            .join("IR")
            .join("resources")
            .join(format!("{}.json", doc_id));

        // Load document
        let content = fs::read_to_string(&ir_path).await?;
        let document: ProfileDocument = serde_json::from_str(&content)?;

        // Create open document metadata
        let now = Utc::now();
        let open_meta = OpenDocument {
            id: doc_id.to_string(),
            project_id: project_id.to_string(),
            ir_path,
            is_dirty: false,
            opened_at: now,
            last_accessed: now,
        };

        // Store document
        self.documents
            .write()
            .unwrap()
            .insert(doc_id.to_string(), document);
        self.metadata
            .write()
            .unwrap()
            .insert(doc_id.to_string(), open_meta);

        Ok(doc_id.to_string())
    }

    /// Get a document by ID (read-only).
    pub fn get_document(&self, doc_id: &DocumentId) -> DocumentResult<ProfileDocument> {
        let documents = self.documents.read().unwrap();
        let doc = documents
            .get(doc_id)
            .ok_or_else(|| DocumentError::NotFound(doc_id.clone()))?;

        // Update last accessed
        if let Some(meta) = self.metadata.write().unwrap().get_mut(doc_id) {
            meta.last_accessed = Utc::now();
        }

        Ok(doc.clone())
    }

    /// Get document metadata.
    pub fn get_metadata(&self, doc_id: &DocumentId) -> DocumentResult<OpenDocument> {
        self.metadata
            .read()
            .unwrap()
            .get(doc_id)
            .cloned()
            .ok_or_else(|| DocumentError::NotFound(doc_id.clone()))
    }

    /// Execute a mutation on a document.
    pub fn with_document_mut<F, R>(&self, doc_id: &DocumentId, f: F) -> DocumentResult<R>
    where
        F: FnOnce(&mut ProfileDocument) -> R,
    {
        let mut documents = self.documents.write().unwrap();
        let doc = documents
            .get_mut(doc_id)
            .ok_or_else(|| DocumentError::NotFound(doc_id.clone()))?;

        let result = f(doc);

        // Mark as dirty
        if let Some(meta) = self.metadata.write().unwrap().get_mut(doc_id) {
            meta.is_dirty = true;
            meta.last_accessed = Utc::now();
        }

        Ok(result)
    }

    /// Save a document to disk.
    pub async fn save_document(&self, doc_id: &DocumentId) -> DocumentResult<()> {
        // Get document and path
        let (document, ir_path) = {
            let documents = self.documents.read().unwrap();
            let doc = documents
                .get(doc_id)
                .ok_or_else(|| DocumentError::NotFound(doc_id.clone()))?
                .clone();

            let meta = self
                .metadata
                .read()
                .unwrap()
                .get(doc_id)
                .ok_or_else(|| DocumentError::NotFound(doc_id.clone()))?
                .clone();

            (doc, meta.ir_path)
        };

        // Ensure directory exists
        if let Some(parent) = ir_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Serialize and write atomically
        let content = serde_json::to_string_pretty(&document)?;
        let temp_path = ir_path.with_extension("json.tmp");

        let mut file = fs::File::create(&temp_path).await?;
        file.write_all(content.as_bytes()).await?;
        file.sync_all().await?;
        fs::rename(&temp_path, &ir_path).await?;

        // Mark as clean and update history saved state
        {
            let mut documents = self.documents.write().unwrap();
            if let Some(doc) = documents.get_mut(doc_id) {
                doc.history.mark_saved();
            }
        }

        if let Some(meta) = self.metadata.write().unwrap().get_mut(doc_id) {
            meta.is_dirty = false;
            meta.last_accessed = Utc::now();
        }

        Ok(())
    }

    /// Close a document.
    pub fn close_document(&self, doc_id: &DocumentId) -> DocumentResult<bool> {
        let was_dirty = self
            .metadata
            .read()
            .unwrap()
            .get(doc_id)
            .map(|m| m.is_dirty)
            .unwrap_or(false);

        self.documents.write().unwrap().remove(doc_id);
        self.metadata.write().unwrap().remove(doc_id);

        Ok(was_dirty)
    }

    /// Close a document if it's not dirty, otherwise return error.
    pub fn close_if_clean(&self, doc_id: &DocumentId) -> DocumentResult<()> {
        let is_dirty = self
            .metadata
            .read()
            .unwrap()
            .get(doc_id)
            .map(|m| m.is_dirty)
            .unwrap_or(false);

        if is_dirty {
            return Err(DocumentError::InvalidState(format!(
                "Document '{}' has unsaved changes",
                doc_id
            )));
        }

        self.documents.write().unwrap().remove(doc_id);
        self.metadata.write().unwrap().remove(doc_id);

        Ok(())
    }

    /// Get history state for a document.
    pub fn get_history_state(&self, doc_id: &DocumentId) -> DocumentResult<HistoryState> {
        let documents = self.documents.read().unwrap();
        let doc = documents
            .get(doc_id)
            .ok_or_else(|| DocumentError::NotFound(doc_id.clone()))?;

        Ok(doc.history.state())
    }

    /// Undo the last operation on a document.
    pub fn undo(&self, doc_id: &DocumentId) -> DocumentResult<Option<String>> {
        self.with_document_mut(doc_id, |doc| {
            if let Some(op) = doc.history.undo() {
                Some(op.description.clone())
            } else {
                None
            }
        })
    }

    /// Redo the next operation on a document.
    pub fn redo(&self, doc_id: &DocumentId) -> DocumentResult<Option<String>> {
        self.with_document_mut(doc_id, |doc| {
            if let Some(op) = doc.history.redo() {
                Some(op.description.clone())
            } else {
                None
            }
        })
    }

    /// Clean up least recently used documents if over limit.
    pub fn cleanup_lru(&self, keep_count: usize) {
        let mut meta_list: Vec<_> = self.metadata.read().unwrap().values().cloned().collect();

        if meta_list.len() <= keep_count {
            return;
        }

        // Sort by last accessed (oldest first)
        meta_list.sort_by(|a, b| a.last_accessed.cmp(&b.last_accessed));

        // Close oldest documents (only if not dirty)
        let to_close = meta_list.len() - keep_count;
        for meta in meta_list.iter().take(to_close) {
            if !meta.is_dirty {
                let _ = self.close_document(&meta.id);
            }
        }
    }
}

/// Simple slug function for document IDs.
mod slug {
    pub fn slugify(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_lowercase()
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> EngineConfig {
        EngineConfig::with_workspace("/tmp/test-workspace")
    }

    #[test]
    fn test_create_profile() {
        let manager = DocumentManager::new(test_config());

        let doc_id = manager
            .create_profile("my-project", "MyPatient", "Patient", "http://example.org/fhir")
            .unwrap();

        assert!(manager.is_open(&doc_id));
        assert_eq!(manager.document_count(), 1);

        let doc = manager.get_document(&doc_id).unwrap();
        assert_eq!(doc.metadata.name, "MyPatient");
    }

    #[test]
    fn test_document_mutation() {
        let manager = DocumentManager::new(test_config());

        let doc_id = manager
            .create_profile("my-project", "MyPatient", "Patient", "http://example.org/fhir")
            .unwrap();

        // Initially dirty (new document)
        let meta = manager.get_metadata(&doc_id).unwrap();
        assert!(meta.is_dirty);

        // Mutate document
        manager
            .with_document_mut(&doc_id, |doc| {
                doc.metadata.description = Some("Test description".to_string());
            })
            .unwrap();

        let doc = manager.get_document(&doc_id).unwrap();
        assert_eq!(doc.metadata.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_close_document() {
        let manager = DocumentManager::new(test_config());

        let doc_id = manager
            .create_profile("my-project", "MyPatient", "Patient", "http://example.org/fhir")
            .unwrap();

        assert!(manager.is_open(&doc_id));

        let was_dirty = manager.close_document(&doc_id).unwrap();
        assert!(was_dirty);
        assert!(!manager.is_open(&doc_id));
    }

    #[test]
    fn test_max_documents() {
        let config = EngineConfig::with_workspace("/tmp/test").max_open_documents(2);
        let manager = DocumentManager::new(config);

        manager
            .create_profile("project", "Profile1", "Patient", "http://example.org")
            .unwrap();
        manager
            .create_profile("project", "Profile2", "Patient", "http://example.org")
            .unwrap();

        let result =
            manager.create_profile("project", "Profile3", "Patient", "http://example.org");

        assert!(matches!(result, Err(DocumentError::MaxDocumentsExceeded(2))));
    }
}
