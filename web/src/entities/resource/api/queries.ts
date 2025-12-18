import { api } from '@shared/api';
import type { ProjectResourceMetadata } from '@shared/types';
import { useQuery } from '@tanstack/react-query';

export const projectResourceKeys = {
  all: ['project-resource'] as const,
  detail: (projectId: string, resourceId: string) =>
    [...projectResourceKeys.all, projectId, resourceId] as const,
};

export function useProjectResource(projectId: string, resourceId: string) {
  return useQuery<ProjectResourceMetadata>({
    queryKey: projectResourceKeys.detail(projectId, resourceId),
    queryFn: () => api.projects.resource(projectId, resourceId),
    enabled: Boolean(projectId && resourceId),
    staleTime: 60 * 1000,
  });
}
