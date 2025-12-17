# Task: Project Management System

## Description
Implement project-level management for Implementation Guides (IGs), supporting multiple profiles, extensions, ValueSets, and instances with cross-reference resolution.

## Requirements

### R1: Project Model
```rust
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub canonical_base: String,
    pub fhir_version: FhirVersion,
    pub dependencies: Vec<PackageDependency>,
    pub resources: HashMap<CanonicalUrl, ProjectResource>,
    pub config: ProjectConfig,
}

pub struct ProjectResource {
    pub canonical_url: CanonicalUrl,
    pub kind: ResourceKind,
    pub source: ResourceSource,
    pub file_path: PathBuf,
    pub ir: Option<ProfiledResource>,
    pub dependencies: Vec<CanonicalUrl>,
}
```

### R2: Project File Format
- Store project metadata in `sushi-config.yaml` (SUSHI-compatible)
- Store IR cache in `.profile-builder/` directory
- Store validation cache
- Git-friendly format (text, deterministic)

### R3: Project Operations
**Create Project:**
- Initialize from template or blank
- Set up directory structure
- Create sushi-config.yaml
- Initialize Git repository (optional)

**Open Project:**
- Load project from directory
- Parse sushi-config.yaml
- Load dependencies
- Load resources (lazy)

**Save Project:**
- Save modified resources
- Update file timestamps
- Export to configured formats

**Close Project:**
- Confirm unsaved changes
- Clean up resources
- Release locks

### R4: Resource Management
**Add Resource:**
- Create new profile/extension/ValueSet/etc.
- Assign canonical URL
- Add to project
- Create file

**Remove Resource:**
- Check for dependents
- Confirm deletion
- Remove from project
- Delete file

**Import Resource:**
- Import SD JSON or FSH
- Detect resource type
- Add to project
- Create file

### R5: Cross-Reference Resolution
- Resolve canonical URLs within project
- Track resource dependencies
- Detect circular dependencies
- Find all dependents of a resource

### R6: Incremental Compilation
- Track modified resources
- Compile only changed resources
- Invalidate dependent resources
- Background compilation worker

### R7: Project Templates
Support creating projects from templates:
- Blank project
- US Core-based project
- IPA-based project
- Custom templates

### R8: API Endpoints
**GET `/api/projects`**
- List recent projects

**POST `/api/projects`**
- Create new project from template

**GET `/api/projects/:id`**
- Get project metadata and resources

**PUT `/api/projects/:id`**
- Update project configuration

**DELETE `/api/projects/:id`**
- Close project (not delete from disk)

**POST `/api/projects/:id/resources`**
- Add resource to project

**DELETE `/api/projects/:id/resources/:canonical`**
- Remove resource from project

**GET `/api/projects/:id/dependencies`**
- Get dependency graph

**POST `/api/projects/:id/build`**
- Build entire project (export all resources)

## Acceptance Criteria

- [ ] Projects can be created from templates
- [ ] Projects load from sushi-config.yaml
- [ ] Multiple resources can be managed in project
- [ ] Cross-references resolve correctly
- [ ] Circular dependency detection works
- [ ] Incremental compilation works
- [ ] Unsaved changes tracking works
- [ ] Project save/load works correctly
- [ ] Git-friendly file formats
- [ ] Templates create valid projects
- [ ] All API endpoints work correctly
- [ ] Documentation for project structure

## Dependencies
- **Backend 02**: IR Data Model Implementation
- **Backend 10**: Package Management

## Related Files
- `crates/profile-builder/src/project/mod.rs` (new)
- `crates/profile-builder/src/project/model.rs` (new)
- `crates/profile-builder/src/project/loader.rs` (new)
- `crates/profile-builder/src/project/saver.rs` (new)
- `crates/profile-builder/src/project/templates.rs` (new)
- `crates/profile-builder/src/project/dependency_graph.rs` (new)
- `crates/server/src/routes/projects.rs` (new)

## Priority
🟡 High - Required for IG authoring

## Estimated Complexity
High - 2-3 weeks
