use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;

use crate::mcprs;

pub struct MCPRSTool;

impl MCPRSTool {
    pub fn tool() -> Tool {
        Tool {
            name: "mcprs_list".to_string(),
            description: Some("List MCPRS servers".to_string()),
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

    pub fn call() -> ToolHandlerFn {
        move |request: CallToolRequest| {
            Box::pin(async move {
                let query = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("query"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let servers = mcprs::list_servers(&query);

                tool_text_response!(servers)
            })
        }
    }
}
