# Task: IR (Intermediate Representation) Data Model Implementation

## Status: COMPLETED

## Description
Implement the core IR data model that represents FHIR profiles in an editable, UI-friendly format. This is the central data structure that sits between the UI and export formats (SD/FSH).

## Requirements

### R1: Core IR Types
Implement the following core types in `src/ir/`:

- [x] `ProfileDocument`: Top-level container for a profile being edited
  - Profile metadata (id, url, name, status, etc.)
  - Reference to base resource/profile
  - Element tree root
  - Edit history
  - Dirty state tracking

- [x] `ProfiledResource`: IR representation of a profiled resource
  - Canonical URL
  - FHIR version
  - Base definition reference
  - Element tree
  - Unknown fields preservation (for lossless round-trip)

- [x] `ElementNode`: Represents a single element in the profile
  - Path (e.g., "Patient.name")
  - Stable UI ID (UUID, never exported)
  - Parent/children relationships
  - Constraints (cardinality, type, binding, etc.)
  - Source tracking (inherited vs. modified)
  - Unknown fields preservation

- [x] `ElementConstraints`: Explicit constraints applied to an element
  - Cardinality (min/max)
  - Type constraints (allowed types, profiles)
  - Flags (mustSupport, isModifier, isSummary, isModifierReason)
  - Short/definition text
  - Fixed/pattern values
  - Terminology binding
  - FHIRPath invariants
  - Mappings

- [x] `SlicingDefinition`: Slicing configuration
  - Discriminator rules (type, path)
  - Slicing rules (closed/open/openAtEnd)
  - Ordered flag
  - Description

- [x] `SliceNode`: Individual slice within a sliced element
  - Slice name
  - Constraints
  - Children

### R2: Change Tracking
- [x] `ChangeTracker`: Track modifications to elements
  - Record which fields are explicitly set vs. inherited
  - Support semantic diff (what changed from base)

- [x] `EditHistory`: Undo/redo support
  - Operation log with reversible operations
  - Maximum history depth configuration
  - Undo/redo stack management

### R3: Serialization
- [x] All IR types implement `serde::Serialize` and `serde::Deserialize`
- [x] Support JSON serialization for API transport
- [x] IndexMap used for deterministic ordering

### R4: Validation Support
- [x] ValidationError type with path references
- [x] ValidationResult for collecting errors/warnings
- [x] ValidationSeverity levels (Info, Warning, Error, Fatal)
- [x] ValidationCategory for error classification

## Acceptance Criteria

- [x] All core IR types are implemented with complete fields
- [x] All types implement `serde::Serialize` and `serde::Deserialize`
- [x] Element tree supports parent/child navigation
- [x] Stable UI IDs are generated and tracked (UUIDs)
- [x] Change tracking correctly identifies inherited vs. modified fields
- [x] Edit history supports undo/redo operations
- [x] Unknown fields are preserved during deserialization
- [x] Documentation with examples for each type
- [x] API documentation (rustdoc) for all public types
- [x] All unit tests pass (28 tests)
- [x] All doc-tests pass (9 tests)

## Implementation Details

### Files Created

| File | Description |
|------|-------------|
| `src/ir/mod.rs` | Module root with architecture docs and re-exports |
| `src/ir/element.rs` | `ElementNode`, `NodeId`, `ElementSource` |
| `src/ir/constraint.rs` | `ElementConstraints`, `Cardinality`, `TypeConstraint`, `Binding`, etc. |
| `src/ir/slicing.rs` | `SlicingDefinition`, `SliceNode`, `Discriminator`, `SlicingRules` |
| `src/ir/resource.rs` | `ProfiledResource`, `FhirVersion`, `BaseDefinition` |
| `src/ir/document.rs` | `ProfileDocument`, `DocumentMetadata`, `ProfileStatus` |
| `src/ir/tracking.rs` | `EditHistory`, `ChangeTracker`, `Operation`, `Change` |
| `src/ir/validation.rs` | `ValidationResult`, `ValidationError`, `ValidationSeverity` |

### Key Design Decisions

1. **Stable UI IDs**: `NodeId` uses UUIDv4 for stable references across edits
2. **Element Source Tracking**: `ElementSource` enum tracks inherited/modified/added
3. **Ordered Maps**: `IndexMap` used for deterministic serialization
4. **Lossless Round-Trip**: `unknown_fields` preserves unrecognized JSON fields
5. **Async-Ready**: All types are `Send + Sync` compatible

### Dependencies Added

```toml
uuid = { version = "1.11", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
indexmap = { version = "2.7", features = ["serde"] }
```

## Test Results

```
running 28 tests
test ir::constraint::tests::test_binding_creation ... ok
test ir::constraint::tests::test_cardinality_comparison ... ok
test ir::constraint::tests::test_cardinality_formatting ... ok
test ir::constraint::tests::test_type_constraint ... ok
test ir::document::tests::test_dirty_tracking ... ok
test ir::document::tests::test_document_creation ... ok
test ir::document::tests::test_profile_status ... ok
test ir::element::tests::test_child_navigation ... ok
test ir::element::tests::test_descendant_search ... ok
test ir::element::tests::test_element_node_creation ... ok
test ir::element::tests::test_node_id_creation ... ok
test ir::resource::tests::test_base_definition ... ok
test ir::resource::tests::test_element_navigation ... ok
test ir::resource::tests::test_fhir_version ... ok
test ir::resource::tests::test_profiled_resource_creation ... ok
test ir::slicing::tests::test_discriminator_creation ... ok
test ir::slicing::tests::test_slice_node ... ok
test ir::slicing::tests::test_slicing_definition ... ok
test ir::slicing::tests::test_slicing_rules ... ok
test ir::tracking::tests::test_change_creation ... ok
test ir::tracking::tests::test_change_inverse ... ok
test ir::tracking::tests::test_change_tracker ... ok
test ir::tracking::tests::test_edit_history_undo_redo ... ok
test ir::tracking::tests::test_history_max_size ... ok
test ir::validation::tests::test_result_merge ... ok
test ir::validation::tests::test_validation_error_creation ... ok
test ir::validation::tests::test_validation_result ... ok
test ir::validation::tests::test_validation_severity ... ok

test result: ok. 28 passed; 0 failed
```

## Dependencies
- **Backend 01**: Toolchain Alignment (completed)

## Related Files
- `src/ir/mod.rs`
- `src/ir/document.rs`
- `src/ir/resource.rs`
- `src/ir/element.rs`
- `src/ir/constraint.rs`
- `src/ir/slicing.rs`
- `src/ir/tracking.rs`
- `src/ir/validation.rs`

## Priority
🔴 Critical - Core foundation for entire system

## Estimated Complexity
High - 2-3 weeks

## Actual Complexity
Moderate - Core types implemented in single session with comprehensive tests
