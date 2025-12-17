# Task: IR (Intermediate Representation) Data Model Implementation

## Description
Implement the core IR data model that represents FHIR profiles in an editable, UI-friendly format. This is the central data structure that sits between the UI and export formats (SD/FSH).

## Requirements

### R1: Core IR Types
Implement the following core types in `crates/profile-builder/src/ir/`:

- `ProfileDocument`: Top-level container for a profile being edited
  - Profile metadata (id, url, name, status, etc.)
  - Reference to base resource/profile
  - Element tree root
  - Edit history
  - Dirty state tracking

- `ProfiledResource`: IR representation of a profiled resource
  - Canonical URL
  - FHIR version
  - Base definition reference
  - Element tree
  - Unknown fields preservation (for lossless round-trip)

- `ElementNode`: Represents a single element in the profile
  - Path (e.g., "Patient.name")
  - Stable UI ID (UUID, never exported)
  - Parent/children relationships
  - Constraints (cardinality, type, binding, etc.)
  - Source tracking (inherited vs. modified)
  - Unknown fields preservation

- `ElementConstraints`: Explicit constraints applied to an element
  - Cardinality (min/max)
  - Type constraints (allowed types, profiles)
  - Flags (mustSupport, isModifier, isSummary, isModifierReason)
  - Short/definition text
  - Fixed/pattern values
  - Terminology binding
  - Slice definitions

- `SlicingDefinition`: Slicing configuration
  - Discriminator rules (type, path)
  - Slicing rules (closed/open/openAtEnd)
  - Ordered flag
  - Description

- `SliceNode`: Individual slice within a sliced element
  - Slice name
  - Constraints
  - Children

### R2: Change Tracking
- `ChangeTracker`: Track modifications to elements
  - Record which fields are explicitly set vs. inherited
  - Support semantic diff (what changed from base)
  - Enable undo/redo functionality

- `EditHistory`: Undo/redo support
  - Operation log with reversible operations
  - State snapshots for efficient undo
  - Maximum history depth configuration

### R3: Serialization
- All IR types must be serializable (serde)
- Support JSON serialization for API transport
- Support efficient binary serialization for caching

### R4: Validation Support
- Each IR type should support validation
- Provide clear error types with element path references
- Support incremental validation (only changed elements)

## Acceptance Criteria

- [ ] All core IR types are implemented with complete fields
- [ ] All types implement `serde::Serialize` and `serde::Deserialize`
- [ ] Element tree supports parent/child navigation
- [ ] Stable UI IDs are generated and tracked (UUIDs)
- [ ] Change tracking correctly identifies inherited vs. modified fields
- [ ] Edit history supports undo/redo operations
- [ ] Unknown fields are preserved during deserialization
- [ ] Documentation with examples for each type
- [ ] API documentation (rustdoc) for all public types

## Dependencies
- **Backend 01**: Toolchain Alignment (must be completed first)

## Related Files
- `crates/profile-builder/Cargo.toml` (new)
- `crates/profile-builder/src/lib.rs` (new)
- `crates/profile-builder/src/ir/mod.rs` (new)
- `crates/profile-builder/src/ir/document.rs` (new)
- `crates/profile-builder/src/ir/resource.rs` (new)
- `crates/profile-builder/src/ir/element.rs` (new)
- `crates/profile-builder/src/ir/constraint.rs` (new)
- `crates/profile-builder/src/ir/slicing.rs` (new)
- `crates/profile-builder/src/ir/tracking.rs` (new)

## Priority
🔴 Critical - Core foundation for entire system

## Estimated Complexity
High - 2-3 weeks
