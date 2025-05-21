use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use mcp_core::{
    server::Server,
    transport::ServerStdioTransport,
    types::{ServerCapabilities, ToolCapabilities},
};

mod tools;

use tools::*;

/// MCP Router and Package manager
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display version information
    Version,

    /// List available MCP Servers
    List,

    /// Start the MCP Server
    Start,
}

async fn start_server() -> Result<()> {
    eprintln!("Starting MCPRS...");

    let server_protocol = Server::builder(
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
        mcp_core::types::ProtocolVersion::V2025_03_26,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(EchoTool::tool(), EchoTool::call())
    .build();

    let transport = ServerStdioTransport::new(server_protocol);
    Server::start(transport).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version) => {
            println!("{}", print_version());
        }
        Some(Commands::List) => {
            println!("{}", list_servers());
        }
        Some(Commands::Start) => {
            return start_server().await;
        }
        None => {
            println!("{}", print_welcome());
        }
    }
    Ok(())
}

fn print_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("{} {}", "MCPRS Version:".green().bold(), version)
}

fn list_servers() -> String {
    format!(
        "{}\n{}",
        "No MCP Servers are currently installed.".yellow(),
        "This functionality will be implemented in a future version."
    )
}

fn print_welcome() -> String {
    let name = env!("CARGO_PKG_NAME").to_uppercase();
    let version = env!("CARGO_PKG_VERSION");

    format!(
        "\n{}\n{} - {}\n{} {}\n\n{}\n  {} {}\n\n{}\n  {} {}\n  {} {}\n  {} {}\n  {} {}\n",
        "✨ Welcome to MCPRS! ✨".bright_cyan().bold(),
        name.bright_magenta().bold(),
        "MCP Router and Package manager".bright_blue(),
        "Version:".green(),
        version,
        "Usage:".yellow().bold(),
        "mcprs".cyan(),
        "[COMMAND]".bright_green(),
        "Commands:".yellow().bold(),
        "version".cyan(),
        "Display version information".bright_white(),
        "list".cyan(),
        "List available MCP Servers".bright_white(),
        "start".cyan(),
        "Start the MCP Server".bright_white(),
        "help".cyan(),
        "Print this help message".bright_white()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_version_string() {
        let version = env!("CARGO_PKG_VERSION");
        // We can't easily test the colored output, but we can check that the version is included
        let output = print_version();
        assert!(output.contains(version));
    }

    #[test]
    fn test_list_servers_string() {
        let output = list_servers();
        assert!(output.contains("No MCP Servers are currently installed"));
    }

    #[test]
    fn test_welcome_string() {
        let version = env!("CARGO_PKG_VERSION");
        let output = print_welcome();
        assert!(output.contains("Welcome to MCPRS"));
        assert!(output.contains(version));
        assert!(output.contains("version"));
        assert!(output.contains("list"));
        assert!(output.contains("help"));
    }

    #[test]
    fn test_cli_version() {
        let mut cmd = Command::cargo_bin("mcprs").unwrap();
        cmd.arg("version");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_cli_list() {
        let mut cmd = Command::cargo_bin("mcprs").unwrap();
        cmd.arg("list");
        cmd.assert().success().stdout(predicate::str::contains(
            "No MCP Servers are currently installed",
        ));
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = Command::cargo_bin("mcprs").unwrap();
        cmd.arg("help");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Usage"))
            .stdout(predicate::str::contains("Commands"));
    }
}
