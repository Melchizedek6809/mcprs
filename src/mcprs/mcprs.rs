use std::fmt::{self, Display};
use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;

pub struct MCPRS {
    servers: Vec<MCPRSEntry>,
}

pub struct MCPRSEntry {
    name: String,
    description: String,
    command: String,
}

impl MCPRSEntry {
    pub fn new(name: &str, description: &str, command: &str) -> Self {
        Self { name: name.to_string(), description: description.to_string(), command: command.to_string() }
    }

    pub fn tool(&self) -> Tool {
        Tool {
            name: self.name.clone(),
            description: Some(self.description.clone()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The query to search for, can be left blank to list all servers"
                    }
                },
                "required": ["query"]
            }),
            annotations: None,
        }
    }

    pub fn call(&self) -> ToolHandlerFn {
        move |request: CallToolRequest| {
            Box::pin(async move {
                let query = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("query"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                tool_text_response!(query)
            })
        }
    }
}

impl Display for MCPRSEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

impl MCPRS {
    pub fn new() -> Self {
        Self { servers: vec![] }
    }

    pub fn load_config(&mut self) {
        self.add_server("MCPRS", "Controls the MCPRS server, allowing you to add/remove servers", "mcprs");
    }

    pub fn add_server(&mut self, name: &str, description: &str, command: &str) {
        self.servers.push(MCPRSEntry::new(name, description, command));
    }

    pub fn get_servers(&self) -> &Vec<MCPRSEntry> {
        &self.servers
    }
}
