//! REST API module for profile operations.
//!
//! This module provides the HTTP API endpoints for managing FHIR profiles,
//! including CRUD operations, import/export, validation, and element editing.
//!
//! # Routes
//!
//! ## Profile Management
//! - `GET    /api/projects/:projectId/profiles` - List profiles
//! - `POST   /api/projects/:projectId/profiles` - Create profile
//! - `GET    /api/projects/:projectId/profiles/:profileId` - Get profile details
//! - `DELETE /api/projects/:projectId/profiles/:profileId` - Delete profile
//! - `PATCH  /api/projects/:projectId/profiles/:profileId/metadata` - Update metadata
//! - `PATCH  /api/projects/:projectId/profiles/:profileId/elements/:path` - Update element
//! - `POST   /api/projects/:projectId/profiles/:profileId/import` - Import SD/FSH
//!
//! ## Export
//! - `GET    /api/projects/:projectId/profiles/:profileId/export/sd` - Export as SD JSON
//! - `GET    /api/projects/:projectId/profiles/:profileId/export/fsh` - Export as FSH
//! - `GET    /api/projects/:projectId/profiles/:profileId/preview` - Preview content
//! - `GET    /api/projects/:projectId/export` - Bulk export all profiles
//!
//! ## Validation
//! - `POST   /api/projects/:projectId/profiles/:profileId/validate` - Full validation
//! - `POST   /api/projects/:projectId/profiles/:profileId/validate/quick` - Quick structural validation
//! - `POST   /api/projects/:projectId/profiles/:profileId/validate/element` - Validate specific element
//!
//! ## Package Management
//! - `GET    /api/packages` - List installed packages
//! - `GET    /api/packages/search?q=` - Search registry for packages
//! - `POST   /api/packages/:packageId/install` - Install package (SSE stream)
//! - `POST   /api/packages/:packageId/uninstall` - Uninstall package
//!
//! ## Resource Search
//! - `GET    /api/search/extensions?q=&package=` - Search extensions
//! - `GET    /api/search/valuesets?q=` - Search value sets
//! - `GET    /api/search/resources?q=&type=&package=` - Generic resource search

pub mod dto;
pub mod export;
pub mod export_dto;
pub mod history;
pub mod packages;
pub mod packages_dto;
pub mod profiles;
pub mod search_api;
pub mod storage;
pub mod validation;

pub use dto::*;
pub use export::{export_routes, project_export_routes};
pub use history::history_routes;
pub use packages::package_routes;
pub use profiles::profile_routes;
pub use search_api::search_routes;
pub use storage::ProfileStorage;
pub use validation::validation_routes;
