//! FSH Error Types
//!
//! Error handling for FSH import/export operations, preserving
//! maki-core diagnostics and providing actionable error messages.

use std::path::PathBuf;

/// Result type for FSH operations.
pub type FshResult<T> = std::result::Result<T, FshError>;

/// FSH operation error.
#[derive(Debug, thiserror::Error)]
pub enum FshError {
    /// Error during FSH import.
    #[error("FSH import error: {0}")]
    Import(#[from] FshImportError),

    /// Error during FSH export.
    #[error("FSH export error: {0}")]
    Export(String),

    /// File system error.
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    /// maki-core error.
    #[error("maki error: {0}")]
    Maki(String),

    /// Decompiler error.
    #[error("Decompiler error: {0}")]
    Decompiler(#[from] crate::decompiler::DecompilerError),
}

/// FSH import-specific error.
#[derive(Debug, thiserror::Error)]
pub enum FshImportError {
    /// Parse error in FSH content.
    #[error("Parse error at {file}:{line}:{column}: {message}")]
    Parse {
        file: PathBuf,
        line: usize,
        column: usize,
        message: String,
    },

    /// Semantic analysis error.
    #[error("Semantic error in {file}: {message}")]
    Semantic { file: PathBuf, message: String },

    /// Resource not found.
    #[error("Resource '{name}' not found in FSH content")]
    ResourceNotFound { name: String },

    /// Unsupported FSH construct.
    #[error("Unsupported FSH construct: {construct}")]
    UnsupportedConstruct { construct: String },

    /// Dependency resolution error.
    #[error("Failed to resolve dependency: {url}")]
    DependencyResolution { url: String },

    /// Mapping error from FSH to IR.
    #[error("Failed to map FSH to IR: {message}")]
    MappingError { message: String },

    /// Multiple errors during import.
    #[error("Multiple import errors occurred")]
    Multiple(Vec<FshImportError>),
}

impl FshImportError {
    /// Create a parse error.
    pub fn parse(file: impl Into<PathBuf>, line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::Parse {
            file: file.into(),
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a semantic error.
    pub fn semantic(file: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::Semantic {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create a mapping error.
    pub fn mapping(message: impl Into<String>) -> Self {
        Self::MappingError {
            message: message.into(),
        }
    }
}

/// Warning during FSH import (non-fatal).
#[derive(Debug, Clone)]
pub struct FshWarning {
    /// Warning code for programmatic handling.
    pub code: FshWarningCode,
    /// Human-readable message.
    pub message: String,
    /// File where the warning occurred.
    pub file: Option<PathBuf>,
    /// Line number (1-based).
    pub line: Option<usize>,
    /// Column number (1-based).
    pub column: Option<usize>,
}

/// Warning codes for FSH operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FshWarningCode {
    /// Unrecognized FSH construct (ignored).
    UnrecognizedConstruct,
    /// Deprecated FSH syntax.
    DeprecatedSyntax,
    /// Missing optional metadata.
    MissingMetadata,
    /// Potential data loss during conversion.
    PotentialDataLoss,
    /// Unresolved reference (non-blocking).
    UnresolvedReference,
    /// Duplicate definition (later one used).
    DuplicateDefinition,
}

impl FshWarning {
    /// Create a new warning.
    pub fn new(code: FshWarningCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            file: None,
            line: None,
            column: None,
        }
    }

    /// Set the location.
    pub fn with_location(mut self, file: PathBuf, line: usize, column: usize) -> Self {
        self.file = Some(file);
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

impl std::fmt::Display for FshWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(file) = &self.file {
            write!(
                f,
                "[{:?}] {}:{}:{}: {}",
                self.code,
                file.display(),
                self.line.unwrap_or(0),
                self.column.unwrap_or(0),
                self.message
            )
        } else {
            write!(f, "[{:?}] {}", self.code, self.message)
        }
    }
}

/// Result with warnings.
#[derive(Debug)]
pub struct FshResultWithWarnings<T> {
    /// The result value.
    pub value: T,
    /// Warnings generated during the operation.
    pub warnings: Vec<FshWarning>,
}

impl<T> FshResultWithWarnings<T> {
    /// Create a result with no warnings.
    pub fn ok(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
        }
    }

    /// Create a result with warnings.
    pub fn with_warnings(value: T, warnings: Vec<FshWarning>) -> Self {
        Self { value, warnings }
    }

    /// Check if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
