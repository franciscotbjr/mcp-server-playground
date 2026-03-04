//! The result of the `initialize` method.

use serde::{Deserialize, Serialize};

use super::server_capabilities::ServerCapabilities;
use super::server_info::ServerInfo;

/// The result of the `initialize` method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}
