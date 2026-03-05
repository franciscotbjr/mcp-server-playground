//! TCP listener wrapper that sets `TCP_NODELAY` on every accepted connection.
//!
//! Axum 0.8 does not set `TCP_NODELAY` by default, which causes Nagle's algorithm
//! to buffer small packets (like SSE events) and add latency.
//! See: <https://github.com/tokio-rs/axum/issues/2521>

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tracing::error;

/// A [`TcpListener`] wrapper that enables `TCP_NODELAY` on each accepted connection.
pub(crate) struct NoDelayListener(pub(crate) TcpListener);

impl axum::serve::Listener for NoDelayListener {
    type Io = TcpStream;
    type Addr = SocketAddr;

    async fn accept(&mut self) -> (TcpStream, SocketAddr) {
        loop {
            match self.0.accept().await {
                Ok((stream, addr)) => {
                    if let Err(e) = stream.set_nodelay(true) {
                        error!("Failed to set TCP_NODELAY: {e}");
                    }
                    return (stream, addr);
                }
                Err(e) => {
                    error!("Failed to accept connection: {e}");
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        }
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.0.local_addr()
    }
}
