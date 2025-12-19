//! FSH Import/Export Module
//!
//! This module provides functionality to import and export FHIR Shorthand (FSH) files.
//! It integrates with `maki-core` for FSH parsing and semantic analysis, and
//! `maki-decompiler` for FSH generation.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
//! │   FSH Files     │────▶│   FshImporter    │────▶│ ProfileDocument │
//! │   (.fsh)        │     │  (maki-core)     │     │     (IR)        │
//! └─────────────────┘     └──────────────────┘     └─────────────────┘
//!                                                          │
//!                                                          ▼
//! ┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
//! │   FSH Output    │◀────│   FshExporter    │◀────│ ProfileDocument │
//! │   (.fsh)        │     │(maki-decompiler) │     │     (IR)        │
//! └─────────────────┘     └──────────────────┘     └─────────────────┘
//! ```
//!
//! # Features
//!
//! - **FSH Import**: Parse FSH files and convert to IR
//! - **FSH Export**: Convert IR to FSH via SD decompilation
//! - **Multi-file Support**: Import entire FSH projects
//! - **FishingContext Integration**: Resolve external dependencies
//! - **Error Handling**: Preserve FSH parser diagnostics
//!
//! # Example
//!
//! ```ignore
//! use niten::fsh::{FshImporter, FshExporter, FshImportOptions};
//! use niten::ir::ProfileDocument;
//!
//! async fn example() -> anyhow::Result<()> {
//!     // Import FSH
//!     let importer = FshImporter::new().await?;
//!     let document = importer.import_file("path/to/profile.fsh").await?;
//!
//!     // Export to FSH
//!     let exporter = FshExporter::new().await?;
//!     let fsh_output = exporter.export(&document).await?;
//!
//!     Ok(())
//! }
//! ```

mod error;
mod export;
mod import;
mod mapper;

pub use error::{FshError, FshImportError, FshResult, FshWarning};
pub use export::{FshExportOptions, FshExporter};
pub use import::{FshImportOptions, FshImporter, FshProjectImporter};
pub use mapper::FshToIrMapper;
