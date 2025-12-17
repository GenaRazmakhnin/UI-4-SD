# Task: Validation Engine Implementation

## Description
Implement a layered validation engine that provides fast incremental validation for the UI while converging on IG Publisher parity for production validation.

## Requirements

### R1: IR Validation Layer (Instant)
Structural validation rules for immediate feedback:
- **Cardinality sanity**:
  - min ≤ max
  - min ≥ 0
  - max = "*" or integer
  - Derived cardinality doesn't exceed base
- **Type refinement validation**:
  - Constrained types are subtypes of base
  - Profile references are valid
  - Type cardinality matches element cardinality
- **Slice validation**:
  - Slice names are unique within parent
  - Discriminator paths are valid element paths
  - Discriminator types are appropriate for path
  - Slice cardinality sum ≤ parent max
- **Binding validation**:
  - Binding strength is valid (required/extensible/preferred/example)
  - ValueSet URL is valid format
  - Binding strength cannot be weakened
- **Invariant validation**:
  - FHIRPath expressions parse correctly
  - Severity is valid (error/warning)
  - Key is unique within resource

### R2: Terminology Validation Layer (Cached, Async)
- Validate ValueSet references resolve
- Check code membership when terminology service available
- Cache expansion results
- Degrade gracefully when offline
- Warn if ValueSet not found
- Validate binding strength appropriateness

### R3: Cross-Reference Validation
- Validate extension URLs resolve to StructureDefinition
- Validate profile references resolve
- Validate targetProfile references resolve
- Check extension context rules match usage
- Warn on circular profile dependencies

### R4: Publisher Parity Validation (On-Demand)
- Export profile to SD JSON
- Run HL7 Validator against exported SD
- Parse validator output
- Map validator errors back to IR elements
- Store parity report
- Track parity regression (no new false negatives)

### R5: Incremental Validation
- Validate only changed elements after edits
- Invalidate dependent element validations
- Efficient re-validation on constraint changes
- Background validation worker

### R6: Validation Result Model
```rust
pub struct ValidationResult {
    pub diagnostics: Vec<Diagnostic>,
    pub is_valid: bool,
    pub validation_level: ValidationLevel,
}

pub struct Diagnostic {
    pub severity: Severity, // Error, Warning, Info
    pub code: String,
    pub message: String,
    pub element_path: Option<String>,
    pub source: DiagnosticSource, // IR, Terminology, Publisher
    pub quick_fix: Option<QuickFix>,
}
```

### R7: Quick Fix Suggestions
Provide automated fixes for common issues:
- Fix cardinality conflicts (adjust to valid range)
- Remove invalid type constraints
- Add missing required metadata
- Fix discriminator path typos (suggest valid paths)

### R8: Validation API
```rust
pub trait Validator {
    fn validate(&self, profile: &ProfileDocument) -> ValidationResult;
    fn validate_element(&self, element: &ElementNode) -> ValidationResult;
    fn validate_incremental(&self, changes: &[Operation]) -> ValidationResult;
}
```

## Acceptance Criteria

- [ ] IR validation catches structural errors instantly (<10ms)
- [ ] Cardinality validation works correctly
- [ ] Type refinement validation prevents invalid types
- [ ] Slice validation catches all slice errors
- [ ] Binding validation checks strength and URL
- [ ] FHIRPath expression validation works
- [ ] Terminology validation with caching
- [ ] Graceful offline operation
- [ ] Cross-reference validation finds broken links
- [ ] Publisher parity harness runs HL7 Validator
- [ ] Parity reports are stored and diffable
- [ ] Incremental validation is fast (<100ms)
- [ ] Quick fixes are suggested for common errors
- [ ] Validation results map to UI elements correctly
- [ ] Documentation for validation rules

## Dependencies
- **Backend 02**: IR Data Model Implementation
- **Backend 04**: SD Export (for parity validation)
- **Backend 10**: Package Management (for reference resolution)

## Related Files
- `crates/profile-builder/src/validation/mod.rs` (new)
- `crates/profile-builder/src/validation/rules/mod.rs` (new)
- `crates/profile-builder/src/validation/rules/cardinality.rs` (new)
- `crates/profile-builder/src/validation/rules/type_refinement.rs` (new)
- `crates/profile-builder/src/validation/rules/slicing.rs` (new)
- `crates/profile-builder/src/validation/rules/binding.rs` (new)
- `crates/profile-builder/src/validation/terminology.rs` (new)
- `crates/profile-builder/src/validation/parity.rs` (new)
- `crates/profile-builder/src/validation/incremental.rs` (new)
- `crates/profile-builder/src/validation/quick_fix.rs` (new)

## Priority
🔴 Critical - Core functionality

## Estimated Complexity
Very High - 3-4 weeks
