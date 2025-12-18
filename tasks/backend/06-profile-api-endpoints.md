# Task: Profile API Endpoints

## Description
Implement REST API endpoints for profile CRUD operations, including create, read, update, import, export, and validation.

## Requirements

### R1: Profile Listing
**GET `/api/projects/:projectId/profiles`**
- Return list of all profiles in a project (Profile = `StructureDefinition` that is editable in the Profile Editor)
- Include profile metadata (id, url, name, status)
- Include `resourceKind` to distinguish Profile vs Extension SDs
- Support filtering by FHIR version
- Support pagination for large lists

### R2: Create New Profile
**POST `/api/projects/:projectId/profiles`**
- Request body: base resource type, FHIR version, metadata
- Create new profile IR from base definition
- Assign unique ID
- Return created profile with ID
- Persist on disk under the project directory:
  - `IR/resources/<profileId>.json`
  - Update `IR/index.json`

### R3: Get Profile Details
**GET `/api/projects/:projectId/profiles/:profileId`**
- Return complete profile IR state
- Include element tree with constraints
- Include edit history metadata
- Return 404 if profile not found

### R4: Update Profile Element
**PATCH `/api/projects/:projectId/profiles/:profileId/elements/:path`**
- Request body: constraint updates (cardinality, flags, etc.)
- Apply constraint to specified element path
- Record operation in edit history
- Run incremental validation
- Return updated element state + validation results
- Mark profile dirty and persist IR changes (immediate or debounced), writing to `IR/resources/<profileId>.json`

### R5: Import Profile
**POST `/api/projects/:projectId/profiles/:profileId/import`**
- Request body: SD JSON or FSH content + format indicator
- Import into existing profile or create new
- Support both SD and FSH formats
- Return profile IR + import diagnostics
- Persist imported source file under:
  - `SD/StructureDefinition/<name>.json` for SD input, or
  - `FSH/profiles/<name>.fsh` for FSH input

### R6: Delete Profile
**DELETE `/api/projects/:projectId/profiles/:profileId`**
- Remove profile from loaded state
- Confirm if unsaved changes exist
- Return 204 on success
- Remove on-disk files (`IR/resources/`, and any tracked source files in `SD/` or `FSH/`)

### R7: Profile Metadata Update
**PATCH `/api/projects/:projectId/profiles/:profileId/metadata`**
- Update profile metadata (name, title, description, status, etc.)
- Validate metadata fields
- Return updated profile

### R8: Non-Profile Resources (Out of Scope)
- Creation and management of **Extension** `StructureDefinition` and **ValueSet** resources is handled by the project-level resources API:
  - `POST /api/projects/:projectId/resources` (see Backend 15)
- The Profile Editor UI must only open `resourceKind=Profile` documents.

### R9: Error Handling
- 400 Bad Request for invalid input
- 404 Not Found for missing profiles
- 409 Conflict for concurrent modifications
- 422 Unprocessable Entity for validation failures
- Detailed error messages in JSON format

### R10: Response Format
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

- [x] All endpoints return correct HTTP status codes
- [x] Profile listing endpoint returns all profiles
- [x] Create endpoint generates valid new profiles
- [x] Get endpoint returns complete profile state
- [x] Update endpoint applies constraints correctly
- [x] Import endpoint handles SD and FSH formats (SD implemented, FSH pending)
- [x] Delete endpoint removes profiles
- [x] Metadata update endpoint validates fields
- [x] All endpoints have proper error handling
- [x] Response format is consistent across endpoints
- [x] Request validation rejects invalid input
- [ ] Concurrent modification detection works (partial - timestamps tracked)
- [ ] API documentation (OpenAPI/Swagger)

## Dependencies
- **Backend 02**: IR Data Model Implementation
- **Backend 03**: SD Import
- **Backend 04**: SD Export
- **Backend 05**: Axum Server Setup
- **Backend 15**: Project Management (project-scoped storage)

## Related Files
- `src/api/mod.rs` - API module entry point
- `src/api/profiles.rs` - Profile route handlers
- `src/api/dto.rs` - Request/response DTOs
- `src/api/storage.rs` - Profile storage service
- `src/server.rs` - Router integration

## Priority
🔴 Critical - Core API functionality

## Status
✅ **Implemented** (2024-12-18)

## Implementation Notes

### Implemented Endpoints
All endpoints are nested under `/api/projects/:projectId/profiles`:

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| GET | `/` | `list_profiles` | List profiles with pagination |
| POST | `/` | `create_profile` | Create new profile |
| GET | `/:profileId` | `get_profile` | Get profile details |
| DELETE | `/:profileId` | `delete_profile` | Delete profile |
| PATCH | `/:profileId/metadata` | `update_metadata` | Update metadata |
| PATCH | `/:profileId/elements/*path` | `update_element` | Update element constraints |
| POST | `/:profileId/import` | `import_profile` | Import SD JSON |

### Storage Layer
- `ProfileStorage` service handles disk persistence
- Atomic writes using temp files for data integrity
- Directory structure:
  ```
  <workspace>/<projectId>/
  ├── IR/
  │   ├── index.json          # Profile index
  │   └── resources/
  │       └── <profileId>.json # Profile IR documents
  ├── SD/
  │   └── StructureDefinition/
  │       └── <name>.json      # Exported SD JSON
  └── FSH/
      └── profiles/
          └── <name>.fsh       # FSH source files
  ```

### Pending Items
- FSH import support (SD JSON works, FSH deferred to task 08)
- Full concurrent modification detection with ETags
- OpenAPI documentation generation

## Estimated Complexity
High - 2 weeks
