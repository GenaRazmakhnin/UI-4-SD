# Task: Export API Endpoints

## Description
Implement API endpoints for exporting profiles to StructureDefinition JSON and FSH formats with deterministic output.

## Requirements

### R1: Export to StructureDefinition
**GET `/api/profiles/:id/export/sd`**
- Export profile IR to SD JSON
- Support both differential and snapshot
- Query parameters:
  - `format`: "differential" | "snapshot" | "both" (default: "both")
  - `pretty`: boolean (default: false for deterministic output)
- Return valid FHIR StructureDefinition JSON
- Content-Type: `application/fhir+json`

### R2: Export to FSH
**GET `/api/profiles/:id/export/fsh`**
- Export profile IR to FSH format
- Use maki's decompiler or direct IR→FSH emitter
- Return valid FSH content
- Content-Type: `text/plain` or `application/fsh`

### R3: Bulk Export (Project Level)
**GET `/api/profiles/export`**
- Export all profiles in current project
- Query parameters:
  - `format`: "sd" | "fsh" | "both"
  - `structure`: "flat" | "packaged" (return as files or package.tgz)
- Return ZIP archive or tarball with all resources
- Include ImplementationGuide scaffold if `packaged` mode

### R4: Download with Filename
- Proper Content-Disposition headers
- Filename includes profile name/id
- Format-specific file extensions (.json, .fsh)

### R5: Preview Mode
**GET `/api/profiles/:id/preview`**
- Return formatted preview without downloading
- Support both SD and FSH formats
- Include syntax highlighting metadata
- Used for in-app preview panel

### R6: Validation Before Export
- Validate IR state before export
- Return 422 if profile has validation errors
- Option to force export despite warnings
- Include validation diagnostics in response headers

### R7: Caching
- Generate ETag based on profile state hash
- Support If-None-Match for conditional requests
- Cache exported artifacts for performance
- Invalidate cache on profile modification

## Acceptance Criteria

- [ ] SD export produces valid StructureDefinition JSON
- [ ] Exported SD passes IG Publisher validation
- [ ] Exported SD passes HL7 Validator
- [ ] FSH export produces valid FSH
- [ ] Exported FSH compiles with SUSHI
- [ ] Deterministic output: same profile → identical export
- [ ] Bulk export includes all profiles
- [ ] Packaged export includes IG scaffold
- [ ] Proper Content-Type headers
- [ ] Proper Content-Disposition with filename
- [ ] Preview mode returns formatted content
- [ ] Validation errors prevent export (or warn)
- [ ] ETag caching works correctly
- [ ] Cache invalidation on modification
- [ ] Error handling for invalid profiles

## Dependencies
- **Backend 04**: SD Export
- **Backend 05**: Axum Server Setup
- **Backend 08**: FSH Import/Export (for FSH export)

## Related Files
- `crates/server/src/routes/export.rs` (new)
- `crates/server/src/api/export_dto.rs` (new)
- `crates/server/src/cache/export_cache.rs` (new)

## Priority
🔴 Critical - Required for MVP

## Estimated Complexity
Medium - 1-2 weeks
