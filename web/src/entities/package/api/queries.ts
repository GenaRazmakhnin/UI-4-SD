import { api } from '@shared/api';
import type { Package, PackageResource } from '@shared/types';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

/**
 * Query keys for package data
 */
export const packageKeys = {
  all: ['packages'] as const,
  lists: () => [...packageKeys.all, 'list'] as const,
  installed: () => [...packageKeys.all, 'installed'] as const,
  detail: (id: string) => [...packageKeys.all, 'detail', id] as const,
  resources: (id: string) => [...packageKeys.all, 'resources', id] as const,
  searches: () => [...packageKeys.all, 'search'] as const,
  search: (query: string) => [...packageKeys.searches(), { query }] as const,
  registrySearch: (query: string, options?: { fhirVersion?: string; sortBy?: string }) =>
    [...packageKeys.searches(), 'registry', { query, ...options }] as const,
};

/**
 * Fetch all packages
 */
export function usePackages() {
  return useQuery({
    queryKey: packageKeys.lists(),
    queryFn: () => api.packages.list(),
    staleTime: 10 * 60 * 1000, // 10 minutes
  });
}

/**
 * Fetch installed packages only
 */
export function useInstalledPackages() {
  return useQuery({
    queryKey: packageKeys.installed(),
    queryFn: () => api.packages.getInstalledPackages(),
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Fetch single package details
 */
export function usePackage(packageId: string) {
  return useQuery({
    queryKey: packageKeys.detail(packageId),
    queryFn: () => api.packages.get(packageId),
    enabled: !!packageId,
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Fetch package resources
 */
export function usePackageResources(
  packageId: string,
  options?: { type?: string; query?: string }
) {
  return useQuery({
    queryKey: [...packageKeys.resources(packageId), options],
    queryFn: () => api.packages.getResources(packageId, options),
    enabled: !!packageId,
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Search packages
 */
export function usePackageSearch(query: string) {
  return useQuery({
    queryKey: packageKeys.search(query),
    queryFn: () => api.packages.search(query),
    enabled: query.length > 0,
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Search packages from registry with filters
 */
export function useRegistrySearch(
  query: string,
  options?: { fhirVersion?: string; sortBy?: 'relevance' | 'downloads' | 'date' }
) {
  return useQuery({
    queryKey: packageKeys.registrySearch(query, options),
    queryFn: () => api.packages.searchRegistry(query, options),
    enabled: query.length > 0,
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Install package
 */
export function useInstallPackage() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (packageId: string) => api.packages.install(packageId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
      queryClient.invalidateQueries({ queryKey: packageKeys.installed() });
    },
  });
}

/**
 * Install specific package version
 */
export function useInstallPackageVersion() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ packageId, version }: { packageId: string; version: string }) =>
      api.packages.installVersion(packageId, version),
    onSuccess: (_, { packageId }) => {
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
      queryClient.invalidateQueries({ queryKey: packageKeys.installed() });
      queryClient.invalidateQueries({ queryKey: packageKeys.detail(packageId) });
    },
  });
}

/**
 * Uninstall package
 */
export function useUninstallPackage() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (packageId: string) => api.packages.uninstall(packageId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
      queryClient.invalidateQueries({ queryKey: packageKeys.installed() });
    },
  });
}

/**
 * Update package to latest version
 */
export function useUpdatePackage() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (packageId: string) => api.packages.update(packageId),
    onSuccess: (_, packageId) => {
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
      queryClient.invalidateQueries({ queryKey: packageKeys.installed() });
      queryClient.invalidateQueries({ queryKey: packageKeys.detail(packageId) });
    },
  });
}
