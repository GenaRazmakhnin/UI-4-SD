# Task: Undo/Redo System

## Description
Implement a robust undo/redo system that tracks all profile editing operations and allows unlimited history navigation with full state restoration.

## Requirements

### R1: Edit History Model
```rust
pub struct EditHistory {
    operations: Vec<Box<dyn Operation>>,
    current_index: usize,
    max_history: usize,
    saved_index: Option<usize>,
}
```

### R2: Operation Recording
- Record every operation applied to profile
- Store operation metadata (timestamp, description)
- Maintain operation order
- Support configurable max history depth

### R3: Undo Implementation
- Reverse operations in LIFO order
- Restore previous state exactly
- Update current index
- Validate state after undo

### R4: Redo Implementation
- Re-apply undone operations
- Advance current index
- Validate state after redo

### R5: History Navigation
- Get list of operations for UI display
- Jump to specific point in history
- Show current position in history
- Mark saved state

### R6: History Branching
- Handle new operations after undo:
  - Option A: Discard redo history (simple)
  - Option B: Create branch (complex, future)
- Document chosen strategy

### R7: API Endpoints
**POST `/api/profiles/:id/undo`**
- Undo last operation
- Return updated profile state
- Return error if nothing to undo

**POST `/api/profiles/:id/redo`**
- Redo next operation
- Return updated profile state
- Return error if nothing to redo

**GET `/api/profiles/:id/history`**
- Return operation history
- Include current index
- Mark saved state

**POST `/api/profiles/:id/history/goto`**
- Jump to specific operation index
- Apply undo/redo as needed
- Return updated state

### R8: Dirty State Tracking
- Track if profile has unsaved changes
- Mark saved state in history
- Update dirty flag on operations
- Clear dirty flag on save

### R9: History Persistence
- Save history to disk with profile
- Load history on profile open
- Compact history on save (remove old operations)

### R10: Performance
- Operations store minimal state for undo
- Lazy state snapshots for efficiency
- History operations complete <50ms

## Acceptance Criteria

- [ ] All operations are recorded in history
- [ ] Undo reverses operations correctly
- [ ] Redo re-applies operations correctly
- [ ] Unlimited undo/redo works
- [ ] History navigation UI shows operations
- [ ] Jump to history point works
- [ ] Dirty state tracking works
- [ ] Saved state is marked correctly
- [ ] History persists across sessions
- [ ] History branching strategy is documented
- [ ] Performance targets met (<50ms)

## Dependencies
- **Backend 13**: Operations Engine

## Related Files
- `crates/profile-builder/src/history/mod.rs` (new)
- `crates/profile-builder/src/history/edit_history.rs` (new)
- `crates/profile-builder/src/history/state_snapshot.rs` (new)
- `crates/server/src/routes/history.rs` (new)

## Priority
🔴 Critical - Core UX feature

## Estimated Complexity
Medium - 1-2 weeks
