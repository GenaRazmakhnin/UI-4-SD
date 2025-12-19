# NITEN - FHIR Profile Builder Server
# Run `just --list` to see available commands

# Default recipe - show help
default:
    @just --list

# === Development ===

# Run backend in development mode (with Vite proxy)
dev:
    cargo run -- --workspace-dir ./workspace --log-level debug

# Run backend only (no UI proxy)
dev-backend:
    cargo run -- --workspace-dir ./workspace --log-level debug

# Run frontend dev server (Vite)
dev-frontend:
    cd web && bun run dev

# Run both frontend and backend in parallel
dev-all:
    #!/usr/bin/env bash
    trap 'kill 0' EXIT
    just dev-frontend &
    sleep 2
    just dev &
    wait

# === Build ===

# Build backend in release mode
build:
    cargo build --release

# Build frontend for production
build-frontend:
    cd web && bun run build

# Build everything for production
build-all: build-frontend build

# === Testing ===

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Check code compiles without building
check:
    cargo check

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# === Server Health Check ===

# Check if server is running
health:
    @curl -s http://localhost:3001/health && echo " - Server is healthy" || echo "Server is not running"

# Get server status
status:
    @curl -s http://localhost:3001/api/v1/status | jq . || echo "Server is not running"

# List projects
projects:
    @curl -s http://localhost:3001/api/v1/projects | jq . || echo "Server is not running"

# === Utilities ===

# Create workspace directory if it doesn't exist
setup:
    mkdir -p workspace
    @echo "Workspace directory created at ./workspace"

# Clean build artifacts
clean:
    cargo clean
    rm -rf web/dist

# Show server help
help-server:
    cargo run -- --help

# === Docker (future) ===

# Build docker image
# docker-build:
#     docker build -t niten .

# Run in docker
# docker-run:
#     docker run -p 3001:3001 -v $(pwd)/workspace:/workspace niten
