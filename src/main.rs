use mcp_server_playground::{McpServer, RequestHandler, ToolRegistry};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("mcp_server_playground=info".parse()?),
        )
        .init();

    info!("Initializing MCP Server Playground v{}", env!("CARGO_PKG_VERSION"));

    info!("Creating tool registry...");
    let registry = ToolRegistry::new();
    // Tools will be registered here in Phase 3
    info!("Tool registry created (0 tools registered)");

    info!("Creating request handler...");
    let handler = RequestHandler::new(registry);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting HTTP server on http://{addr}");
    info!("Endpoints: GET /sse, POST /message");
    let server = McpServer::new(handler, addr);

    server.run().await?;

    Ok(())
}
