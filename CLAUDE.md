# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust CLI tool for bulk downloading footage from Unifi Protect NVR systems. Uses the `unifi_protect` crate to communicate with the Unifi Protect API.

## Build Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run clippy for linting
cargo clippy

# Run tests
cargo test

# Install locally
cargo install --path .

# Run directly without installing
cargo run -- download <args>
```

## Docker

```bash
# Build Docker image
docker build -t unifi-protect-bulk-download .

# Run via Docker
docker run -it unifi-protect-bulk-download download <uri> <username> <password> <path> <mode> <recording_type> <start_date> <end_date>
```

## Architecture

This is a simple two-file Rust CLI application:

- **`src/main.rs`** - Entry point and core download logic
  - Uses `tokio` async runtime for network operations
  - Handles authentication with Unifi Protect server via `unifi_protect` crate
  - Generates time frames (hourly or daily) between start/end dates
  - Iterates through all cameras and time frames to download footage
  - Skips existing files to enable resumable downloads

- **`src/parse_args.rs`** - CLI argument parsing using `clap`
  - Defines the `download` subcommand with 8 required arguments
  - Validates mode (daily/hourly) and recording_type (rotating/timelapse)

## CLI Usage

```bash
unifi_protect_bulk_download download <uri> <username> <password> <path> <mode> <recording_type> <start_date> <end_date>
```

- `mode`: `daily` (one file per camera per day) or `hourly` (one file per camera per hour)
- `recording_type`: `rotating` (real-time recordings) or `timelapse`
- Date format: `YYYY-MM-DD`

## Key Dependencies

- `unifi_protect` - API communication with Unifi Protect NVR
- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `chrono` - Date/time handling
- `sanitize-filename` - Safe filename generation
