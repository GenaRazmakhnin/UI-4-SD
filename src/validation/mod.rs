//! Validation Engine
//!
//! Provides layered validation for FHIR profiles with instant feedback
//! for structural errors and async validation for terminology and external references.
//!
//! # Architecture
//!
//! The validation engine is organized in layers:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Validation Engine                             │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  Layer 1: IR Validation (Instant, <10ms)                        │
//! │  - Cardinality sanity checks                                    │
//! │  - Type refinement validation                                   │
//! │  - Slice validation                                             │
//! │  - Binding strength validation                                  │
//! │  - FHIRPath expression parsing (optional)                       │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  Layer 2: Cross-Reference Validation (Async)                    │
//! │  - Extension URL resolution                                     │
//! │  - Profile reference validation                                 │
//! │  - Target profile resolution                                    │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  Layer 3: Terminology Validation (Async, Cached)                │
//! │  - ValueSet reference resolution                                │
//! │  - Code membership checking                                     │
//! │  - Binding strength appropriateness                             │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Usage
//!
//! ```rust,no_run
//! use niten::validation::{ValidationEngine, ValidationLevel};
//! use niten::ir::ProfileDocument;
//!
//! async fn validate_profile(document: &ProfileDocument) {
//!     let engine = ValidationEngine::new();
//!
//!     // Quick structural validation
//!     let result = engine.validate(document, ValidationLevel::Structural).await;
//!
//!     for diagnostic in &result.diagnostics {
//!         println!("{}: {}", diagnostic.severity, diagnostic.message);
//!     }
//! }
//! ```

pub mod diagnostic;
pub mod engine;
pub mod quick_fix;
pub mod rules;

pub use diagnostic::{Diagnostic, DiagnosticSeverity, DiagnosticSource, ValidationLevel, ValidationResult};
pub use engine::{ValidationEngine, ValidationOptions, Validator};
pub use quick_fix::{QuickFix, QuickFixKind};
