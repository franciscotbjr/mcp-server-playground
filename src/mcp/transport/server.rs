//! MCP server — HTTP bootstrap with Streamable HTTP transport (MCP spec 2025-03-26).
//!
//! Exposes a single `/mcp` endpoint via the handlers in `streamable_handler`:
//! - `POST /mcp` — client sends JSON-RPC messages, server responds with JSON
//! - `GET  /mcp` — client opens passive SSE stream for server-initiated messages
//! - `DELETE /mcp` — client terminates session

use crate::error::Result;
use crate::mcp::handler::RequestHandler;
use super::app_state::AppState;
use super::no_delay_listener::NoDelayListener;
use super::streamable_handler::{handle_delete_mcp, handle_get_mcp, handle_post_mcp};

use axum::routing::{delete, get, post};
use axum::Router;
use super::session::SessionStore;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};
use tracing::info;

/// The MCP server that communicates over HTTP with Streamable HTTP transport.
#[derive(Debug)]
pub struct McpServer {
    handler: RequestHandler,
    addr: SocketAddr,
}

impl McpServer {
    /// Create a new server with the given request handler and bind address.
    pub fn new(handler: RequestHandler, addr: SocketAddr) -> Self {
        Self { handler, addr }
    }

    /// Run the HTTP server — serves `POST /mcp`, `GET /mcp`, `DELETE /mcp`.
    pub async fn run(self) -> Result<()> {
        let sessions: SessionStore = Arc::new(Mutex::new(HashMap::new()));
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let state = AppState {
            sessions: sessions.clone(),
            handler: Arc::new(self.handler),
            shutdown: shutdown_rx,
        };

        let app = Router::new()
            .route("/mcp", post(handle_post_mcp))
            .route("/mcp", get(handle_get_mcp))
            .route("/mcp", delete(handle_delete_mcp))
            .with_state(state);

        info!("MCP server listening on {}", self.addr);
        info!("Endpoint: POST|GET|DELETE /mcp");

        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(|e| crate::error::Error::IoError(format!("Failed to bind: {e}")))?;

        axum::serve(NoDelayListener(listener), app)
            .with_graceful_shutdown(shutdown_signal(sessions, shutdown_tx))
            .await
            .map_err(|e| crate::error::Error::IoError(format!("Server error: {e}")))?;

        info!("MCP server shut down.");
        Ok(())
    }
}

/// Wait for SIGINT (Ctrl-C) for graceful shutdown.
/// Clears all sessions (dropping mpsc senders) so SSE streams end naturally,
/// then installs a second Ctrl-C handler for forced exit.
async fn shutdown_signal(sessions: SessionStore, shutdown_tx: watch::Sender<bool>) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl-C handler");
    info!("Shutdown signal received — closing SSE streams...");

    // Signal cleanup tasks to drop their tx clones
    let _ = shutdown_tx.send(true);

    // Drop all sessions — this drops the mpsc senders in SessionStore
    {
        let mut store = sessions.lock().await;
        let count = store.len();
        store.clear();
        info!("Cleared {count} active session(s)");
    }

    // Second Ctrl-C forces immediate exit
    tokio::spawn(async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install second Ctrl-C handler");
        info!("Forced shutdown.");
        std::process::exit(1);
    });
}
