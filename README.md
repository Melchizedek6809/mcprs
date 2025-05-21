# MCPRS

MCP Router and Package manager, written in Rust.

## How does it work

MCPRS should feel familiar to you if you've used a package manager like Cargo, NPM or Pip.

The way it works is that you only need to install a single MCP Server in your Client, MCPRS. It then automatically provides access to all the MCP Servers you have installed/configured. It also makes configuring super simple by supporting some of the more popular Clients where it can automatically add itself to the config in a very simple onboarding dialog.

It also provides a repository of MCP Servers including statistics/reviews by users, that way people can easily find new MCP Servers to try, this functionality is also exposed to the MCP Client, so you can easily add new functionality without leaving your client.

## Why Rust?

Rust seems like a good language for these low-level plumbing applications, Go would have also worked quite well I'm sure but I'm more familiar with Rust. Also some other languages have a certain startup delay which I think is not good for programs that might be spawned over and over again (via the stdio transport model for example)

## ToDo

### Core Functionality
- [X] Implement basic CLI (just printing version/help to the CLI, maybe with some colors)
- [X] Implement basic MCP Server functionality (using mcp-core)
- [X] Expose a single hardcoded MCP Server via CLI/MCP (simple proof of concept)
- [ ] Expose installed MCP Servers via CLI/MCP (hardcoded list for now)
- [ ] Track how much certain MCP Servers are used / by which client

### Client Integration
- [ ] Implement support for auto-configuring Cursor (first supported client)
- [ ] Add client support for VSCode Copilot
- [ ] Add client support for Windsurf
- [ ] Add client support for Claude Desktop

### Server Management
- [ ] Implement internal DB (JSON/TOML file) for installed MCP Servers
- [ ] Add CRUD operations on installed MCP Server DB via CLI/MCP
- [ ] Allow installation from hardcoded list of MCP Servers
- [ ] Create list of MCP Servers that can be easily installed (from npm via npx)

### Repository & Distribution
- [ ] Setup static file host for MCP Server repository (e.g., mcp.cocz.net)
- [ ] Implement fetching MCP DB from remote repository
- [ ] Allow searching in the remote DB for available Servers
- [ ] Add user ratings and statistics for MCP Servers