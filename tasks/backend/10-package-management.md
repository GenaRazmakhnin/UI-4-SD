# Task: Package Management System

## Description
Implement FHIR package management using maki's CanonicalFacade for resolving base definitions, extensions, ValueSets, and other resources from loaded packages.

## Requirements

### R1: Package Storage Integration
- Use maki's `CanonicalFacade` for package indexing
- Leverage `~/.maki/packages/` cache directory
- Support loading packages from cache
- Index package resources by canonical URL

### R2: Package Registry Integration
- Connect to `packages.fhir.org` registry
- Search for packages by name/description
- Retrieve package metadata (versions, dependencies)
- Download packages from registry

### R3: Package Installation
- Install package from registry by ID and version
- Resolve and install transitive dependencies
- Verify package integrity (checksums)
- Store in cache directory
- Index resources after installation

### R4: Package Resolution
- Resolve canonical URLs to resources
- Search priority: project-local → loaded packages → core spec
- Support version-specific resolution
- Handle multiple versions of same package

### R5: Dependency Management
- Parse `sushi-config.yaml` dependencies
- Load all declared package dependencies
- Detect and warn about missing dependencies
- Detect circular dependencies
- Validate version compatibility

### R6: Resource Search
- Search resources across loaded packages
- Filter by resource type (Profile, Extension, ValueSet, etc.)
- Full-text search in resource names/descriptions
- Filter by package
- Fuzzy search for resource discovery

### R7: Package API Model
```rust
pub struct PackageManager {
    canonical_facade: CanonicalFacade,
    loaded_packages: HashMap<PackageId, Package>,
    registry_client: RegistryClient,
}

pub struct Package {
    pub id: String,
    pub version: String,
    pub fhir_version: Vec<FhirVersion>,
    pub dependencies: Vec<PackageDependency>,
    pub resources: HashMap<CanonicalUrl, Resource>,
}
```

### R8: Caching and Performance
- Cache registry search results
- Cache package metadata
- Lazy-load package resources
- Efficient resource lookup by canonical URL

## Acceptance Criteria

- [ ] Successfully loads packages from maki cache
- [ ] Connects to packages.fhir.org registry
- [ ] Searches registry for packages
- [ ] Downloads and installs packages
- [ ] Resolves transitive dependencies
- [ ] Indexes package resources
- [ ] Resolves canonical URLs correctly
- [ ] Search priority (local > packages > core) works
- [ ] Resource search finds resources by name
- [ ] Full-text search works efficiently
- [ ] Package metadata is cached
- [ ] Lazy loading improves performance
- [ ] Missing dependency detection works
- [ ] Circular dependency detection works
- [ ] Documentation for package management

## Dependencies
- **Backend 01**: Toolchain Alignment (maki-core integration)

## Related Files
- `crates/profile-builder/src/packages/mod.rs` (new)
- `crates/profile-builder/src/packages/manager.rs` (new)
- `crates/profile-builder/src/packages/registry.rs` (new)
- `crates/profile-builder/src/packages/resolver.rs` (new)
- `crates/profile-builder/src/packages/cache.rs` (new)
- `crates/server/src/routes/packages.rs` (new)

## Priority
🟡 High - Required for package dependencies

## Estimated Complexity
High - 2-3 weeks
