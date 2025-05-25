# CloudRay Agent

[![Licence: MIT](https://img.shields.io/badge/Licence-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Repository](https://img.shields.io/badge/GitHub-Repo-blue.svg)](https://github.com/cloudray-io/cloudray-agent)

Agent for [CloudRay](https://cloudray.io), a server management & monitoring service.

For detailed information and installation instructions please visit the [Agent Docs](https://cloudray.io/docs/agent).

## Building from Source

> [!NOTE]
> These instructions are intended for developers building the agent from source.
> If you're looking to install the agent, please follow
> the [official installation guide](https://cloudray.io/docs/agent).

1. Install Rust from https://www.rust-lang.org/tools/install
2. **Clone the repository:**
   ```bash
   git clone https://github.com/cloudray-io/cloudray-agent.git
   cd cloudray-agent
   ```
3. **Build the agent:**
   ```bash
   cargo build --release
   ```
4. **To test installing service:**
   ```bash
   sudo ./target/release/cloudray-agent install-service --reg-code <REG_CODE>
   ```
5. **To run directly:**
   ```bash
   ./target/release/cloudray-agent run
   ```
