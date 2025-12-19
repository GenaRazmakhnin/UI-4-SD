//! NITEN: FHIR Profile Builder Server
//!
//! This crate provides the backend server for the FHIR Profile Builder UI.
//! It integrates with `maki-core` for FSH parsing and FHIR operations.
//!
//! # Architecture
//!
//! The backend follows an async-first design using Tokio and Axum:
//!
//! - **HTTP API**: RESTful endpoints for profile management
//! - **Profile Engine**: IR-based profile editing with maki-core integration
//! - **Package Manager**: FHIR package discovery and dependency resolution
//!
//! # Modules
//!
//! - [`ir`]: Intermediate Representation for FHIR profiles
//! - [`config`]: Server configuration
//! - [`error`]: Error types
//! - [`server`]: HTTP server implementation
//!
//! # Example
//!
//! ```no_run
//! use niten::{Config, Server};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::default();
//!     let server = Server::new(config).await?;
//!     server.run().await
//! }
//! ```

pub mod api;
pub mod config;
pub mod decompiler;
pub mod error;
pub mod export;
pub mod fsh;
pub mod import;
pub mod ir;
pub mod operations;
pub mod server;
pub mod state;
pub mod static_files;
pub mod validation;

pub use config::Config;
pub use error::{Error, Result};
pub use server::Server;
pub use state::AppState;
