// Export main component

// Export types
export type {
  PackageFilters,
  PackageSortBy,
  PackageView,
  ResourceFilters,
  ResourceTypeFilter,
} from './model';
// Export model (stores, events, effects)
export {
  // Stores
  $currentView,
  $error,
  $filteredInstalledPackages,
  $filters,
  $installedPackages,
  $installProgress,
  $isInstalling,
  $isLoadingDetails,
  $isLoadingPackages,
  $isLoadingResources,
  $isSearching,
  $packageResources,
  $packages,
  $packagesWithUpdates,
  $resourceFilters,
  $searchResults,
  $selectedPackage,
  $selectedPackageId,
  $selectedResource,
  $updateCount,
  // Effects
  fetchInstalledPackagesFx,
  fetchPackageDetailsFx,
  fetchPackageResourcesFx,
  fetchPackagesFx,
  // Events
  filtersChanged,
  filtersReset,
  installPackageFx,
  installPackageVersionFx,
  installRequested,
  installVersionRequested,
  packageSelected,
  registrySearchTriggered,
  resourceFiltersChanged,
  resourceSelected,
  searchQueryChanged,
  searchRegistryFx,
  uninstallPackageFx,
  uninstallRequested,
  updatePackageFx,
  updateRequested,
  viewChanged,
} from './model';
// Export sub-components
export { InstallProgressModal } from './ui/InstallProgressModal';
export { PackageBrowser } from './ui/PackageBrowser';
export { PackageDetails } from './ui/PackageDetails';
export { PackageList } from './ui/PackageList';
export { PackageSearch } from './ui/PackageSearch';
export { ResourceBrowser } from './ui/ResourceBrowser';
export type { ResourceType } from './ui/ResourceSearchPanel';
export { ResourceSearchPanel } from './ui/ResourceSearchPanel';
export { UninstallConfirmModal } from './ui/UninstallConfirmModal';
