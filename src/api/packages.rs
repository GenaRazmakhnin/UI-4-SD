//! Package management API routes.
//!
//! Provides REST endpoints for managing FHIR packages through the canonical manager.

use std::convert::Infallible;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use octofhir_canonical_manager::registry::DownloadProgress;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::state::AppState;

use super::packages_dto::{
    parse_package_id, InstallProgressEvent, PackageDto, PackageErrorResponse,
    PackageResourceCountsDto, PackageSearchQuery, PackageSearchResultDto,
};

/// Create package management routes.
pub fn package_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_packages))
        .route("/search", get(search_packages))
        .route("/{packageId}/install", post(install_package))
        .route("/{packageId}/uninstall", post(uninstall_package))
}

/// GET /api/packages - List installed packages.
async fn list_packages(State(state): State<AppState>) -> Response {
    let manager = match state.canonical_manager().await {
        Ok(m) => m,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PackageErrorResponse::install_failed(format!(
                    "Failed to initialize package manager: {e}"
                ))),
            )
                .into_response();
        }
    };

    match manager.storage().list_packages().await {
        Ok(packages) => {
            let dtos: Vec<PackageDto> = packages
                .into_iter()
                .map(|info| {
                    PackageDto {
                        id: format!("{}@{}", info.name, info.version),
                        name: info.name,
                        version: info.version,
                        description: None, // Not available from storage
                        fhir_version: "4.0.1".to_string(), // Default, could be improved
                        installed: true,
                        installed_at: Some(info.installed_at),
                        resource_counts: Some(PackageResourceCountsDto {
                            profiles: 0,     // Would need additional query
                            extensions: 0,   // Would need additional query
                            value_sets: 0,   // Would need additional query
                            code_systems: 0, // Would need additional query
                            search_parameters: 0,
                            total: info.resource_count as u32,
                        }),
                    }
                })
                .collect();

            Json(dtos).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PackageErrorResponse::install_failed(format!(
                "Failed to list packages: {e}"
            ))),
        )
            .into_response(),
    }
}

/// GET /api/packages/search?q=... - Search registry for packages.
async fn search_packages(
    State(_state): State<AppState>,
    Query(query): Query<PackageSearchQuery>,
) -> Response {
    // Note: The canonical manager doesn't expose a direct registry search API.
    // For now, return an empty list. This could be enhanced by:
    // 1. Adding a search_registry method to canonical manager
    // 2. Directly calling the registry API
    tracing::info!("Package search query: {}", query.q);

    let results: Vec<PackageSearchResultDto> = vec![];
    Json(results).into_response()
}

/// POST /api/packages/:packageId/install - Install package with SSE progress.
async fn install_package(
    State(state): State<AppState>,
    Path(package_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel::<InstallProgressEvent>(100);
    let (name, version) = parse_package_id(&package_id);

    // Spawn installation task
    tokio::spawn(async move {
        // Send start event
        let _ = tx
            .send(InstallProgressEvent::Start {
                package_id: package_id.clone(),
                total_bytes: None,
            })
            .await;

        // Get the canonical manager
        let manager = match state.canonical_manager().await {
            Ok(m) => m,
            Err(e) => {
                let _ = tx
                    .send(InstallProgressEvent::Error {
                        package_id,
                        message: format!("Failed to initialize package manager: {e}"),
                        code: "INIT_FAILED".to_string(),
                    })
                    .await;
                return;
            }
        };

        // Create progress reporter
        let progress = SseProgressReporter::new(tx.clone(), package_id.clone());

        // Send progress event for download start
        progress.on_start(None);

        // Install the package
        match manager.install_package(&name, &version).await {
            Ok(()) => {
                // Send extracting event
                let _ = tx
                    .send(InstallProgressEvent::Extracting {
                        package_id: package_id.clone(),
                    })
                    .await;

                // Send indexing event
                let _ = tx
                    .send(InstallProgressEvent::Indexing {
                        package_id: package_id.clone(),
                    })
                    .await;

                // Get the installed package info
                match manager.storage().list_packages().await {
                    Ok(packages) => {
                        if let Some(pkg) =
                            packages.iter().find(|p| p.name == name && p.version == version)
                        {
                            let dto = PackageDto {
                                id: format!("{}@{}", pkg.name, pkg.version),
                                name: pkg.name.clone(),
                                version: pkg.version.clone(),
                                description: None,
                                fhir_version: "4.0.1".to_string(),
                                installed: true,
                                installed_at: Some(pkg.installed_at),
                                resource_counts: Some(PackageResourceCountsDto {
                                    profiles: 0,
                                    extensions: 0,
                                    value_sets: 0,
                                    code_systems: 0,
                                    search_parameters: 0,
                                    total: pkg.resource_count as u32,
                                }),
                            };
                            let _ = tx.send(InstallProgressEvent::Complete { package: dto }).await;
                        } else {
                            // Package installed but couldn't find in list (unlikely)
                            let dto = PackageDto {
                                id: package_id.clone(),
                                name: name.clone(),
                                version: version.clone(),
                                description: None,
                                fhir_version: "4.0.1".to_string(),
                                installed: true,
                                installed_at: None,
                                resource_counts: None,
                            };
                            let _ = tx.send(InstallProgressEvent::Complete { package: dto }).await;
                        }
                    }
                    Err(e) => {
                        // Package was installed but we couldn't get info
                        tracing::warn!("Failed to get package info after install: {e}");
                        let dto = PackageDto {
                            id: package_id.clone(),
                            name: name.clone(),
                            version: version.clone(),
                            description: None,
                            fhir_version: "4.0.1".to_string(),
                            installed: true,
                            installed_at: None,
                            resource_counts: None,
                        };
                        let _ = tx.send(InstallProgressEvent::Complete { package: dto }).await;
                    }
                }
            }
            Err(e) => {
                let _ = tx
                    .send(InstallProgressEvent::Error {
                        package_id,
                        message: e.to_string(),
                        code: "INSTALL_FAILED".to_string(),
                    })
                    .await;
            }
        }
    });

    // Convert channel to SSE stream
    let stream = ReceiverStream::new(rx).map(|event| {
        let json = serde_json::to_string(&event).unwrap_or_else(|_| "{}".to_string());
        Ok(Event::default().data(json))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// POST /api/packages/:packageId/uninstall - Uninstall a package.
async fn uninstall_package(
    State(state): State<AppState>,
    Path(package_id): Path<String>,
) -> Response {
    let manager = match state.canonical_manager().await {
        Ok(m) => m,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PackageErrorResponse::install_failed(format!(
                    "Failed to initialize package manager: {e}"
                ))),
            )
                .into_response();
        }
    };

    let (name, version) = parse_package_id(&package_id);

    match manager.remove_package(&name, &version).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PackageErrorResponse::install_failed(format!(
                "Failed to uninstall package: {e}"
            ))),
        )
            .into_response(),
    }
}

/// Progress reporter that sends SSE events.
struct SseProgressReporter {
    tx: mpsc::Sender<InstallProgressEvent>,
    package_id: String,
}

impl SseProgressReporter {
    fn new(tx: mpsc::Sender<InstallProgressEvent>, package_id: String) -> Self {
        Self { tx, package_id }
    }
}

impl DownloadProgress for SseProgressReporter {
    fn on_progress(&self, downloaded: u64, total: Option<u64>) {
        let percentage = total.map(|t| ((downloaded * 100) / t) as u8).unwrap_or(0);
        let _ = self.tx.try_send(InstallProgressEvent::Progress {
            package_id: self.package_id.clone(),
            downloaded_bytes: downloaded,
            total_bytes: total,
            percentage,
        });
    }

    fn on_start(&self, total: Option<u64>) {
        let _ = self.tx.try_send(InstallProgressEvent::Start {
            package_id: self.package_id.clone(),
            total_bytes: total,
        });
    }

    fn on_complete(&self) {
        let _ = self.tx.try_send(InstallProgressEvent::Extracting {
            package_id: self.package_id.clone(),
        });
    }
}

// Need to implement StreamExt for the stream
use futures::StreamExt;
