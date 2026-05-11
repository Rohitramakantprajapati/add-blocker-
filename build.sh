#!/usr/bin/env bash
set -euo pipefail

echo "==> VoidBlock build"
echo "==> Building Rust core..."
cargo build --release --workspace

echo "==> Running tests..."
cargo test --workspace

echo "==> Build complete."
echo "    Artifacts in target/release/"
