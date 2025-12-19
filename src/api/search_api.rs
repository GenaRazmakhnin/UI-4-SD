//! Resource search API routes.
//!
//! Provides REST endpoints for searching FHIR resources across installed packages.

use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::state::AppState;

use super::packages_dto::{
    BaseResourceDto, BaseResourceSearchQuery, ElementDto, ElementSearchQuery, ExtensionContextDto,
    ExtensionDto, ExtensionSearchQuery, FacetsDto, PackageErrorResponse, ProfileDto,
    ProfileSearchQuery, ResourceSearchQuery, SearchResponseWithFacets, SearchResultDto,
    ValueSetDto, ValueSetSearchQuery,
};

/// Create resource search routes.
pub fn search_routes() -> Router<AppState> {
    Router::new()
        .route("/extensions", get(search_extensions))
        .route("/valuesets", get(search_valuesets))
        .route("/resources", get(search_resources))
        .route("/profiles", get(search_profiles))
        .route("/elements", get(search_elements))
        .route("/base-resources", get(search_base_resources))
}

/// Extract extension contexts from a StructureDefinition.
fn extract_extension_contexts(content: &serde_json::Value) -> Vec<ExtensionContextDto> {
    let mut contexts = Vec::new();

    if let Some(context_arr) = content.get("context").and_then(|v| v.as_array()) {
        for ctx in context_arr {
            if let (Some(ctx_type), Some(expression)) = (
                ctx.get("type").and_then(|v| v.as_str()),
                ctx.get("expression").and_then(|v| v.as_str()),
            ) {
                contexts.push(ExtensionContextDto {
                    context_type: ctx_type.to_lowercase(),
                    expression: expression.to_string(),
                });
            }
        }
    }

    contexts
}

/// Check if an extension matches the context filter.
fn matches_context_filter(
    contexts: &[ExtensionContextDto],
    context_type: Option<&str>,
    context_path: Option<&str>,
) -> bool {
    if context_type.is_none() && context_path.is_none() {
        return true;
    }

    for ctx in contexts {
        let type_matches = context_type
            .map(|t| ctx.context_type.eq_ignore_ascii_case(t))
            .unwrap_or(true);

        let path_matches = context_path
            .map(|p| {
                ctx.expression == p
                    || ctx.expression.starts_with(&format!("{}.", p))
                    || ctx.expression.ends_with(&format!(".{}", p))
            })
            .unwrap_or(true);

        if type_matches && path_matches {
            return true;
        }
    }

    false
}

/// GET /api/search/extensions - Search extensions with context filtering.
async fn search_extensions(
    State(state): State<AppState>,
    Query(query): Query<ExtensionSearchQuery>,
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

    // Request more results since we'll filter further
    builder = builder.limit(query.limit.unwrap_or(50) * 3);
    if let Some(offset) = query.offset {
        builder = builder.offset(offset);
    }

    match builder.execute().await {
        Ok(result) => {
            let limit = query.limit.unwrap_or(50);

            // Filter to only extensions with context matching
            let extensions: Vec<ExtensionDto> = result
                .resources
                .into_iter()
                .filter(|r| r.resource.resource_type == "StructureDefinition")
                .filter(|r| {
                    r.resource
                        .content
                        .get("type")
                        .and_then(|v| v.as_str())
                        .map(|t| t == "Extension")
                        .unwrap_or(false)
                })
                .filter_map(|r| {
                    let contexts = extract_extension_contexts(&r.resource.content);

                    // Apply context filters
                    if !matches_context_filter(
                        &contexts,
                        query.context.as_deref(),
                        query.context_path.as_deref(),
                    ) {
                        return None;
                    }

                    // Apply FHIR version filter if specified
                    if let Some(ref fhir_ver) = query.fhir_version {
                        let res_fhir_ver = r
                            .resource
                            .content
                            .get("fhirVersion")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if !res_fhir_ver.starts_with(fhir_ver) {
                            return None;
                        }
                    }

                    let index = &r.index;
                    Some(ExtensionDto {
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
                        contexts,
                        score: Some(r.score),
                    })
                })
                .take(limit)
                .collect();

            // Build facets
            let mut facets = FacetsDto::default();
            for ext in &extensions {
                *facets
                    .packages
                    .entry(ext.package_name.clone())
                    .or_insert(0) += 1;
            }

            Json(SearchResponseWithFacets {
                total_count: extensions.len(),
                results: extensions,
                facets: Some(facets),
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

/// GET /api/search/valuesets - Search value sets with system filtering.
async fn search_valuesets(
    State(state): State<AppState>,
    Query(query): Query<ValueSetSearchQuery>,
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

    // Request more results for system filtering
    builder = builder.limit(query.limit.unwrap_or(50) * 2);
    if let Some(offset) = query.offset {
        builder = builder.offset(offset);
    }

    match builder.execute().await {
        Ok(result) => {
            let limit = query.limit.unwrap_or(50);

            let valuesets: Vec<ValueSetDto> = result
                .resources
                .into_iter()
                .filter_map(|r| {
                    // Apply system filter if specified
                    if let Some(ref system) = query.system {
                        let has_system = check_valueset_uses_system(&r.resource.content, system);
                        if !has_system {
                            return None;
                        }
                    }

                    // Apply FHIR version filter
                    if let Some(ref fhir_ver) = query.fhir_version {
                        let res_fhir_ver = r
                            .resource
                            .content
                            .get("fhirVersion")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if !res_fhir_ver.starts_with(fhir_ver) {
                            return None;
                        }
                    }

                    let index = &r.index;
                    Some(ValueSetDto {
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
                        score: Some(r.score),
                    })
                })
                .take(limit)
                .collect();

            // Build facets
            let mut facets = FacetsDto::default();
            for vs in &valuesets {
                *facets
                    .packages
                    .entry(vs.package_name.clone())
                    .or_insert(0) += 1;
            }

            Json(SearchResponseWithFacets {
                total_count: valuesets.len(),
                results: valuesets,
                facets: Some(facets),
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

/// Check if a ValueSet uses a specific code system.
fn check_valueset_uses_system(content: &serde_json::Value, system: &str) -> bool {
    // Check compose.include
    if let Some(compose) = content.get("compose") {
        if let Some(includes) = compose.get("include").and_then(|v| v.as_array()) {
            for include in includes {
                if let Some(sys) = include.get("system").and_then(|v| v.as_str()) {
                    if sys == system || sys.contains(system) {
                        return true;
                    }
                }
            }
        }
    }

    // Check expansion.contains
    if let Some(expansion) = content.get("expansion") {
        if let Some(contains) = expansion.get("contains").and_then(|v| v.as_array()) {
            for concept in contains {
                if let Some(sys) = concept.get("system").and_then(|v| v.as_str()) {
                    if sys == system || sys.contains(system) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// GET /api/search/profiles - Search StructureDefinition profiles.
async fn search_profiles(
    State(state): State<AppState>,
    Query(query): Query<ProfileSearchQuery>,
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

    builder = builder.resource_type("StructureDefinition");

    if let Some(packages) = &query.package {
        for pkg in packages {
            builder = builder.package(pkg);
        }
    }

    // Request more results for filtering
    builder = builder.limit(query.limit.unwrap_or(50) * 3);
    if let Some(offset) = query.offset {
        builder = builder.offset(offset);
    }

    match builder.execute().await {
        Ok(result) => {
            let limit = query.limit.unwrap_or(50);

            let profiles: Vec<ProfileDto> = result
                .resources
                .into_iter()
                .filter_map(|r| {
                    let content = &r.resource.content;

                    // Must be a resource profile (kind = "resource" and have derivation)
                    let kind = content.get("kind").and_then(|v| v.as_str()).unwrap_or("");
                    if kind != "resource" {
                        return None;
                    }

                    // Get derivation (must have one to be a profile)
                    let derivation = content
                        .get("derivation")
                        .and_then(|v| v.as_str())
                        .map(String::from);

                    if derivation.is_none() {
                        return None;
                    }

                    // Skip extensions
                    let type_val = content.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    if type_val == "Extension" {
                        return None;
                    }

                    // Apply base type filter
                    if let Some(ref base_type) = query.base_type {
                        if type_val != base_type {
                            return None;
                        }
                    }

                    // Apply derivation filter
                    if let Some(ref deriv_filter) = query.derivation {
                        if derivation.as_deref() != Some(deriv_filter.as_str()) {
                            return None;
                        }
                    }

                    // Apply FHIR version filter
                    if let Some(ref fhir_ver) = query.fhir_version {
                        let res_fhir_ver = content
                            .get("fhirVersion")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if !res_fhir_ver.starts_with(fhir_ver) {
                            return None;
                        }
                    }

                    let index = &r.index;
                    Some(ProfileDto {
                        id: index.id.clone().unwrap_or_default(),
                        url: index.canonical_url.clone(),
                        name: index.name.clone().unwrap_or_default(),
                        title: content
                            .get("title")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        description: content
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        base_type: type_val.to_string(),
                        derivation,
                        package_name: index.package_name.clone(),
                        package_version: index.package_version.clone(),
                        score: Some(r.score),
                    })
                })
                .take(limit)
                .collect();

            // Build facets
            let mut facets = FacetsDto::default();
            for profile in &profiles {
                *facets
                    .resource_types
                    .entry(profile.base_type.clone())
                    .or_insert(0) += 1;
                *facets
                    .packages
                    .entry(profile.package_name.clone())
                    .or_insert(0) += 1;
            }

            Json(SearchResponseWithFacets {
                total_count: profiles.len(),
                results: profiles,
                facets: Some(facets),
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

/// GET /api/search/elements - Search elements within a profile.
async fn search_elements(
    State(state): State<AppState>,
    Query(query): Query<ElementSearchQuery>,
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

    // Get the profile by URL or ID
    let profile = match manager.resolve(&query.profile_id).await {
        Ok(p) => p,
        Err(_) => {
            // Try by ID pattern with full URL
            match manager
                .resolve(&format!(
                    "http://hl7.org/fhir/StructureDefinition/{}",
                    query.profile_id
                ))
                .await
            {
                Ok(p) => p,
                Err(_) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(PackageErrorResponse::not_found(format!(
                            "Profile not found: {}",
                            query.profile_id
                        ))),
                    )
                        .into_response();
                }
            }
        }
    };

    // Extract elements from snapshot or differential
    let content = &profile.resource.content;
    let elements = extract_elements(content, query.q.as_deref(), query.limit.unwrap_or(100));

    Json(SearchResponseWithFacets {
        total_count: elements.len(),
        results: elements,
        facets: None,
    })
    .into_response()
}

/// Extract elements from a StructureDefinition.
fn extract_elements(
    content: &serde_json::Value,
    query: Option<&str>,
    limit: usize,
) -> Vec<ElementDto> {
    let mut elements = Vec::new();

    // Prefer snapshot, fall back to differential
    let element_source = content
        .get("snapshot")
        .and_then(|s| s.get("element"))
        .or_else(|| content.get("differential").and_then(|d| d.get("element")));

    if let Some(element_arr) = element_source.and_then(|e| e.as_array()) {
        let query_lower = query.map(|q| q.to_lowercase());

        for elem in element_arr {
            let path = elem.get("path").and_then(|v| v.as_str()).unwrap_or("");
            let short = elem.get("short").and_then(|v| v.as_str());
            let definition = elem.get("definition").and_then(|v| v.as_str());

            // Apply text filter if provided
            if let Some(ref q) = query_lower {
                let path_match = path.to_lowercase().contains(q);
                let short_match = short.map(|s| s.to_lowercase().contains(q)).unwrap_or(false);
                let def_match = definition
                    .map(|d| d.to_lowercase().contains(q))
                    .unwrap_or(false);

                if !path_match && !short_match && !def_match {
                    continue;
                }
            }

            // Extract types
            let types: Vec<String> = elem
                .get("type")
                .and_then(|t| t.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|t| t.get("code").and_then(|c| c.as_str()).map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            // Extract cardinality
            let min = elem.get("min").and_then(|v| v.as_u64()).map(|n| n as u32);
            let max = elem.get("max").and_then(|v| v.as_str()).map(String::from);

            elements.push(ElementDto {
                path: path.to_string(),
                short: short.map(String::from),
                definition: definition.map(String::from),
                types,
                min,
                max,
            });

            if elements.len() >= limit {
                break;
            }
        }
    }

    elements
}

/// GET /api/search/resources - Generic resource search with facets.
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

            // Build facet counts before filtering
            let mut type_counts: HashMap<String, usize> = HashMap::new();
            let mut package_counts: HashMap<String, usize> = HashMap::new();

            let resources: Vec<SearchResultDto> = result
                .resources
                .into_iter()
                .filter_map(|r| {
                    // Apply FHIR version filter
                    if let Some(ref fhir_ver) = query.fhir_version {
                        let res_fhir_ver = r
                            .resource
                            .content
                            .get("fhirVersion")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if !res_fhir_ver.starts_with(fhir_ver) {
                            return None;
                        }
                    }

                    let index = &r.index;

                    // Count for facets
                    *type_counts
                        .entry(index.resource_type.clone())
                        .or_insert(0) += 1;
                    *package_counts
                        .entry(index.package_name.clone())
                        .or_insert(0) += 1;

                    Some(SearchResultDto {
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
                    })
                })
                .collect();

            let facets = FacetsDto {
                resource_types: type_counts,
                packages: package_counts,
            };

            Json(SearchResponseWithFacets {
                results: resources,
                total_count: total,
                facets: Some(facets),
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

/// GET /api/search/base-resources - Get available base FHIR resource types for profile creation.
/// Uses fast index-only query without loading full resource content.
async fn search_base_resources(
    State(state): State<AppState>,
    Query(query): Query<BaseResourceSearchQuery>,
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

    // Determine the FHIR version to use
    let fhir_version = query.fhir_version.as_deref().unwrap_or("4.0.1");

    // Use fast index-only query to get base resource type names
    match manager.list_base_resource_type_names(fhir_version).await {
        Ok(type_names) => {
            let limit = query.limit.unwrap_or(200);
            let text_filter = query.q.as_ref().map(|q| q.to_lowercase());

            // Filter by text query if provided and build response
            let mut resources: Vec<BaseResourceDto> = type_names
                .into_iter()
                .filter(|name| {
                    // Skip Extension type
                    if name == "Extension" {
                        return false;
                    }
                    // Apply text filter if provided
                    if let Some(ref filter) = text_filter {
                        return name.to_lowercase().contains(filter);
                    }
                    true
                })
                .map(|name| {
                    // Build canonical URL from type name
                    let url = format!("http://hl7.org/fhir/StructureDefinition/{}", name);
                    BaseResourceDto {
                        name: name.clone(),
                        url,
                        title: Some(name.clone()), // Use name as title for base resources
                        description: None,         // Skip description for fast response
                        package_name: format!("hl7.fhir.r4.core"), // Core package
                        package_version: fhir_version.to_string(),
                    }
                })
                .collect();

            // Sort alphabetically
            resources.sort_by(|a, b| a.name.cmp(&b.name));
            resources.truncate(limit);

            Json(SearchResponseWithFacets {
                total_count: resources.len(),
                results: resources,
                facets: None,
            })
            .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PackageErrorResponse::install_failed(format!(
                "Failed to list base resources: {e}"
            ))),
        )
            .into_response(),
    }
}
