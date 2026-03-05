//! Information about the MCP server.

use serde::{Deserialize, Serialize};

/// Information about the MCP server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}
