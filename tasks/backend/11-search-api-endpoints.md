# Task: Search API Endpoints

## Status: Completed

Core search endpoints implemented with all enhancements.

## Implemented Endpoints

| Endpoint | Status | Notes |
|----------|--------|-------|
| `GET /api/search/resources` | Done | Generic resource search with type/package/fhirVersion filters, facets |
| `GET /api/search/extensions` | Done | Extension search with context/contextPath filtering |
| `GET /api/search/valuesets` | Done | ValueSet search with system filtering |
| `GET /api/search/profiles` | Done | NEW: Profile search with baseType/derivation filters |
| `GET /api/search/elements` | Done | NEW: Element search within a profile |

### Implementation Details
- Location: `src/api/search_api.rs`, `src/api/packages_dto.rs`
- Uses `CanonicalManager.search()` with `SearchQueryBuilder`
- All endpoints return `SearchResponseWithFacets` with facet counts

## Completed Requirements

### R1: Enhanced Extension Search ✅
- [x] Add `context` filter (Resource, Element, DataType)
- [x] Add `contextPath` filter (e.g., "Patient", "Observation.value")
- [x] Check extension context rules for compatibility
- [x] Return context metadata in results

### R2: Enhanced ValueSet Search ✅
- [x] Add `system` filter for code system URL
- [x] Filters by compose.include and expansion.contains

### R3: Profile Search ✅
- [x] `GET /api/search/profiles` endpoint
- [x] Query parameters: `q`, `baseType`, `derivation`, `package[]`, `fhirVersion`
- [x] Filters StructureDefinitions with kind=resource and non-null derivation
- [x] Returns base type and derivation info

### R4: Element Path Search ✅
- [x] `GET /api/search/elements` endpoint
- [x] Query parameters: `profileId`, `q`, `limit`
- [x] Search element paths within a profile
- [x] Fuzzy matching for element paths
- [x] Returns element tree info (path, types, cardinality)

### R5: Search Result Enhancements ✅
- [x] Add `fhirVersion` filter to all endpoints
- [x] Add faceted search results (counts by type/package)
- [x] Score included in all results

### R6: Performance
- [x] Delegates to canonical manager's SQLite FTS (fast by design)
- [ ] Verify <200ms response time (needs runtime testing)

## Query Parameter Reference

### Common Parameters (all endpoints)
| Parameter | Type | Description |
|-----------|------|-------------|
| `q` | string | Text search query |
| `package[]` | array | Filter by package name(s) |
| `fhir_version` | string | Filter by FHIR version (prefix match) |
| `limit` | int | Max results (default 50) |
| `offset` | int | Pagination offset |

### Extension-specific
| Parameter | Type | Description |
|-----------|------|-------------|
| `context` | string | Context type filter (resource, element, datatype) |
| `context_path` | string | Context expression filter (e.g., "Patient") |

### ValueSet-specific
| Parameter | Type | Description |
|-----------|------|-------------|
| `system` | string | Code system URL filter |

### Profile-specific
| Parameter | Type | Description |
|-----------|------|-------------|
| `base_type` | string | Base resource type (Patient, Observation, etc.) |
| `derivation` | string | Derivation type (constraint, specialization) |

### Element-specific
| Parameter | Type | Description |
|-----------|------|-------------|
| `profile_id` | string | Profile URL or ID to search within (required) |

## Response Format

All endpoints return:
```json
{
  "results": [...],
  "totalCount": 150,
  "facets": {
    "resourceTypes": { "StructureDefinition": 120, "ValueSet": 30 },
    "packages": { "hl7.fhir.us.core": 100, "hl7.fhir.r4.core": 50 }
  }
}
```

## Implementation Files

- `src/api/search_api.rs` - All search endpoint handlers
- `src/api/packages_dto.rs` - DTOs for queries and responses

## Dependencies

- **Backend 10**: Package Management (completed)

## Additional Changes

### Proxy Removal
Removed backend proxy to Vite dev server:
- `src/server.rs` - Removed `dev_proxy_handler`, `proxy_request`
- `src/config.rs` - Removed `dev_mode`, `vite_dev_url` config fields
- `Cargo.toml` - Removed `reqwest` dependency

UI now uses Vite's built-in proxy (`web/vite.config.ts`) to forward `/api` requests to the backend.



