//! Project management module.
//!
//! Provides project-level management for FHIR Implementation Guides (IGs),
//! supporting multiple profiles, extensions, and ValueSets with cross-reference
//! resolution.
//!
//! # Architecture
//!
//! ```text
//! <workspace_dir>/
//!   <project_id>/
//!     project.json                   # Project configuration
//!     IR/
//!       index.json                   # Resource index + metadata
//!       resources/
//!         <resource_id>.json         # Serialized IR documents
//!     SD/
//!       StructureDefinition/
//!         <name>.json                # Exported SD JSON
//!       ValueSet/
//!         <name>.json
//!     FSH/
//!       profiles/
//!         <name>.fsh
//!       extensions/
//!         <name>.fsh
//!       valuesets/
//!         <name>.fsh
//! ```

mod model;
mod service;

pub use model::*;
pub use service::*;
