# Task: FSH Import/Export via Maki Integration

## Description
Integrate with `maki-core` to support FSH import (parsing → IR) and export (IR → FSH). This enables interoperability with FSH-based workflows.

## Requirements

### R1: FSH Import (Parsing)
- Use `maki-core` parser to parse FSH content
- Convert Rowan CST to semantic model
- Extract profile definitions, rules, and metadata
- Map FSH semantic model to IR

### R2: Semantic Model to IR Mapping
Map FSH constructs to IR:
- **Profile declarations** → ProfileDocument
- **Cardinality rules** → ElementConstraints.cardinality
- **Type rules** → ElementConstraints.type_constraints
- **Flag rules** (MS, SU, etc.) → ElementConstraints.flags
- **Binding rules** → ElementConstraints.binding
- **Slicing rules** → SlicingDefinition
- **Extension rules** → Extension application
- **Invariant rules** → FHIRPath constraints
- **Fixed/pattern rules** → Fixed/pattern values

### R3: FSH Export Strategy
Choose export approach:
- **Option A**: IR → SD → FSH (via maki decompiler/GoFSH)
- **Option B**: Direct IR → FSH emitter (higher quality)
- Implement chosen strategy with deterministic output

### R4: Fishing Context Integration
- Use `maki-core`'s `FishingContext` to resolve references
- Load base definitions from packages
- Resolve extension URLs
- Resolve ValueSet URLs

### R5: Dependency Resolution
- Parse `sushi-config.yaml` for package dependencies
- Load dependencies via `CanonicalFacade`
- Ensure all referenced resources are available

### R6: Multi-File Support
- Import multiple FSH files from a directory
- Resolve cross-file references
- Build complete project IR from FSH sources

### R7: Error Handling
- Preserve FSH parser diagnostics
- Map FSH errors to IR element paths
- Provide actionable error messages
- Support partial import with warnings

### R8: Round-Trip Fidelity
- Aim for semantic equivalence (not textual)
- Preserve FSH comments where possible (future)
- Document known round-trip limitations

## Acceptance Criteria

- [ ] Successfully parses valid FSH files
- [ ] Converts FSH semantic model to IR correctly
- [ ] All FSH rule types are mapped to IR
- [ ] Exports IR to valid FSH
- [ ] Exported FSH compiles with SUSHI
- [ ] Exported FSH produces semantically equivalent SD
- [ ] FishingContext resolves dependencies correctly
- [ ] Multi-file FSH projects import correctly
- [ ] FSH parser errors are surfaced clearly
- [ ] Documentation for FSH workflow

## Dependencies
- **Backend 01**: Toolchain Alignment (maki-core dependency)
- **Backend 02**: IR Data Model Implementation
- **Backend 03**: SD Import (for comparison)
- **Backend 04**: SD Export (for validation)

## Related Files
- `crates/profile-builder/src/import/fsh_import.rs` (new)
- `crates/profile-builder/src/import/fsh_semantic_mapper.rs` (new)
- `crates/profile-builder/src/export/fsh_export.rs` (new)
- `crates/profile-builder/src/export/fsh_emitter.rs` (new)

## Priority
🟡 High - Required for Beta

## Estimated Complexity
Very High - 3-4 weeks
