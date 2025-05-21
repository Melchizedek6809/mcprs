use mcp_core::server::Server;
use mcp_core::types::Tool;
use tokio::io::{stdin, stdout};
use std::error::Error;
use std::sync::Arc;

/// MCPRS MCP Server implementation.
/// This struct provides basic MCP Server functionality.
pub struct MCPServer {
    name: String,
    version: String,
}

impl MCPServer {
    /// Creates a new instance of the MCPRS MCP Server.
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }

    /// Starts the MCP Server using stdin/stdout as the transport.
    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        // Create an MCP Server
        let mut server = Server::builder(self.name.clone(), self.version.clone(), mcp_core::types::ProtocolVersion::V2025_03_26);

        server.register_tool(VersionTool::tool(), VersionTool::call());
        
        // Create stdin/stdout transport
        let input = stdin();
        let output = stdout();
        
        
        
        Ok(())
    }

    /// Get the name of the server
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the version of the server
    pub fn version(&self) -> &str {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let name = "test-server".to_string();
        let version = "1.0.0".to_string();
        let server = MCPServer::new(name.clone(), version.clone());
        
        // Test the server's name and version
        assert_eq!(server.name(), name);
        assert_eq!(server.version(), version);
    }
} 