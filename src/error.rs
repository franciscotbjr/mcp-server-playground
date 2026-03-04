//! Centralized error handling for the MCP server.

use thiserror::Error;

/// All error types for the MCP server, following the `{Type}Error` suffix convention.
#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(String),

    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(String),

    #[error("MCP protocol error: {0}")]
    ProtocolError(String),

    #[error("Tool execution error: {0}")]
    ToolError(String),

    #[error("Data file not found: {0}")]
    DataNotFoundError(String),

    #[error("Invalid request: {0}")]
    InvalidRequestError(String),
}

/// Convenience type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

// Manual From implementations to avoid exposing external types in the public API.

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err.to_string())
    }
}
