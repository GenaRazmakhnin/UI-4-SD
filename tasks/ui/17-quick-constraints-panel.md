# Task: Quick Constraints Panel Feature

## Description
Implement the quick constraints panel/toolbar that provides one-click access to common profiling operations.

## Requirements

### R1: Toolbar Component
- Appears on element selection
- Contextual actions based on element type
- Icon buttons with tooltips
- Keyboard shortcuts for each action

### R2: Quick Actions
**Common Actions:**
- Make required (0..1 → 1..1)
- Make optional (1..1 → 0..1)
- Allow multiple (1..1 → 1..*)
- Toggle mustSupport
- Add extension
- Create slice
- Set binding

**Contextual Actions:**
- Constrain type (if multiple types)
- Add pattern (if primitive)
- Add fixed value
- Add invariant

### R3: Context Menu Integration
- Right-click on element
- Show quick actions menu
- Recent actions
- Favorites

### R4: Favorites System
- User can favorite actions
- Favorites appear first
- Customize toolbar
- Sync preferences

### R5: Recent Bindings/Extensions
- Quick access to recently used ValueSets
- Quick access to recently used extensions
- Configurable list size
- Clear recents option

### R6: Action Confirmation
- Some actions require confirmation
- Preview impact before applying
- Undo immediately after
- Skip confirmation option

### R7: Keyboard Shortcuts
- Configurable shortcuts
- Chord shortcuts (e.g., Ctrl+K, M for mustSupport)
- Shortcut help overlay (?)

## Acceptance Criteria
- [x] Quick actions toolbar displays
- [x] Actions work correctly
- [x] Context menu shows correct actions
- [x] Favorites can be set
- [x] Recent items tracked
- [x] Keyboard shortcuts work
- [x] Confirmation dialogs show when needed
- [x] Preferences persist
- [ ] Unit tests pass (TODO)
- [ ] Storybook stories exist (TODO)

## Dependencies
- **UI 04**: Element Tree Viewer
- **UI 05**: Inspector Panel
- **Backend 13**: Operations Engine

## Priority
🟡 High - UX enhancement

## Estimated Complexity
Medium - 1-2 weeks

## Implementation Progress

### Status: 🟢 UI Complete (Integration Pending)

### Implementation Plan
1. ✅ Explore element selection model and types
2. ✅ Create state management (features/quick-constraints/model/)
3. ✅ Implement QuickActionsToolbar UI component
4. ✅ Add context menu integration (ElementContextMenu)
5. ✅ Implement favorites and recents system
6. ✅ Add keyboard shortcuts (chord: Ctrl+K, letter)
7. ✅ Add action confirmation dialogs
8. ✅ Export and integrate with app

### Files Created
```
features/quick-constraints/
├── index.ts                          # Public exports
├── lib/
│   ├── index.ts                      # Lib exports
│   ├── types.ts                      # QuickAction, QuickActionId types
│   ├── actions.ts                    # Action definitions with availability checks
│   └── useQuickActionShortcuts.ts    # Keyboard shortcuts hook (Ctrl+K chord)
├── model/
│   └── index.ts                      # Effector stores, events, effects
└── ui/
    ├── QuickActionsToolbar.tsx       # Main toolbar component (full/compact modes)
    ├── QuickActionsToolbar.module.css
    ├── ElementContextMenu.tsx        # Right-click context menu
    ├── ShortcutHelpOverlay.tsx       # Keyboard shortcuts help modal
    └── ActionConfirmationDialog.tsx  # Confirmation dialog for dangerous actions
```

### Quick Actions Implemented
**Cardinality:** make-required, make-optional, allow-multiple, make-prohibited
**Flags:** toggle-must-support, toggle-is-modifier, toggle-is-summary
**Constraints:** set-binding, add-extension, create-slice, constrain-type, add-pattern, add-fixed-value, add-invariant

### Keyboard Shortcuts (Chord: Ctrl+K, then letter)
- R: Make required
- O: Make optional
- M: Allow multiple
- S: Toggle mustSupport
- B: Set binding
- E: Add extension
- L: Create slice
- T: Constrain type
- ?: Show shortcut help

### Features
- Contextual actions based on element type
- Favorites system with persistence
- Recent ValueSets and Extensions
- Confirmation dialogs for destructive actions
- "Skip confirmation" preference

### Remaining Work
- ~~Integrate QuickActionsToolbar into Inspector Panel or Element Tree~~ ✅ Done
- Connect effects to backend API
- Add unit tests
- Add Storybook stories

### Integration
- QuickActionsToolbar integrated into ProfileEditorPage
- ElementContextMenu, ActionConfirmationDialog, ShortcutHelpOverlay rendered as overlays
- Keyboard shortcuts registered via useQuickActionShortcuts hook
