# Task: Package Browser Widget

## Description
Implement the package browser for discovering, installing, and managing FHIR packages.

## Requirements

### R1: Package List View
- Display installed packages
- Package metadata (id, version, FHIR version)
- Dependency tree
- Uninstall button
- Update available indicator

### R2: Package Search
- Search packages.fhir.org registry
- Search by name/description
- Filter by FHIR version
- Sort by relevance/downloads/date

### R3: Package Installation
- Install from registry
- Select specific version
- Show install progress
- Handle dependencies automatically
- Error handling

### R4: Package Details
- Package description
- Version history
- Dependencies list
- Contained resources count
- License information
- Publisher

### R5: Resource Browser
- Browse resources in package
- Filter by resource type
- Search within package
- Preview resource
- Use resource as base for profile

### R6: Dependency Management
- Show dependency graph
- Detect conflicts
- Resolve dependencies
- Warn on circular dependencies

## Acceptance Criteria
- [x] Package list displays installed packages
- [x] Search finds packages from registry
- [x] Install package works
- [ ] Dependency resolution works
- [x] Package details display correctly
- [x] Resource browser shows package contents
- [x] Uninstall package works
- [x] Update detection works
- [ ] Unit tests pass
- [ ] Integration tests with API

## Dependencies
- **UI 03**: Mock Data Layer
- **Backend 10**: Package Management

## Priority
🟡 High - Beta feature

## Estimated Complexity
High - 2 weeks

## Implementation Progress

### Completed
1. **Package types enhanced** (`web/src/shared/types/package.ts`)
   - Added `PackageResourceCounts`, `PackageVersion`, `PackageResource`
   - Added `PackageSearchResult`, `PackageInstallStatus`, `PackageInstallProgress`
   - Extended `Package` interface with publisher, license, downloadCount, versions, etc.

2. **Mock API extended** (`web/src/shared/api/mock/`)
   - Enhanced mock packages with full metadata
   - Added 7 packages with realistic data (US Core, IPA, SDC, IPS, etc.)
   - Added `mockPackageResources` for resource browsing
   - New API methods: `get`, `searchRegistry`, `installVersion`, `update`, `getResources`, `getInstalledPackages`

3. **Effector model** (`web/src/widgets/package-browser/model/index.ts`)
   - Stores: packages, installedPackages, searchResults, selectedPackage, packageResources
   - Events: viewChanged, packageSelected, searchQueryChanged, filtersChanged, resourceFiltersChanged
   - Effects: fetchPackagesFx, searchRegistryFx, installPackageFx, uninstallPackageFx, updatePackageFx
   - Derived stores: filteredInstalledPackages, packagesWithUpdates, updateCount

4. **UI Components** (`web/src/widgets/package-browser/ui/`)
   - `PackageList.tsx` - displays installed packages with update badges, menu actions
   - `PackageSearch.tsx` - registry search with filters (FHIR version, sort by)
   - `PackageDetails.tsx` - detailed view with tabs (overview, resources, dependencies, versions)
   - `ResourceBrowser.tsx` - browse package resources with type filtering
   - `PackageBrowser.tsx` - main widget with installed/browse toggle

5. **React Query hooks** (`web/src/entities/package/api/queries.ts`)
   - Added: `useInstalledPackages`, `usePackage`, `usePackageResources`, `useRegistrySearch`
   - Added: `useInstallPackageVersion`, `useUpdatePackage`

6. **Widget exports** (`web/src/widgets/package-browser/index.ts`)
   - All components, stores, events, effects exported
   - Widget added to `web/src/widgets/index.ts`

7. **Page integration** (`web/src/pages/packages/`)
   - Created `PackagesPage` component
   - Added route `/packages` in `web/src/app/routes/index.tsx`
   - Added navigation icon in `TopNavigation.tsx`

8. **Effector scope fix** (all UI components)
   - Fixed critical issue where inputs/tabs weren't working
   - Root cause: events called directly (e.g., `viewChanged(value)`) update root store, not scoped store
   - Solution: use `useUnit` for both stores AND events, then call the returned functions
   - Fixed in: `PackageBrowser.tsx`, `PackageList.tsx`, `PackageSearch.tsx`, `PackageDetails.tsx`, `ResourceBrowser.tsx`

### TODO
- Unit tests
- Integration tests with real API
- Dependency graph visualization (R6)
- Circular dependency detection (R6)
