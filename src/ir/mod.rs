//! IR (Intermediate Representation) Data Model
//!
//! This module provides the core data structures for representing FHIR profiles
//! in an editable, UI-friendly format. The IR sits between the UI layer and
//! export formats (StructureDefinition/FSH).
//!
//! # Architecture
//!
//! ```text
//!                    ┌─────────────────────┐
//!                    │   UI Interactions   │
//!                    └──────────┬──────────┘
//!                               │
//!                               ▼
//! ┌──────────────┐    ┌─────────────────────┐    ┌──────────────┐
//! │     FSH      │───▶│   ProfileDocument   │◀───│ StructureDef │
//! │   (Import)   │    │       (IR)          │    │   (Import)   │
//! └──────────────┘    └──────────┬──────────┘    └──────────────┘
//!                               │
//!                    ┌──────────┴──────────┐
//!                    ▼                      ▼
//!             ┌──────────────┐      ┌──────────────┐
//!             │     FSH      │      │ StructureDef │
//!             │   (Export)   │      │   (Export)   │
//!             └──────────────┘      └──────────────┘
//! ```
//!
//! # Key Types
//!
//! - [`ProfileDocument`] - Top-level container for a profile being edited
//! - [`ProfiledResource`] - IR representation of a profiled FHIR resource
//! - [`ElementNode`] - A node in the element tree with stable UI IDs
//! - [`ElementConstraints`] - Constraints applied to an element
//! - [`SlicingDefinition`] - Slicing configuration for an element
//! - [`SliceNode`] - Individual slice within a sliced element
//! - [`ChangeTracker`] - Tracks modifications for undo/redo support
//!
//! # Design Principles
//!
//! 1. **Stable UI IDs**: Every node has a UUID that persists across edits
//! 2. **Change Tracking**: Distinguish inherited vs. explicitly modified fields
//! 3. **Lossless Round-Trip**: Unknown fields are preserved during import/export
//! 4. **Deterministic Serialization**: Consistent ordering for reproducible exports

pub mod constraint;
pub mod document;
pub mod element;
pub mod resource;
pub mod slicing;
pub mod tracking;
mod validation;

// Re-export main types at module level
pub use constraint::{
    Binding, BindingStrength, Cardinality, ElementConstraints, FixedValue, Invariant,
    InvariantSeverity, TypeConstraint,
};
pub use document::{DocumentMetadata, ProfileDocument, ProfileStatus};
pub use element::{ElementNode, ElementSource, NodeId};
pub use resource::{BaseDefinition, FhirVersion, ProfiledResource, StructureKind};
pub use slicing::{Discriminator, DiscriminatorType, SliceNode, SlicingDefinition, SlicingRules};
pub use tracking::{
    Change, ChangeKind, ChangeTracker, EditHistory, HistoryState, Operation, OperationSummary,
};
pub use validation::{ValidationError, ValidationResult, ValidationSeverity};
