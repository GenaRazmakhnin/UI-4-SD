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
- [x] New project dialog opens
- [x] Templates display correctly
- [x] Template selection works
- [x] Configuration form validates
- [x] Project creation succeeds
- [x] Project opens after creation
- [x] Recent projects list works
- [x] Quick open works
- [ ] Unit tests pass (TODO)
- [ ] Integration tests with backend (TODO)

## Dependencies
- **UI 02**: App Initialization
- **Backend 15**: Project Management

## Priority
🟡 High - Onboarding feature

## Estimated Complexity
Medium - 1 week

## Implementation Progress

### Status: 🟢 UI Complete (Backend Integration Pending)

### Implementation Plan
1. ✅ Create project templates types and data
2. ✅ Implement state management for project creation
3. ✅ Create TemplateGallery component
4. ✅ Create ProjectConfigForm component
5. ✅ Create NewProjectDialog with wizard flow
6. ✅ Create RecentProjects component
7. ✅ Export feature and integrate

### Files Created
```
features/project-templates/
├── index.ts                     # Public exports
├── lib/
│   ├── index.ts                 # Lib exports
│   ├── types.ts                 # ProjectConfig, ProjectTemplate types
│   ├── templates.ts             # 6 project templates (Blank, US Core, IPA, mCODE, SMART, R5)
│   └── validation.ts            # Form validation utilities
├── model/
│   └── index.ts                 # Effector stores, events, effects
└── ui/
    ├── NewProjectDialog.tsx     # 3-step wizard modal
    ├── TemplateGallery.tsx      # Template selection grid
    ├── TemplateGallery.module.css
    ├── ProjectConfigForm.tsx    # Configuration form
    ├── RecentProjects.tsx       # Recent projects list
    └── RecentProjects.module.css
```

### Available Templates
1. **Blank Project** - Empty FHIR R4 project
2. **US Core Based** - US Core IG with common profiles
3. **IPA** - International Patient Access
4. **mCODE Oncology** - Cancer data elements
5. **SMART on FHIR App** - SMART App Launch
6. **FHIR R5** - Latest FHIR R5 specification

### Features
- 3-step wizard: Template → Configure → Review
- Form validation (name, URL, packageId, version)
- Auto-generated package ID suggestions
- Dependency management
- Git initialization option
- Recent projects with persistence
- Quick open functionality

### Remaining Work
- Connect createProjectFx/openProjectFx to backend API
- Add unit tests
- Add integration tests

### Integration
- RecentProjects and NewProjectDialog integrated into ProjectBrowserPage
- NewProjectDialog opens via "New Project" button in RecentProjects component
