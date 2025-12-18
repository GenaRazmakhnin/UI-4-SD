import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@shared/api';
import type { Profile, ElementNode } from '@shared/types';

/**
 * Query keys for profile data
 */
export const profileKeys = {
  all: ['profiles'] as const,
  lists: () => [...profileKeys.all, 'list'] as const,
  list: (filters: string) => [...profileKeys.lists(), { filters }] as const,
  details: () => [...profileKeys.all, 'detail'] as const,
  detail: (id: string) => [...profileKeys.details(), id] as const,
};

/**
 * Fetch all profiles
 */
export function useProfiles() {
  return useQuery({
    queryKey: profileKeys.lists(),
    queryFn: () => api.profiles.list(),
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

/**
 * Fetch single profile by ID
 */
export function useProfile(id: string) {
  return useQuery({
    queryKey: profileKeys.detail(id),
    queryFn: () => api.profiles.get(id),
    enabled: !!id,
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Create new profile
 */
export function useCreateProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: Partial<Profile>) => api.profiles.create(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: profileKeys.lists() });
    },
  });
}

/**
 * Update profile
 */
export function useUpdateProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<Profile> }) =>
      api.profiles.update(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: profileKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: profileKeys.lists() });
    },
  });
}

/**
 * Delete profile
 */
export function useDeleteProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => api.profiles.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: profileKeys.lists() });
    },
  });
}

/**
 * Update element in profile
 */
export function useUpdateElement() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      profileId,
      elementPath,
      updates,
    }: {
      profileId: string;
      elementPath: string;
      updates: Partial<ElementNode>;
    }) => api.profiles.updateElement(profileId, elementPath, updates),
    onSuccess: (_, { profileId }) => {
      queryClient.invalidateQueries({ queryKey: profileKeys.detail(profileId) });
    },
  });
}
