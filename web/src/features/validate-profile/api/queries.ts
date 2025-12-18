import { api } from '@shared/api';
import type { ValidationResult } from '@shared/types';
import { useMutation, useQuery } from '@tanstack/react-query';

/**
 * Query keys for validation
 */
export const validationKeys = {
  all: ['validation'] as const,
  validate: (profileId: string) => [...validationKeys.all, profileId] as const,
};

/**
 * Validate a profile
 */
export function useValidateProfile(profileId: string, enabled = false) {
  return useQuery({
    queryKey: validationKeys.validate(profileId),
    queryFn: () => api.validation.validate(profileId),
    enabled: enabled && !!profileId,
    staleTime: 0, // Always refetch
  });
}

/**
 * Manual validation trigger
 */
export function useTriggerValidation() {
  return useMutation({
    mutationFn: (profileId: string) => api.validation.validate(profileId),
  });
}
