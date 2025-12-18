# Task: IR to StructureDefinition Export

## Status: ✅ Implemented

## Description
Implement deterministic export from IR to valid FHIR StructureDefinition JSON. The output must pass IG Publisher validation and produce byte-identical results for the same IR state.

## Implementation Summary

The export module has been implemented with the following components:

### Module Structure (`src/export/`)
- `mod.rs` - Module entry point with re-exports
- `error.rs` - Export error types and warnings
- `deterministic.rs` - Deterministic JSON builder with FHIR field ordering
- `element_serializer.rs` - Converts IR elements to ElementDefinition JSON
- `snapshot_generator.rs` - Generates complete snapshot from element tree
- `differential_generator.rs` - Generates minimal differential
- `field_preservation.rs` - Preserves unknown fields for lossless round-trip
- `sd_exporter.rs` - Main exporter coordinating all components

### Key Features Implemented
- **Async-first design** - All export operations are async
- **Deterministic output** - Same IR state produces byte-identical JSON
- **FHIR spec field ordering** - Fields ordered per FHIR StructureDefinition spec
- **Configurable export** - Snapshot-only, differential-only, or both
- **Validation** - Validates required metadata before export
- **Warning system** - Non-fatal issues reported as warnings

## Requirements

### R1: Snapshot Generation ✅
- Generate complete snapshot from element tree
- Merge constraints with base definition snapshot
- Apply all explicit constraints to inherited elements
- Ensure element paths are correctly formed

### R2: Differential Generation ✅
- Generate minimal differential (only modified elements)
- Include only explicitly constrained fields
- Preserve must-have metadata (url, name, status, etc.)
- Optimize for minimal diff size and clarity

### R3: Constraint Serialization ✅
Export all constraint types correctly:
- Cardinality (min/max)
- Type constraints (type array with profiles)
- Flags (mustSupport, isModifier, isSummary, isModifierReason)
- Short/definition text overrides
- Fixed/pattern values
- Terminology bindings (strength, valueSet reference)

### R4: Slicing Export ✅
- Generate slicing element with discriminator rules
- Export individual slice elements with correct naming
- Apply slicing rules (closed/open/openAtEnd, ordered)
- Ensure discriminator paths are valid

### R5: Deterministic Serialization ✅
- Canonical JSON formatting (no whitespace variations)
- Stable field ordering (FHIR spec order)
- Stable array element ordering where semantically valid
- Consistent handling of optional fields (omit vs. null)

### R6: Unknown Field Preservation ✅
- Re-insert preserved unknown fields at export time
- Place fields at correct JSON paths
- Maintain original field ordering where possible

### R7: Validation ✅
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

### R8: Error Handling ✅
- Validate IR state before export
- Provide clear errors for invalid states
- Support partial export with warnings

## Acceptance Criteria

- [x] Exports valid StructureDefinition JSON (R4, R4B, R5)
- [ ] Generated SD passes IG Publisher validation (requires integration testing)
- [ ] Generated SD passes HL7 Validator (requires integration testing)
- [x] Deterministic output: same IR → byte-identical JSON
- [x] Differential contains only modified elements
- [x] Snapshot is complete and correct
- [x] Slicing exports correctly (discriminators, rules, slices)
- [x] Unknown fields are preserved in output
- [x] Field ordering is stable and deterministic
- [x] Clear error messages for invalid IR states

## Test Results

All 32 export tests pass:
- `test_deterministic_builder` - Verifies FHIR field ordering
- `test_element_path_ordering` - Verifies slice/child ordering
- `test_generate_snapshot` - Snapshot generation
- `test_generate_differential` - Differential generation
- `test_serialize_*` - Element serialization tests
- `test_export_basic` - Basic export workflow
- `test_deterministic_output` - Same IR produces identical JSON
- `test_differential_only_export` - Differential-only config
- `test_snapshot_only` - Snapshot-only config
- And more...

## Usage Example

```rust
use niten::export::{StructureDefinitionExporter, ExportConfig};
use niten::ir::ProfileDocument;

async fn export_profile(document: &ProfileDocument) -> anyhow::Result<String> {
    let mut exporter = StructureDefinitionExporter::new();
    let json = exporter.export(document).await?;
    Ok(json)
}

// Differential only
let config = ExportConfig::differential_only();
let mut exporter = StructureDefinitionExporter::with_config(config);

// Pretty-printed output
let config = ExportConfig::default().pretty();
let mut exporter = StructureDefinitionExporter::with_config(config);
```

## Dependencies
- **Backend 01**: Toolchain Alignment ✅
- **Backend 02**: IR Data Model Implementation ✅
- **Backend 03**: SD Import (for round-trip testing) ✅

## Related Files
- `src/export/mod.rs`
- `src/export/error.rs`
- `src/export/sd_exporter.rs`
- `src/export/snapshot_generator.rs`
- `src/export/differential_generator.rs`
- `src/export/deterministic.rs`
- `src/export/element_serializer.rs`
- `src/export/field_preservation.rs`

## Priority
🔴 Critical - Required for MVP

## Estimated Complexity
High - 2-3 weeks

## Actual Implementation Time
Completed in single session
