//! Profile Builder Engine - Central Orchestration Layer
//!
//! This module provides the `ProfileBuilderEngine` that orchestrates all subsystems:
//! - Document lifecycle management (open, save, close)
//! - IR operations with undo/redo
//! - Import/export (SD JSON, FSH)
//! - Validation
//! - Package management integration
//!
//! # Architecture
//!
//! ```text
//!                     ┌─────────────────────────┐
//!                     │  ProfileBuilderEngine   │
//!                     └───────────┬─────────────┘
//!                                 │
//!        ┌────────────────────────┼────────────────────────┐
//!        │                        │                        │
//!        ▼                        ▼                        ▼
//! ┌─────────────┐         ┌─────────────┐         ┌─────────────┐
//! │  Document   │         │  Validation │         │   Package   │
//! │  Manager    │         │   Engine    │         │   Manager   │
//! └─────────────┘         └─────────────┘         └─────────────┘
//!        │                        │                        │
//!        ▼                        ▼                        ▼
//! ┌─────────────┐         ┌─────────────┐         ┌─────────────┐
//! │ ProfileDoc  │         │  Validation │         │  Canonical  │
//! │ Operations  │         │    Rules    │         │   Manager   │
//! │ History     │         │             │         │             │
//! └─────────────┘         └─────────────┘         └─────────────┘
//! ```
//!
//! # Example
//!
//! ```no_run
//! use niten::engine::{ProfileBuilderEngine, EngineConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = EngineConfig::default();
//!     let engine = ProfileBuilderEngine::new(config).await?;
//!
//!     // Create a new profile
//!     let doc_id = engine.create_profile(
//!         "my-project",
//!         "MyPatient",
//!         "Patient",
//!         "http://example.org/fhir"
//!     )?;
//!
//!     // Get the document
//!     let doc = engine.get_document(&doc_id)?;
//!     println!("Created profile: {}", doc.metadata.name);
//!
//!     Ok(())
//! }
//! ```

mod config;
mod document_manager;
mod engine;
mod events;

pub use config::*;
pub use document_manager::*;
pub use engine::*;
pub use events::*;
