//! Quick Fix Suggestions
//!
//! Provides automated fix suggestions for common validation errors.

use serde::{Deserialize, Serialize};

/// A quick fix suggestion for a validation error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickFix {
    /// Display title for the fix.
    pub title: String,
    /// The kind of fix.
    pub kind: QuickFixKind,
    /// Whether this fix is preferred (highlighted in UI).
    #[serde(default)]
    pub is_preferred: bool,
}

impl QuickFix {
    /// Create a new quick fix.
    pub fn new(title: impl Into<String>, kind: QuickFixKind) -> Self {
        Self {
            title: title.into(),
            kind,
            is_preferred: false,
        }
    }

    /// Mark as preferred fix.
    pub fn preferred(mut self) -> Self {
        self.is_preferred = true;
        self
    }
}

/// The kind of quick fix operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QuickFixKind {
    /// Set cardinality to a specific value.
    SetCardinality {
        path: String,
        min: u32,
        max: Option<u32>,
    },

    /// Remove a type constraint.
    RemoveType { path: String, type_code: String },

    /// Add a required type constraint.
    AddType { path: String, type_code: String },

    /// Set binding strength.
    SetBindingStrength { path: String, strength: String },

    /// Remove an invalid binding.
    RemoveBinding { path: String },

    /// Fix discriminator path.
    FixDiscriminatorPath {
        path: String,
        current: String,
        suggested: String,
    },

    /// Remove duplicate slice.
    RemoveSlice { path: String, slice_name: String },

    /// Add required metadata field.
    AddMetadata { field: String, suggested_value: String },

    /// Remove element constraint.
    RemoveElement { path: String },

    /// Set element flag.
    SetFlag {
        path: String,
        flag: String,
        value: bool,
    },

    /// Custom fix with operation details.
    Custom {
        operation: String,
        parameters: serde_json::Value,
    },
}

/// Quick fix factory for common scenarios.
pub struct QuickFixFactory;

impl QuickFixFactory {
    /// Create fix for invalid cardinality (min > max).
    pub fn fix_cardinality_min_exceeds_max(path: &str, min: u32, max: u32) -> QuickFix {
        QuickFix::new(
            format!("Set cardinality to {}..{}", min, min),
            QuickFixKind::SetCardinality {
                path: path.to_string(),
                min,
                max: Some(min),
            },
        )
        .preferred()
    }

    /// Create fix for negative cardinality min.
    pub fn fix_negative_cardinality_min(path: &str) -> QuickFix {
        QuickFix::new(
            "Set minimum cardinality to 0",
            QuickFixKind::SetCardinality {
                path: path.to_string(),
                min: 0,
                max: None,
            },
        )
        .preferred()
    }

    /// Create fix for removing invalid type.
    pub fn remove_invalid_type(path: &str, type_code: &str) -> QuickFix {
        QuickFix::new(
            format!("Remove type '{}'", type_code),
            QuickFixKind::RemoveType {
                path: path.to_string(),
                type_code: type_code.to_string(),
            },
        )
    }

    /// Create fix for invalid binding strength.
    pub fn fix_binding_strength(path: &str, suggested: &str) -> QuickFix {
        QuickFix::new(
            format!("Change binding strength to '{}'", suggested),
            QuickFixKind::SetBindingStrength {
                path: path.to_string(),
                strength: suggested.to_string(),
            },
        )
        .preferred()
    }

    /// Create fix for removing invalid binding.
    pub fn remove_binding(path: &str) -> QuickFix {
        QuickFix::new(
            "Remove binding",
            QuickFixKind::RemoveBinding {
                path: path.to_string(),
            },
        )
    }

    /// Create fix for discriminator path typo.
    pub fn fix_discriminator_path(path: &str, current: &str, suggested: &str) -> QuickFix {
        QuickFix::new(
            format!("Change discriminator path to '{}'", suggested),
            QuickFixKind::FixDiscriminatorPath {
                path: path.to_string(),
                current: current.to_string(),
                suggested: suggested.to_string(),
            },
        )
        .preferred()
    }

    /// Create fix for duplicate slice name.
    pub fn remove_duplicate_slice(path: &str, slice_name: &str) -> QuickFix {
        QuickFix::new(
            format!("Remove duplicate slice '{}'", slice_name),
            QuickFixKind::RemoveSlice {
                path: path.to_string(),
                slice_name: slice_name.to_string(),
            },
        )
    }

    /// Create fix for missing required metadata.
    pub fn add_required_metadata(field: &str, suggested_value: &str) -> QuickFix {
        QuickFix::new(
            format!("Add {} = '{}'", field, suggested_value),
            QuickFixKind::AddMetadata {
                field: field.to_string(),
                suggested_value: suggested_value.to_string(),
            },
        )
        .preferred()
    }

    /// Create fix for setting must-support flag.
    pub fn set_must_support(path: &str, value: bool) -> QuickFix {
        let title = if value {
            "Set as Must Support"
        } else {
            "Remove Must Support"
        };
        QuickFix::new(
            title,
            QuickFixKind::SetFlag {
                path: path.to_string(),
                flag: "must_support".to_string(),
                value,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_fix_creation() {
        let fix = QuickFixFactory::fix_cardinality_min_exceeds_max("Patient.name", 2, 1);
        assert!(fix.is_preferred);
        assert!(fix.title.contains("2..2"));
    }

    #[test]
    fn test_quick_fix_serialization() {
        let fix = QuickFixFactory::remove_invalid_type("Patient.contact", "Timing");
        let json = serde_json::to_string(&fix).unwrap();
        assert!(json.contains("remove_type"));
        assert!(json.contains("Timing"));
    }
}
