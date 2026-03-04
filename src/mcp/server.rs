//! MCP server — HTTP bootstrap with SSE transport (MCP spec 2024-11-05).
//!
//! Exposes two endpoints via the handlers in `sse_handler`:
//! - `GET /sse` — client connects here to open an SSE stream
//! - `POST /message?sessionId=<uuid>` — client sends JSON-RPC messages here

use crate::error::Result;
use crate::mcp::handler::RequestHandler;
use crate::mcp::app_state::AppState;
use crate::mcp::sse_handler::{handle_message, handle_sse};

use axum::routing::{get, post};
use axum::Router;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

/// The MCP server that communicates over HTTP with SSE transport.
pub struct McpServer {
    handler: RequestHandler,
    addr: SocketAddr,
}

impl McpServer {
    /// Create a new server with the given request handler and bind address.
    pub fn new(handler: RequestHandler, addr: SocketAddr) -> Self {
        Self { handler, addr }
    }

    /// Run the HTTP server — serves `GET /sse` and `POST /message`.
    pub async fn run(self) -> Result<()> {
        let state = AppState {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            handler: Arc::new(self.handler),
        };

        let app = Router::new()
            .route("/sse", get(handle_sse))
            .route("/message", post(handle_message))
            .with_state(state);

        info!("MCP server listening on {}", self.addr);

        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(|e| crate::error::Error::IoError(format!("Failed to bind: {e}")))?;

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| crate::error::Error::IoError(format!("Server error: {e}")))?;

        info!("MCP server shut down.");
        Ok(())
    }
}

/// Wait for SIGINT (Ctrl-C) for graceful shutdown.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl-C handler");
    info!("Shutdown signal received.");
}
