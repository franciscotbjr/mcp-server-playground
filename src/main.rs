use mcp_server_playground::{McpServer, RequestHandler, ToolRegistry};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let registry = ToolRegistry::new();
    // Tools will be registered here in Phase 3

    let handler = RequestHandler::new(registry);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = McpServer::new(handler, addr);

    server.run().await?;

    Ok(())
}
