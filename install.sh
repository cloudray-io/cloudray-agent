#!/usr/bin/env bash
#
# Usage:
#   curl -sSfL https://cloudray.io/install.sh | sudo bash

set -euo pipefail

LATEST_VERSION_URL="https://raw.githubusercontent.com/cloudray-io/cloudray-agent/refs/heads/main/latest.txt"
REPO_BASE_URL="https://github.com/cloudray-io/cloudray-agent/releases/download"
BINARY_NAME="cloudray-agent"

# Set default installation directory based on OS
if [ "$(uname -s)" = "Darwin" ]; then
    DEFAULT_INSTALL_DIR="/usr/local/bin"  # macOS standard location for third-party binaries
else
    DEFAULT_INSTALL_DIR="/usr/bin"        # Linux standard location
fi

INSTALL_DIR="${INSTALL_DIR:-$DEFAULT_INSTALL_DIR}"

get_architecture() {
    local arch="$(uname -m)"
    local os="$(uname -s)"

    if [ "${os}" = "Linux" ]; then
        case "${arch}" in
            x86_64)
                echo "${BINARY_NAME}-x86_64-unknown-linux-musl.tar.gz"
                ;;
            aarch64|arm64)
                echo "${BINARY_NAME}-aarch64-unknown-linux-musl.tar.gz"
                ;;
            *)
                echo ""
                return 1
                ;;
        esac
    elif [ "${os}" = "Darwin" ]; then
        case "${arch}" in
            x86_64)
                echo "${BINARY_NAME}-x86_64-apple-darwin.tar.gz"
                ;;
            aarch64|arm64)
                echo "${BINARY_NAME}-aarch64-apple-darwin.tar.gz"
                ;;
            *)
                echo ""
                return 1
                ;;
        esac
    else
        echo ""
        return 1
    fi
}

get_installed_version() {
    local install_dir="$1"
    if [ -x "${install_dir}/${BINARY_NAME}" ]; then
        "${install_dir}/${BINARY_NAME}" --version 2>/dev/null | awk '{print $2}'
    fi
}

try_version() {
    local version="$1"
    local archive="$2"
    local download_url="${REPO_BASE_URL}/v${version}/${archive}"

    echo "Trying to download ${BINARY_NAME} ${version}..."
    echo "Downloading from ${download_url}..."
    if curl -sSfL "${download_url}" -o "${archive}" 2>/dev/null; then
        echo "Download successful for version ${version}. Extracting..."
        if tar -xzf "${archive}"; then
            rm -f "${archive}"
            return 0
        fi
        rm -f "${archive}"
    fi
    return 1
}

install_binary() {
    local install_dir="$1"
    local version="$2"

    echo "Installing ${BINARY_NAME} version ${version} to '${install_dir}'..."
    mv "${BINARY_NAME}" "${install_dir}/${BINARY_NAME}"
    chmod +x "${install_dir}/${BINARY_NAME}"

    # Remove quarantine attribute on macOS
    if [ "$(uname -s)" = "Darwin" ]; then
        xattr -d com.apple.quarantine "${install_dir}/${BINARY_NAME}" 2>/dev/null || true
    fi

    return 0
}

print_success() {
    local install_dir="$1"
    echo "Installation successful!"
    echo
    echo "To register, run:"
    echo "  cloudray-agent register REG_CODE"
    echo
    echo "To uninstall, run:"
    echo "  cloudray-agent uninstall"
    echo
    echo "Learn more at https://cloudray.io"
}

main() {
    # First check if binary already exists
    local installed_version="$(get_installed_version "${INSTALL_DIR}")"
    if [ -n "${installed_version}" ]; then
        echo "Installed version: ${installed_version}"
        echo "${BINARY_NAME} is already installed. No action needed."
        exit 0
    fi

    # Check if installation directory is writable
    if [ ! -w "${INSTALL_DIR}" ]; then
        echo "You do not have write permissions to '${INSTALL_DIR}'."
        echo "To perform a system-wide install, re-run with sudo:"
        echo "  curl -sSfL https://cloudray.io/install.sh | sudo bash"
        echo
        echo "Or specify a custom installation directory via the INSTALL_DIR environment variable, for example:"
        echo "  curl -sSfL https://cloudray.io/install.sh | INSTALL_DIR=\"\$HOME/bin\" bash"

        exit 1
    fi

    echo "Checking available ${BINARY_NAME} versions..."

    local versions=($(curl -sSfL "${LATEST_VERSION_URL}" | tr -d '\r'))
    if [ ${#versions[@]} -eq 0 ]; then
        echo "Unable to fetch versions from ${LATEST_VERSION_URL}. Please check your internet connection or try again."
        exit 1
    fi
    echo "Available versions: ${versions[*]}"

    local archive
    archive="$(get_architecture)" || {
        echo "Unsupported architecture: $(uname -m) on $(uname -s)"
        echo "Please reach out to CloudRay Support to request support for your system."
        exit 1
    }

    local success=0
    local version
    for version in "${versions[@]}"; do
        if try_version "${version}" "${archive}"; then
            success=1
            break
        else
            echo "Version ${version} not available, trying next version..."
        fi
    done

    if [ ${success} -eq 0 ]; then
        echo "Failed to download any version. Please check your internet connection or try again later."
        exit 1
    fi

    install_binary "${INSTALL_DIR}" "${version}" || exit 1
    print_success "${INSTALL_DIR}"
}

main
