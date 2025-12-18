//! Metadata Validation Rules
//!
//! Validates profile metadata:
//! - Required fields are present (id, url, name)
//! - Name follows conventions (PascalCase, no spaces)
//! - URL is valid canonical URL
//! - Status is valid

use crate::ir::ProfileDocument;
use crate::validation::diagnostic::{Diagnostic, DiagnosticSource};
use crate::validation::quick_fix::QuickFixFactory;

/// Error codes for metadata validation.
pub mod codes {
    pub const META_MISSING_ID: &str = "META_001";
    pub const META_MISSING_URL: &str = "META_002";
    pub const META_MISSING_NAME: &str = "META_003";
    pub const META_INVALID_NAME: &str = "META_004";
    pub const META_NAME_HAS_SPACES: &str = "META_005";
    pub const META_INVALID_URL: &str = "META_006";
    pub const META_MISSING_BASE: &str = "META_007";
    pub const META_MISSING_TITLE: &str = "META_008";
    pub const META_MISSING_DESCRIPTION: &str = "META_009";
}

/// Validate profile metadata.
pub fn validate_metadata(document: &ProfileDocument) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Check required fields
    if document.metadata.id.is_empty() {
        let mut diag = Diagnostic::error(codes::META_MISSING_ID, "Profile ID is required")
            .with_source(DiagnosticSource::Ir);

        // Suggest deriving from name
        if !document.metadata.name.is_empty() {
            let suggested_id = to_kebab_case(&document.metadata.name);
            diag = diag.with_quick_fix(QuickFixFactory::add_required_metadata("id", &suggested_id));
        }

        diagnostics.push(diag);
    }

    if document.metadata.url.is_empty() {
        let mut diag = Diagnostic::error(codes::META_MISSING_URL, "Profile URL is required")
            .with_source(DiagnosticSource::Ir);

        // Suggest URL based on name
        if !document.metadata.name.is_empty() {
            let suggested_url = format!(
                "http://example.org/fhir/StructureDefinition/{}",
                document.metadata.name
            );
            diag = diag.with_quick_fix(QuickFixFactory::add_required_metadata("url", &suggested_url));
        }

        diagnostics.push(diag);
    } else if !is_valid_canonical_url(&document.metadata.url) {
        diagnostics.push(
            Diagnostic::error(
                codes::META_INVALID_URL,
                format!("Invalid profile URL: '{}'", document.metadata.url),
            )
            .with_source(DiagnosticSource::Ir),
        );
    }

    if document.metadata.name.is_empty() {
        diagnostics.push(
            Diagnostic::error(codes::META_MISSING_NAME, "Profile name is required")
                .with_source(DiagnosticSource::Ir),
        );
    } else {
        // Validate name format
        if document.metadata.name.contains(' ') {
            let suggested_name = document.metadata.name.replace(' ', "");
            let mut diag = Diagnostic::error(
                codes::META_NAME_HAS_SPACES,
                "Profile name cannot contain spaces",
            )
            .with_source(DiagnosticSource::Ir);

            diag =
                diag.with_quick_fix(QuickFixFactory::add_required_metadata("name", &suggested_name));
            diagnostics.push(diag);
        }

        if !is_valid_profile_name(&document.metadata.name) {
            diagnostics.push(
                Diagnostic::warning(
                    codes::META_INVALID_NAME,
                    format!(
                        "Profile name '{}' should start with uppercase letter",
                        document.metadata.name
                    ),
                )
                .with_source(DiagnosticSource::Ir),
            );
        }
    }

    // Check base definition
    if document.resource.base.url.is_empty() {
        diagnostics.push(
            Diagnostic::error(codes::META_MISSING_BASE, "Base definition URL is required")
                .with_source(DiagnosticSource::Ir),
        );
    }

    // Recommend optional but important fields
    if document.metadata.title.is_none() {
        diagnostics.push(
            Diagnostic::info(
                codes::META_MISSING_TITLE,
                "Profile should have a title for display purposes",
            )
            .with_source(DiagnosticSource::Ir),
        );
    }

    if document.metadata.description.is_none() {
        diagnostics.push(
            Diagnostic::info(
                codes::META_MISSING_DESCRIPTION,
                "Profile should have a description",
            )
            .with_source(DiagnosticSource::Ir),
        );
    }

    diagnostics
}

/// Check if a profile name is valid.
fn is_valid_profile_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // First character should be uppercase letter
    let first = name.chars().next().unwrap();
    if !first.is_ascii_uppercase() {
        return false;
    }

    // Rest should be alphanumeric
    name.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Check if a URL is a valid canonical URL.
fn is_valid_canonical_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }

    // Must start with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        if !url.starts_with("urn:") {
            return false;
        }
    }

    // Should not contain spaces or control characters
    if url.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return false;
    }

    true
}

/// Convert a name to kebab-case for ID generation.
fn to_kebab_case(name: &str) -> String {
    let mut result = String::new();

    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                result.push('-');
            }
            result.push(c.to_ascii_lowercase());
        } else if c.is_ascii_alphanumeric() {
            result.push(c);
        } else if c == ' ' || c == '_' {
            result.push('-');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, DocumentMetadata, FhirVersion, ProfiledResource};

    fn create_test_document() -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            "test-profile",
            "http://example.org/fhir/StructureDefinition/TestProfile",
            "TestProfile",
        );
        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestProfile",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        ProfileDocument::new(metadata, resource)
    }

    #[test]
    fn test_valid_document() {
        let doc = create_test_document();
        let diagnostics = validate_metadata(&doc);

        // Only info-level warnings expected for missing title/description
        assert!(diagnostics.iter().all(|d| {
            d.code == codes::META_MISSING_TITLE || d.code == codes::META_MISSING_DESCRIPTION
        }));
    }

    #[test]
    fn test_missing_id() {
        let mut doc = create_test_document();
        doc.metadata.id = String::new();

        let diagnostics = validate_metadata(&doc);
        assert!(diagnostics
            .iter()
            .any(|d| d.code == codes::META_MISSING_ID));
    }

    #[test]
    fn test_name_with_spaces() {
        let mut doc = create_test_document();
        doc.metadata.name = "Test Profile".to_string();

        let diagnostics = validate_metadata(&doc);
        assert!(diagnostics
            .iter()
            .any(|d| d.code == codes::META_NAME_HAS_SPACES));
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("TestProfile"), "test-profile");
        assert_eq!(to_kebab_case("USCorePatient"), "u-s-core-patient");
        assert_eq!(to_kebab_case("test"), "test");
    }

    #[test]
    fn test_valid_profile_names() {
        assert!(is_valid_profile_name("Patient"));
        assert!(is_valid_profile_name("USCorePatient"));
        assert!(is_valid_profile_name("MyProfile123"));
    }

    #[test]
    fn test_invalid_profile_names() {
        assert!(!is_valid_profile_name(""));
        assert!(!is_valid_profile_name("patient"));  // lowercase start
        assert!(!is_valid_profile_name("My Profile"));  // space
    }
}
