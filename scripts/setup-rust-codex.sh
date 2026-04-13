#!/usr/bin/env bash
set -euo pipefail

# Codex-friendly Rust setup script.
# - Ensures required tools are installed.
# - Pins and activates the repository Rust toolchain.
# - Installs rustfmt/clippy components used in maintenance workflows.
# - Primes dependency cache using the existing lockfile.
# - Exports environment defaults that improve diagnostics and terminal UX.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

require_tool() {
  local tool="$1"
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "error: required tool not found: $tool" >&2
    exit 1
  fi
}

for tool in bash cargo rustup git; do
  require_tool "$tool"
done

TOOLCHAIN_FILE="${REPO_ROOT}/rust-toolchain.toml"
if [[ ! -f "${TOOLCHAIN_FILE}" ]]; then
  echo "error: ${TOOLCHAIN_FILE} is missing. Commit rust-toolchain.toml before running setup." >&2
  exit 1
fi

CHANNEL="$({ sed -nE 's/^[[:space:]]*channel[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' "${TOOLCHAIN_FILE}" || true; } | head -n 1)"
if [[ -z "${CHANNEL}" ]]; then
  echo "error: unable to determine toolchain channel from rust-toolchain.toml" >&2
  exit 1
fi

ACTIVE_TOOLCHAIN="$(rustup show active-toolchain 2>/dev/null || true)"
ACTIVE_NAME="${ACTIVE_TOOLCHAIN%% *}"

if [[ "${ACTIVE_NAME}" != "${CHANNEL}" ]]; then
  echo "Installing toolchain '${CHANNEL}' (if needed) and setting repo override..."
  rustup toolchain install "${CHANNEL}"
  rustup override set "${CHANNEL}"
else
  echo "Toolchain already active for repo: ${ACTIVE_NAME}"
fi

echo "Ensuring required Rust components are installed..."
rustup component add rustfmt clippy

echo "Priming cargo cache and lockfile dependencies..."
cargo fetch --locked

# Export defaults for Codex sessions.
# - CARGO_TERM_COLOR=always keeps command output readable in logs.
# - RUST_BACKTRACE=1 enables actionable stack traces on failures.
# - CARGO_INCREMENTAL=0 (optional) biases toward deterministic CI-like checks.
export CARGO_TERM_COLOR=always
export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=0

echo
echo "Codex-friendly environment defaults (exported for this shell):"
echo "  export CARGO_TERM_COLOR=${CARGO_TERM_COLOR}"
echo "  export RUST_BACKTRACE=${RUST_BACKTRACE}"
echo "  export CARGO_INCREMENTAL=${CARGO_INCREMENTAL}"
echo
echo "Next commands:"
echo "  scripts/maintain-rust-codex.sh"
echo "  cargo run"
