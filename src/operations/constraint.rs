//! Constraint operations for profile elements.
//!
//! This module provides operations for modifying element constraints:
//! - Cardinality (min/max)
//! - Type constraints
//! - Flags (mustSupport, isModifier, isSummary)
//! - Bindings (terminology)
//! - Text (short, definition, comment)

use serde_json::json;

use crate::ir::{
    Binding, BindingStrength, Cardinality, Change, ElementNode, NodeId,
    ProfileDocument, TypeConstraint,
};

use super::error::{OperationError, OperationResult};
use super::traits::Operation;

// =============================================================================
// SetCardinality
// =============================================================================

/// Set cardinality for an element.
#[derive(Debug, Clone)]
pub struct SetCardinality {
    /// Element path.
    pub path: String,
    /// New minimum cardinality.
    pub min: u32,
    /// New maximum cardinality (None = unbounded).
    pub max: Option<u32>,
    /// Previous cardinality (for undo).
    prev_cardinality: Option<Option<Cardinality>>,
}

impl SetCardinality {
    /// Create a new set cardinality operation.
    pub fn new(path: impl Into<String>, min: u32, max: Option<u32>) -> Self {
        Self {
            path: path.into(),
            min,
            max,
            prev_cardinality: None,
        }
    }

    /// Create a required (1..1) cardinality operation.
    pub fn required(path: impl Into<String>) -> Self {
        Self::new(path, 1, Some(1))
    }

    /// Create an optional (0..1) cardinality operation.
    pub fn optional(path: impl Into<String>) -> Self {
        Self::new(path, 0, Some(1))
    }

    fn find_element<'a>(&self, doc: &'a ProfileDocument) -> Option<&'a ElementNode> {
        doc.resource.find_element(&self.path)
    }

    fn find_element_mut<'a>(&self, doc: &'a mut ProfileDocument) -> Option<&'a mut ElementNode> {
        doc.resource.find_element_mut(&self.path)
    }
}

impl Operation for SetCardinality {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        // Check element exists
        if self.find_element(document).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }

        // Validate min <= max
        if let Some(max) = self.max {
            if self.min > max {
                return Err(OperationError::invalid_cardinality(self.min, max));
            }
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = self
            .find_element_mut(document)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Store previous value for undo (we need interior mutability pattern here)
        // For now, we'll just apply the change
        element.constraints.cardinality = Some(Cardinality::new(self.min, self.max));
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = self
            .find_element_mut(document)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Restore previous value
        if let Some(prev) = &self.prev_cardinality {
            element.constraints.cardinality = prev.clone();
        } else {
            element.constraints.cardinality = None;
        }

        Ok(())
    }

    fn description(&self) -> String {
        let max_str = self.max.map_or("*".to_string(), |m| m.to_string());
        format!("Set cardinality of {} to {}..{}", self.path, self.min, max_str)
    }

    fn as_change(&self) -> Change {
        let node_id = NodeId::new(); // Placeholder - should get actual node ID
        Change::set(
            node_id,
            "constraints.cardinality",
            self.prev_cardinality
                .as_ref()
                .map(|c| json!(c)),
            json!({
                "min": self.min,
                "max": self.max
            }),
        )
    }
}

// =============================================================================
// AddTypeConstraint
// =============================================================================

/// Add a type constraint to an element.
#[derive(Debug, Clone)]
pub struct AddTypeConstraint {
    /// Element path.
    pub path: String,
    /// Type code to add.
    pub type_code: String,
    /// Profile URL for the type (optional).
    pub profile: Option<String>,
}

impl AddTypeConstraint {
    /// Create a new add type constraint operation.
    pub fn new(path: impl Into<String>, type_code: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            type_code: type_code.into(),
            profile: None,
        }
    }

    /// Add a profile constraint.
    pub fn with_profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    fn find_element_mut<'a>(&self, doc: &'a mut ProfileDocument) -> Option<&'a mut ElementNode> {
        doc.resource.find_element_mut(&self.path)
    }
}

impl Operation for AddTypeConstraint {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = self
            .find_element_mut(document)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let type_constraint = if let Some(ref profile) = self.profile {
            TypeConstraint::with_profile(&self.type_code, profile)
        } else {
            TypeConstraint::simple(&self.type_code)
        };

        element.constraints.types.push(type_constraint);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = self
            .find_element_mut(document)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element
            .constraints
            .types
            .retain(|t| t.code != self.type_code);

        Ok(())
    }

    fn description(&self) -> String {
        format!("Add type {} to {}", self.type_code, self.path)
    }

    fn as_change(&self) -> Change {
        Change::add(
            NodeId::new(),
            "constraints.types",
            json!({
                "code": self.type_code,
                "profile": self.profile
            }),
        )
    }
}

// =============================================================================
// RemoveTypeConstraint
// =============================================================================

/// Remove a type constraint from an element.
#[derive(Debug, Clone)]
pub struct RemoveTypeConstraint {
    /// Element path.
    pub path: String,
    /// Type code to remove.
    pub type_code: String,
    /// Previous type constraint (for undo).
    prev_type: Option<TypeConstraint>,
}

impl RemoveTypeConstraint {
    /// Create a new remove type constraint operation.
    pub fn new(path: impl Into<String>, type_code: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            type_code: type_code.into(),
            prev_type: None,
        }
    }
}

impl Operation for RemoveTypeConstraint {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        // Check type exists
        if !element.constraints.types.iter().any(|t| t.code == self.type_code) {
            return Err(OperationError::TypeNotFound {
                type_code: self.type_code.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element
            .constraints
            .types
            .retain(|t| t.code != self.type_code);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        if let Some(ref prev) = self.prev_type {
            let element = document
                .resource
                .find_element_mut(&self.path)
                .ok_or_else(|| OperationError::element_not_found(&self.path))?;

            element.constraints.types.push(prev.clone());
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Remove type {} from {}", self.type_code, self.path)
    }

    fn as_change(&self) -> Change {
        Change::remove(
            NodeId::new(),
            "constraints.types",
            json!({ "code": self.type_code }),
        )
    }
}

// =============================================================================
// SetMustSupport
// =============================================================================

/// Set the mustSupport flag on an element.
#[derive(Debug, Clone)]
pub struct SetMustSupport {
    /// Element path.
    pub path: String,
    /// New value.
    pub value: bool,
    /// Previous value (for undo).
    prev_value: Option<bool>,
}

impl SetMustSupport {
    /// Create a new set must support operation.
    pub fn new(path: impl Into<String>, value: bool) -> Self {
        Self {
            path: path.into(),
            value,
            prev_value: None,
        }
    }
}

impl Operation for SetMustSupport {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.flags.must_support = self.value;
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.flags.must_support = self.prev_value.unwrap_or(false);

        Ok(())
    }

    fn description(&self) -> String {
        if self.value {
            format!("Set mustSupport on {}", self.path)
        } else {
            format!("Clear mustSupport on {}", self.path)
        }
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.flags.must_support",
            self.prev_value.map(|v| json!(v)),
            json!(self.value),
        )
    }
}

// =============================================================================
// SetIsModifier
// =============================================================================

/// Set the isModifier flag on an element.
#[derive(Debug, Clone)]
pub struct SetIsModifier {
    /// Element path.
    pub path: String,
    /// New value.
    pub value: bool,
    /// Reason (required if value is true).
    pub reason: Option<String>,
    /// Previous values (for undo).
    prev_value: Option<bool>,
    prev_reason: Option<String>,
}

impl SetIsModifier {
    /// Create a new set is modifier operation.
    pub fn new(path: impl Into<String>, value: bool, reason: Option<String>) -> Self {
        Self {
            path: path.into(),
            value,
            reason,
            prev_value: None,
            prev_reason: None,
        }
    }
}

impl Operation for SetIsModifier {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.flags.is_modifier = self.value;
        element.constraints.flags.is_modifier_reason = self.reason.clone();
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.flags.is_modifier = self.prev_value.unwrap_or(false);
        element.constraints.flags.is_modifier_reason = self.prev_reason.clone();

        Ok(())
    }

    fn description(&self) -> String {
        if self.value {
            format!("Set isModifier on {}", self.path)
        } else {
            format!("Clear isModifier on {}", self.path)
        }
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.flags.is_modifier",
            self.prev_value.map(|v| json!(v)),
            json!(self.value),
        )
    }
}

// =============================================================================
// SetIsSummary
// =============================================================================

/// Set the isSummary flag on an element.
#[derive(Debug, Clone)]
pub struct SetIsSummary {
    /// Element path.
    pub path: String,
    /// New value.
    pub value: bool,
    /// Previous value (for undo).
    prev_value: Option<bool>,
}

impl SetIsSummary {
    /// Create a new set is summary operation.
    pub fn new(path: impl Into<String>, value: bool) -> Self {
        Self {
            path: path.into(),
            value,
            prev_value: None,
        }
    }
}

impl Operation for SetIsSummary {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.flags.is_summary = self.value;
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.flags.is_summary = self.prev_value.unwrap_or(false);

        Ok(())
    }

    fn description(&self) -> String {
        if self.value {
            format!("Set isSummary on {}", self.path)
        } else {
            format!("Clear isSummary on {}", self.path)
        }
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.flags.is_summary",
            self.prev_value.map(|v| json!(v)),
            json!(self.value),
        )
    }
}

// =============================================================================
// SetBinding
// =============================================================================

/// Set terminology binding on an element.
#[derive(Debug, Clone)]
pub struct SetBinding {
    /// Element path.
    pub path: String,
    /// Value set URL.
    pub value_set: String,
    /// Binding strength.
    pub strength: BindingStrength,
    /// Description (optional).
    pub description: Option<String>,
    /// Previous binding (for undo).
    prev_binding: Option<Binding>,
}

impl SetBinding {
    /// Create a new set binding operation.
    pub fn new(
        path: impl Into<String>,
        value_set: impl Into<String>,
        strength: BindingStrength,
    ) -> Self {
        Self {
            path: path.into(),
            value_set: value_set.into(),
            strength,
            description: None,
            prev_binding: None,
        }
    }

    /// Add a description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

impl Operation for SetBinding {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }

        // Validate value set URL format
        if !self.value_set.starts_with("http://") && !self.value_set.starts_with("https://") {
            return Err(OperationError::InvalidValueSetUrl {
                url: self.value_set.clone(),
            });
        }

        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        let mut binding = Binding::new(self.strength, &self.value_set);
        if let Some(ref desc) = self.description {
            binding = binding.with_description(desc);
        }

        element.constraints.binding = Some(binding);
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.binding = self.prev_binding.clone();

        Ok(())
    }

    fn description(&self) -> String {
        format!(
            "Set {} binding to {} on {}",
            self.strength, self.value_set, self.path
        )
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.binding",
            self.prev_binding.as_ref().map(|b| json!(b)),
            json!({
                "strength": self.strength.as_str(),
                "valueSet": self.value_set,
                "description": self.description
            }),
        )
    }
}

// =============================================================================
// RemoveBinding
// =============================================================================

/// Remove terminology binding from an element.
#[derive(Debug, Clone)]
pub struct RemoveBinding {
    /// Element path.
    pub path: String,
    /// Previous binding (for undo).
    prev_binding: Option<Binding>,
}

impl RemoveBinding {
    /// Create a new remove binding operation.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            prev_binding: None,
        }
    }
}

impl Operation for RemoveBinding {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.binding = None;
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        if let Some(ref prev) = self.prev_binding {
            let element = document
                .resource
                .find_element_mut(&self.path)
                .ok_or_else(|| OperationError::element_not_found(&self.path))?;

            element.constraints.binding = Some(prev.clone());
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Remove binding from {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::clear(
            NodeId::new(),
            "constraints.binding",
            json!(self.prev_binding),
        )
    }
}

// =============================================================================
// SetShort
// =============================================================================

/// Set short description on an element.
#[derive(Debug, Clone)]
pub struct SetShort {
    /// Element path.
    pub path: String,
    /// New short description.
    pub text: String,
    /// Previous value (for undo).
    prev_text: Option<String>,
}

impl SetShort {
    /// Create a new set short operation.
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
            prev_text: None,
        }
    }
}

impl Operation for SetShort {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.short = Some(self.text.clone());
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.short = self.prev_text.clone();

        Ok(())
    }

    fn description(&self) -> String {
        format!("Set short description on {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.short",
            self.prev_text.as_ref().map(|t| json!(t)),
            json!(self.text),
        )
    }
}

// =============================================================================
// SetDefinition
// =============================================================================

/// Set definition on an element.
#[derive(Debug, Clone)]
pub struct SetDefinition {
    /// Element path.
    pub path: String,
    /// New definition.
    pub text: String,
    /// Previous value (for undo).
    prev_text: Option<String>,
}

impl SetDefinition {
    /// Create a new set definition operation.
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
            prev_text: None,
        }
    }
}

impl Operation for SetDefinition {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.definition = Some(self.text.clone());
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.definition = self.prev_text.clone();

        Ok(())
    }

    fn description(&self) -> String {
        format!("Set definition on {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.definition",
            self.prev_text.as_ref().map(|t| json!(t)),
            json!(self.text),
        )
    }
}

// =============================================================================
// SetComment
// =============================================================================

/// Set comment on an element.
#[derive(Debug, Clone)]
pub struct SetComment {
    /// Element path.
    pub path: String,
    /// New comment.
    pub text: String,
    /// Previous value (for undo).
    prev_text: Option<String>,
}

impl SetComment {
    /// Create a new set comment operation.
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
            prev_text: None,
        }
    }
}

impl Operation for SetComment {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.comment = Some(self.text.clone());
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.comment = self.prev_text.clone();

        Ok(())
    }

    fn description(&self) -> String {
        format!("Set comment on {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.comment",
            self.prev_text.as_ref().map(|t| json!(t)),
            json!(self.text),
        )
    }
}

// =============================================================================
// SetFixedValue
// =============================================================================

/// Set fixed value on an element.
#[derive(Debug, Clone)]
pub struct SetFixedValue {
    /// Element path.
    pub path: String,
    /// Fixed value (JSON).
    pub value: serde_json::Value,
    /// Previous fixed value (for undo).
    prev_value: Option<crate::ir::FixedValue>,
}

impl SetFixedValue {
    /// Create a new set fixed value operation.
    pub fn new(path: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            path: path.into(),
            value,
            prev_value: None,
        }
    }
}

impl Operation for SetFixedValue {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.fixed_value = Some(crate::ir::FixedValue::fixed(self.value.clone()));
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.fixed_value = self.prev_value.clone();

        Ok(())
    }

    fn description(&self) -> String {
        format!("Set fixed value on {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.fixed_value",
            self.prev_value.as_ref().map(|v| json!(v)),
            json!({ "type": "Fixed", "value": self.value }),
        )
    }
}

// =============================================================================
// SetPatternValue
// =============================================================================

/// Set pattern value on an element.
#[derive(Debug, Clone)]
pub struct SetPatternValue {
    /// Element path.
    pub path: String,
    /// Pattern value (JSON).
    pub value: serde_json::Value,
    /// Previous fixed value (for undo).
    prev_value: Option<crate::ir::FixedValue>,
}

impl SetPatternValue {
    /// Create a new set pattern value operation.
    pub fn new(path: impl Into<String>, value: serde_json::Value) -> Self {
        Self {
            path: path.into(),
            value,
            prev_value: None,
        }
    }
}

impl Operation for SetPatternValue {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()> {
        if document.resource.find_element(&self.path).is_none() {
            return Err(OperationError::element_not_found(&self.path));
        }
        Ok(())
    }

    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.fixed_value = Some(crate::ir::FixedValue::pattern(self.value.clone()));
        element.source = crate::ir::ElementSource::Modified;

        Ok(())
    }

    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()> {
        let element = document
            .resource
            .find_element_mut(&self.path)
            .ok_or_else(|| OperationError::element_not_found(&self.path))?;

        element.constraints.fixed_value = self.prev_value.clone();

        Ok(())
    }

    fn description(&self) -> String {
        format!("Set pattern value on {}", self.path)
    }

    fn as_change(&self) -> Change {
        Change::set(
            NodeId::new(),
            "constraints.fixed_value",
            self.prev_value.as_ref().map(|v| json!(v)),
            json!({ "type": "Pattern", "value": self.value }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{BaseDefinition, DocumentMetadata, FhirVersion, ProfiledResource};

    fn create_test_document() -> ProfileDocument {
        let metadata = DocumentMetadata::new(
            "test-patient",
            "http://example.org/fhir/StructureDefinition/TestPatient",
            "TestPatient",
        );
        let resource = ProfiledResource::new(
            "http://example.org/fhir/StructureDefinition/TestPatient",
            FhirVersion::R4,
            BaseDefinition::resource("Patient"),
        );
        let mut doc = ProfileDocument::new(metadata, resource);

        // Add some test elements
        let name = crate::ir::ElementNode::new("Patient.name".to_string());
        doc.resource.root.add_child(name);

        doc
    }

    #[test]
    fn test_set_cardinality_validation() {
        let doc = create_test_document();

        // Valid operation
        let op = SetCardinality::new("Patient.name", 1, Some(5));
        assert!(op.validate(&doc).is_ok());

        // Invalid path
        let op = SetCardinality::new("Patient.invalid", 1, Some(1));
        assert!(op.validate(&doc).is_err());

        // Invalid cardinality (min > max)
        let op = SetCardinality::new("Patient.name", 5, Some(1));
        assert!(op.validate(&doc).is_err());
    }

    #[test]
    fn test_set_cardinality_apply() {
        let mut doc = create_test_document();

        let op = SetCardinality::required("Patient.name");
        op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.name").unwrap();
        let card = element.constraints.cardinality.as_ref().unwrap();
        assert_eq!(card.min, 1);
        assert_eq!(card.max, Some(1));
    }

    #[test]
    fn test_set_must_support() {
        let mut doc = create_test_document();

        let op = SetMustSupport::new("Patient.name", true);
        op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.name").unwrap();
        assert!(element.constraints.flags.must_support);
    }

    #[test]
    fn test_set_binding() {
        let mut doc = create_test_document();

        let op = SetBinding::new(
            "Patient.name",
            "http://example.org/ValueSet/names",
            BindingStrength::Required,
        );
        op.apply(&mut doc).unwrap();

        let element = doc.resource.find_element("Patient.name").unwrap();
        let binding = element.constraints.binding.as_ref().unwrap();
        assert_eq!(binding.strength, BindingStrength::Required);
        assert_eq!(binding.value_set, "http://example.org/ValueSet/names");
    }
}
