#!/usr/bin/env bash
set -euo pipefail

# Run from repository root regardless of invocation directory.
cd "$(dirname "$0")/.."

cargo fmt --all -- --check
cargo check --all-targets
