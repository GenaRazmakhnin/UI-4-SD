//! HTTP server implementation.
//!
//! Provides the main Axum-based HTTP server with:
//! - API routes for profile management
//! - Static file serving with embedded assets
//! - SPA routing fallback
//! - Graceful shutdown

use std::time::Duration;

use axum::{
    body::Body,
    extract::State,
    http::{header, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    api::{
        export_routes, history_routes, package_routes, profile_routes, project_export_routes,
        project_routes, search_routes, validation_routes,
    },
    state::AppState,
    static_files::{has_embedded_assets, serve_static},
    Config, Result,
};

/// Main server struct.
pub struct Server {
    config: Config,
    router: Router,
}

impl Server {
    /// Create a new server instance.
    pub async fn new(config: Config) -> Result<Self> {
        let state = AppState::new(config.clone(), config.workspace_dir.clone());
        let router = Self::build_router(&config, state).await;

        Ok(Self { config, router })
    }

    /// Build the API router with all routes and middleware.
    async fn build_router(config: &Config, state: AppState) -> Router {
        // Build CORS layer
        let cors = Self::build_cors_layer(config);

        // Build timeout layer (408 Request Timeout for timed out requests)
        let timeout = TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            config.request_timeout_duration(),
        );

        // API routes
        let api_routes = Router::new()
            .route("/status", get(status))
            // Project management routes (includes list)
            .nest("/projects", project_routes())
            // Profile routes with export, validation, and history
            .nest(
                "/projects/{projectId}/profiles",
                profile_routes()
                    .merge(export_routes())
                    .merge(validation_routes())
                    .merge(history_routes()),
            )
            .nest("/projects/{projectId}", project_export_routes())
            // Package management routes
            .nest("/packages", package_routes())
            // Resource search routes
            .nest("/search", search_routes())
            .with_state(state.clone());

        // Main router
        let mut router = Router::new()
            .route("/health", get(health_check))
            .nest("/api", api_routes);

        // Add static file serving
        if has_embedded_assets() {
            tracing::info!("Serving embedded static assets");
            router = router.fallback(spa_fallback);
        } else {
            tracing::warn!("No embedded assets found, UI will not be available");
            router = router.fallback(no_ui_handler);
        }

        // Apply middleware
        router
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(timeout)
                    .layer(cors),
            )
            .with_state(state)
    }

    /// Build CORS layer based on configuration.
    fn build_cors_layer(config: &Config) -> CorsLayer {
        let origins = config.cors_origins_list();

        let cors = CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ]);

        if origins.is_empty() || origins.iter().any(|o| o == "*") {
            cors.allow_origin(Any)
        } else {
            // Parse specific origins
            let parsed_origins: Vec<_> = origins
                .iter()
                .filter_map(|o| o.parse().ok())
                .collect();

            if parsed_origins.is_empty() {
                cors.allow_origin(Any)
            } else {
                cors.allow_origin(parsed_origins)
            }
        }
    }

    /// Run the server with graceful shutdown.
    pub async fn run(self) -> anyhow::Result<()> {
        let addr = self.config.bind_addr();
        let shutdown_timeout = self.config.shutdown_timeout_duration();

        tracing::info!("Server listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;

        // Run server with graceful shutdown
        axum::serve(listener, self.router)
            .with_graceful_shutdown(shutdown_signal(shutdown_timeout))
            .await?;

        tracing::info!("Server shutdown complete");
        Ok(())
    }
}

/// Wait for shutdown signal (SIGTERM, SIGINT, or Ctrl+C).
async fn shutdown_signal(timeout: Duration) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            tracing::info!("Received Ctrl+C, initiating graceful shutdown");
        }
        () = terminate => {
            tracing::info!("Received SIGTERM, initiating graceful shutdown");
        }
    }

    // Allow time for in-flight requests to complete
    tracing::info!(
        "Waiting up to {} seconds for requests to complete...",
        timeout.as_secs()
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
}

// --- Route Handlers ---

/// Health check endpoint.
async fn health_check() -> &'static str {
    "OK"
}

/// Status endpoint returning server information.
async fn status(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "niten-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "status": "running",
        "uptime_seconds": state.uptime_seconds(),
        "active_sessions": state.active_sessions().len()
    }))
}

/// SPA fallback handler - serves index.html for unknown routes.
async fn spa_fallback(uri: Uri) -> Response<Body> {
    let path = uri.path();

    // API requests that don't match should 404
    if path.starts_with("/api/") {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"error":"Not found","status":404}"#))
            .unwrap();
    }

    // Try to serve static file first
    serve_static(uri).await.into_response()
}

/// Handler when no UI is available.
async fn no_ui_handler(uri: Uri) -> Response<Body> {
    let path = uri.path();

    if path.starts_with("/api/") {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"error":"Not found","status":404}"#))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(
            r#"<!DOCTYPE html>
<html>
<head><title>UI Not Available</title></head>
<body style="font-family: sans-serif; padding: 2rem; text-align: center;">
<h1>UI Not Available</h1>
<p>The embedded UI assets are not available.</p>
<p>Build the UI with <code>cd web && bun run build</code></p>
<p>API endpoints are still available at <code>/api/*</code></p>
</body>
</html>"#,
        ))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let result = health_check().await;
        assert_eq!(result, "OK");
    }

    #[test]
    fn test_build_cors_layer_permissive() {
        let config = Config {
            cors_origins: Some("*".to_string()),
            ..Default::default()
        };
        // Should not panic
        let _ = Server::build_cors_layer(&config);
    }

    #[test]
    fn test_build_cors_layer_specific_origins() {
        let config = Config {
            cors_origins: Some("http://localhost:3000".to_string()),
            ..Default::default()
        };
        // Should not panic
        let _ = Server::build_cors_layer(&config);
    }
}
