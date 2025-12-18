# Task: Axum HTTP Server Setup

## Status: ✅ Implemented

## Description
Create the HTTP server using Axum that will serve the React UI and provide REST API endpoints for profile operations. The server should support embedded UI deployment.

## Implementation Summary

The HTTP server has been fully implemented with the following components:

### Module Structure (`src/`)
- `main.rs` - Entry point with CLI argument parsing and tracing setup
- `server.rs` - Main Axum server with routing, middleware, and graceful shutdown
- `config.rs` - Configuration with clap CLI parsing and environment variable support
- `state.rs` - Shared application state with session management
- `static_files.rs` - Static file embedding and SPA routing
- `error.rs` - Error types with HTTP status code mapping

### Key Features Implemented
- **Async-first design** - All server operations are async using Tokio
- **CLI argument parsing** - Full clap integration with environment variable support
- **Static file embedding** - rust-embed for production deployment
- **SPA routing** - Serves index.html for unknown routes
- **Development proxy** - Proxies to Vite dev server when in dev mode
- **Graceful shutdown** - Handles SIGTERM/SIGINT with configurable timeout
- **Request timeout** - Configurable timeout middleware
- **CORS configuration** - Configurable origins for dev/production

## Requirements

### R1: Server Crate Setup ✅
- ✅ Axum dependencies configured
- ✅ Tokio async runtime with full features and signal handling
- ✅ Basic HTTP server with configurable port
- ✅ Support graceful shutdown on SIGTERM/SIGINT

### R2: Routing Structure ✅
- ✅ Static file serving (embedded React UI via rust-embed)
- ✅ API routes under `/api/*`
- ✅ Health check endpoint at `/health`
- ✅ 404 handling for SPA routing (serves index.html)
- ✅ API 404s return JSON error responses

### R3: Application State ✅
- ✅ `AppState`: Shared state across handlers
  - ✅ Configuration access
  - ✅ Workspace directory path
  - ✅ Session management (project sessions)
  - ✅ Server uptime and request counting

### R4: Middleware Setup ✅
- ✅ CORS configuration for development (permissive) and production (configurable origins)
- ✅ Request logging (TraceLayer)
- ✅ Error handling with JSON responses
- ✅ Request timeout middleware (configurable duration)

### R5: Static Asset Embedding ✅
- ✅ Uses `rust-embed` to embed React build from `web/dist`
- ✅ Serves index.html for SPA routes
- ✅ Serves static assets (JS, CSS, images)
- ✅ Proper content-type headers via mime_guess
- ✅ Smart cache-control headers (immutable for hashed assets)

### R6: Configuration ✅
- ✅ Port configuration (env var `PORT`, CLI `-p/--port`, default 3001)
- ✅ Host configuration (env var `HOST`, CLI `--host`, default 127.0.0.1)
- ✅ CORS allowed origins (env var `CORS_ORIGINS`, CLI `--cors-origins`)
- ✅ Log level configuration (env var `LOG_LEVEL`, CLI `--log-level`)
- ✅ Base path configuration (env var `BASE_PATH`, CLI `--base-path`)
- ✅ Workspace directory configuration (env var `WORKSPACE_DIR`, CLI `--workspace-dir`)
  - Validates path exists or creates it on startup
  - Refuses to start if directory is not writable
- ✅ Request timeout (env var `REQUEST_TIMEOUT`, CLI `--request-timeout`, default 30s)
- ✅ Shutdown timeout (env var `SHUTDOWN_TIMEOUT`, CLI `--shutdown-timeout`, default 10s)

### R7: Error Handling ✅
- ✅ Consistent JSON error response format: `{"error": "message", "status": code}`
- ✅ HTTP status codes mapped to error types
- ✅ User-friendly error messages
- ✅ Detailed error logging via tracing

### R8: Development Support ✅
- ✅ Development mode flag (`--dev-mode` or `DEV_MODE=true`)
- ✅ Proxy to Vite dev server during UI development
- ✅ Vite dev URL configurable (`--vite-dev-url`, default http://localhost:5173)
- ✅ Environment-based configuration with clap

## Acceptance Criteria

- [x] Server starts and listens on configured port
- [x] `/health` endpoint returns 200 OK
- [x] Embedded static files are served correctly
- [x] SPA routing works (all routes serve index.html)
- [x] CORS headers are set correctly for dev mode
- [x] Request logging shows all incoming requests
- [x] Graceful shutdown on SIGTERM/SIGINT
- [x] Error responses are JSON formatted
- [x] 404 errors are handled appropriately (JSON for API, HTML for UI)
- [x] Server can run in production mode (embedded UI)
- [x] Server can proxy to Vite in dev mode
- [x] Configuration via environment variables works

## Test Results

All 102 unit tests pass plus 12 doc tests:
- `test_health_check` - Verifies health endpoint returns "OK"
- `test_build_cors_layer_dev_mode` - CORS layer for dev mode
- `test_build_cors_layer_specific_origins` - CORS with specific origins
- `test_session_management` - AppState session operations
- `test_project_path` - Project directory path generation
- `test_request_counter` - Async request counting
- `test_cache_control_for_path` - Cache header generation
- `test_not_found_html` - 404 page generation
- Configuration tests (default, bind_addr, cors_origins, timeouts)

## Usage Examples

### Production Mode (Embedded UI)
```bash
# Start with workspace directory
niten --workspace-dir ./workspace

# With all options
niten --host 0.0.0.0 --port 8080 --workspace-dir ./workspace --log-level debug
```

### Development Mode (Vite Proxy)
```bash
# Start with Vite proxy
niten --workspace-dir ./workspace --dev-mode

# Custom Vite URL
niten --workspace-dir ./workspace --dev-mode --vite-dev-url http://localhost:3000
```

### Environment Variables
```bash
export WORKSPACE_DIR=/data/projects
export PORT=8080
export DEV_MODE=true
export LOG_LEVEL=debug
niten
```

### CLI Help
```bash
niten --help
```

## Dependencies
- **Backend 01**: Toolchain Alignment ✅

## Related Files
- `src/main.rs` - Entry point
- `src/server.rs` - HTTP server implementation
- `src/config.rs` - Configuration with CLI parsing
- `src/state.rs` - Application state
- `src/static_files.rs` - Static file embedding
- `src/error.rs` - Error types
- `Cargo.toml` - Dependencies (axum, tower-http, clap, rust-embed, reqwest)

## Priority
🟡 High - Required for MVP

## Estimated Complexity
Medium - 1 week

## Actual Implementation Time
Completed in single session
