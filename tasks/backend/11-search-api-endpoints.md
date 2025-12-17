# Task: Search API Endpoints

## Description
Implement search API endpoints for discovering resources (profiles, extensions, ValueSets) across loaded packages and project resources.

## Requirements

### R1: Resource Search
**GET `/api/search/resources`**
- Query parameters:
  - `q`: search query (full-text)
  - `type`: filter by resource type (Profile, Extension, ValueSet, etc.)
  - `package`: filter by package ID
  - `fhirVersion`: filter by FHIR version
  - `limit`: max results (default: 50)
  - `offset`: pagination offset
- Search across:
  - Resource names
  - Resource titles
  - Resource descriptions
  - Canonical URLs
- Return ranked results with match highlights

### R2: Extension Search
**GET `/api/search/extensions`**
- Query parameters:
  - `q`: search query
  - `context`: filter by context type (Resource, Element, etc.)
  - `contextPath`: filter by context path (e.g., "Patient")
- Search extension definitions
- Check context rules for compatibility
- Return extensions with context metadata

### R3: ValueSet Search
**GET `/api/search/valuesets`**
- Query parameters:
  - `q`: search query
  - `system`: filter by code system
  - `expansion`: include expansion (true/false)
- Search ValueSet definitions
- Optionally include expansion
- Cache expansions for performance

### R4: Profile Search
**GET `/api/search/profiles`**
- Query parameters:
  - `q`: search query
  - `baseType`: filter by base resource type
  - `derivation`: filter by derivation type
- Search profile definitions
- Show base type and derivation chain
- Include profile snapshot summary

### R5: Element Path Search
**GET `/api/search/elements`**
- Query parameters:
  - `profileId`: search within specific profile
  - `q`: element path or description search
- Search element paths within a profile
- Fuzzy matching for element paths
- Return element tree position

### R6: Search Indexing
- Build search index from loaded packages
- Update index when packages are added/removed
- Support incremental index updates
- Efficient prefix and fuzzy matching

### R7: Search Result Format
```json
{
  "results": [
    {
      "resourceType": "StructureDefinition",
      "id": "...",
      "url": "...",
      "name": "...",
      "title": "...",
      "description": "...",
      "package": {
        "id": "hl7.fhir.us.core",
        "version": "5.0.1"
      },
      "score": 0.95,
      "highlights": [
        { "field": "description", "snippet": "...match..." }
      ]
    }
  ],
  "total": 150,
  "limit": 50,
  "offset": 0
}
```

### R8: Performance
- Search results return in <200ms
- Index updates are incremental
- Cache frequent queries
- Limit result set size

## Acceptance Criteria

- [ ] Resource search finds resources by name/description
- [ ] Extension search filters by context compatibility
- [ ] ValueSet search works efficiently
- [ ] Profile search filters by base type
- [ ] Element path search finds elements within profiles
- [ ] Search ranking is relevant
- [ ] Fuzzy matching works for typos
- [ ] Pagination works correctly
- [ ] Search index updates incrementally
- [ ] Search performance <200ms for most queries
- [ ] Result highlights show match context
- [ ] Documentation for search API

## Dependencies
- **Backend 10**: Package Management (for resource access)

## Related Files
- `crates/server/src/routes/search.rs` (new)
- `crates/profile-builder/src/search/mod.rs` (new)
- `crates/profile-builder/src/search/indexer.rs` (new)
- `crates/profile-builder/src/search/query.rs` (new)
- `crates/profile-builder/src/search/ranking.rs` (new)

## Priority
🟡 High - Important for usability

## Estimated Complexity
Medium - 2 weeks
