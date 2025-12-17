# Task: Round-Trip Fidelity Testing

## ⚠️ TESTING DISABLED FOR RAPID DEVELOPMENT

This task is **SKIPPED** - backend testing is forbidden during rapid development phase.

## Description
~~Implement comprehensive round-trip testing to ensure lossless import/export of StructureDefinition and semantic fidelity for FSH.~~

## Requirements

### R1: SD Round-Trip Tests
Test: **SD Import → IR → SD Export → Compare**

Scenarios:
- Basic profiles (US Core Patient, Observation)
- Complex slicing (mCODE)
- Extensions (multiple extensions per element)
- Deep inheritance chains
- Large profiles (500+ elements)
- All FHIR versions (R4, R4B, R5)

Assertions:
- Exported SD validates with IG Publisher
- Unknown fields are preserved
- Semantic equivalence (same constraints)
- Byte-identical export (deterministic)

### R2: FSH Round-Trip Tests
Test: **FSH → IR → FSH → SUSHI Compile → Compare SD**

Scenarios:
- Profile definitions with all rule types
- Extension definitions
- ValueSet definitions
- Instance examples
- Complex slicing rules
- Invariants with FHIRPath

Assertions:
- Exported FSH compiles with SUSHI
- Generated SD is semantically equivalent to original
- No silent information loss

### R3: Lossless Preservation Tests
Test unknown JSON fields are preserved:

Test cases:
- Custom extensions
- Vendor-specific fields
- Future FHIR fields (unknown to tool)
- Non-standard metadata

Assertions:
- All unknown fields survive import → export
- Field placement is correct
- Field order is preserved where possible

### R4: Determinism Tests
Test: **Same IR → Multiple Exports → Byte-Identical**

Test cases:
- Export same profile 100 times
- Compare byte-by-byte
- Test with different element ordering in IR
- Test with different operation history

Assertions:
- All exports are byte-identical
- JSON field ordering is stable
- Array ordering is deterministic

### R5: Parity Tests (IG Publisher)
Test: **Exported SD → IG Publisher → Validation**

Test suite:
- All US Core profiles
- All IPA profiles
- Selected mCODE profiles
- Edge cases from spec

Assertions:
- IG Publisher validates successfully
- No errors introduced by our tool
- Warnings are expected/documented

### R6: SUSHI Comparison Tests
Test: **FSH → Our Tool → SD vs FSH → SUSHI → SD**

Test cases:
- Identical FSH input to both tools
- Compare generated SDs semantically
- Document differences

Assertions:
- Semantic equivalence (same constraints)
- Differences are documented and justified
- No functional regressions

### R7: Regression Test Suite
- Maintain golden test files
- Automated CI testing
- Track parity metrics over time
- No new false negatives allowed

### R8: Test Data Organization
```
tests/
├── round_trip/
│   ├── sd/
│   │   ├── us_core/
│   │   ├── ipa/
│   │   ├── mcode/
│   │   └── edge_cases/
│   ├── fsh/
│   │   ├── profiles/
│   │   ├── extensions/
│   │   └── valuesets/
│   └── golden/
│       ├── exported_sd/
│       └── exported_fsh/
└── fixtures/
    └── real_world_profiles/
```

### R9: Test Automation
- CI/CD integration
- Nightly regression runs
- Performance benchmarks
- Coverage reports

## Acceptance Criteria

**All testing requirements removed - rapid development mode enabled**

## Dependencies
- **Backend 03**: SD Import
- **Backend 04**: SD Export
- **Backend 08**: FSH Import/Export

## Related Files
- `tests/round_trip_sd.rs` (new)
- `tests/round_trip_fsh.rs` (new)
- `tests/determinism.rs` (new)
- `tests/parity.rs` (new)
- `tests/fixtures/` (test data)
- `tests/golden/` (expected outputs)

## Priority
🔴 Critical - Quality assurance

## Estimated Complexity
High - 2-3 weeks
