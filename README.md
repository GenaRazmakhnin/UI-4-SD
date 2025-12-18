# NITEN - FHIR Profile Builder

A modern, async-first FHIR Profile Builder with a Rust backend and React frontend.

## Prerequisites

### Rust Toolchain

This project requires **Rust 1.85 or higher** to support Rust Edition 2024.

```bash
# Install rustup if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# The rust-toolchain.toml will automatically select the correct version
rustup show
```

The project includes a `rust-toolchain.toml` that pins the Rust version, so the correct toolchain will be installed automatically when you first build.

### Required Components

The toolchain configuration includes:
- `rustfmt` - Code formatting
- `clippy` - Linting
- `rust-analyzer` - IDE support

## Project Structure

```
niten/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── main.rs         # Binary entry point
│   ├── config.rs       # Server configuration
│   ├── error.rs        # Error types
│   └── server.rs       # HTTP server (Axum)
├── web/                # React frontend
├── Cargo.toml          # Rust dependencies
├── rust-toolchain.toml # Rust version pinning
└── .cargo/
    └── config.toml     # Cargo configuration
```

## Dependencies

This project depends on:
- `maki-core` - FHIR Shorthand parser and semantic analyzer (local path: `../maki/crates/maki-core`)
- `octofhir-canonical-manager` - FHIR package management (local path: `../canonical-manager`)

Ensure these sibling repositories are available:
```
octofhir/
├── maki/                    # FSH toolchain
├── canonical-manager/       # FHIR package management
└── UI-4-SD/                 # This project (niten)
```

## Building

```bash
# Check the project compiles
cargo check

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

## Running

```bash
# Run the development server
cargo run

# Or run with environment variables
HOST=127.0.0.1 PORT=3001 cargo run
```

The server will start on `http://127.0.0.1:3001` by default.

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `127.0.0.1` | Server bind address |
| `PORT` | `3001` | Server port |
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |
| `PACKAGES_CACHE_DIR` | System default | FHIR packages cache directory |

## API Endpoints

- `GET /health` - Health check
- `GET /api/v1/status` - Server status

## Development

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

## Architecture

NITEN follows an async-first design using:
- **Tokio** - Async runtime
- **Axum** - Web framework
- **maki-core** - FSH parsing and FHIR operations
- **octofhir-canonical-manager** - FHIR package resolution

## License

MIT OR Apache-2.0
