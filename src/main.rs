//! NITEN Backend Server Entry Point
//!
//! Starts the FHIR Profile Builder backend server.

use niten::{Config, Server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse configuration from CLI args and environment
    let config = Config::parse_args();

    // Initialize tracing with configured log level
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting NITEN FHIR Profile Builder backend");
    tracing::debug!("Configuration: {:?}", config);

    // Validate configuration
    config.validate()?;

    // Create and run server
    let server = Server::new(config).await?;
    server.run().await
}
