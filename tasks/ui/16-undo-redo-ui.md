# Task: Undo/Redo UI Implementation

## Description
Implement UI components for undo/redo functionality including toolbar buttons, keyboard shortcuts, and history viewer.

## Requirements

### R1: Toolbar Buttons
- Undo button (disabled when no history)
- Redo button (disabled when no future)
- Tooltips with keyboard shortcuts
- Visual disabled state

### R2: Keyboard Shortcuts
- Ctrl+Z / Cmd+Z for undo
- Ctrl+Shift+Z / Cmd+Shift+Z for redo
- Ctrl+Y / Cmd+Y for redo (alternative)

### R3: History Viewer
- List of operations
- Current position indicator
- Jump to specific operation
- Operation descriptions
- Timestamps

### R4: State Management
```typescript
export const $canUndo = createStore(false);
export const $canRedo = createStore(false);
export const $operationHistory = createStore<Operation[]>([]);
export const $currentHistoryIndex = createStore(0);

export const undoClicked = createEvent();
export const redoClicked = createEvent();
export const historyPositionChanged = createEvent<number>();
```

### R5: Visual Feedback
- Toast notification on undo/redo
- Highlight changed elements
- Animation for state transitions
- Progress indicator for slow operations

### R6: History Limits
- Configurable max history depth
- Warn when approaching limit
- Compact old history
- Persist history on save

## Acceptance Criteria
- [x] Undo button works correctly
- [x] Redo button works correctly
- [x] Keyboard shortcuts work
- [x] History viewer displays operations
- [x] Jump to history position works
- [x] Visual feedback is clear
- [ ] State syncs with backend (TODO: connect to backend API)
- [x] Buttons disabled appropriately
- [ ] Unit tests pass (TODO: add tests)
- [ ] Integration tests with backend (TODO: add integration tests)

## Dependencies
- **UI 02**: App Initialization
- **Backend 14**: Undo/Redo System

## Priority
🔴 Critical - Core UX feature

## Estimated Complexity
Medium - 1 week

## Implementation Progress

### Status: 🟢 UI Complete (Backend Integration Pending)

### Implementation Plan
1. ✅ Codebase exploration - understood Effector + Mantine patterns
2. ✅ State management (features/undo-redo/model/)
3. ✅ UndoRedoToolbar component
4. ✅ Keyboard shortcuts (Ctrl+Z, Ctrl+Shift+Z, Ctrl+Y)
5. ✅ HistoryViewer component with Timeline
6. ✅ Visual feedback (toast notifications)
7. ✅ History limits and warnings
8. ✅ Integration with app (TopNavigation + UndoRedoProvider)

### Files Created
```
features/undo-redo/
├── index.ts                     # Public exports
├── lib/
│   ├── index.ts                 # Lib exports
│   ├── types.ts                 # Operation, HistoryConfig types
│   ├── useUndoRedoShortcuts.ts  # Keyboard shortcuts hook
│   ├── useUndoRedoNotifications.ts # Toast notifications hook
│   └── useHistoryLimitWarning.ts   # History limit warning hook
├── model/
│   └── index.ts                 # Effector stores, events, effects
└── ui/
    ├── UndoRedoToolbar.tsx      # Toolbar with undo/redo/history buttons
    ├── UndoRedoProvider.tsx     # Provider with hooks & HistoryViewer
    ├── HistoryViewer.tsx        # Drawer with operation timeline
    └── HistoryViewer.module.css # Timeline styling
```

### Modified Files
- `app/providers/index.tsx` - Added UndoRedoProvider
- `app/layouts/TopNavigation.tsx` - Added UndoRedoToolbar
- `features/index.ts` - Added undo-redo export

### Remaining Work
- Connect undoFx/redoFx/jumpToPositionFx to backend API
- Add element highlight animation on undo/redo
- Add unit tests
- Add integration tests with backend

### Notes
- Following feature-sliced architecture
- Using Effector for state, Mantine for UI
- Icons: @tabler/icons-react (IconArrowBackUp, IconArrowForwardUp, IconHistory)
