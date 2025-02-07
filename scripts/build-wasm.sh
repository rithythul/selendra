#!/bin/bash
set -e

# Ensure we're using the right Rust toolchain
rustup default nightly
rustup target add wasm32-unknown-unknown

# Build the entire workspace
cargo build --release --workspace --target wasm32-unknown-unknown

# Build the runtime specifically
cargo build --release -p selendra-runtime --target wasm32-unknown-unknown

# Optional: Check the built WASM binaries
find target/wasm32-unknown-unknown/release -name "*.wasm"
