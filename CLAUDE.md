# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust CLI tool for bulk downloading footage from Unifi Protect NVR systems. Originally used the `unifi_protect` crate, but was rewritten in 2026 to use a custom API client (`api.rs`) since the old crate used deprecated endpoints.

## Build Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run clippy for linting (ALWAYS run this - see global CLAUDE.md)
cargo clippy

# Run tests
cargo test

# Run a single test
cargo test test_build_filename

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
docker run -it unifi-protect-bulk-download download <uri> <username> <password> <path> <mode> <start_date> <end_date>
```

## Architecture

Three-file Rust CLI application:

- **`src/main.rs`** - Entry point and download orchestration
  - Uses `tokio` async runtime for concurrent network operations
  - Parses date range and calculates time frames (hourly or daily segments)
  - Iterates through all cameras and time frames to download footage
  - Skips existing files to enable resumable downloads
  - Summary output shows downloaded/skipped/no-recording counts

- **`src/api.rs`** - Custom Unifi Protect API client (replaced deprecated `unifi_protect` crate)
  - `UnifiProtectClient` struct with cookie-based session management
  - Accepts self-signed certs (common in Unifi setups)
  - Two-step video download: PREPARE endpoint returns filename, then DOWNLOAD fetches the video
  - Streams large video files to disk without loading into memory

- **`src/parse_args.rs`** - CLI argument parsing using `clap`
  - Defines the `download` subcommand with 7 required arguments
  - Validates mode (daily/hourly)

## CLI Usage

```bash
unifi_protect_bulk_download download <uri> <username> <password> <path> <mode> <start_date> <end_date>
```

- `uri`: Unifi Protect server URL (e.g., `https://192.168.1.1`)
- `mode`: `daily` (one file per camera per day) or `hourly` (one file per camera per hour)
- Date format: `YYYY-MM-DD`

## API Flow

1. **Login** - POST to `/api/auth/login` with credentials, cookies stored for session
2. **Get Cameras** - GET `/proxy/protect/api/cameras` returns camera list
3. **Prepare Video** - GET `/proxy/protect/api/video/prepare?camera=X&start=Y&end=Z...` returns prepared filename
4. **Download Video** - GET `/proxy/protect/api/video/download?camera=X&filename=Y` streams the video file

## Key Dependencies

- `reqwest` - HTTP client with cookie jar and streaming support
- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `chrono` - Date/time handling
- `sanitize-filename` - Safe filename generation
- `serde`/`serde_json` - JSON serialization
- `futures-util` - Async stream handling
