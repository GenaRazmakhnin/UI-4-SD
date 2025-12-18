import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@shared/api';
import type { Package } from '@shared/types';

/**
 * Query keys for package data
 */
export const packageKeys = {
  all: ['packages'] as const,
  lists: () => [...packageKeys.all, 'list'] as const,
  searches: () => [...packageKeys.all, 'search'] as const,
  search: (query: string) => [...packageKeys.searches(), { query }] as const,
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
 * Install package
 */
export function useInstallPackage() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (packageId: string) => api.packages.install(packageId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
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
    },
  });
}
