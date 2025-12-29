//! Error types for xforge-core

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid UUID format: {0}")]
    InvalidUuid(String),
    
    #[error("Object not found: {0}")]
    ObjectNotFound(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
