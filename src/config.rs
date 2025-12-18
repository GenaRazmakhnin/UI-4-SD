//! Server configuration with CLI argument parsing.
//!
//! Provides configuration options for the backend server with support for:
//! - CLI arguments
//! - Environment variables
//! - Default values

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

/// NITEN FHIR Profile Builder Server
#[derive(Debug, Clone, Parser)]
#[command(name = "niten")]
#[command(about = "Backend server for FHIR Profile Builder UI")]
#[command(version)]
pub struct Config {
    /// Address to bind the server to
    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,

    /// Port to listen on
    #[arg(short, long, env = "PORT", default_value = "8080")]
    pub port: u16,

    /// Workspace directory for project storage
    /// Projects will be stored as <workspace_dir>/<project_id>/...
    #[arg(long, env = "WORKSPACE_DIR")]
    pub workspace_dir: PathBuf,

    /// Path to FHIR packages cache directory
    #[arg(long, env = "PACKAGES_CACHE_DIR")]
    pub packages_cache_dir: Option<PathBuf>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// Base path for reverse proxy support (e.g., "/niten")
    #[arg(long, env = "BASE_PATH")]
    pub base_path: Option<String>,

    /// CORS allowed origins (comma-separated, or "*" for all)
    /// In development mode, defaults to "*"
    #[arg(long, env = "CORS_ORIGINS")]
    pub cors_origins: Option<String>,

    /// Enable development mode
    /// - Proxies UI requests to Vite dev server
    /// - Enables permissive CORS
    #[arg(long, env = "DEV_MODE")]
    pub dev_mode: bool,

    /// Vite dev server URL (used when dev_mode is enabled)
    #[arg(long, env = "VITE_DEV_URL", default_value = "http://localhost:5173")]
    pub vite_dev_url: String,

    /// Request timeout in seconds
    #[arg(long, env = "REQUEST_TIMEOUT", default_value = "30")]
    pub request_timeout: u64,

    /// Shutdown timeout in seconds (time to wait for requests to complete)
    #[arg(long, env = "SHUTDOWN_TIMEOUT", default_value = "10")]
    pub shutdown_timeout: u64,
}

impl Config {
    /// Parse configuration from CLI arguments and environment variables.
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Create configuration from environment variables only (legacy support).
    pub fn from_env() -> anyhow::Result<Self> {
        // Parse with empty args to use only env vars and defaults
        let config = Self::try_parse_from(std::iter::empty::<String>()).map_err(|e| {
            anyhow::anyhow!("Configuration error: {}", e)
        })?;

        // Validate workspace directory
        config.validate()?;

        Ok(config)
    }

    /// Get the socket address to bind to.
    #[must_use]
    pub fn bind_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid host:port combination")
    }

    /// Validate configuration.
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate workspace directory exists or can be created
        if !self.workspace_dir.exists() {
            std::fs::create_dir_all(&self.workspace_dir).map_err(|e| {
                anyhow::anyhow!(
                    "Cannot create workspace directory '{}': {}",
                    self.workspace_dir.display(),
                    e
                )
            })?;
            tracing::info!(
                "Created workspace directory: {}",
                self.workspace_dir.display()
            );
        }

        // Verify the directory is writable
        let test_file = self.workspace_dir.join(".niten-write-test");
        std::fs::write(&test_file, "test").map_err(|e| {
            anyhow::anyhow!(
                "Workspace directory '{}' is not writable: {}",
                self.workspace_dir.display(),
                e
            )
        })?;
        let _ = std::fs::remove_file(&test_file);

        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.log_level.to_lowercase().as_str()) {
            anyhow::bail!(
                "Invalid log level '{}'. Valid levels: {:?}",
                self.log_level,
                valid_levels
            );
        }

        // Validate base path format
        if let Some(ref base_path) = self.base_path {
            if !base_path.starts_with('/') {
                anyhow::bail!("Base path must start with '/', got: '{}'", base_path);
            }
            if base_path.ends_with('/') && base_path.len() > 1 {
                anyhow::bail!("Base path must not end with '/', got: '{}'", base_path);
            }
        }

        Ok(())
    }

    /// Get CORS allowed origins as a vector.
    #[must_use]
    pub fn cors_origins_list(&self) -> Vec<String> {
        match &self.cors_origins {
            Some(origins) if origins == "*" => vec!["*".to_string()],
            Some(origins) => origins
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            None if self.dev_mode => vec!["*".to_string()],
            None => vec![],
        }
    }

    /// Get request timeout as Duration.
    #[must_use]
    pub fn request_timeout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.request_timeout)
    }

    /// Get shutdown timeout as Duration.
    #[must_use]
    pub fn shutdown_timeout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.shutdown_timeout)
    }

    /// Check if the server should proxy to Vite.
    #[must_use]
    pub fn should_proxy_to_vite(&self) -> bool {
        self.dev_mode
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3001,
            workspace_dir: PathBuf::from("./workspace"),
            packages_cache_dir: None,
            log_level: "info".to_string(),
            base_path: None,
            cors_origins: None,
            dev_mode: false,
            vite_dev_url: "http://localhost:5173".to_string(),
            request_timeout: 30,
            shutdown_timeout: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3001);
        assert!(!config.dev_mode);
    }

    #[test]
    fn test_bind_addr() {
        let config = Config {
            host: "0.0.0.0".to_string(),
            port: 8080,
            ..Default::default()
        };
        assert_eq!(config.bind_addr().to_string(), "0.0.0.0:8080");
    }

    #[test]
    fn test_cors_origins_list() {
        // No origins set, not dev mode
        let config = Config::default();
        assert!(config.cors_origins_list().is_empty());

        // No origins set, dev mode
        let config = Config {
            dev_mode: true,
            ..Default::default()
        };
        assert_eq!(config.cors_origins_list(), vec!["*"]);

        // Wildcard
        let config = Config {
            cors_origins: Some("*".to_string()),
            ..Default::default()
        };
        assert_eq!(config.cors_origins_list(), vec!["*"]);

        // Multiple origins
        let config = Config {
            cors_origins: Some("http://localhost:3000, https://example.com".to_string()),
            ..Default::default()
        };
        assert_eq!(
            config.cors_origins_list(),
            vec!["http://localhost:3000", "https://example.com"]
        );
    }

    #[test]
    fn test_timeout_durations() {
        let config = Config {
            request_timeout: 60,
            shutdown_timeout: 30,
            ..Default::default()
        };
        assert_eq!(
            config.request_timeout_duration(),
            std::time::Duration::from_secs(60)
        );
        assert_eq!(
            config.shutdown_timeout_duration(),
            std::time::Duration::from_secs(30)
        );
    }
}
