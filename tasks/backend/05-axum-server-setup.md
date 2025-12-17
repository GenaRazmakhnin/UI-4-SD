# Task: Axum HTTP Server Setup

## Description
Create the HTTP server using Axum that will serve the React UI and provide REST API endpoints for profile operations. The server should support embedded UI deployment.

## Requirements

### R1: Server Crate Setup
- Create `crates/server` with Axum dependencies
- Configure Tokio async runtime
- Set up basic HTTP server with configurable port
- Support graceful shutdown

### R2: Routing Structure
Create organized route handlers:
- Static file serving (embedded React UI)
- API routes under `/api/*`
- Health check endpoint
- 404 handling for SPA routing

### R3: Application State
- `AppState`: Shared state across handlers
  - Profile builder engine instance
  - Package manager instance
  - Configuration
  - Logging/tracing

### R4: Middleware Setup
- CORS configuration for development
- Request logging (tracing)
- Error handling middleware
- Request timeout middleware

### R5: Static Asset Embedding
- Use `rust-embed` or similar to embed React build
- Serve index.html for SPA routes
- Serve static assets (JS, CSS, images)
- Proper content-type headers

### R6: Configuration
- Port configuration (env var, CLI arg, default)
- CORS allowed origins (development vs production)
- Log level configuration
- Base path configuration (for reverse proxy support)

### R7: Error Handling
- Consistent error response format (JSON)
- HTTP status codes mapped to error types
- User-friendly error messages
- Detailed error logging

### R8: Development Support
- Auto-reload on code changes (cargo-watch)
- Proxy to Vite dev server during UI development
- Environment-based configuration

## Acceptance Criteria

- [ ] Server starts and listens on configured port
- [ ] `/health` endpoint returns 200 OK
- [ ] Embedded static files are served correctly
- [ ] SPA routing works (all routes serve index.html)
- [ ] CORS headers are set correctly for dev mode
- [ ] Request logging shows all incoming requests
- [ ] Graceful shutdown on SIGTERM/SIGINT
- [ ] Error responses are JSON formatted
- [ ] 404 errors are handled appropriately
- [ ] Server can run in production mode (embedded UI)
- [ ] Server can proxy to Vite in dev mode
- [ ] Configuration via environment variables works
- [ ] Documentation for running server

## Dependencies
- **Backend 01**: Toolchain Alignment

## Related Files
- `crates/server/Cargo.toml` (new)
- `crates/server/src/main.rs` (new)
- `crates/server/src/server.rs` (new)
- `crates/server/src/state.rs` (new)
- `crates/server/src/routes/mod.rs` (new)
- `crates/server/src/routes/static_files.rs` (new)
- `crates/server/src/middleware/mod.rs` (new)
- `crates/server/src/config.rs` (new)

## Priority
🟡 High - Required for MVP

## Estimated Complexity
Medium - 1 week
