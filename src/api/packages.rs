//! Package management API routes.
//!
//! Provides REST endpoints for managing FHIR packages through the canonical manager.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

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
use chrono::Utc;
use futures::stream::Stream;
use octofhir_canonical_manager::registry::DownloadProgress;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

use crate::state::AppState;

use super::packages_dto::{
    parse_package_id, InstallJobDto, InstallJobStatus, InstallProgressEvent, PackageDetailsDto,
    PackageDto, PackageErrorResponse, PackageResourceCountsDto, PackageSearchQuery,
    PackageSearchResultDto,
};

/// In-memory store for install jobs (for polling).
type InstallJobs = Arc<RwLock<HashMap<String, InstallJobDto>>>;

/// Create package management routes.
pub fn package_routes() -> Router<AppState> {
    // Create shared job store
    let jobs: InstallJobs = Arc::new(RwLock::new(HashMap::new()));

    Router::new()
        .route("/", get(list_packages))
        .route("/search", get(search_packages))
        .route("/{packageId}", get(get_package_details))
        .route("/{packageId}/install", post(install_package))
        .route("/{packageId}/install/start", post(start_install_job))
        .route("/{packageId}/uninstall", post(uninstall_package))
        .route("/jobs/{jobId}", get(get_install_job))
        .layer(axum::Extension(jobs))
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
///
/// Searches packages.fhir.org catalog and returns name + version.
/// Installation uses fs.get-ig.org via the canonical manager.
async fn search_packages(
    State(state): State<AppState>,
    Query(query): Query<PackageSearchQuery>,
) -> Response {
    tracing::info!(
        "Package search: q={}, fhir_version={:?}",
        query.q,
        query.fhir_version
    );

    // Get installed packages to check installed status
    let installed_packages: HashMap<String, String> =
        match state.canonical_manager().await {
            Ok(manager) => match manager.storage().list_packages().await {
                Ok(pkgs) => pkgs
                    .into_iter()
                    .map(|p| (p.name.clone(), p.version.clone()))
                    .collect(),
                Err(_) => HashMap::new(),
            },
            Err(_) => HashMap::new(),
        };

    let catalog = state.registry_catalog();
    match catalog.search(&query.q).await {
        Ok(packages) => {
            let limit = query.limit.unwrap_or(50);

            let results: Vec<PackageSearchResultDto> = packages
                .into_iter()
                .filter_map(|pkg| {
                    // Get FHIR version from API (e.g., "R4", "STU3", "4.0.1")
                    let fhir_version = pkg.fhir_version.clone().unwrap_or_else(|| "R4".to_string());

                    // Filter by FHIR version if specified
                    if let Some(ref filter_version) = query.fhir_version {
                        let matches = matches_fhir_version(&fhir_version, filter_version);
                        if !matches {
                            return None;
                        }
                    }

                    // Use version from catalog if available, otherwise "latest"
                    // The canonical manager will resolve "latest" to actual version via fs.get-ig.org
                    let version = pkg.version.clone().unwrap_or_else(|| "latest".to_string());
                    let name = pkg.name.clone();

                    // Check if installed
                    let installed_ver = installed_packages.get(&name);
                    let installed = installed_ver.is_some();
                    let installed_version = if installed && installed_ver != Some(&version) {
                        installed_ver.cloned()
                    } else {
                        None
                    };

                    Some(PackageSearchResultDto {
                        id: format!("{}@{}", name, version),
                        name,
                        version,
                        description: pkg.description,
                        fhir_version,
                        publisher: pkg.author,
                        installed,
                        installed_version,
                        download_count: None, // Not available from catalog API
                    })
                })
                .take(limit)
                .collect();

            Json(results).into_response()
        }
        Err(e) => {
            tracing::error!("Package search failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PackageErrorResponse::network_error(format!(
                    "Failed to search packages: {e}"
                ))),
            )
                .into_response()
        }
    }
}

/// Check if a package FHIR version matches the filter.
fn matches_fhir_version(pkg_version: &str, filter: &str) -> bool {
    let pkg_lower = pkg_version.to_lowercase();
    let filter_lower = filter.to_lowercase();

    // Exact match
    if pkg_lower == filter_lower {
        return true;
    }

    // Map common aliases
    let pkg_normalized = normalize_fhir_version(&pkg_lower);
    let filter_normalized = normalize_fhir_version(&filter_lower);

    pkg_normalized == filter_normalized
}

/// Normalize FHIR version to a standard form.
fn normalize_fhir_version(version: &str) -> &str {
    match version {
        "r4" | "4.0" | "4.0.0" | "4.0.1" => "r4",
        "r5" | "5.0" | "5.0.0" => "r5",
        "stu3" | "3.0" | "3.0.0" | "3.0.1" | "3.0.2" => "stu3",
        "r4b" | "4.1" | "4.1.0" | "4.3.0" => "r4b",
        "dstu2" | "1.0" | "1.0.0" | "1.0.2" => "dstu2",
        _ => version,
    }
}

/// GET /api/packages/{packageId} - Get package details.
async fn get_package_details(
    State(state): State<AppState>,
    Path(package_id): Path<String>,
) -> Response {
    let (name, version) = parse_package_id(&package_id);

    // Check if installed locally
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

    let installed_info = manager
        .storage()
        .list_packages()
        .await
        .ok()
        .and_then(|pkgs| pkgs.into_iter().find(|p| p.name == name));

    // Try to get additional info from registry search (search for exact package name)
    let catalog = state.registry_catalog();
    let registry_info = catalog.search(&name).await.ok().and_then(|results| {
        results.into_iter().find(|p| p.name == name)
    });

    // Build response
    let details = PackageDetailsDto {
        id: package_id.clone(),
        name: name.clone(),
        version: version.clone(),
        description: registry_info.as_ref().and_then(|r| r.description.clone()),
        fhir_version: registry_info
            .as_ref()
            .and_then(|r| r.fhir_version.clone())
            .or_else(|| installed_info.as_ref().map(|_| "4.0.1".to_string()))
            .unwrap_or_else(|| "R4".to_string()),
        publisher: registry_info.as_ref().and_then(|r| r.author.clone()),
        installed: installed_info.is_some(),
        installed_at: installed_info.as_ref().map(|i| i.installed_at),
        license: None,
        homepage: None,
        repository: None,
        canonical: None,
        resource_counts: installed_info.as_ref().map(|i| PackageResourceCountsDto {
            profiles: 0,
            extensions: 0,
            value_sets: 0,
            code_systems: 0,
            search_parameters: 0,
            total: i.resource_count as u32,
        }),
        dependencies: Vec::new(),
        versions: Vec::new(), // Would need separate API call for version history
    };

    Json(details).into_response()
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

/// POST /api/packages/:packageId/install/start - Start install job (polling-based).
async fn start_install_job(
    State(state): State<AppState>,
    Path(package_id): Path<String>,
    axum::Extension(jobs): axum::Extension<InstallJobs>,
) -> Response {
    let (name, version) = parse_package_id(&package_id);
    let job_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // Create initial job entry
    let job = InstallJobDto {
        job_id: job_id.clone(),
        package_id: package_id.clone(),
        status: InstallJobStatus::Pending,
        progress: 0,
        message: Some("Starting installation...".to_string()),
        downloaded_bytes: None,
        total_bytes: None,
        error: None,
        package: None,
        created_at: now,
        updated_at: now,
    };

    // Store job
    {
        let mut jobs_lock = jobs.write().await;
        jobs_lock.insert(job_id.clone(), job.clone());
    }

    // Spawn installation task
    let jobs_clone = jobs.clone();
    let job_id_clone = job_id.clone();

    tokio::spawn(async move {
        // Update to downloading status with initial progress
        {
            let mut jobs_lock = jobs_clone.write().await;
            if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                job.status = InstallJobStatus::Downloading;
                job.progress = 5;
                job.message = Some("Connecting to registry...".to_string());
                job.updated_at = Utc::now();
            }
        }

        // Get the canonical manager
        let manager = match state.canonical_manager().await {
            Ok(m) => m,
            Err(e) => {
                let mut jobs_lock = jobs_clone.write().await;
                if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                    job.status = InstallJobStatus::Failed;
                    job.error = Some(format!("Failed to initialize package manager: {e}"));
                    job.updated_at = Utc::now();
                }
                return;
            }
        };

        // Update progress: starting download
        {
            let mut jobs_lock = jobs_clone.write().await;
            if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                job.progress = 10;
                job.message = Some(format!("Downloading {}@{}...", name, version));
                job.updated_at = Utc::now();
            }
        }

        // Spawn a background task to simulate download progress
        // Since install_package doesn't provide progress callbacks, we simulate it
        let jobs_for_progress = jobs_clone.clone();
        let job_id_for_progress = job_id_clone.clone();
        let progress_task = tokio::spawn(async move {
            // Simulate progress updates during download (10% to 50%)
            for progress in (15..=50).step_by(5) {
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                let mut jobs_lock = jobs_for_progress.write().await;
                if let Some(job) = jobs_lock.get_mut(&job_id_for_progress) {
                    if job.status == InstallJobStatus::Downloading {
                        job.progress = progress as u8;
                        job.updated_at = Utc::now();
                    } else {
                        // Installation moved to next phase, stop progress simulation
                        break;
                    }
                }
            }
        });

        // Install the package (this downloads, extracts, and indexes via fs.get-ig.org)
        let install_result = manager.install_package(&name, &version).await;

        // Cancel progress simulation
        progress_task.abort();

        match install_result {
            Ok(()) => {
                // Update to extracting
                {
                    let mut jobs_lock = jobs_clone.write().await;
                    if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                        job.status = InstallJobStatus::Extracting;
                        job.progress = 60;
                        job.message = Some("Extracting package contents...".to_string());
                        job.updated_at = Utc::now();
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

                // Update progress during extraction
                {
                    let mut jobs_lock = jobs_clone.write().await;
                    if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                        job.progress = 70;
                        job.updated_at = Utc::now();
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

                // Update to indexing
                {
                    let mut jobs_lock = jobs_clone.write().await;
                    if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                        job.status = InstallJobStatus::Indexing;
                        job.progress = 80;
                        job.message = Some("Indexing resources...".to_string());
                        job.updated_at = Utc::now();
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

                // Update progress during indexing
                {
                    let mut jobs_lock = jobs_clone.write().await;
                    if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                        job.progress = 90;
                        job.updated_at = Utc::now();
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

                // Get installed package info
                let package_dto = match manager.storage().list_packages().await {
                    Ok(packages) => packages
                        .iter()
                        .find(|p| p.name == name && p.version == version)
                        .map(|pkg| PackageDto {
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
                        }),
                    Err(_) => None,
                };

                // Update to completed
                let mut jobs_lock = jobs_clone.write().await;
                if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                    job.status = InstallJobStatus::Completed;
                    job.progress = 100;
                    job.message = Some("Installation complete!".to_string());
                    job.package = package_dto;
                    job.updated_at = Utc::now();
                }
            }
            Err(e) => {
                let mut jobs_lock = jobs_clone.write().await;
                if let Some(job) = jobs_lock.get_mut(&job_id_clone) {
                    job.status = InstallJobStatus::Failed;
                    job.error = Some(e.to_string());
                    job.message = Some("Installation failed".to_string());
                    job.updated_at = Utc::now();
                }
            }
        }
    });

    // Return job info immediately
    Json(job).into_response()
}

/// GET /api/packages/jobs/{jobId} - Get install job status.
async fn get_install_job(
    Path(job_id): Path<String>,
    axum::Extension(jobs): axum::Extension<InstallJobs>,
) -> Response {
    let jobs_lock = jobs.read().await;

    match jobs_lock.get(&job_id) {
        Some(job) => Json(job.clone()).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(PackageErrorResponse::not_found(format!(
                "Install job {} not found",
                job_id
            ))),
        )
            .into_response(),
    }
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
