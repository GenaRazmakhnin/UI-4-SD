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
- [ ] Quick actions toolbar displays
- [ ] Actions work correctly
- [ ] Context menu shows correct actions
- [ ] Favorites can be set
- [ ] Recent items tracked
- [ ] Keyboard shortcuts work
- [ ] Confirmation dialogs show when needed
- [ ] Preferences persist
- [ ] Unit tests pass
- [ ] Storybook stories exist

## Dependencies
- **UI 04**: Element Tree Viewer
- **UI 05**: Inspector Panel
- **Backend 13**: Operations Engine

## Priority
🟡 High - UX enhancement

## Estimated Complexity
Medium - 1-2 weeks
