# Task: Profile API Endpoints

## Description
Implement REST API endpoints for profile CRUD operations, including create, read, update, import, export, and validation.

## Requirements

### R1: Profile Listing
**GET `/api/profiles`**
- Return list of all open/loaded profiles
- Include profile metadata (id, url, name, status)
- Support filtering by FHIR version
- Support pagination for large lists

### R2: Create New Profile
**POST `/api/profiles`**
- Request body: base resource type, FHIR version, metadata
- Create new profile IR from base definition
- Assign unique ID
- Return created profile with ID

### R3: Get Profile Details
**GET `/api/profiles/:id`**
- Return complete profile IR state
- Include element tree with constraints
- Include edit history metadata
- Return 404 if profile not found

### R4: Update Profile Element
**PATCH `/api/profiles/:id/elements/:path`**
- Request body: constraint updates (cardinality, flags, etc.)
- Apply constraint to specified element path
- Record operation in edit history
- Run incremental validation
- Return updated element state + validation results

### R5: Import Profile
**POST `/api/profiles/:id/import`**
- Request body: SD JSON or FSH content + format indicator
- Import into existing profile or create new
- Support both SD and FSH formats
- Return profile IR + import diagnostics

### R6: Delete Profile
**DELETE `/api/profiles/:id`**
- Remove profile from loaded state
- Confirm if unsaved changes exist
- Return 204 on success

### R7: Profile Metadata Update
**PATCH `/api/profiles/:id/metadata`**
- Update profile metadata (name, title, description, status, etc.)
- Validate metadata fields
- Return updated profile

### R8: Error Handling
- 400 Bad Request for invalid input
- 404 Not Found for missing profiles
- 409 Conflict for concurrent modifications
- 422 Unprocessable Entity for validation failures
- Detailed error messages in JSON format

### R9: Response Format
Consistent JSON response structure:
```json
{
  "data": { /* profile data */ },
  "diagnostics": [ /* warnings/errors */ ],
  "metadata": {
    "timestamp": "...",
    "version": 1
  }
}
```

## Acceptance Criteria

- [ ] All endpoints return correct HTTP status codes
- [ ] Profile listing endpoint returns all profiles
- [ ] Create endpoint generates valid new profiles
- [ ] Get endpoint returns complete profile state
- [ ] Update endpoint applies constraints correctly
- [ ] Import endpoint handles SD and FSH formats
- [ ] Delete endpoint removes profiles
- [ ] Metadata update endpoint validates fields
- [ ] All endpoints have proper error handling
- [ ] Response format is consistent across endpoints
- [ ] Request validation rejects invalid input
- [ ] Concurrent modification detection works
- [ ] API documentation (OpenAPI/Swagger)

## Dependencies
- **Backend 02**: IR Data Model Implementation
- **Backend 03**: SD Import
- **Backend 04**: SD Export
- **Backend 05**: Axum Server Setup

## Related Files
- `crates/server/src/routes/profiles.rs` (new)
- `crates/server/src/api/types.rs` (new)
- `crates/server/src/api/profile_dto.rs` (new)

## Priority
🔴 Critical - Core API functionality

## Estimated Complexity
High - 2 weeks
