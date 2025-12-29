//! Error types for query operations

use thiserror::Error;

/// Result type for query operations
pub type QueryResult<T> = Result<T, QueryError>;

/// Errors that can occur during query operations
#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Target not found: {0}")]
    TargetNotFound(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Configuration not found: {0}")]
    ConfigurationNotFound(String),

    #[error("Build phase not found: {0}")]
    BuildPhaseNotFound(String),

    #[error("Group not found: {0}")]
    GroupNotFound(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Registry error: {0}")]
    RegistryError(String),
}
