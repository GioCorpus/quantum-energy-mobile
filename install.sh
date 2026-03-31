#!/bin/bash

echo "========================================"
echo "Quantum Energy Mobile - Installation"
echo "========================================"
echo ""

echo "[1/4] Checking prerequisites..."
echo ""

if ! command -v node &> /dev/null; then
    echo "ERROR: Node.js is not installed or not in PATH"
    echo "Please install Node.js 18+ from https://nodejs.org/"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust/Cargo is not installed or not in PATH"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "Node.js and Rust found!"
echo ""

echo "[2/4] Installing npm dependencies..."
npm install
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to install npm dependencies"
    exit 1
fi
echo ""

echo "[3/4] Installing Tauri CLI..."
cargo install tauri-cli --version "^2.0.0"
if [ $? -ne 0 ]; then
    echo "WARNING: Tauri CLI installation failed or already installed"
fi
echo ""

echo "[4/4] Verifying installation..."
echo ""
echo "Checking Rust dependencies..."
cargo check --manifest-path src-tauri/Cargo.toml
if [ $? -ne 0 ]; then
    echo "WARNING: Some Rust dependencies may need attention"
fi
echo ""

echo "========================================"
echo "Installation Complete!"
echo "========================================"
echo ""
echo "To run the application:"
echo "  npm run tauri dev"
echo ""
echo "To build for production:"
echo "  npm run tauri build"
echo ""
echo "For Android build:"
echo "  npm run tauri android build"
echo ""
echo "For iOS build:"
echo "  npm run tauri ios build"
echo ""
