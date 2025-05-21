use mcp_core::{
    tool_text_response,
    tools::ToolHandlerFn,
    types::{CallToolRequest, Tool},
};
use serde_json::json;

pub struct EchoTool;

impl EchoTool {
    pub fn tool() -> Tool {
        Tool {
            name: "echo".to_string(),
            description: Some("Echo back the message you send".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "The message to echo back"
                    }
                },
                "required": ["message"]
            }),
            annotations: None,
        }
    }

    pub fn call() -> ToolHandlerFn {
        move |request: CallToolRequest| {
            Box::pin(async move {
                let message = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("message"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                tool_text_response!(message)
            })
        }
    }
}
