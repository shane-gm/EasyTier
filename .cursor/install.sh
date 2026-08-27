#!/usr/bin/env bash
# Idempotent bootstrap for the EasyTier development environment.
# Builds the core (easytier-core / easytier-cli) and the web dashboard
# (easytier-web). Safe to re-run.
set -euo pipefail

# --- System dependencies (Ubuntu) --------------------------------------------
# protoc            : RPC / protobuf code generation used across the workspace
# libssl-dev        : OpenSSL headers (some crates link against system OpenSSL)
# pkg-config/clang  : native builds + bindgen (e.g. kcp-sys)
# bridge-utils      : required by the core integration tests
export DEBIAN_FRONTEND=noninteractive
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    protobuf-compiler \
    libssl-dev \
    pkg-config \
    build-essential \
    clang \
    llvm \
    bridge-utils

# --- Rust toolchain -----------------------------------------------------------
# The workspace pins rust-version = "1.89.0" (see easytier/Cargo.toml).
rustup set auto-self-update disable
rustup install 1.89.0
rustup default 1.89.0

# --- Frontend workspace -------------------------------------------------------
# easytier-web embeds frontend/dist/ at compile time, so the web assets must be
# built before the Rust crate is compiled with the `embed` feature.
pnpm -r install
pnpm -r --filter "./easytier-web/*" build

# --- Warm the Cargo build cache for the default workspace members -------------
cargo build -p easytier --bins
cargo build -p easytier-web --features embed
