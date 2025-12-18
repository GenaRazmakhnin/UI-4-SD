# Task: StructureDefinition Import to IR

## Status: COMPLETED (Core Implementation)

## Description
Implement the importer that converts FHIR StructureDefinition JSON into the internal IR format. This must support lossless import (preserve all fields, including unknown ones) to enable safe round-tripping.

## Requirements

### R1: SD Parser
- [x] Parse StructureDefinition JSON (R4, R4B, R5)
- [x] Validate JSON structure against FHIR specification
- [x] Preserve all JSON fields, including extensions and unknown fields
- [x] Handle malformed JSON gracefully with clear error messages

### R2: Differential to Snapshot Conversion
- [x] Support import from both differential and snapshot
- [ ] If only differential is present, generate snapshot by merging with base (deferred - requires base resolution)
- [ ] Use maki's `CanonicalFacade` to resolve base definitions (deferred - integration task)
- [x] Preserve the original differential for round-trip fidelity

### R3: Element Tree Construction
- [x] Build element tree from snapshot elements
- [x] Establish parent-child relationships based on paths
- [x] Assign stable UI IDs to each element
- [x] Mark which elements are inherited vs. defined in differential

### R4: Constraint Extraction
- [x] Extract explicit constraints from differential elements:
  - [x] Cardinality (min/max)
  - [x] Type constraints
  - [x] Flags (mustSupport, isModifier, isSummary)
  - [x] Short/definition text
  - [x] Fixed/pattern values
  - [x] Terminology bindings
  - [x] Slicing definitions

### R5: Slicing Import
- [x] Detect sliced elements by discriminator presence
- [x] Build slice tree structure
- [x] Import slice definitions with names and constraints
- [x] Preserve discriminator rules and slicing rules

### R6: Unknown Field Preservation
- [x] Store all unrecognized JSON fields in a preservation map
- [x] Associate preserved fields with their element paths
- [x] Ensure preserved fields survive export

### R7: Error Handling
- [x] Validate canonical URL is present
- [x] Validate resourceType is StructureDefinition
- [x] Provide clear error messages for missing required fields
- [x] Support partial import with warnings for recoverable issues

### R8: Project Workspace Integration
- [ ] Import must support project-scoped workflows (deferred - separate integration task)

## Acceptance Criteria

- [x] Successfully imports valid SD JSON (R4, R4B, R5)
- [x] Preserves all standard FHIR fields
- [x] Preserves unknown/extension fields for lossless round-trip
- [x] Correctly builds element tree with parent-child relationships
- [x] Extracts all constraint types correctly
- [x] Handles slicing definitions (discriminators, rules, slices)
- [ ] Resolves base definitions via maki's CanonicalFacade (deferred)
- [ ] Generates snapshot from differential when needed (deferred)
- [x] Clear error messages for invalid input
- [x] Graceful handling of missing base definitions (returns error)
- [ ] Project-scoped import persists SD and IR files correctly (deferred)

## Implementation Details

### Files Created

| File | Description |
|------|-------------|
| `src/import/mod.rs` | Module root with `StructureDefinitionImporter` orchestrator |
| `src/import/error.rs` | `ImportError`, `ImportWarning`, `ImportResult` types |
| `src/import/sd_parser.rs` | `StructureDefinitionParser`, `ParsedStructureDefinition` |
| `src/import/element_builder.rs` | `ElementTreeBuilder` - builds element tree from snapshot |
| `src/import/constraint_extractor.rs` | `ConstraintExtractor` - applies differential constraints |
| `src/import/slicing_importer.rs` | `SlicingImporter` - handles slicing definitions and slices |

### Architecture

```text
StructureDefinition JSON
        │
        ▼
┌───────────────────┐
│    SD Parser      │  Parse JSON, validate structure
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Element Builder  │  Build tree from snapshot/differential
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│ Constraint Extract│  Extract cardinality, types, bindings
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Slicing Import   │  Handle discriminators, slices
└─────────┬─────────┘
          │
          ▼
   ProfileDocument (IR)
```

### Key Features

1. **Lossless Round-Trip**: Unknown fields preserved at SD and element levels
2. **Differential Tracking**: Elements marked as `Inherited` vs `Modified`
3. **Slicing Support**: Full discriminator and slice parsing
4. **Error Handling**: Clear error types with context (path, field name)
5. **FHIR Version Support**: Handles R4, R4B, R5 formats

## Test Results

```
running 58 tests
test import::element_builder::tests::test_binding_parsing ... ok
test import::constraint_extractor::tests::test_apply_nested_constraint ... ok
test import::constraint_extractor::tests::test_apply_fixed_value ... ok
test import::constraint_extractor::tests::test_skip_slice_paths ... ok
test import::constraint_extractor::tests::test_apply_must_support ... ok
test import::constraint_extractor::tests::test_apply_cardinality_constraint ... ok
test import::constraint_extractor::tests::test_apply_short_definition ... ok
test import::constraint_extractor::tests::test_apply_binding ... ok
test import::element_builder::tests::test_cardinality_parsing ... ok
test import::element_builder::tests::test_build_simple_tree ... ok
... (30 import tests + 28 IR tests)

test result: ok. 58 passed; 0 failed
```

## Dependencies
- **Backend 01**: Toolchain Alignment (completed)
- **Backend 02**: IR Data Model Implementation (completed)

## Related Files
- `src/import/mod.rs`
- `src/import/error.rs`
- `src/import/sd_parser.rs`
- `src/import/element_builder.rs`
- `src/import/constraint_extractor.rs`
- `src/import/slicing_importer.rs`

## Deferred Items (Future Tasks)
- Base definition resolution via maki's CanonicalFacade
- Snapshot generation from differential when snapshot is missing
- Project workspace file layout integration

## Priority
🔴 Critical - Required for MVP

## Estimated Complexity
High - 2-3 weeks

## Actual Complexity
Moderate - Core implementation completed in single session with 30 tests
