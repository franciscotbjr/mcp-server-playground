use mcp_server_playground::{CalendarTool, ContactsTool, McpServer, RequestHandler, ToolRegistry};
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
    let mut registry = ToolRegistry::new();

    let calendar_tool = CalendarTool::new("calendar.json")?;
    info!("Loaded calendar tool ({} events)", calendar_tool.event_count());
    registry.register(Box::new(calendar_tool));

    let contacts_tool = ContactsTool::new("contacts.json")?;
    info!("Loaded contacts tool ({} contacts)", contacts_tool.contact_count());
    registry.register(Box::new(contacts_tool));

    info!("Tool registry created ({} tool(s) registered)", registry.len());

    info!("Creating request handler...");
    let handler = RequestHandler::new(registry);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting HTTP server on http://{addr}");
    info!("Endpoint: POST|GET|DELETE /mcp");
    let server = McpServer::new(handler, addr);

    server.run().await?;

    Ok(())
}
