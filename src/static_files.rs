//! Static file serving with embedded assets.
//!
//! Embeds the React build output for production deployment
//! and provides SPA-friendly routing.

use axum::{
    body::Body,
    extract::Path,
    http::{header, Response, StatusCode, Uri},
    response::IntoResponse,
};
use rust_embed::Embed;

/// Embedded static files from the React build.
/// The path points to the web build output directory.
#[derive(Embed)]
#[folder = "web/dist"]
#[prefix = ""]
struct Assets;

/// Serve a static file from embedded assets.
pub async fn serve_static(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    serve_file(path).await
}

/// Serve a static file by path.
pub async fn serve_static_path(Path(path): Path<String>) -> impl IntoResponse {
    serve_file(&path).await
}

/// Serve a file from embedded assets, with SPA fallback.
async fn serve_file(path: &str) -> Response<Body> {
    // Try to serve the exact file
    if let Some(content) = Assets::get(path) {
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime)
            .header(header::CACHE_CONTROL, cache_control_for_path(path))
            .body(Body::from(content.data.into_owned()))
            .unwrap();
    }

    // For non-file paths, serve index.html (SPA routing)
    serve_index().await
}

/// Serve the index.html file for SPA routing.
pub async fn serve_index() -> Response<Body> {
    match Assets::get("index.html") {
        Some(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .header(header::CACHE_CONTROL, "no-cache")
            .body(Body::from(content.data.into_owned()))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from(not_found_html()))
            .unwrap(),
    }
}

/// Determine cache control headers based on file path.
fn cache_control_for_path(path: &str) -> &'static str {
    // Immutable assets (hashed filenames) can be cached forever
    if path.contains("/assets/") || path.ends_with(".js") || path.ends_with(".css") {
        if path.contains('.') {
            let parts: Vec<&str> = path.split('.').collect();
            // Files with hash in name like "main.abc123.js" are immutable
            if parts.len() >= 3 {
                return "public, max-age=31536000, immutable";
            }
        }
        // Regular assets - cache for 1 day
        "public, max-age=86400"
    } else if path.ends_with(".html") {
        // HTML files should not be cached
        "no-cache"
    } else if path.starts_with("static/") {
        // Static assets - cache for 1 week
        "public, max-age=604800"
    } else {
        // Default - cache for 1 hour
        "public, max-age=3600"
    }
}

/// Generate a simple 404 HTML page.
fn not_found_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Not Found</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            background: #f5f5f5;
        }
        .container {
            text-align: center;
            padding: 2rem;
        }
        h1 {
            font-size: 4rem;
            color: #333;
            margin: 0;
        }
        p {
            color: #666;
            font-size: 1.2rem;
        }
        a {
            color: #0066cc;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <p>The page you're looking for doesn't exist.</p>
        <p><a href="/">Go back home</a></p>
    </div>
</body>
</html>"#.to_string()
}

/// Check if embedded assets are available.
#[must_use]
pub fn has_embedded_assets() -> bool {
    Assets::get("index.html").is_some()
}

/// Get list of all embedded file paths.
#[must_use]
pub fn list_embedded_files() -> Vec<String> {
    Assets::iter().map(|f| f.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_control_for_path() {
        // Hashed assets
        assert_eq!(
            cache_control_for_path("assets/main.abc123.js"),
            "public, max-age=31536000, immutable"
        );

        // HTML files
        assert_eq!(cache_control_for_path("index.html"), "no-cache");

        // Static files
        assert_eq!(
            cache_control_for_path("static/images/logo.png"),
            "public, max-age=604800"
        );

        // Default
        assert_eq!(
            cache_control_for_path("manifest.json"),
            "public, max-age=3600"
        );
    }

    #[test]
    fn test_not_found_html() {
        let html = not_found_html();
        assert!(html.contains("404"));
        assert!(html.contains("Not Found"));
    }
}
