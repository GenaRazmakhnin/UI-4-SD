# Task: Project Templates UI

## Description
Implement the UI for creating new projects from templates with configuration wizard.

## Requirements

### R1: New Project Dialog
- Modal dialog for project creation
- Template selection
- Project configuration form
- Validation feedback

### R2: Template Gallery
Display available templates:
- Blank project
- US Core based
- IPA based
- Custom templates

Each template shows:
- Name and description
- Preview of structure
- Included dependencies
- FHIR version

### R3: Project Configuration Form
```typescript
interface ProjectConfig {
  name: string;
  canonicalBase: string;
  fhirVersion: FhirVersion;
  packageId: string;
  version: string;
  dependencies: PackageDependency[];
}
```

Fields:
- Project name (required)
- Canonical base URL (required)
- FHIR version selector
- Package ID (auto-generated suggestion)
- Initial dependencies

### R4: Validation
- Project name: no special characters
- Canonical base: valid URL format
- Package ID: valid npm package format
- Check for existing projects

### R5: Template Customization
- Add/remove dependencies
- Configure metadata
- Set up directory structure
- Initialize Git repository option

### R6: Project Creation Flow
1. Select template
2. Configure project
3. Review settings
4. Create project
5. Open in editor

### R7: Recent Projects
- List recent projects
- Quick open
- Remove from recents
- Project metadata display

## Acceptance Criteria
- [ ] New project dialog opens
- [ ] Templates display correctly
- [ ] Template selection works
- [ ] Configuration form validates
- [ ] Project creation succeeds
- [ ] Project opens after creation
- [ ] Recent projects list works
- [ ] Quick open works
- [ ] Unit tests pass
- [ ] Integration tests with backend

## Dependencies
- **UI 02**: App Initialization
- **Backend 15**: Project Management

## Priority
🟡 High - Onboarding feature

## Estimated Complexity
Medium - 1 week
