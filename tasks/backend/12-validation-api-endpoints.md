# Task: Validation API Endpoints

## Description
Implement API endpoints for profile validation, including incremental validation, on-demand full validation, and publisher parity checks.

## Requirements

### R1: Validate Profile
**POST `/api/projects/:projectId/profiles/:profileId/validate`**
- Request body: validation options (level, include_terminology, run_parity)
- Run validation at specified level:
  - `fast`: IR validation only
  - `full`: IR + terminology + cross-reference
  - `parity`: IR + full + IG Publisher validation
- Return validation diagnostics
- Cache validation results

### R2: Validate Element
**POST `/api/projects/:projectId/profiles/:profileId/elements/:path/validate`**
- Validate a specific element and its constraints
- Return element-specific diagnostics
- Include inherited constraint validation
- Suggest quick fixes

### R3: Get Validation Results
**GET `/api/projects/:projectId/profiles/:profileId/validation`**
- Return cached validation results
- Include validation metadata (timestamp, level)
- Return 404 if no validation has been run

### R4: Publisher Parity Check
**POST `/api/projects/:projectId/profiles/:profileId/parity`**
- Export profile to SD JSON
- Run HL7 Validator against SD
- Parse and map validator output
- Store parity report
- Return mapped diagnostics

### R5: Batch Validation
**POST `/api/projects/:projectId/validate/batch`**
- Request body: array of profile IDs
- Validate multiple profiles
- Return validation results for each
- Run in parallel for performance

### R6: Validation Diagnostics Format
```json
{
  "profileId": "...",
  "isValid": true,
  "level": "full",
  "timestamp": "...",
  "diagnostics": [
    {
      "severity": "error",
      "code": "CARDINALITY_CONFLICT",
      "message": "Min cardinality (2) exceeds max (1)",
      "elementPath": "Patient.name",
      "line": null,
      "quickFix": {
        "title": "Set max to 2",
        "operation": { /* ... */ }
      }
    }
  ],
  "stats": {
    "errors": 1,
    "warnings": 3,
    "info": 0
  }
}
```

### R7: Quick Fix Application
**POST `/api/projects/:projectId/profiles/:profileId/apply-fix`**
- Request body: quick fix operation
- Apply suggested fix
- Re-validate element
- Return updated element state

### R8: Validation Configuration
**GET/PUT `/api/validation/config`**
- Get/set validation configuration:
  - Terminology service URL
  - Parity harness path
  - Validation level defaults
  - Cache settings

## Acceptance Criteria

- [x] Validation endpoint runs at specified level
- [x] Fast validation returns results <100ms
- [x] Full validation includes terminology checks
- [ ] Parity validation runs HL7 Validator (requires external validator binary)
- [x] Element validation works correctly
- [x] Batch validation processes multiple profiles
- [x] Validation results are cached
- [x] Cache invalidates on profile changes
- [x] Quick fixes are suggested appropriately
- [x] Quick fix application works
- [x] Diagnostics map to correct elements
- [x] Error messages are actionable
- [x] Configuration endpoints work
- [x] Performance meets targets (<100ms fast, <500ms full)

## Dependencies
- **Backend 09**: Validation Engine

## Related Files
- `src/api/validation.rs` - Validation API endpoints
- `src/state.rs` - Validation caching (CachedValidation, ValidationConfig)
- `src/ir/constraint.rs` - BindingStrength::from_str added
- `src/ir/element.rs` - find_descendant_mut added
- `src/ir/resource.rs` - find_element_mut added

## Implementation Notes

### Completed (2024-12)

**R1: Validate Profile** ✅
- POST `/api/projects/:projectId/profiles/:profileId/validate`
- Supports validation levels: Structural, References, Terminology, Full
- Returns diagnostics with stats (error/warning/info counts)
- Caches results in AppState

**R2: Validate Element** ✅
- POST `/api/projects/:projectId/profiles/:profileId/elements/:path/validate`
- Validates specific element constraints
- Returns element-scoped diagnostics with quick fixes

**R3: Get Validation Results** ✅
- GET `/api/projects/:projectId/profiles/:profileId/validation`
- Returns cached validation results
- Returns 404 if no cached validation exists

**R4: Publisher Parity Check** ⏳
- Endpoint exists but requires external HL7 Validator binary
- Configuration supports `hl7_validator_path` setting

**R5: Batch Validation** ✅
- POST `/api/projects/:projectId/validate/batch`
- Accepts array of profile IDs
- Returns results for each with aggregate counts

**R6: Validation Diagnostics Format** ✅
- Enhanced with `stats` field containing error/warning/info counts
- All fields match specification

**R7: Quick Fix Application** ✅
- POST `/api/projects/:projectId/profiles/:profileId/apply-fix`
- Applies quick fix to profile document
- Re-validates and returns updated state
- Supports: SetCardinality, SetBindingStrength, AddMustSupport

**R8: Validation Configuration** ✅
- GET `/api/validation/config` - Returns current config
- PUT `/api/validation/config` - Updates config
- Supports: default_level, terminology_service_url, hl7_validator_path, cache_enabled, cache_ttl_seconds

### State Management
- `CachedValidation` stores results with profile modification timestamp
- Cache invalidation on profile changes via `invalidate_validation()`
- Project-wide invalidation via `invalidate_project_validations()`

## Priority
🔴 Critical - Core functionality

## Status
🟢 **COMPLETE** - All API endpoints implemented except HL7 Validator parity (requires external binary)
