import { api } from '@shared/api';
import type { CreateProjectInput, Project, UpdateProjectInput } from '@shared/types';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

export const projectKeys = {
  all: ['projects'] as const,
  list: () => [...projectKeys.all, 'list'] as const,
  detail: (projectId: string) => [...projectKeys.all, 'detail', projectId] as const,
};

export function useProjects() {
  return useQuery({
    queryKey: projectKeys.list(),
    queryFn: () => api.projects.list(),
    staleTime: 2 * 60 * 1000,
  });
}

export function useProject(projectId: string | null | undefined) {
  return useQuery({
    queryKey: projectId ? projectKeys.detail(projectId) : projectKeys.detail(''),
    queryFn: () => api.projects.get(projectId as string),
    enabled: Boolean(projectId),
    staleTime: 2 * 60 * 1000,
  });
}

export function useCreateProject() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (payload: CreateProjectInput) => api.projects.create(payload),
    onSuccess: (project: Project) => {
      queryClient.invalidateQueries({ queryKey: projectKeys.list() });
      queryClient.setQueryData(projectKeys.detail(project.id), project);
    },
  });
}

export function useUpdateProject(projectId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (payload: UpdateProjectInput) => api.projects.update(projectId, payload),
    onSuccess: (project: Project) => {
      queryClient.invalidateQueries({ queryKey: projectKeys.list() });
      queryClient.setQueryData(projectKeys.detail(project.id), project);
    },
  });
}
