import type {
  CreateArtifactInput,
  CreatedArtifact,
  CreateProjectInput,
  Project,
  ProjectTreeNode,
  UpdateProjectInput,
  ProjectResourceMetadata,
} from '@shared/types';
import { apiClient } from './client';

export const projectsApi = {
  async list(): Promise<Project[]> {
    return apiClient.get<Project[]>('/api/projects');
  },

  async get(projectId: string): Promise<Project> {
    return apiClient.get<Project>(`/api/projects/${projectId}`);
  },

  async create(payload: CreateProjectInput): Promise<Project> {
    return apiClient.post<Project>('/api/projects', payload);
  },

  async update(projectId: string, payload: UpdateProjectInput): Promise<Project> {
    return apiClient.patch<Project>(`/api/projects/${projectId}`, payload);
  },

  async tree(projectId: string): Promise<ProjectTreeNode[]> {
    return apiClient.get<ProjectTreeNode[]>(`/api/projects/${projectId}/tree`);
  },

  async resource(projectId: string, resourceId: string): Promise<ProjectResourceMetadata> {
    return apiClient.get<ProjectResourceMetadata>(
      `/api/projects/${projectId}/resources/${resourceId}`
    );
  },

  async createArtifact(projectId: string, input: CreateArtifactInput): Promise<CreatedArtifact> {
    return apiClient.post<CreatedArtifact>(`/api/projects/${projectId}/artifacts`, input);
  },
};
