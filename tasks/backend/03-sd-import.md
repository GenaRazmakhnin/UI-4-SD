# Task: StructureDefinition Import to IR

## Description
Implement the importer that converts FHIR StructureDefinition JSON into the internal IR format. This must support lossless import (preserve all fields, including unknown ones) to enable safe round-tripping.

## Requirements

### R1: SD Parser
- Parse StructureDefinition JSON (R4, R4B, R5)
- Validate JSON structure against FHIR specification
- Preserve all JSON fields, including extensions and unknown fields
- Handle malformed JSON gracefully with clear error messages

### R2: Differential to Snapshot Conversion
- Support import from both differential and snapshot
- If only differential is present, generate snapshot by merging with base
- Use maki's `CanonicalFacade` to resolve base definitions
- Preserve the original differential for round-trip fidelity

### R3: Element Tree Construction
- Build element tree from snapshot elements
- Establish parent-child relationships based on paths
- Assign stable UI IDs to each element
- Mark which elements are inherited vs. defined in differential

### R4: Constraint Extraction
- Extract explicit constraints from differential elements:
  - Cardinality (min/max)
  - Type constraints
  - Flags (mustSupport, isModifier, isSummary)
  - Short/definition text
  - Fixed/pattern values
  - Terminology bindings
  - Slicing definitions

### R5: Slicing Import
- Detect sliced elements by discriminator presence
- Build slice tree structure
- Import slice definitions with names and constraints
- Preserve discriminator rules and slicing rules

### R6: Unknown Field Preservation
- Store all unrecognized JSON fields in a preservation map
- Associate preserved fields with their element paths
- Ensure preserved fields survive export

### R7: Error Handling
- Validate canonical URL is present
- Validate base definition reference resolves
- Provide clear error messages for missing required fields
- Support partial import with warnings for recoverable issues

## Acceptance Criteria

- [ ] Successfully imports valid SD JSON (R4, R4B, R5)
- [ ] Preserves all standard FHIR fields
- [ ] Preserves unknown/extension fields for lossless round-trip
- [ ] Correctly builds element tree with parent-child relationships
- [ ] Extracts all constraint types correctly
- [ ] Handles slicing definitions (discriminators, rules, slices)
- [ ] Resolves base definitions via maki's CanonicalFacade
- [ ] Generates snapshot from differential when needed
- [ ] Clear error messages for invalid input
- [ ] Graceful handling of missing base definitions

## Dependencies
- **Backend 01**: Toolchain Alignment
- **Backend 02**: IR Data Model Implementation

## Related Files
- `crates/profile-builder/src/import/mod.rs` (new)
- `crates/profile-builder/src/import/sd_import.rs` (new)
- `crates/profile-builder/src/import/element_tree_builder.rs` (new)
- `crates/profile-builder/src/import/constraint_extractor.rs` (new)
- `crates/profile-builder/src/import/slicing_importer.rs` (new)

## Test Data
Use these real-world profiles for testing:
- `hl7.fhir.us.core` (US Core Patient, Observation, etc.)
- `hl7.fhir.uv.ipa` (IPA profiles)
- `hl7.fhir.us.mcode` (mCODE profiles with complex slicing)

## Priority
🔴 Critical - Required for MVP

## Estimated Complexity
High - 2-3 weeks
