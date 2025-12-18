import { api } from '@shared/api';
import type { CreateArtifactInput, CreatedArtifact, ProjectTreeNode } from '@shared/types';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

export const fileTreeKeys = {
  all: ['project-tree'] as const,
  detail: (projectId: string) => [...fileTreeKeys.all, projectId] as const,
};

export function useProjectTree(projectId: string) {
  return useQuery({
    queryKey: fileTreeKeys.detail(projectId),
    queryFn: () => api.projects.tree(projectId),
    enabled: Boolean(projectId),
    staleTime: 60 * 1000,
  });
}

export function useCreateArtifact() {
  const queryClient = useQueryClient();

  return useMutation<CreatedArtifact, Error, { projectId: string; input: CreateArtifactInput }>({
    mutationFn: ({ projectId, input }) => api.projects.createArtifact(projectId, input),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: fileTreeKeys.detail(variables.projectId) });
    },
  });
}
