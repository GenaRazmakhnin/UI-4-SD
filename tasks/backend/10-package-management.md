# Task: Package Management System

## Description

Implement a thin API layer on top of `octofhir-canonical-manager` for FHIR package management. The canonical manager handles all storage, registry interaction, and SQLite-based search. Backend exposes REST endpoints with SSE for installation progress.

## Architecture

```
Frontend (React)                    Backend (Axum)                  octofhir-canonical-manager
    |                                    |                                    |
    |-- GET /api/packages ---------->    |-- list_packages() ------------->  |
    |-- GET /api/packages/search?q= -->  |-- search_registry() ---------->   |
    |-- POST /api/packages/:id/install   |-- install_with_progress() ---->   |
    |   (SSE stream)                     |   (DownloadProgress trait)        |
    |-- GET /api/search/extensions -->   |-- search_resources() --------->   |
```

## Requirements

### R1: Package List Endpoint
- `GET /api/packages` - List installed packages
- Returns package info from canonical manager's SQLite storage
- Includes resource counts, FHIR version, installation date

### R2: Package Installation with Progress (SSE)
- `POST /api/packages/:id/install` - Install package with SSE progress
- Stream real-time progress events to UI:
  - `start` - Installation started
  - `progress` - Download progress (bytes, percentage)
  - `extracting` - Extracting package contents
  - `indexing` - Indexing resources in SQLite
  - `complete` - Installation successful
  - `error` - Installation failed
- Resolve transitive dependencies automatically

### R3: Package Uninstall
- `POST /api/packages/:id/uninstall` - Remove installed package
- Clean up from canonical manager's storage

### R4: Registry Search
- `GET /api/packages/search?q=` - Search packages.fhir.org registry
- Returns available packages matching query
- Shows latest version, FHIR compatibility

### R5: Resource Search
- `GET /api/search/extensions?q=&package=` - Search extensions
- `GET /api/search/valuesets?q=` - Search value sets
- `GET /api/search/resources?q=&type=&package=` - Generic search
- Leverages canonical manager's SQLite full-text search
- Filter by resource type, package, text query

### R6: Global Package Storage
- Packages stored globally in `~/.maki/packages/`
- Managed entirely by canonical manager
- No custom storage implementation
- Shared across all projects

## API Endpoints

### Package Management

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| GET | `/api/packages` | List installed packages | `Package[]` |
| GET | `/api/packages/search?q=` | Search registry | `PackageSearchResult[]` |
| POST | `/api/packages/:id/install` | Install package | SSE stream |
| POST | `/api/packages/:id/uninstall` | Uninstall package | 204 No Content |

### Resource Search

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| GET | `/api/search/extensions?q=&package=` | Search extensions | `Extension[]` |
| GET | `/api/search/valuesets?q=` | Search value sets | `ValueSet[]` |
| GET | `/api/search/resources?q=&type=&package=` | Generic search | `SearchResult[]` |

## SSE Event Format

```typescript
type InstallEvent =
  | { type: 'start'; data: { packageId: string; totalBytes?: number } }
  | { type: 'progress'; data: { packageId: string; downloadedBytes: number; totalBytes?: number; percentage: number } }
  | { type: 'extracting'; data: { packageId: string } }
  | { type: 'indexing'; data: { packageId: string } }
  | { type: 'complete'; data: { package: Package } }
  | { type: 'error'; data: { packageId: string; message: string; code: string } };
```

## Data Types

```rust
// Package info returned from API
pub struct PackageDto {
    pub id: String,              // "hl7.fhir.us.core@6.1.0"
    pub name: String,            // "hl7.fhir.us.core"
    pub version: String,         // "6.1.0"
    pub description: Option<String>,
    pub fhir_version: String,
    pub installed: bool,
    pub size: String,            // "12.5 MB"
    pub installed_at: Option<DateTime<Utc>>,
    pub resource_counts: Option<PackageResourceCounts>,
}

// Resource counts in a package
pub struct PackageResourceCounts {
    pub profiles: u32,
    pub extensions: u32,
    pub value_sets: u32,
    pub code_systems: u32,
    pub total: u32,
}
```

## Implementation Files

| File | Purpose |
|------|---------|
| `src/api/packages.rs` | Package API routes |
| `src/api/packages_dto.rs` | DTOs and SSE event types |
| `src/api/search_api.rs` | Resource search endpoints |
| `src/state.rs` | Add CanonicalManager to AppState |

## Acceptance Criteria

- [ ] `GET /api/packages` returns installed packages
- [ ] `POST /api/packages/:id/install` streams SSE progress events
- [ ] `POST /api/packages/:id/uninstall` removes package
- [ ] `GET /api/packages/search` queries registry
- [ ] `GET /api/search/extensions` searches SQLite index
- [ ] `GET /api/search/valuesets` searches SQLite index
- [ ] `GET /api/search/resources` supports filtering
- [ ] Error responses follow standard format
- [ ] Unit tests for DTOs and helpers

## Dependencies

- **Backend 01**: Toolchain Alignment (maki-core, octofhir-canonical-manager)

## Priority

High - Required for package-based profile development

## Notes

- Uses `octofhir-canonical-manager` for all storage and search
- Default registry: packages.fhir.org
- Global installation (not per-project)
- SSE for real-time progress updates
