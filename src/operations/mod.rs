//! Operations Engine for Profile Editing.
//!
//! This module provides high-level operations for editing FHIR profiles.
//! Each operation is reversible for undo/redo and validates constraints
//! before applying.
//!
//! # Architecture
//!
//! Operations follow the Command pattern:
//! - Each operation captures its intent and can be applied/undone
//! - Operations validate before applying to maintain document consistency
//! - Operations are atomic - they either complete fully or not at all
//!
//! # Operation Types
//!
//! - **Constraint Operations**: Cardinality, types, flags, bindings, text
//! - **Slicing Operations**: Create slicing, add/remove slices, discriminators
//! - **Extension Operations**: Add/remove/configure extensions
//! - **Fixed/Pattern Operations**: Set fixed or pattern values
//! - **Invariant Operations**: Add/update/remove FHIRPath invariants
//!
//! # Example
//!
//! ```no_run
//! use niten::operations::{Operation, SetCardinality};
//! use niten::ir::{ProfileDocument, Cardinality};
//!
//! let mut doc = // ... load document
//! # todo!();
//! let op = SetCardinality::new("Patient.name", 1, Some(5));
//!
//! // Validate before applying
//! op.validate(&doc)?;
//!
//! // Apply the operation
//! op.apply(&mut doc)?;
//!
//! // Undo if needed
//! op.undo(&mut doc)?;
//! # Ok::<(), niten::operations::OperationError>(())
//! ```

mod constraint;
mod error;
mod extension;
mod invariant;
mod slicing;
mod traits;

pub use constraint::*;
pub use error::{OperationError, OperationResult};
pub use extension::*;
pub use invariant::*;
pub use slicing::*;
pub use traits::{Operation, OperationContext};

use crate::ir::ProfileDocument;

/// Apply an operation to a document with undo support.
///
/// This function:
/// 1. Validates the operation
/// 2. Applies it to the document
/// 3. Records it in the edit history
pub fn apply_operation<O: Operation>(
    doc: &mut ProfileDocument,
    op: &O,
) -> OperationResult<()> {
    // Validate first
    op.validate(doc)?;

    // Apply the operation
    op.apply(doc)?;

    // Mark document as modified
    doc.mark_dirty();

    // Record in history
    let tracking_op = crate::ir::tracking::Operation::single(
        op.description(),
        op.as_change(),
    );
    doc.history.push(tracking_op);

    Ok(())
}

/// Apply multiple operations as a single batch.
///
/// If any operation fails, all changes are rolled back.
pub fn apply_batch<O: Operation>(
    doc: &mut ProfileDocument,
    ops: &[O],
) -> OperationResult<()> {
    // Validate all operations first
    for op in ops {
        op.validate(doc)?;
    }

    // Collect changes for batch history
    let mut changes = Vec::with_capacity(ops.len());

    // Apply all operations
    for op in ops {
        op.apply(doc)?;
        changes.push(op.as_change());
    }

    // Mark document as modified
    doc.mark_dirty();

    // Record as single batch operation
    if !changes.is_empty() {
        let description = if ops.len() == 1 {
            ops[0].description()
        } else {
            format!("Batch: {} operations", ops.len())
        };

        let tracking_op = crate::ir::tracking::Operation::batch(description, changes);
        doc.history.push(tracking_op);
    }

    Ok(())
}
