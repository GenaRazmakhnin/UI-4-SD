# Task: Slicing Wizard Feature

## Description
Implement the step-by-step slicing wizard that guides users through creating and configuring slicing definitions.

## Requirements

### R1: Wizard Steps
**Step 1: Discriminator Selection**
- Choose discriminator type (value/exists/pattern/type/profile)
- Select discriminator path from available element children
- Add multiple discriminators
- Validation for path existence

**Step 2: Slicing Rules**
- Select rule (closed/open/openAtEnd)
- Set ordered flag
- Add description

**Step 3: Create Slices**
- Add named slices
- Set cardinality for each
- Quick templates (common slicing patterns)

**Step 4: Review & Apply**
- Preview impact
- Show what will change in SD
- Apply or cancel

### R2: Discriminator Path Selector
- Tree view of available paths
- FHIRPath expression support
- Validation of path existence
- Common paths suggestions

### R3: Slicing Templates
Provide templates for common patterns:
- Extension slicing by URL
- Coding slicing by system
- Identifier slicing by system
- Reference slicing by type

### R4: Impact Preview
- Show discriminator rules
- Show slice structure
- Explain what discriminator does
- Validate completeness

### R5: Slice Management
- Add/remove slices after creation
- Edit slice constraints
- Re-order slices
- Delete slicing entirely

## Acceptance Criteria
- [ ] Wizard opens for sliceable elements
- [ ] All steps navigate correctly
- [ ] Discriminator selection works
- [ ] Path selector shows valid paths
- [ ] Slicing rules can be set
- [ ] Slices can be created
- [ ] Templates apply correctly
- [ ] Impact preview is accurate
- [ ] Validation prevents errors
- [ ] Changes persist to backend
- [ ] Undo/redo works
- [ ] Unit tests pass

## Dependencies
- **UI 03**: Mock Data Layer
- **Backend 13**: Operations Engine (slicing ops)

## Priority
🟡 High - Beta feature

## Estimated Complexity
Very High - 2-3 weeks
