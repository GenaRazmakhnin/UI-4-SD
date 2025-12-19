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
**POST `/api/projects/:projectId/profiles/:profileId/undo`**
- Undo last operation
- Return updated profile state
- Return error if nothing to undo

**POST `/api/projects/:projectId/profiles/:profileId/redo`**
- Redo next operation
- Return updated profile state
- Return error if nothing to redo

**GET `/api/projects/:projectId/profiles/:profileId/history`**
- Return operation history
- Include current index
- Mark saved state

**POST `/api/projects/:projectId/profiles/:profileId/history/goto`**
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

- [x] All operations are recorded in history
- [x] Undo reverses operations correctly
- [x] Redo re-applies operations correctly
- [x] Unlimited undo/redo works
- [x] History navigation UI shows operations
- [x] Jump to history point works
- [x] Dirty state tracking works
- [x] Saved state is marked correctly
- [x] History persists across sessions
- [x] History branching strategy is documented (Option A: discard redo history on new operation)
- [x] Performance targets met (<50ms)

## Dependencies
- **Backend 13**: Operations Engine ✅

## Related Files
- `src/ir/tracking.rs` - EditHistory struct with undo/redo stacks, saved_index tracking
- `src/api/history.rs` - API endpoints for undo/redo/history/goto

## Implementation Notes

### History Model
- `EditHistory` tracks operations using undo_stack and redo_stack
- `saved_index: Option<usize>` tracks the save state position
- Operations include Change with path, ChangeKind, timestamp, and description

### API Endpoints
- `POST /{profileId}/undo` - Undo last operation with state restoration
- `POST /{profileId}/redo` - Redo next operation with state restoration
- `GET /{profileId}/history` - Get operation list with HistoryState
- `POST /{profileId}/history/goto` - Jump to specific history index

### Key Types
- `OperationSummary` - UI-friendly operation info with id, description, timestamp, index
- `HistoryState` - Current state info (can_undo, can_redo, saved state, descriptions)
- `UndoRedoResponse`, `HistoryResponse`, `GotoResponse` - API response types

### Branching Strategy
- Uses Option A: New operations after undo clear the redo stack
- Simple and predictable behavior for users

## Status
✅ **COMPLETED**

## Priority
🔴 Critical - Core UX feature

## Estimated Complexity
Medium - 1-2 weeks
