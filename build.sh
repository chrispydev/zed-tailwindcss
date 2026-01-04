#!/bin/bash

# TailwindCSS Enhanced Extension Build Script
# This script builds the Zed extension for WebAssembly

set -e

echo "🚀 Building TailwindCSS Enhanced Extension for Zed..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust via rustup:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check if wasm32-wasip1 target is installed
if ! rustup target list --installed | grep -q wasm32-wasip1; then
    echo "📦 Installing WebAssembly target..."
    rustup target add wasm32-wasip1
fi

# Clean previous build
echo "🧹 Cleaning previous build..."
cargo clean

# Build the extension
echo "🔨 Building extension..."
cargo build --target wasm32-wasip1 --release

# Check if build was successful
if [ -f "target/wasm32-wasip1/release/zed_tailwindcss.wasm" ]; then
    echo "✅ Build successful!"
    echo "📊 WebAssembly file size: $(ls -lh target/wasm32-wasip1/release/zed_tailwindcss.wasm | awk '{print $5}')"
    echo ""
    echo "📋 To install as dev extension in Zed:"
    echo "   1. Open Zed"
    echo "   2. Press Cmd+Shift+P (Mac) or Ctrl+Shift+P (Linux/Windows)"
    echo "   3. Type 'Install Dev Extension' and select it"
    echo "   4. Navigate to and select this directory: $(pwd)"
    echo ""
    echo "🎉 Extension is ready to use!"
else
    echo "❌ Build failed!"
    exit 1
fi
