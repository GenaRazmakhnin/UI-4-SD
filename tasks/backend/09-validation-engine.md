# Task: Validation Engine Implementation

## Status: ✅ Implemented

## Description
Implement a layered validation engine that provides fast incremental validation for the UI while converging on IG Publisher parity for production validation.

## Implementation Summary

The validation engine has been implemented with the following components:

### Implemented Files
- `src/validation/mod.rs` - Module entry point with architecture documentation
- `src/validation/diagnostic.rs` - ValidationResult, Diagnostic, DiagnosticSeverity, DiagnosticSource
- `src/validation/engine.rs` - ValidationEngine with layered validation
- `src/validation/quick_fix.rs` - QuickFix and QuickFixKind for automated fixes
- `src/validation/rules/mod.rs` - Rule modules coordination
- `src/validation/rules/cardinality.rs` - Cardinality validation (min≤max, slice sums)
- `src/validation/rules/type_refinement.rs` - Type constraint validation
- `src/validation/rules/slicing.rs` - Slicing definition validation
- `src/validation/rules/binding.rs` - Binding strength and ValueSet URL validation
- `src/validation/rules/metadata.rs` - Profile metadata validation
- `src/validation/rules/fhirpath.rs` - FHIRPath expression validation using octofhir-fhirpath

### API Endpoints (in src/api/validation.rs)
- `POST /api/projects/:projectId/profiles/:profileId/validate` - Full validation
- `POST /api/projects/:projectId/profiles/:profileId/validate/quick` - Quick structural validation
- `POST /api/projects/:projectId/profiles/:profileId/validate/element` - Validate specific element

### Key Features
- Layered validation: Structural → References → Terminology
- FHIRPath expression parsing/validation using octofhir-fhirpath
- Quick fix suggestions for common validation errors
- Incremental validation for changed elements
- 54 unit tests passing

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

- [x] IR validation catches structural errors instantly (<10ms)
- [x] Cardinality validation works correctly
- [x] Type refinement validation prevents invalid types
- [x] Slice validation catches all slice errors
- [x] Binding validation checks strength and URL
- [x] FHIRPath expression validation works (using octofhir-fhirpath)
- [ ] Terminology validation with caching (structure in place, full implementation pending)
- [x] Graceful offline operation
- [x] Cross-reference validation finds broken links
- [ ] Publisher parity harness runs HL7 Validator (deferred to future task)
- [ ] Parity reports are stored and diffable (deferred to future task)
- [x] Incremental validation is fast (<100ms)
- [x] Quick fixes are suggested for common errors
- [x] Validation results map to UI elements correctly
- [x] Documentation for validation rules

## Dependencies
- **Backend 02**: IR Data Model Implementation
- **Backend 04**: SD Export (for parity validation)
- **Backend 10**: Package Management (for reference resolution)

## Related Files
- `src/validation/mod.rs` - Module entry point
- `src/validation/diagnostic.rs` - Diagnostic types
- `src/validation/engine.rs` - Main validation engine
- `src/validation/quick_fix.rs` - Quick fix suggestions
- `src/validation/rules/mod.rs` - Rule modules coordination
- `src/validation/rules/cardinality.rs` - Cardinality validation
- `src/validation/rules/type_refinement.rs` - Type constraint validation
- `src/validation/rules/slicing.rs` - Slicing validation
- `src/validation/rules/binding.rs` - Binding validation
- `src/validation/rules/metadata.rs` - Metadata validation
- `src/validation/rules/fhirpath.rs` - FHIRPath expression validation
- `src/api/validation.rs` - REST API endpoints

## Priority
🔴 Critical - Core functionality

## Estimated Complexity
Very High - 3-4 weeks
