# Task: Validation API Endpoints

## Description
Implement API endpoints for profile validation, including incremental validation, on-demand full validation, and publisher parity checks.

## Requirements

### R1: Validate Profile
**POST `/api/profiles/:id/validate`**
- Request body: validation options (level, include_terminology, run_parity)
- Run validation at specified level:
  - `fast`: IR validation only
  - `full`: IR + terminology + cross-reference
  - `parity`: IR + full + IG Publisher validation
- Return validation diagnostics
- Cache validation results

### R2: Validate Element
**POST `/api/profiles/:id/elements/:path/validate`**
- Validate a specific element and its constraints
- Return element-specific diagnostics
- Include inherited constraint validation
- Suggest quick fixes

### R3: Get Validation Results
**GET `/api/profiles/:id/validation`**
- Return cached validation results
- Include validation metadata (timestamp, level)
- Return 404 if no validation has been run

### R4: Publisher Parity Check
**POST `/api/profiles/:id/parity`**
- Export profile to SD JSON
- Run HL7 Validator against SD
- Parse and map validator output
- Store parity report
- Return mapped diagnostics

### R5: Batch Validation
**POST `/api/validate/batch`**
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
**POST `/api/profiles/:id/apply-fix`**
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

- [ ] Validation endpoint runs at specified level
- [ ] Fast validation returns results <100ms
- [ ] Full validation includes terminology checks
- [ ] Parity validation runs HL7 Validator
- [ ] Element validation works correctly
- [ ] Batch validation processes multiple profiles
- [ ] Validation results are cached
- [ ] Cache invalidates on profile changes
- [ ] Quick fixes are suggested appropriately
- [ ] Quick fix application works
- [ ] Diagnostics map to correct elements
- [ ] Error messages are actionable
- [ ] Configuration endpoints work
- [ ] Performance meets targets (<100ms fast, <500ms full)

## Dependencies
- **Backend 09**: Validation Engine

## Related Files
- `crates/server/src/routes/validation.rs` (new)
- `crates/server/src/api/validation_dto.rs` (new)

## Priority
🔴 Critical - Core functionality

## Estimated Complexity
Medium - 1-2 weeks
