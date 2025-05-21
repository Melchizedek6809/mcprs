pub mod mcprs;

use mcprs::MCPRS;

pub fn list_servers(query: &str) -> String {
    let mut mcprs = MCPRS::new();
    mcprs.add_server("MCPRS", "MCPRS Server", "mcprs");

    format!(
        "Listing servers with query: {}\n{}",
        query,
        mcprs
            .get_servers()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
