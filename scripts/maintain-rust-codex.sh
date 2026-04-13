#!/usr/bin/env bash
set -euo pipefail

steps=(
  "fmt"
  "clippy"
  "test"
  "check"
  "audit"
)

declare -A status
for step in "${steps[@]}"; do
  status["$step"]="SKIP"
done

print_summary() {
  echo
  echo "=== Codex Rust quality gate summary ==="
  printf 'PASS/FAIL by step:\n'
  for step in "${steps[@]}"; do
    printf ' - %-7s %s\n' "$step:" "${status[$step]}"
  done
}

run_step() {
  local key=$1
  local label=$2
  shift 2

  echo
  echo "[RUN] $label"
  if "$@"; then
    status["$key"]="PASS"
  else
    status["$key"]="FAIL"
    print_summary
    exit 1
  fi
}

run_step "fmt" "cargo fmt --all -- --check" cargo fmt --all -- --check
run_step "clippy" "cargo clippy --all-targets --all-features -- -D warnings" cargo clippy --all-targets --all-features -- -D warnings
run_step "test" "cargo test --all-targets --all-features" cargo test --all-targets --all-features
run_step "check" "cargo check --all-targets --all-features" cargo check --all-targets --all-features

if command -v cargo-audit >/dev/null 2>&1; then
  run_step "audit" "cargo audit" cargo audit
else
  status["audit"]="SKIP (cargo-audit not installed)"
  echo
  echo "[HINT] Optional: install cargo-audit to enable dependency/security checks."
  echo "       cargo install cargo-audit"
fi

print_summary
