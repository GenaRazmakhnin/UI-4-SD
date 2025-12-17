# Task: Preview Panel Widget

## Description
Implement the preview panel that shows real-time SD JSON and FSH output with syntax highlighting and diff view.

## Requirements

### R1: Panel Layout
- Tabbed view: SD JSON / FSH / Diff
- Syntax highlighting
- Line numbers
- Copy button
- Download button
- Fullscreen mode

### R2: SD JSON Preview
- Real-time generation from IR
- Formatted JSON output
- Syntax highlighting (JSON)
- Fold/unfold sections
- Search in preview

### R3: FSH Preview
- Real-time FSH generation
- Syntax highlighting (FSH)
- Show profile declaration
- Show all rules
- Formatted output

### R4: Diff View
- Compare with base definition
- Highlight added lines (green)
- Highlight removed lines (red)
- Highlight modified lines (yellow)
- Side-by-side or unified view

### R5: Monaco Editor Integration
- Use Monaco Editor for syntax highlighting
- Custom language definitions for FSH
- Read-only mode
- Minimap for navigation

### R6: Performance
- Debounce preview generation (500ms)
- Lazy loading for large outputs
- Incremental updates
- Cancel pending generations

### R7: Export Options
- Copy to clipboard
- Download as file
- Export differential only
- Export snapshot only
- Export both

## Acceptance Criteria
- [ ] Preview updates in real-time
- [ ] Syntax highlighting works (SD & FSH)
- [ ] Diff view highlights changes correctly
- [ ] Copy button works
- [ ] Download button works
- [ ] Fullscreen mode works
- [ ] Search in preview works
- [ ] Performance targets met (<500ms update)
- [ ] Unit tests pass
- [ ] Storybook stories exist

## Dependencies
- **UI 03**: Mock Data Layer
- **Backend 07**: Export API

## Priority
🟡 High - Important feature

## Estimated Complexity
Medium - 1-2 weeks
