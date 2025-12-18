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
    /// Absolute path to this project's root directory on disk.
    /// Resolved as: <workspace_dir>/<project_id>/
    pub root_dir: PathBuf,
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

### R2: Workspace Storage Layout (Server-Managed)
The backend must be started with a **workspace base directory** (passed as a CLI arg to the server). All project files live under that directory.

**Workspace root (argument):**
- `--workspace-dir /path/to/niten-data` (absolute or relative)

**On-disk layout:**
```
<workspace_dir>/
  <project_id>/
    project.json
    sushi-config.yaml                # optional, generated for export compatibility
    IR/
      index.json                     # resource index + metadata
      resources/
        <resource_id>.json           # serialized IR (UI editing source-of-truth)
    SD/
      StructureDefinition/
        <name>.json                  # exported/round-tripped SD JSON
      ValueSet/
        <name>.json
    FSH/
      profiles/
        <name>.fsh
      extensions/
        <name>.fsh
      valuesets/
        <name>.fsh
```

**Rules:**
- All file reads/writes must be confined to `<workspace_dir>/<project_id>/` (no path traversal).
- Writes are atomic (`write temp` → `fsync` → `rename`) to avoid corruption on crash.
- `IR/` is the canonical editing store; `SD/` and `FSH/` are derived artifacts unless explicitly imported as source.

### R3: Project Operations
**Create Project:**
- Allocate new `project_id`
- Create directory structure under `<workspace_dir>/<project_id>/` (`IR/`, `SD/`, `FSH/`)
- Create `project.json` with minimal metadata (name, fhirVersion, canonicalBase)
- Optionally generate `sushi-config.yaml` for compatibility/export
- Optionally initialize Git repository (optional, future)

**Open Project:**
- Load project by `project_id`
- Read `project.json`
- Read `IR/index.json` (if present)
- Load dependencies
- Load resources (lazy, per-resource IR file)

**Save Project:**
- Persist modified IR documents into `IR/resources/`
- Update `IR/index.json`
- Optionally export updated resources into `SD/` and/or `FSH/`

**Close Project:**
- Confirm unsaved changes
- Clean up resources
- Release locks

### R4: Resource Management
**Add Resource:**
- Create new profile / extension / ValueSet (and later: CodeSystem, Instance)
- Assign canonical URL
- Add to project
- Create on-disk files inside project directory:
  - Always create `IR/resources/<resource_id>.json`
  - Optionally create initial `FSH/.../<name>.fsh` or `SD/.../<name>.json` based on requested `source.format`

**Remove Resource:**
- Check for dependents
- Confirm deletion
- Remove from project
- Delete file

**Import Resource:**
- Import SD JSON or FSH into a project
- Detect resource type
- Add to project
- Persist:
  - `IR/resources/<resource_id>.json`
  - Store the imported source under `SD/` or `FSH/` (matching import format) for round-trip workflows

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
- Add resource to project (Profile / Extension / ValueSet)
- Request body includes `kind`, `name`, optional `canonicalUrl`, optional `base`:
  - Profile: base is a resource type (e.g. `"Patient"`) or canonical base URL
  - Extension: base defaults to `http://hl7.org/fhir/StructureDefinition/Extension`
  - ValueSet: base is not applicable
- Response includes `resourceId`, `canonicalUrl`, `kind`, and file paths (`IR/`, optional `FSH/` or `SD/`)

**DELETE `/api/projects/:id/resources/:canonical`**
- Remove resource from project

**GET `/api/projects/:id/tree`**
- Return a virtual file tree rooted at `IR/`, `SD/`, `FSH/` for the UI project explorer
- Include `resourceKind` classification so UI can decide whether to open the Profile Editor

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
