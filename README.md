# Dynamic MCP Agent Networks

A powerful cross-platform agent built in Rust that automatically discovers and catalogs your organization's APIs and network services. This tool intelligently scans your infrastructure to identify APIs, endpoints, and services, then generates MCP (Model Context Protocol) connectors for seamless integration with your AI and LLM systems using the Model Context Protocol.
![imagedadada](https://github.com/user-attachments/assets/1ad73233-0e16-4116-bfba-bc6eeef6af29)

## Key Features

- **Automatic API Discovery**: Scans networks to identify internal and external APIs, endpoints, and services in your organization
- **Cross-Platform Support**: Runs on Windows, macOS, Linux, and other UNIX-like systems
- **Intelligent API Classification**: Leverages heuristics and signature matching to determine API types, authentication methods, and appropriate connector configurations
- **API Documentation Extraction**: Automatically discovers and parses OpenAPI/Swagger specs, GraphQL schemas, and other API documentation
- **Lightweight and Efficient**: Built in Rust for maximum performance with minimal resource usage
- **Easy MCP Integration**: Generates standardized Model Context Protocol connector files enabling your LLMs and AI systems to interact with your APIs
- **Extensible Plugin System**: Create custom API detectors and connectors for proprietary systems

Perfect for AI integration teams, backend developers, and organizations looking to connect their existing APIs and services to language models and AI systems through the Model Context Protocol.

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/yourusername/dynamic-mcp-agent-networks/releases).

### Using Cargo

```bash
cargo install dynamic-mcp-agent-networks
```

## Quick Start

1. Run a basic API discovery scan:
   ```bash
   dmcp-agent scan --network 192.168.1.0/24
   ```

2. Generate MCP connector configurations:
   ```bash
   dmcp-agent generate --output ./mcp-connectors
   ```

3. Connect to your MCP server:
   ```bash
   dmcp-agent connect --server your-mcp-server.example.com --api-key YOUR_API_KEY
   ```

## Building from Source

### Prerequisites

- Rust toolchain (1.70.0 or newer)
- libpcap development files (for packet capture capabilities)
- OpenSSL development files (for secure connections)

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/dynamic-mcp-agent-networks.git
   cd dynamic-mcp-agent-networks
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. The compiled binary will be available at `target/release/dmcp-agent`

## MCP Integration

The Dynamic MCP Agent Networks tool creates standardized connector files that follow the Model Context Protocol specification, enabling:

- AI systems to discover and interact with your internal APIs
- LLMs to access appropriate company data and services
- Automated documentation and context generation for your APIs
- Secure, managed access to internal tools through the MCP server-client architecture

## Contributing

We welcome contributions from the community! Here's how you can help:

### Getting Started

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Implement your changes
4. Write or update tests as needed
5. Update documentation
6. Submit a pull request

### Development Environment Setup

We recommend using VS Code with the rust-analyzer extension or any Rust-compatible IDE.

For local development, you can use our development container or set up your environment with:

```bash
# Install additional development dependencies
cargo install cargo-watch cargo-expand cargo-audit
```

### Project Structure

```
src/
├── main.rs              # Application entry point
├── config/              # Configuration management
├── discovery/           # API discovery mechanisms
│   ├── port_scan.rs     # TCP/UDP port scanning
│   ├── mdns.rs          # mDNS discovery
│   ├── upnp.rs          # UPnP discovery
│   └── ...
├── connectors/          # MCP connector generation
├── api_analyzers/       # API type detection and schema extraction
├── utils/               # Utility functions
└── cli/                 # Command-line interface
```

### Coding Standards

- Follow the Rust API guidelines
- All public APIs must be documented
- Write tests for new functionality
- Use Rust idioms like Result for error handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The Rust community for exceptional libraries and tools
- Contributors to the Model Context Protocol specification
