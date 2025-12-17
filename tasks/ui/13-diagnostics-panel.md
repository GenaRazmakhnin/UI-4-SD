# Task: Diagnostics Panel Widget

## Description
Implement the diagnostics panel that displays validation errors, warnings, and info messages with navigation to affected elements.

## Requirements

### R1: Panel Layout
- Collapsible panel (bottom or side)
- Tabbed view: Errors / Warnings / Info
- Count badges for each severity
- Clear all button

### R2: Diagnostic List
- Grouped by element path
- Severity icon (error/warning/info)
- Diagnostic message
- Diagnostic code
- Quick fix button (if available)

### R3: Diagnostic Item
```typescript
interface Diagnostic {
  severity: 'error' | 'warning' | 'info';
  code: string;
  message: string;
  elementPath?: string;
  quickFix?: QuickFix;
}
```

### R4: Navigation
- Click diagnostic to jump to element
- Highlight element in tree
- Show element in inspector
- Scroll element into view

### R5: Quick Fixes
- "Apply Fix" button for fixable issues
- Preview fix before applying
- Apply all fixes button
- Undo after applying fix

### R6: Filtering
- Filter by severity
- Filter by element path
- Search in messages
- Show/hide fixed issues

### R7: State Management
```typescript
export const $diagnostics = createStore<Diagnostic[]>([]);
export const $errorCount = $diagnostics.map(d => 
  d.filter(d => d.severity === 'error').length
);
export const diagnosticClicked = createEvent<Diagnostic>();
export const quickFixApplied = createEvent<QuickFix>();
```

### R8: Real-time Updates
- Update on validation complete
- Incremental updates
- Highlight new diagnostics
- Auto-scroll to new errors

## Acceptance Criteria
- [ ] Panel displays diagnostics correctly
- [ ] Severity counts are accurate
- [ ] Tabs filter by severity
- [ ] Click diagnostic navigates to element
- [ ] Quick fixes can be applied
- [ ] Filtering works correctly
- [ ] Real-time updates work
- [ ] Performance with 100+ diagnostics
- [ ] Unit tests pass
- [ ] Storybook stories exist

## Dependencies
- **UI 04**: Element Tree Viewer
- **Backend 12**: Validation API

## Priority
🔴 Critical - Core feature

## Estimated Complexity
Medium - 1 week
