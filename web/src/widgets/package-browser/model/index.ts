import { api } from '@shared/api';
import type { Package, PackageInstallProgress, PackageResource } from '@shared/types';
import { combine, createEffect, createEvent, createStore, sample } from 'effector';

// ============================================================================
// Types
// ============================================================================

export type PackageView = 'installed' | 'browse' | 'details';
export type PackageSortBy = 'relevance' | 'downloads' | 'date' | 'name';
export type ResourceTypeFilter =
  | 'all'
  | 'StructureDefinition'
  | 'ValueSet'
  | 'CodeSystem'
  | 'SearchParameter';

export interface PackageFilters {
  fhirVersion?: string;
  sortBy: PackageSortBy;
  searchQuery: string;
}

export interface ResourceFilters {
  type: ResourceTypeFilter;
  searchQuery: string;
}

// ============================================================================
// Events
// ============================================================================

// Navigation
export const viewChanged = createEvent<PackageView>();
export const packageSelected = createEvent<string | null>();

// Search and filters
export const searchQueryChanged = createEvent<string>();
export const registrySearchTriggered = createEvent<string>();
export const filtersChanged = createEvent<Partial<PackageFilters>>();
export const filtersReset = createEvent();

// Resource browser
export const resourceFiltersChanged = createEvent<Partial<ResourceFilters>>();
export const resourceSelected = createEvent<PackageResource | null>();

// Installation
export const installRequested = createEvent<string>();
export const installVersionRequested = createEvent<{ packageId: string; version: string }>();
export const uninstallRequested = createEvent<string>();
export const updateRequested = createEvent<string>();

// ============================================================================
// Effects
// ============================================================================

export const fetchPackagesFx = createEffect(async () => {
  return api.packages.list();
});

export const fetchInstalledPackagesFx = createEffect(async () => {
  return api.packages.getInstalledPackages();
});

export const fetchPackageDetailsFx = createEffect(async (packageId: string) => {
  return api.packages.get(packageId);
});

export const searchRegistryFx = createEffect(
  async (params: { query: string; fhirVersion?: string; sortBy?: PackageSortBy }) => {
    return api.packages.searchRegistry(params.query, {
      fhirVersion: params.fhirVersion,
      sortBy:
        params.sortBy === 'name' || params.sortBy === 'relevance' ? 'relevance' : params.sortBy,
    });
  }
);

export const fetchPackageResourcesFx = createEffect(
  async (params: { packageId: string; type?: string; query?: string }) => {
    return api.packages.getResources(params.packageId, {
      type: params.type === 'all' ? undefined : params.type,
      query: params.query,
    });
  }
);

export const installPackageFx = createEffect(async (packageId: string) => {
  return api.packages.install(packageId);
});

export const installPackageVersionFx = createEffect(
  async (params: { packageId: string; version: string }) => {
    return api.packages.installVersion(params.packageId, params.version);
  }
);

export const uninstallPackageFx = createEffect(async (packageId: string) => {
  await api.packages.uninstall(packageId);
  return packageId;
});

export const updatePackageFx = createEffect(async (packageId: string) => {
  return api.packages.update(packageId);
});

// ============================================================================
// Stores
// ============================================================================

// View state
export const $currentView = createStore<PackageView>('installed').on(
  viewChanged,
  (_, view) => view
);

export const $selectedPackageId = createStore<string | null>(null)
  .on(packageSelected, (_, id) => id)
  .reset(viewChanged);

// Package lists
export const $packages = createStore<Package[]>([])
  .on(fetchPackagesFx.doneData, (_, packages) => packages)
  .on(installPackageFx.doneData, (packages, installed) =>
    packages.map((pkg) => (pkg.id === installed.id ? installed : pkg))
  )
  .on(installPackageVersionFx.doneData, (packages, installed) =>
    packages.map((pkg) => (pkg.id === installed.id ? installed : pkg))
  )
  .on(uninstallPackageFx.doneData, (packages, uninstalledId) =>
    packages.map((pkg) => (pkg.id === uninstalledId ? { ...pkg, installed: false } : pkg))
  )
  .on(updatePackageFx.doneData, (packages, updated) =>
    packages.map((pkg) => (pkg.id === updated.id ? updated : pkg))
  );

export const $installedPackages = $packages.map((packages) =>
  packages.filter((pkg) => pkg.installed)
);

export const $searchResults = createStore<Package[]>([]).on(
  searchRegistryFx.doneData,
  (_, results) => results
);

// Selected package details
export const $selectedPackage = createStore<Package | null>(null)
  .on(fetchPackageDetailsFx.doneData, (_, pkg) => pkg)
  .reset(packageSelected);

// Package resources
export const $packageResources = createStore<PackageResource[]>([])
  .on(fetchPackageResourcesFx.doneData, (_, resources) => resources)
  .reset(packageSelected);

export const $selectedResource = createStore<PackageResource | null>(null)
  .on(resourceSelected, (_, resource) => resource)
  .reset(packageSelected);

// Filters
export const $filters = createStore<PackageFilters>({
  sortBy: 'downloads',
  searchQuery: '',
})
  .on(filtersChanged, (state, updates) => ({ ...state, ...updates }))
  .on(searchQueryChanged, (state, query) => ({ ...state, searchQuery: query }))
  .reset(filtersReset);

export const $resourceFilters = createStore<ResourceFilters>({
  type: 'all',
  searchQuery: '',
})
  .on(resourceFiltersChanged, (state, updates) => ({ ...state, ...updates }))
  .reset(packageSelected);

// Loading states
export const $isLoadingPackages = fetchPackagesFx.pending;
export const $isSearching = searchRegistryFx.pending;
export const $isLoadingDetails = fetchPackageDetailsFx.pending;
export const $isLoadingResources = fetchPackageResourcesFx.pending;

// Installation progress
export const $installProgress = createStore<Record<string, PackageInstallProgress>>({})
  .on(installPackageFx, (state, packageId) => ({
    ...state,
    [packageId]: { packageId, status: 'installing', progress: 0, message: 'Installing...' },
  }))
  .on(installPackageFx.doneData, (state, pkg) => ({
    ...state,
    [pkg.id]: { packageId: pkg.id, status: 'installed', progress: 100, message: 'Installed' },
  }))
  .on(installPackageFx.failData, (state, error) => {
    // Find the installing package and mark as error
    const installing = Object.entries(state).find(([, p]) => p.status === 'installing');
    if (installing) {
      return {
        ...state,
        [installing[0]]: {
          ...installing[1],
          status: 'error',
          error: error.message,
        },
      };
    }
    return state;
  });

export const $isInstalling = (packageId: string) =>
  $installProgress.map((progress) => progress[packageId]?.status === 'installing');

// Errors
export const $error = createStore<string | null>(null)
  .on(fetchPackagesFx.failData, (_, error) => error.message)
  .on(searchRegistryFx.failData, (_, error) => error.message)
  .on(fetchPackageDetailsFx.failData, (_, error) => error.message)
  .on(installPackageFx.failData, (_, error) => error.message)
  .on(uninstallPackageFx.failData, (_, error) => error.message)
  .reset([fetchPackagesFx, searchRegistryFx, fetchPackageDetailsFx]);

// ============================================================================
// Derived stores
// ============================================================================

export const $filteredInstalledPackages = combine(
  $installedPackages,
  $filters,
  (packages, filters) => {
    let result = [...packages];

    // Filter by search query
    if (filters.searchQuery) {
      const query = filters.searchQuery.toLowerCase();
      result = result.filter(
        (pkg) =>
          pkg.name.toLowerCase().includes(query) ||
          pkg.description?.toLowerCase().includes(query) ||
          pkg.publisher?.toLowerCase().includes(query)
      );
    }

    // Sort
    switch (filters.sortBy) {
      case 'name':
        result.sort((a, b) => a.name.localeCompare(b.name));
        break;
      case 'downloads':
        result.sort((a, b) => (b.downloadCount || 0) - (a.downloadCount || 0));
        break;
      case 'date':
        result.sort((a, b) => {
          const dateA = a.publishedDate ? new Date(a.publishedDate).getTime() : 0;
          const dateB = b.publishedDate ? new Date(b.publishedDate).getTime() : 0;
          return dateB - dateA;
        });
        break;
    }

    return result;
  }
);

export const $packagesWithUpdates = $installedPackages.map((packages) =>
  packages.filter((pkg) => pkg.hasUpdate)
);

export const $updateCount = $packagesWithUpdates.map((packages) => packages.length);

// ============================================================================
// Sample connections (side effects)
// ============================================================================

// Trigger search on registry when search query changes (with debounce would be ideal)
sample({
  clock: registrySearchTriggered,
  source: $filters,
  fn: (filters, query) => ({
    query,
    fhirVersion: filters.fhirVersion,
    sortBy: filters.sortBy,
  }),
  target: searchRegistryFx,
});

// Fetch package details when selected
sample({
  clock: packageSelected,
  filter: (id): id is string => id !== null,
  target: fetchPackageDetailsFx,
});

// Fetch resources when package is selected and view is details
sample({
  clock: fetchPackageDetailsFx.doneData,
  fn: (pkg) => ({ packageId: pkg.id }),
  target: fetchPackageResourcesFx,
});

// Connect install/uninstall/update events to effects
sample({
  clock: installRequested,
  target: installPackageFx,
});

sample({
  clock: installVersionRequested,
  target: installPackageVersionFx,
});

sample({
  clock: uninstallRequested,
  target: uninstallPackageFx,
});

sample({
  clock: updateRequested,
  target: updatePackageFx,
});

// Refetch resources when filters change
sample({
  clock: resourceFiltersChanged,
  source: { packageId: $selectedPackageId, filters: $resourceFilters },
  filter: ({ packageId }) => packageId !== null,
  fn: ({ packageId, filters }) => ({
    packageId: packageId!,
    type: filters.type,
    query: filters.searchQuery,
  }),
  target: fetchPackageResourcesFx,
});
