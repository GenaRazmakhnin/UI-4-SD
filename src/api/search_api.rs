//! Resource search API routes.
//!
//! Provides REST endpoints for searching FHIR resources across installed packages.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::state::AppState;

use super::packages_dto::{
    ExtensionDto, PackageErrorResponse, ResourceSearchQuery, SearchResponse, SearchResultDto,
    ValueSetDto,
};

/// Create resource search routes.
pub fn search_routes() -> Router<AppState> {
    Router::new()
        .route("/extensions", get(search_extensions))
        .route("/valuesets", get(search_valuesets))
        .route("/resources", get(search_resources))
}

/// GET /api/search/extensions?q=&package= - Search extensions.
async fn search_extensions(
    State(state): State<AppState>,
    Query(query): Query<ResourceSearchQuery>,
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

    // Build search query
    let mut builder = manager.search().await;

    if let Some(text) = &query.q {
        if !text.is_empty() {
            builder = builder.text(text);
        }
    }

    // Filter to StructureDefinition for extensions
    builder = builder.resource_type("StructureDefinition");

    if let Some(packages) = &query.package {
        for pkg in packages {
            builder = builder.package(pkg);
        }
    }

    builder = builder.limit(query.limit.unwrap_or(50));
    if let Some(offset) = query.offset {
        builder = builder.offset(offset);
    }

    match builder.execute().await {
        Ok(result) => {
            // Filter to only extensions (sd_flavor == "Extension")
            let extensions: Vec<ExtensionDto> = result
                .resources
                .into_iter()
                .filter(|r| r.resource.resource_type == "StructureDefinition")
                .filter(|r| {
                    // Check if it's an extension by looking at sd_flavor or derivation
                    r.resource
                        .content
                        .get("type")
                        .and_then(|v| v.as_str())
                        .map(|t| t == "Extension")
                        .unwrap_or(false)
                })
                .map(|r| {
                    let index = &r.index;
                    ExtensionDto {
                        id: index.id.clone().unwrap_or_default(),
                        url: index.canonical_url.clone(),
                        name: index.name.clone().unwrap_or_default(),
                        title: r
                            .resource
                            .content
                            .get("title")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        description: r
                            .resource
                            .content
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        package_name: index.package_name.clone(),
                        package_version: index.package_version.clone(),
                    }
                })
                .collect();

            Json(SearchResponse {
                results: extensions.clone(),
                total_count: extensions.len(),
            })
            .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PackageErrorResponse::install_failed(format!(
                "Search failed: {e}"
            ))),
        )
            .into_response(),
    }
}

/// GET /api/search/valuesets?q= - Search value sets.
async fn search_valuesets(
    State(state): State<AppState>,
    Query(query): Query<ResourceSearchQuery>,
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

    // Build search query
    let mut builder = manager.search().await;

    if let Some(text) = &query.q {
        if !text.is_empty() {
            builder = builder.text(text);
        }
    }

    builder = builder.resource_type("ValueSet");

    if let Some(packages) = &query.package {
        for pkg in packages {
            builder = builder.package(pkg);
        }
    }

    builder = builder.limit(query.limit.unwrap_or(50));
    if let Some(offset) = query.offset {
        builder = builder.offset(offset);
    }

    match builder.execute().await {
        Ok(result) => {
            let valuesets: Vec<ValueSetDto> = result
                .resources
                .into_iter()
                .map(|r| {
                    let index = &r.index;
                    ValueSetDto {
                        id: index.id.clone().unwrap_or_default(),
                        url: index.canonical_url.clone(),
                        name: index.name.clone().unwrap_or_default(),
                        title: r
                            .resource
                            .content
                            .get("title")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        description: r
                            .resource
                            .content
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        status: r
                            .resource
                            .content
                            .get("status")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        package_name: index.package_name.clone(),
                        package_version: index.package_version.clone(),
                    }
                })
                .collect();

            Json(SearchResponse {
                results: valuesets.clone(),
                total_count: valuesets.len(),
            })
            .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PackageErrorResponse::install_failed(format!(
                "Search failed: {e}"
            ))),
        )
            .into_response(),
    }
}

/// GET /api/search/resources?q=&type=&package= - Generic resource search.
async fn search_resources(
    State(state): State<AppState>,
    Query(query): Query<ResourceSearchQuery>,
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

    // Build search query
    let mut builder = manager.search().await;

    if let Some(text) = &query.q {
        if !text.is_empty() {
            builder = builder.text(text);
        }
    }

    if let Some(types) = &query.resource_type {
        for t in types {
            builder = builder.resource_type(t);
        }
    }

    if let Some(packages) = &query.package {
        for pkg in packages {
            builder = builder.package(pkg);
        }
    }

    builder = builder.limit(query.limit.unwrap_or(50));
    if let Some(offset) = query.offset {
        builder = builder.offset(offset);
    }

    match builder.execute().await {
        Ok(result) => {
            let total = result.total_count;
            let resources: Vec<SearchResultDto> = result
                .resources
                .into_iter()
                .map(|r| {
                    let index = &r.index;
                    SearchResultDto {
                        id: index.id.clone().unwrap_or_default(),
                        url: index.canonical_url.clone(),
                        name: index.name.clone().unwrap_or_default(),
                        resource_type: index.resource_type.clone(),
                        title: r
                            .resource
                            .content
                            .get("title")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        description: r
                            .resource
                            .content
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        package_name: index.package_name.clone(),
                        package_version: index.package_version.clone(),
                        score: Some(r.score),
                    }
                })
                .collect();

            Json(SearchResponse {
                results: resources,
                total_count: total,
            })
            .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PackageErrorResponse::install_failed(format!(
                "Search failed: {e}"
            ))),
        )
            .into_response(),
    }
}
