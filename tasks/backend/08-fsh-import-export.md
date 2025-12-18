# Task: FSH Import/Export via Maki Integration

## Status: ✅ Core Implementation Complete

## Description
Integrate with `maki-core` to support FSH import (parsing → IR) and export (IR → FSH). This enables interoperability with FSH-based workflows.

## Implementation Summary

### Completed Components

#### 1. FSH Module Structure (`src/fsh/`)
```
src/fsh/
├── mod.rs          # Module exports and documentation
├── error.rs        # FSH error types (FshError, FshImportError, FshWarning)
├── import.rs       # FshImporter, FshProjectImporter
├── export.rs       # FshExporter with maki-decompiler integration
└── mapper.rs       # FshToIrMapper (semantic model → IR)
```

#### 2. FSH Import (`FshImporter`)
- Uses `maki-core::FshParser` for parsing FSH content
- Uses `maki-core::DefaultSemanticAnalyzer` for semantic analysis
- Integrates with `FishingContext` for dependency resolution
- **Respects project configuration** for canonical URL and FHIR version
- Supports both single file and project-level imports

#### 3. FSH to IR Mapping (`FshToIrMapper`)
- Maps `FhirResource` → `ProfileDocument`
- Maps FSH elements to IR `ElementNode` with constraints:
  - Cardinality rules
  - Type constraints
  - Flags (MS, SU, ?!)
  - Value set bindings
  - Fixed/pattern values
- Preserves metadata (title, description, status, etc.)

#### 4. FSH Export (`FshExporter`)
- Strategy: IR → SD → FSH (via `maki-decompiler`)
- Provides fallback `generate_basic_fsh()` for decompiler unavailable
- Supports export formatting options (line endings, indentation)

#### 5. Multi-File Project Import (`FshProjectImporter`)
- Recursively discovers `.fsh` files in project directories
- Searches standard locations: `input/fsh/`, `fsh/`, root
- Uses shared `FshTank` for cross-file reference resolution

#### 6. Project Configuration (`ProjectConfig`)
- Added `project.json` support with:
  - `canonical`: Base URL for canonical URLs
  - `fhir_version`: FHIR version string
  - `name`, `description`, `publisher` metadata
- FSH import respects project configuration (no hardcoded values)

#### 7. API Integration
- `POST /api/projects/:projectId/profiles/:profileId/import`
  - Accepts `format: "fsh"` for FSH content
  - Loads project config for canonical URL and FHIR version
  - Returns imported profile with diagnostics/warnings

### Tests (7 passing)
- `test_escape_fsh_string` - String escaping for FSH output
- `test_map_cardinality` - Cardinality mapping
- `test_import_options_builder` - Options builder pattern
- `test_parse_binding` - Binding strength parsing
- `test_generate_basic_fsh` - Basic FSH generation
- `test_map_profile_basic` - Profile mapping
- `test_import_simple_fsh` - End-to-end FSH import

## Requirements Status

### R1: FSH Import (Parsing) ✅
- [x] Use `maki-core` parser to parse FSH content
- [x] Convert Rowan CST to semantic model
- [x] Extract profile definitions, rules, and metadata
- [x] Map FSH semantic model to IR

### R2: Semantic Model to IR Mapping ✅
- [x] **Profile declarations** → ProfileDocument
- [x] **Cardinality rules** → ElementConstraints.cardinality
- [x] **Type rules** → ElementConstraints.type_constraints
- [x] **Flag rules** (MS, SU, etc.) → ElementConstraints.flags
- [x] **Binding rules** → ElementConstraints.binding
- [x] **Slicing rules** → SlicingDefinition (basic)
- [x] **Extension rules** → Extension application (basic)
- [ ] **Invariant rules** → FHIRPath constraints (TODO)
- [x] **Fixed/pattern rules** → Fixed/pattern values

### R3: FSH Export Strategy ✅
- [x] **Option A**: IR → SD → FSH (via maki-decompiler) - **Implemented**
- [ ] **Option B**: Direct IR → FSH emitter (future enhancement)
- [x] Deterministic output via decompiler
- [x] Writing to workspace supported via storage

### R4: Fishing Context Integration ✅
- [x] Use `maki-core`'s `FishingContext` to resolve references
- [x] Load base definitions from packages (via CanonicalFacade)
- [x] Resolve extension URLs
- [x] Resolve ValueSet URLs

### R5: Dependency Resolution ✅
- [x] Load dependencies via `CanonicalFacade`
- [x] Configure FHIR releases (R4, R5)
- [x] Use project configuration for canonical base

### R6: Multi-File Support ✅
- [x] Import multiple FSH files from a directory
- [x] Default project source directory patterns
- [x] Resolve cross-file references (via shared tank)
- [x] Build complete project IR from FSH sources

### R7: Workspace Storage Integration ✅
- [x] Save imported FSH source files
- [x] Save resulting IR documents
- [x] API integration for import endpoint

### R8: Error Handling ✅
- [x] Preserve FSH parser diagnostics
- [x] Map FSH errors to file locations
- [x] Provide actionable error messages
- [x] Support partial import with warnings

### R9: Round-Trip Fidelity ⏳
- [ ] Validate semantic equivalence (needs integration testing)
- [ ] Document known limitations
- [ ] Preserve FSH comments (future)

## Acceptance Criteria

- [x] Successfully parses valid FSH files
- [x] Converts FSH semantic model to IR correctly
- [x] Most FSH rule types are mapped to IR
- [x] Exports IR to valid FSH (via decompiler)
- [ ] Exported FSH compiles with SUSHI (needs integration testing)
- [ ] Exported FSH produces semantically equivalent SD (needs integration testing)
- [x] FishingContext resolves dependencies correctly
- [x] Multi-file FSH projects import correctly
- [x] FSH parser errors are surfaced clearly
- [x] Uses project configuration for import settings

## Dependencies
- **Backend 01**: Toolchain Alignment (maki-core dependency) ✅
- **Backend 02**: IR Data Model Implementation ✅
- **Backend 03**: SD Import (for comparison) ✅
- **Backend 04**: SD Export (for validation) ✅

## Related Files
- `src/fsh/mod.rs` - Module entry point
- `src/fsh/error.rs` - Error types and warnings
- `src/fsh/import.rs` - FshImporter, FshProjectImporter
- `src/fsh/export.rs` - FshExporter
- `src/fsh/mapper.rs` - FshToIrMapper
- `src/api/profiles.rs` - FSH import endpoint
- `src/api/storage.rs` - ProjectConfig storage

## Priority
🟡 High - Required for Beta

## Known Limitations
1. Invariant rules (obeys) not fully mapped
2. Complex slicing discriminators need more testing
3. RuleSet expansion not implemented
4. sushi-config.yaml parsing not implemented (uses project.json)

## Usage Examples

### API FSH Import
```bash
curl -X POST "http://localhost:3000/api/projects/my-project/profiles/my-profile/import" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "fsh",
    "content": "Profile: MyPatient\nParent: Patient\n* name 1..* MS",
    "replace": true
  }'
```

### Project Configuration (`project.json`)
```json
{
  "version": 1,
  "canonical": "http://example.org/fhir",
  "fhir_version": "4.0.1",
  "name": "My Implementation Guide",
  "publisher": "My Organization"
}
```

### Programmatic Usage
```rust
use niten::fsh::{FshImporter, FshImportOptions};
use niten::ir::FhirVersion;

let options = FshImportOptions::default()
    .with_canonical_base("http://example.org/fhir")
    .with_fhir_version(FhirVersion::R4);

let importer = FshImporter::with_options(options).await?;
let result = importer.import_file("profile.fsh").await?;

for doc in result.value {
    println!("Imported: {}", doc.metadata.name);
}
```
