#!/usr/bin/env bash
#
# Usage:
#   curl -sSfL https://cloudray.io/uninstall.sh | sudo bash

set -euo pipefail

BINARY_NAME="cloudray-agent"

# Set default installation directory based on OS
if [ "$(uname -s)" = "Darwin" ]; then
    DEFAULT_INSTALL_DIR="/usr/local/bin"  # macOS standard location for third-party binaries
else
    DEFAULT_INSTALL_DIR="/usr/bin"        # Linux standard location
fi

INSTALL_DIR="${INSTALL_DIR:-$DEFAULT_INSTALL_DIR}"

uninstall_linux_service() {
    echo "Stopping and disabling cloudray-agent service..."
    systemctl stop cloudray-agent.service 2>/dev/null || true
    systemctl disable cloudray-agent.service 2>/dev/null || true

    echo "Removing systemd service file..."
    rm -f /etc/systemd/system/cloudray-agent.service

    echo "Reloading systemd..."
    systemctl daemon-reload 2>/dev/null || true
}

uninstall_macos_service() {
    echo "Unloading cloudray-agent service..."
    launchctl unload /Library/LaunchDaemons/io.cloudray.agent.plist 2>/dev/null || true

    echo "Removing launchd plist file..."
    rm -f /Library/LaunchDaemons/io.cloudray.agent.plist
}

remove_binary() {
    local install_dir="$1"
    if [ -f "${install_dir}/${BINARY_NAME}" ]; then
        echo "Removing ${BINARY_NAME} binary from ${install_dir}..."
        rm -f "${install_dir}/${BINARY_NAME}"
    else
        echo "${BINARY_NAME} binary not found in ${install_dir}."
        echo "If the binary is installed in a different location, please specify the directory using the INSTALL_DIR environment variable:"
        echo "  curl -sSfL https://cloudray.io/uninstall.sh | INSTALL_DIR=\"/path/to/binary\" sudo bash"
    fi
}

cleanup_environment_files() {
    # Remove environment file if it exists
    if [ -f "/etc/cloudray-agent/environment" ]; then
        echo "Removing environment file..."
        rm -f "/etc/cloudray-agent/environment"
    fi

    # Remove /etc/cloudray-agent directory if it is empty
    if [ -d "/etc/cloudray-agent" ]; then
        if [ -z "$(ls -A /etc/cloudray-agent)" ]; then
            echo "Removing empty /etc/cloudray-agent directory..."
            rmdir "/etc/cloudray-agent"
        fi
    fi
}

print_success() {
    echo "Uninstallation successful!"
    echo
    echo "CloudRay Agent has been completely removed from your system."
    echo
    echo "To reinstall, run:"
    echo "  curl -sSfL https://cloudray.io/install.sh | sudo bash"
    echo
    echo "Learn more at https://cloudray.io/docs/agent"
}

main() {
    echo "Uninstalling CloudRay Agent..."

    # Check if we have the necessary permissions
    if [ ! -w "${INSTALL_DIR}" ]; then
        echo "You do not have write permissions to '${INSTALL_DIR}'."
        echo "To perform a system-wide uninstall, re-run with sudo:"
        echo "  curl -sSfL https://cloudray.io/uninstall.sh | sudo bash"
        exit 1
    fi

    # Uninstall service based on OS
    if [ "$(uname -s)" = "Linux" ]; then
        uninstall_linux_service
    elif [ "$(uname -s)" = "Darwin" ]; then
        uninstall_macos_service
    else
        echo "Unsupported operating system: $(uname -s)"
        exit 1
    fi

    remove_binary "${INSTALL_DIR}"
    cleanup_environment_files
    print_success
}

main
