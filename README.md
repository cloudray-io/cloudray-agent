# CloudRay Agent

[![Licence: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Repository](https://img.shields.io/badge/GitHub-Repo-blue.svg)](https://github.com/cloudray-io/cloudray-agent)

Agent for CloudRay, a server management & monitoring service. Learn more at [https://cloudray.io](https://cloudray.io).

This agent runs on your server, collects metrics, manages tasks, and communicates with the CloudRay platform.

## Installation

```bash
curl -sSfL https://cloudray.io/install.sh | bash
# Run the cloudray-agent in the background.
# You can find the value of REG_CODE in your CloudRay project at https://cloudray.io
sudo cloudray-agent -d --reg-code <REG_CODE>
```

## Building for Development

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
   sudo ./target/release/cloudray-agent install-service <REG_CODE>
   ```
5. **To run directly:**
   ```bash
   ./target/release/cloudray-agent run
   ```
