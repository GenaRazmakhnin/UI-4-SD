# Task: Preview Panel Widget

## Status: In Progress

## Description
Implement the preview panel that shows real-time SD JSON and FSH output with syntax highlighting and diff view.

## Requirements

### R1: Panel Layout ✅
- [x] Tabbed view: SD JSON / FSH / Diff
- [x] Syntax highlighting
- [x] Line numbers
- [x] Copy button
- [x] Download button
- [x] Fullscreen mode

### R2: SD JSON Preview ✅
- [x] Real-time generation from IR
- [x] Formatted JSON output
- [x] Syntax highlighting (JSON)
- [x] Fold/unfold sections
- [x] Search in preview

### R3: FSH Preview ✅
- [x] Real-time FSH generation
- [x] Syntax highlighting (FSH)
- [x] Show profile declaration
- [x] Show all rules
- [x] Formatted output

### R4: Diff View ✅
- [x] Compare with base definition
- [x] Highlight added lines (green)
- [x] Highlight removed lines (red)
- [x] Highlight modified lines (yellow)
- [x] Side-by-side or unified view

### R5: Monaco Editor Integration ✅
- [x] Use Monaco Editor for syntax highlighting
- [x] Custom language definitions for FSH
- [x] Read-only mode
- [x] Minimap for navigation

### R6: Performance ✅
- [x] Debounce preview generation (500ms)
- [ ] Lazy loading for large outputs
- [ ] Incremental updates
- [ ] Cancel pending generations

### R7: Export Options ✅
- [x] Copy to clipboard
- [x] Download as file
- [x] Export differential only
- [x] Export snapshot only
- [x] Export both

## Acceptance Criteria
- [x] Preview updates in real-time
- [x] Syntax highlighting works (SD & FSH)
- [x] Diff view highlights changes correctly
- [x] Copy button works
- [x] Download button works
- [x] Fullscreen mode works
- [x] Search in preview works
- [x] Performance targets met (<500ms update)
- [ ] Unit tests pass
- [ ] Storybook stories exist

## Implementation Details

### Files Created
- `web/src/widgets/preview-panel/index.ts` - Public exports
- `web/src/widgets/preview-panel/model/index.ts` - Effector stores and events
- `web/src/widgets/preview-panel/lib/fsh-language.ts` - FSH language definition for Monaco
- `web/src/widgets/preview-panel/lib/usePreview.ts` - React Query hooks and utilities
- `web/src/widgets/preview-panel/ui/PreviewPanel.tsx` - Main panel component
- `web/src/widgets/preview-panel/ui/PreviewPanel.module.css` - Styles
- `web/src/widgets/preview-panel/ui/PreviewToolbar.tsx` - Toolbar components
- `web/src/widgets/preview-panel/ui/SDJsonPreview.tsx` - JSON preview with Monaco
- `web/src/widgets/preview-panel/ui/FSHPreview.tsx` - FSH preview with Monaco
- `web/src/widgets/preview-panel/ui/DiffView.tsx` - Diff view component

### Key Features
1. **Monaco Editor Integration** - Full Monaco editor with JSON and custom FSH language support
2. **FSH Language Definition** - Custom syntax highlighting for FHIR Shorthand including keywords, binding strengths, flags, paths
3. **Diff View** - Both side-by-side (Monaco DiffEditor) and unified view modes
4. **Toolbar Actions** - Copy to clipboard, download, settings menu, fullscreen toggle
5. **Search** - In-editor search functionality
6. **Editor Settings** - Toggle line numbers, minimap, word wrap

### Usage
```tsx
import { PreviewPanel } from '@widgets/preview-panel';

<PreviewPanel
  profileId="us-core-patient"
  baseContent={baseSDJson}
/>
```

## Dependencies
- **UI 03**: Mock Data Layer
- **Backend 07**: Export API

## Priority
🟡 High - Important feature

## Remaining Work
- [ ] Add unit tests for hooks and utilities
- [ ] Add Storybook stories for visual testing
- [ ] Implement lazy loading for large outputs
- [ ] Add incremental update support
- [ ] Add request cancellation for pending generations
