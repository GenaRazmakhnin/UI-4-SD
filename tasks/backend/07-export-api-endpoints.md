# Task: Export API Endpoints

**Status:** ✅ Completed
**Started:** 2025-12-19
**Completed:** 2025-12-19
**Last Updated:** 2025-12-19

## Description
Implement API endpoints for exporting profiles to StructureDefinition JSON and FSH formats with deterministic output.

## Implementation Summary

### Files Created
- `src/api/export_dto.rs` - DTOs for export request/response types
- `src/api/export.rs` - Export route handlers implementation

### Files Modified
- `src/api/mod.rs` - Added export module exports
- `src/api/profiles.rs` - Made ErrorResponse public for reuse
- `src/server.rs` - Wired up export routes
- `Cargo.toml` - Added `sha2` and `zip` dependencies

## Requirements

### R1: Export to StructureDefinition ✅
**GET `/api/projects/:projectId/profiles/:profileId/export/sd`**
- Export resource IR to SD JSON (only applicable to `StructureDefinition` resources)
- Support both differential and snapshot
- Query parameters:
  - `format`: "differential" | "snapshot" | "both" (default: "both")
  - `pretty`: boolean (default: false for deterministic output)
- Optional query parameter:
  - `persist`: boolean (default: false). If true, write exported files into `SD/` under the project directory.
  - `force`: boolean (default: false). If true, export despite validation warnings.
- Return valid FHIR StructureDefinition JSON
- Content-Type: `application/fhir+json`

### R2: Export to FSH ✅
**GET `/api/projects/:projectId/profiles/:profileId/export/fsh`**
- Export resource IR to FSH format (Profile / Extension / ValueSet where supported)
- Uses maki-decompiler for high-quality FSH output (SD JSON → FSH decompilation)
- Return valid FSH content
- Content-Type: `text/plain; charset=utf-8`
- Optional query parameter:
  - `persist`: boolean (default: false). If true, write exported files into `FSH/` under the project directory.
  - `force`: boolean (default: false). If true, export despite validation warnings.

### R3: Bulk Export (Project Level) ✅
**GET `/api/projects/:projectId/export`**
- Export all project resources (profiles, extensions, ValueSets)
- Query parameters:
  - `format`: "sd" | "fsh" | "both" (default: "both")
  - `structure`: "flat" | "packaged" (default: "flat")
  - `pretty`: boolean (default: false)
- Flat mode: Returns JSON response with all files
- Packaged mode: Returns ZIP archive with:
  - IG scaffold (`ig.ini`, `ImplementationGuide-*.json`)
  - SUSHI config (`sushi-config.yaml`) when FSH included
  - All SD files in `input/resources/`
  - All FSH files in `input/fsh/profiles/`

### R4: Download with Filename ✅
- Proper Content-Disposition headers
- Filename includes profile name/id
- Format-specific file extensions (.json, .fsh, .zip)

### R5: Preview Mode ✅
**GET `/api/projects/:projectId/profiles/:profileId/preview`**
- Return formatted preview without downloading
- Support both SD and FSH formats via `format` query param
- Include syntax highlighting metadata via `highlight` query param
- JSON and FSH tokenization for syntax highlighting
- Used for in-app preview panel

### R6: Validation Before Export ✅
- Validate IR state before export
- Return 422 if profile has validation errors
- Option to force export despite warnings (`force=true`)
- Include validation diagnostics in response body
- Validates:
  - Required metadata (url, name, baseDefinition)
  - Name format (uppercase, no spaces)
  - Cardinality consistency
  - Slicing discriminators

### R7: Caching ✅
- Generate ETag based on content SHA-256 hash (truncated to 16 chars)
- Support If-None-Match for conditional requests (304 Not Modified)
- HEAD endpoints for SD/FSH exports to check ETag without full export
- Cache-Control headers: `private, must-revalidate`

## Acceptance Criteria

- [x] SD export produces valid StructureDefinition JSON
- [ ] Exported SD passes IG Publisher validation (requires integration test)
- [ ] Exported SD passes HL7 Validator (requires integration test)
- [x] FSH export produces valid FSH syntax
- [ ] Exported FSH compiles with SUSHI (requires integration test)
- [x] Deterministic output: same profile → identical export (via ExportConfig)
- [x] Bulk export includes all project resources
- [x] Packaged export includes IG scaffold
- [x] Proper Content-Type headers
- [x] Proper Content-Disposition with filename
- [x] Preview mode returns formatted content
- [x] Validation errors prevent export (or warn)
- [x] ETag caching works correctly
- [x] Cache invalidation on modification (ETag changes with content)
- [x] Error handling for invalid profiles

## API Endpoints Summary

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/projects/:projectId/profiles/:profileId/export/sd` | Export profile as SD JSON |
| HEAD | `/api/projects/:projectId/profiles/:profileId/export/sd` | Get SD export headers/ETag |
| GET | `/api/projects/:projectId/profiles/:profileId/export/fsh` | Export profile as FSH |
| HEAD | `/api/projects/:projectId/profiles/:profileId/export/fsh` | Get FSH export headers/ETag |
| GET | `/api/projects/:projectId/profiles/:profileId/preview` | Preview profile content |
| GET | `/api/projects/:projectId/export` | Bulk export all profiles |

## Dependencies
- **Backend 04**: SD Export ✅ (Used `StructureDefinitionExporter`)
- **Backend 05**: Axum Server Setup ✅ (Routes integrated)
- **Backend 08**: FSH Import/Export ✅ (Uses maki-decompiler for SD→FSH)
- **Backend 15**: Project Management ✅ (Uses `ProfileStorage`)

## Related Files
- `src/api/export.rs` - Main export route handlers
- `src/api/export_dto.rs` - Export DTOs
- `src/decompiler.rs` - FSH decompiler service (maki-decompiler integration)
- `src/export/sd_exporter.rs` - SD export engine (existing)

## Priority
🔴 Critical - Required for MVP

## Notes
- FSH export uses maki-decompiler exclusively (SD JSON → FSH) for high-quality output
- Updated maki-decompiler to be thread-safe (`Send + Sync` bounds on `Exportable` and `ExportableRule`)
- No fallback FSH generator - if improvements needed, improve the decompiler itself
- Syntax highlighting is basic tokenization for JSON/FSH (keyword, string, number, comment)
- Packaged export generates valid SUSHI project structure
- Integration tests with IG Publisher and SUSHI validators would provide additional coverage

## Implementation Details

### FSH Export Flow
1. Export IR → SD JSON using `StructureDefinitionExporter`
2. Parse SD JSON → `maki_decompiler::models::StructureDefinition`
3. Process with `StructureDefinitionProcessor` → `Exportable`
4. Call `.to_fsh()` → FSH string
5. Return error if decompilation fails (no fallback)

### Thread-Safety Updates (maki-decompiler)
- Added `Send + Sync` bounds to `ExportableRule` trait
- Added `Send + Sync` bounds to `Exportable` trait
- Updated all `Box<dyn ExportableRule>` to `Box<dyn ExportableRule + Send + Sync>`
- Updated all `Box<dyn Exportable>` to `Box<dyn Exportable + Send + Sync>`
