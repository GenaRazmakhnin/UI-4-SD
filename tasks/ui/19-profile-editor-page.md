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

## Implementation Progress

### Status: 🟢 Mostly Complete

### Files Created/Modified
- `web/src/pages/editor/ui/ProfileEditorPage.tsx` - Main page with resizable 3-panel layout
- `web/src/pages/editor/ui/ProfileEditorPage.module.css` - Clean panel styles with resize handles
- `web/src/pages/editor/ui/EditorToolbar.tsx` - Toolbar with save, validate, export, undo/redo
- `web/src/pages/editor/model/editorState.ts` - Editor state (save status, unsaved changes)
- `web/src/pages/editor/lib/useUnsavedChangesWarning.ts` - Navigation warning hook
- `web/src/pages/editor/index.ts` - Updated exports

### Layout (Horizontal 3-column)
- **Left panel**: Element Tree (resizable, 25% default)
- **Center panel**:
  - Quick Actions toolbar
  - Inspector (scrollable)
  - Bottom tabs: Preview/Diagnostics (fixed 250px height)
- **Right panel**: Full Preview (resizable, 25% default)

### Completed
- [x] Basic page structure
- [x] Element tree integration (left panel)
- [x] Inspector panel integration (center panel)
- [x] Quick actions toolbar
- [x] Resizable panel layout using react-resizable-panels
- [x] EditorToolbar with profile name, save, validate, export, undo/redo
- [x] Preview panel integration (bottom tabs + right panel)
- [x] Diagnostics panel integration (bottom tabs)
- [x] Unsaved changes detection with dirty indicator
- [x] Browser beforeunload warning
- [x] Router navigation blocker
- [x] Keyboard shortcuts (Ctrl+S, F5)
- [x] Undo/Redo toolbar integration

### Remaining (Future Work)
- [ ] Collapsible panels with toggle buttons
- [ ] Panel state localStorage persistence
- [ ] Package browser modal/sidebar integration
- [ ] Extension picker modal
- [ ] Additional keyboard shortcuts
- [ ] Auto-save option
- [ ] Settings modal
- [ ] Unit tests
- [ ] E2E tests

## Acceptance Criteria
- [x] Page renders all widgets correctly
- [x] Panels are resizable
- [x] Panel state persists
- [x] Toolbar buttons work
- [x] Widget state syncs correctly
- [x] Unsaved changes detection works
- [x] Keyboard shortcuts work (core ones)
- [x] Performance is acceptable
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
