//! Change tracking and undo/redo support.
//!
//! This module provides types for tracking changes to profile elements
//! and maintaining edit history for undo/redo operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::element::NodeId;

/// Maximum number of operations to keep in history.
const DEFAULT_MAX_HISTORY: usize = 100;

/// Type of change made to an element or field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    /// Value was set (from None or different value).
    Set,
    /// Value was cleared (set to None).
    Clear,
    /// Item was added to a list.
    Add,
    /// Item was removed from a list.
    Remove,
    /// Item was moved within a list.
    Move,
    /// Multiple changes in one operation.
    Batch,
}

/// A recorded change to the profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// Type of change.
    pub kind: ChangeKind,

    /// Target element ID.
    pub target_id: NodeId,

    /// Path to the changed field (e.g., "constraints.cardinality.min").
    pub field_path: String,

    /// Previous value (for undo).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<serde_json::Value>,

    /// New value (for redo).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<serde_json::Value>,

    /// When the change was made.
    pub timestamp: DateTime<Utc>,
}

impl Change {
    /// Create a set change.
    #[must_use]
    pub fn set(
        target_id: NodeId,
        field_path: impl Into<String>,
        old_value: Option<serde_json::Value>,
        new_value: serde_json::Value,
    ) -> Self {
        Self {
            kind: ChangeKind::Set,
            target_id,
            field_path: field_path.into(),
            old_value,
            new_value: Some(new_value),
            timestamp: Utc::now(),
        }
    }

    /// Create a clear change.
    #[must_use]
    pub fn clear(
        target_id: NodeId,
        field_path: impl Into<String>,
        old_value: serde_json::Value,
    ) -> Self {
        Self {
            kind: ChangeKind::Clear,
            target_id,
            field_path: field_path.into(),
            old_value: Some(old_value),
            new_value: None,
            timestamp: Utc::now(),
        }
    }

    /// Create an add change.
    #[must_use]
    pub fn add(
        target_id: NodeId,
        field_path: impl Into<String>,
        added_value: serde_json::Value,
    ) -> Self {
        Self {
            kind: ChangeKind::Add,
            target_id,
            field_path: field_path.into(),
            old_value: None,
            new_value: Some(added_value),
            timestamp: Utc::now(),
        }
    }

    /// Create a remove change.
    #[must_use]
    pub fn remove(
        target_id: NodeId,
        field_path: impl Into<String>,
        removed_value: serde_json::Value,
    ) -> Self {
        Self {
            kind: ChangeKind::Remove,
            target_id,
            field_path: field_path.into(),
            old_value: Some(removed_value),
            new_value: None,
            timestamp: Utc::now(),
        }
    }

    /// Create the inverse of this change (for undo).
    #[must_use]
    pub fn inverse(&self) -> Self {
        Self {
            kind: match self.kind {
                ChangeKind::Set => ChangeKind::Set,
                ChangeKind::Clear => ChangeKind::Set,
                ChangeKind::Add => ChangeKind::Remove,
                ChangeKind::Remove => ChangeKind::Add,
                ChangeKind::Move => ChangeKind::Move,
                ChangeKind::Batch => ChangeKind::Batch,
            },
            target_id: self.target_id,
            field_path: self.field_path.clone(),
            old_value: self.new_value.clone(),
            new_value: self.old_value.clone(),
            timestamp: Utc::now(),
        }
    }
}

/// An operation that can be undone/redone.
///
/// Operations group one or more changes that should be treated as a single
/// atomic unit for undo/redo purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Unique operation ID.
    pub id: uuid::Uuid,

    /// Human-readable description of the operation.
    pub description: String,

    /// Changes that make up this operation.
    pub changes: Vec<Change>,

    /// When the operation was performed.
    pub timestamp: DateTime<Utc>,
}

impl Operation {
    /// Create a new operation with a single change.
    #[must_use]
    pub fn single(description: impl Into<String>, change: Change) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            description: description.into(),
            changes: vec![change],
            timestamp: Utc::now(),
        }
    }

    /// Create a new operation with multiple changes.
    #[must_use]
    pub fn batch(description: impl Into<String>, changes: Vec<Change>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            description: description.into(),
            changes,
            timestamp: Utc::now(),
        }
    }

    /// Create the inverse operation (for undo).
    #[must_use]
    pub fn inverse(&self) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            description: format!("Undo: {}", self.description),
            changes: self.changes.iter().rev().map(Change::inverse).collect(),
            timestamp: Utc::now(),
        }
    }

    /// Check if this operation is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

/// Edit history for undo/redo support.
///
/// Maintains a stack of operations that have been performed, allowing
/// users to undo and redo changes.
///
/// # Example
///
/// ```
/// use niten::ir::{EditHistory, Operation, Change, ChangeKind, NodeId};
///
/// let mut history = EditHistory::new(50);
///
/// // Record an operation
/// let change = Change::set(
///     NodeId::new(),
///     "constraints.cardinality.min",
///     Some(serde_json::json!(0)),
///     serde_json::json!(1),
/// );
/// let op = Operation::single("Set minimum cardinality", change);
/// history.push(op);
///
/// assert!(history.can_undo());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditHistory {
    /// Stack of operations that can be undone.
    undo_stack: Vec<Operation>,

    /// Stack of operations that can be redone.
    redo_stack: Vec<Operation>,

    /// Maximum number of operations to keep.
    #[serde(default = "default_max_history")]
    max_history: usize,
}

fn default_max_history() -> usize {
    DEFAULT_MAX_HISTORY
}

impl Default for EditHistory {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_HISTORY)
    }
}

impl EditHistory {
    /// Create a new edit history with the specified maximum size.
    #[must_use]
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_history),
            redo_stack: Vec::new(),
            max_history,
        }
    }

    /// Push a new operation onto the history.
    ///
    /// This clears the redo stack since we're starting a new branch.
    pub fn push(&mut self, operation: Operation) {
        // Clear redo stack - we're diverging from the redo path
        self.redo_stack.clear();

        // Enforce max history limit
        if self.undo_stack.len() >= self.max_history {
            self.undo_stack.remove(0);
        }

        self.undo_stack.push(operation);
    }

    /// Check if undo is available.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of operations in the undo stack.
    #[must_use]
    pub fn undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of operations in the redo stack.
    #[must_use]
    pub fn redo_stack_size(&self) -> usize {
        self.redo_stack.len()
    }

    /// Undo the last operation, returning it for application.
    ///
    /// Returns `None` if there's nothing to undo.
    pub fn undo(&mut self) -> Option<Operation> {
        let operation = self.undo_stack.pop()?;
        let inverse = operation.inverse();
        self.redo_stack.push(operation);
        Some(inverse)
    }

    /// Redo the last undone operation, returning it for application.
    ///
    /// Returns `None` if there's nothing to redo.
    pub fn redo(&mut self) -> Option<Operation> {
        let operation = self.redo_stack.pop()?;
        self.undo_stack.push(operation.clone());
        Some(operation)
    }

    /// Get the description of the next undo operation.
    #[must_use]
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.last().map(|op| op.description.as_str())
    }

    /// Get the description of the next redo operation.
    #[must_use]
    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.last().map(|op| op.description.as_str())
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of undoable operations.
    #[must_use]
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redoable operations.
    #[must_use]
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

/// Tracks which fields have been modified vs. inherited.
///
/// This is used to generate semantic diffs and highlight modified
/// elements in the UI.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeTracker {
    /// Set of modified field paths for each element.
    modified_fields: std::collections::HashMap<NodeId, std::collections::HashSet<String>>,
}

impl ChangeTracker {
    /// Create a new change tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark a field as modified.
    pub fn mark_modified(&mut self, node_id: NodeId, field_path: impl Into<String>) {
        self.modified_fields
            .entry(node_id)
            .or_default()
            .insert(field_path.into());
    }

    /// Mark a field as inherited (unmodified).
    pub fn mark_inherited(&mut self, node_id: NodeId, field_path: &str) {
        if let Some(fields) = self.modified_fields.get_mut(&node_id) {
            fields.remove(field_path);
            if fields.is_empty() {
                self.modified_fields.remove(&node_id);
            }
        }
    }

    /// Check if a field is modified.
    #[must_use]
    pub fn is_modified(&self, node_id: NodeId, field_path: &str) -> bool {
        self.modified_fields
            .get(&node_id)
            .is_some_and(|fields| fields.contains(field_path))
    }

    /// Check if any field of an element is modified.
    #[must_use]
    pub fn has_modifications(&self, node_id: NodeId) -> bool {
        self.modified_fields
            .get(&node_id)
            .is_some_and(|fields| !fields.is_empty())
    }

    /// Get all modified fields for an element.
    #[must_use]
    pub fn modified_fields(&self, node_id: NodeId) -> Option<&std::collections::HashSet<String>> {
        self.modified_fields.get(&node_id)
    }

    /// Get all elements with modifications.
    pub fn modified_elements(&self) -> impl Iterator<Item = &NodeId> {
        self.modified_fields.keys()
    }

    /// Clear all tracking data.
    pub fn clear(&mut self) {
        self.modified_fields.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_creation() {
        let node_id = NodeId::new();
        let change = Change::set(
            node_id,
            "constraints.cardinality.min",
            Some(serde_json::json!(0)),
            serde_json::json!(1),
        );

        assert_eq!(change.kind, ChangeKind::Set);
        assert_eq!(change.target_id, node_id);
    }

    #[test]
    fn test_change_inverse() {
        let node_id = NodeId::new();
        let change = Change::set(
            node_id,
            "field",
            Some(serde_json::json!("old")),
            serde_json::json!("new"),
        );

        let inverse = change.inverse();
        assert_eq!(inverse.old_value, Some(serde_json::json!("new")));
        assert_eq!(inverse.new_value, Some(serde_json::json!("old")));
    }

    #[test]
    fn test_edit_history_undo_redo() {
        let mut history = EditHistory::new(10);

        let op = Operation::single(
            "Test operation",
            Change::set(
                NodeId::new(),
                "field",
                None,
                serde_json::json!("value"),
            ),
        );

        history.push(op);

        assert!(history.can_undo());
        assert!(!history.can_redo());

        let undone = history.undo();
        assert!(undone.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());

        let redone = history.redo();
        assert!(redone.is_some());
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_history_max_size() {
        let mut history = EditHistory::new(3);

        for i in 0..5 {
            history.push(Operation::single(
                format!("Op {i}"),
                Change::set(NodeId::new(), "field", None, serde_json::json!(i)),
            ));
        }

        assert_eq!(history.undo_count(), 3);
    }

    #[test]
    fn test_change_tracker() {
        let mut tracker = ChangeTracker::new();
        let node_id = NodeId::new();

        tracker.mark_modified(node_id, "cardinality");
        assert!(tracker.is_modified(node_id, "cardinality"));
        assert!(tracker.has_modifications(node_id));

        tracker.mark_inherited(node_id, "cardinality");
        assert!(!tracker.is_modified(node_id, "cardinality"));
        assert!(!tracker.has_modifications(node_id));
    }
}
