# Task: IR to StructureDefinition Export

## Description
Implement deterministic export from IR to valid FHIR StructureDefinition JSON. The output must pass IG Publisher validation and produce byte-identical results for the same IR state.

## Requirements

### R1: Snapshot Generation
- Generate complete snapshot from element tree
- Merge constraints with base definition snapshot
- Apply all explicit constraints to inherited elements
- Ensure element paths are correctly formed

### R2: Differential Generation
- Generate minimal differential (only modified elements)
- Include only explicitly constrained fields
- Preserve must-have metadata (url, name, status, etc.)
- Optimize for minimal diff size and clarity

### R3: Constraint Serialization
Export all constraint types correctly:
- Cardinality (min/max)
- Type constraints (type array with profiles)
- Flags (mustSupport, isModifier, isSummary, isModifierReason)
- Short/definition text overrides
- Fixed/pattern values
- Terminology bindings (strength, valueSet reference)

### R4: Slicing Export
- Generate slicing element with discriminator rules
- Export individual slice elements with correct naming
- Apply slicing rules (closed/open/openAtEnd, ordered)
- Ensure discriminator paths are valid

### R5: Deterministic Serialization
- Canonical JSON formatting (no whitespace variations)
- Stable field ordering (alphabetical or FHIR spec order)
- Stable array element ordering where semantically valid
- Consistent handling of optional fields (omit vs. null)

### R6: Unknown Field Preservation
- Re-insert preserved unknown fields at export time
- Place fields at correct JSON paths
- Maintain original field ordering where possible

### R7: Validation
- Ensure exported SD has required metadata:
  - url (canonical)
  - name
  - status
  - fhirVersion
  - kind
  - type (base resource)
  - derivation ("constraint")
- Validate snapshot element count matches tree
- Validate all element IDs are correct

### R8: Error Handling
- Validate IR state before export
- Provide clear errors for invalid states
- Support partial export with warnings

## Acceptance Criteria

- [ ] Exports valid StructureDefinition JSON (R4, R4B, R5)
- [ ] Generated SD passes IG Publisher validation
- [ ] Generated SD passes HL7 Validator
- [ ] Deterministic output: same IR → byte-identical JSON
- [ ] Differential contains only modified elements
- [ ] Snapshot is complete and correct
- [ ] Slicing exports correctly (discriminators, rules, slices)
- [ ] Unknown fields are preserved in output
- [ ] Field ordering is stable and deterministic
- [ ] Clear error messages for invalid IR states

## Dependencies
- **Backend 01**: Toolchain Alignment
- **Backend 02**: IR Data Model Implementation
- **Backend 03**: SD Import (for round-trip testing)

## Related Files
- `crates/profile-builder/src/export/mod.rs` (new)
- `crates/profile-builder/src/export/sd_export.rs` (new)
- `crates/profile-builder/src/export/snapshot_generator.rs` (new)
- `crates/profile-builder/src/export/differential_generator.rs` (new)
- `crates/profile-builder/src/export/deterministic.rs` (new)
- `crates/profile-builder/src/export/field_preservation.rs` (new)

## Priority
🔴 Critical - Required for MVP

## Estimated Complexity
High - 2-3 weeks
