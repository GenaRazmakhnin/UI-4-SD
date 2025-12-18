# Task: Profile Builder Engine (Orchestration Layer)

## Description
Implement the central ProfileBuilderEngine that orchestrates all subsystems (IR operations, import/export, validation, package management) and maintains application state.

## Requirements

### R1: Engine Architecture
```rust
pub struct ProfileBuilderEngine {
    documents: HashMap<DocumentId, ProfileDocument>,
    package_manager: PackageManager,
    validator: ValidationEngine,
    config: EngineConfig,
}
```

### R2: Document Lifecycle Management
**Open Document:**
- Load from file or create from base
- Initialize IR state
- Load edit history
- Run initial validation

**Close Document:**
- Confirm unsaved changes
- Clean up resources
- Remove from memory

**Save Document:**
- Export to configured format(s)
- Update file on disk
- Mark as saved in history
- Clear dirty flag

### R3: Document Operations
- Create new profile from base definition
- Import existing SD/FSH
- Apply operations to document
- Validate document
- Export document
- Undo/redo operations

### R4: Concurrent Document Management
- Support multiple open documents
- Thread-safe document access
- Document locking for mutations
- Optimistic concurrency control

### R5: Validation Orchestration
- Run incremental validation after operations
- Schedule background full validation
- Cache validation results
- Invalidate cache on changes

### R6: Package Integration
- Initialize package manager
- Load project dependencies
- Resolve canonical URLs
- Search across packages

### R7: Event System
Emit events for UI updates:
- DocumentOpened
- DocumentModified
- DocumentClosed
- ValidationCompleted
- OperationApplied
- OperationUndone

### R8: Configuration Management
```rust
pub struct EngineConfig {
    pub fhir_version: FhirVersion,
    pub package_cache_dir: PathBuf,
    /// Root folder for server-managed project storage.
    /// Projects are stored as: <workspace_dir>/<project_id>/(IR|SD|FSH)/...
    pub workspace_dir: PathBuf,
    pub validation_level: ValidationLevel,
    pub terminology_service_url: Option<String>,
    pub max_history_depth: usize,
}
```

### R9: Error Handling
- Graceful error recovery
- Detailed error logging
- User-friendly error messages
- Rollback on operation failures

### R10: Performance
- Lazy loading of documents
- Incremental validation
- Background workers for slow tasks
- Memory-efficient large profile handling

## Acceptance Criteria

- [ ] Engine initializes with configuration
- [ ] Multiple documents can be open simultaneously
- [ ] Document lifecycle (open/save/close) works
- [ ] Operations are applied atomically
- [ ] Validation runs incrementally
- [ ] Undo/redo works across all documents
- [ ] Events are emitted for UI updates
- [ ] Package manager is integrated
- [ ] Thread-safe concurrent access
- [ ] Error handling is robust
- [ ] Performance targets met:
  - [ ] Document open <500ms
  - [ ] Operation apply <50ms
  - [ ] Incremental validation <100ms
- [ ] Documentation for engine API

## Dependencies
- **Backend 02**: IR Data Model
- **Backend 09**: Validation Engine
- **Backend 10**: Package Management
- **Backend 13**: Operations Engine
- **Backend 14**: Undo/Redo System

## Related Files
- `crates/profile-builder/src/engine/mod.rs` (new)
- `crates/profile-builder/src/engine/document_manager.rs` (new)
- `crates/profile-builder/src/engine/events.rs` (new)
- `crates/profile-builder/src/engine/config.rs` (new)

## Priority
🔴 Critical - Central orchestration

## Estimated Complexity
High - 2 weeks
