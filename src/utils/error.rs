//! Custom error types for Neat CLI

use thiserror::Error;

/// Errors that can occur during Neat operations
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum NeatError {
    #[error("Path does not exist: {0}")]
    PathNotFound(String),

    #[error("Not a directory: {0}")]
    NotADirectory(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Failed to move file from {from} to {to}: {reason}")]
    MoveError {
        from: String,
        to: String,
        reason: String,
    },

    #[error("Failed to read file: {0}")]
    ReadError(String),

    #[error("Failed to create directory: {0}")]
    CreateDirError(String),

    #[error("Invalid duration format: {0}. Use formats like 30d, 7d, 1w")]
    InvalidDuration(String),

    #[error("No history found. Nothing to undo.")]
    NoHistory,

    #[error("Operation cancelled by user")]
    Cancelled,
}
