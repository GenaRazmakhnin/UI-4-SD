# Task: Profile Editor Page (Main Page)

## Description
Implement the main profile editor page that composes all widgets and provides the primary editing interface.

## Requirements

### R1: Page Layout
- Three-panel layout (tree, inspector, preview/diagnostics)
- Resizable panels
- Collapsible panels
- Save panel state to localStorage

### R2: Top Toolbar
- Profile name/title display
- Save button with status indicator
- Validation button
- Export button (SD/FSH)
- Undo/Redo buttons
- Settings menu

### R3: Widget Composition
- Element tree widget (left panel)
- Inspector panel widget (right panel)
- Preview/Diagnostics tabs (bottom panel)
- Package browser (modal/sidebar)
- Extension picker (modal)

### R4: State Coordination
- Sync element selection across widgets
- Update validation on changes
- Update preview on changes
- Debounce expensive operations

### R5: Unsaved Changes Handling
- Detect unsaved changes
- Warn on navigation
- Auto-save option
- Dirty state indicator

### R6: Keyboard Shortcuts
- Ctrl+S to save
- Ctrl+E to export
- Ctrl+F to search elements
- Ctrl+/ to toggle panels
- F5 to validate

## Acceptance Criteria
- [ ] Page renders all widgets correctly
- [ ] Panels are resizable
- [ ] Panel state persists
- [ ] Toolbar buttons work
- [ ] Widget state syncs correctly
- [ ] Unsaved changes detection works
- [ ] Keyboard shortcuts work
- [ ] Performance is acceptable
- [ ] Unit tests pass
- [ ] E2E tests pass

## Dependencies
- **UI 04**: Element Tree Viewer
- **UI 05**: Inspector Panel
- **UI 13**: Diagnostics Panel
- **UI 14**: Preview Panel

## Priority
🔴 Critical - Main UI

## Estimated Complexity
Medium - 1 week
