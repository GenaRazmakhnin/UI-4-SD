//! Profile merge helpers for differential-based IR.
//!
//! Ensures a loaded document has a merged element tree for UI/export.

use axum::http::StatusCode;

use crate::base::BaseResolver;
use crate::ir::{ElementNode, ProfileDocument};
use crate::merge::ElementTreeMerger;
use crate::state::AppState;

use super::profiles::ErrorResponse;

/// Hydrate a profile document by merging its differential onto the base tree.
pub async fn hydrate_profile_document(
    state: &AppState,
    mut doc: ProfileDocument,
) -> Result<ProfileDocument, ErrorResponse> {
    if doc.resource.differential.is_empty() && !doc.resource.root.is_empty() {
        doc.resource.extract_differential();
    }

    let base_url = doc.resource.base.url.clone();
    let fhir_version = doc.resource.fhir_version;
    let resource_type = doc.resource.resource_type().to_string();

    let canonical_manager = state
        .canonical_manager()
        .await
        .map_err(|e| {
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                format!("Canonical manager error: {}", e),
            )
        })?;
    let resolver = BaseResolver::new(canonical_manager.clone());
    let merger = ElementTreeMerger::new();

    let base_tree = match resolver.load_base_tree(&base_url, fhir_version).await {
        Ok(tree) => tree,
        Err(e) => {
            tracing::warn!(
                "Failed to resolve base '{}' for profile '{}': {}",
                base_url,
                doc.metadata.id,
                e
            );
            ElementNode::new(resource_type)
        }
    };

    doc.resource.root = merger.merge(base_tree, &doc.resource.differential);

    Ok(doc)
}
