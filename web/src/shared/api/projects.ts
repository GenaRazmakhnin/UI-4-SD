import type {
  CreateArtifactInput,
  CreatedArtifact,
  CreateProjectInput,
  Project,
  ProjectResourceMetadata,
  ProjectTreeNode,
  UpdateProjectInput,
} from '@shared/types';
import { apiClient } from './client';

/** Generic API response wrapper */
interface ApiResponse<T> {
  success: boolean;
  data: T;
  error?: { code: string; message: string };
}

/** Profile details response from backend */
export interface ProfileDetailsResponse {
  documentId: string;
  metadata: ProfileMetadata;
  resource: ProfiledResource;
  history: HistorySummary;
  isDirty: boolean;
  filePath?: string;
}

/** Profile metadata */
export interface ProfileMetadata {
  id: string;
  url: string;
  name: string;
  title?: string;
  description?: string;
  status: string;
  version?: string;
  publisher?: string;
  purpose?: string;
  copyright?: string;
  experimental: boolean;
}

/** Profiled resource with element tree */
export interface ProfiledResource {
  url: string;
  version?: string;
  fhirVersion: string;
  base: { url: string; type?: string };
  kind: string;
  root: BackendElementNode;
}

/** Backend element node structure */
export interface BackendElementNode {
  id?: string;
  path: string;
  sliceName?: string;
  source: 'Base' | 'Modified' | 'Added';
  constraints: ElementConstraints;
  children: BackendElementNode[];
}

/** Element constraints */
export interface ElementConstraints {
  cardinality?: { min: number; max?: number | string };
  types?: Array<{ code: string; profile?: string[]; targetProfile?: string[] }>;
  binding?: { strength: string; valueSet: string; description?: string };
  flags: { mustSupport: boolean; isModifier: boolean; isSummary: boolean };
  short?: string;
  definition?: string;
  comment?: string;
}

/** History summary */
export interface HistorySummary {
  canUndo: boolean;
  canRedo: boolean;
  undoCount: number;
  redoCount: number;
}

/** Update profile metadata request */
export interface UpdateProfileMetadata {
  name?: string;
  title?: string;
  description?: string;
  status?: string;
  version?: string;
  publisher?: string;
  purpose?: string;
  copyright?: string;
  experimental?: boolean;
}

/** Update element request */
export interface UpdateElementRequest {
  cardinality?: { min?: number; max?: string };
  flags?: { mustSupport?: boolean; isModifier?: boolean; isSummary?: boolean };
  types?: Array<{ code: string; profile?: string[]; targetProfile?: string[] }>;
  binding?: { strength: string; valueSet: string; description?: string };
  short?: string;
  definition?: string;
  comment?: string;
}

/** Update element response */
export interface UpdateElementResponse {
  path: string;
  constraints: ElementConstraints;
  validation: Array<{ severity: string; code: string; message: string; path?: string }>;
}

/**
 * Backend returns projects in a list wrapper
 */
interface ProjectListResponse {
  projects: Project[];
}

/**
 * Backend returns project details with resources
 */
interface ProjectDetailsResponse {
  project: Project;
  resources: ProjectResourceMetadata[];
}

export const projectsApi = {
  async list(): Promise<Project[]> {
    const response = await apiClient.get<ProjectListResponse>('/api/projects');
    return response.projects;
  },

  async get(projectId: string): Promise<Project> {
    const response = await apiClient.get<ProjectDetailsResponse>(`/api/projects/${projectId}`);
    return response.project;
  },

  async getWithResources(projectId: string): Promise<ProjectDetailsResponse> {
    return apiClient.get<ProjectDetailsResponse>(`/api/projects/${projectId}`);
  },

  async create(payload: CreateProjectInput): Promise<Project> {
    return apiClient.post<Project>('/api/projects', payload);
  },

  async update(projectId: string, payload: UpdateProjectInput): Promise<Project> {
    return apiClient.put<Project>(`/api/projects/${projectId}`, payload);
  },

  async delete(projectId: string): Promise<void> {
    return apiClient.delete(`/api/projects/${projectId}`);
  },

  async tree(projectId: string): Promise<ProjectTreeNode[]> {
    // Backend returns a single root node with children, we extract the children
    const root = await apiClient.get<ProjectTreeNode>(`/api/projects/${projectId}/tree`);
    return root.children ?? [];
  },

  async resource(projectId: string, resourceId: string): Promise<ProjectResourceMetadata> {
    return apiClient.get<ProjectResourceMetadata>(
      `/api/projects/${projectId}/resources/${resourceId}`
    );
  },

  async createArtifact(projectId: string, input: CreateArtifactInput): Promise<CreatedArtifact> {
    return apiClient.post<CreatedArtifact>(`/api/projects/${projectId}/artifacts`, input);
  },

  async deleteArtifact(projectId: string, resourceId: string): Promise<void> {
    return apiClient.delete(`/api/projects/${projectId}/resources/${resourceId}`);
  },

  /** Get project dependency graph */
  async dependencies(projectId: string): Promise<DependencyGraph> {
    return apiClient.get<DependencyGraph>(`/api/projects/${projectId}/dependencies`);
  },

  /** Get a profile's details including element tree */
  async getProfile(projectId: string, profileId: string): Promise<ProfileDetailsResponse> {
    const response = await apiClient.get<ApiResponse<ProfileDetailsResponse>>(
      `/api/projects/${projectId}/profiles/${encodeURIComponent(profileId)}`
    );
    return response.data;
  },

  /** Update a profile's metadata */
  async updateProfileMetadata(
    projectId: string,
    profileId: string,
    updates: UpdateProfileMetadata
  ): Promise<ProfileDetailsResponse> {
    const response = await apiClient.patch<ApiResponse<ProfileDetailsResponse>>(
      `/api/projects/${projectId}/profiles/${encodeURIComponent(profileId)}/metadata`,
      updates
    );
    return response.data;
  },

  /** Update an element's constraints */
  async updateElement(
    projectId: string,
    profileId: string,
    elementPath: string,
    updates: UpdateElementRequest
  ): Promise<UpdateElementResponse> {
    const response = await apiClient.patch<ApiResponse<UpdateElementResponse>>(
      `/api/projects/${projectId}/profiles/${encodeURIComponent(profileId)}/elements/${encodeURIComponent(elementPath)}`,
      updates
    );
    return response.data;
  },

  /** Save profile (persists current state) */
  async saveProfile(projectId: string, profileId: string): Promise<ProfileDetailsResponse> {
    const response = await apiClient.post<ApiResponse<ProfileDetailsResponse>>(
      `/api/projects/${projectId}/profiles/${encodeURIComponent(profileId)}/save`
    );
    return response.data;
  },
};

/** Dependency graph for a project */
export interface DependencyGraph {
  /** Root resources (no dependencies) */
  roots: string[];
  /** All nodes in the graph */
  nodes: DependencyNode[];
  /** Edges representing dependencies */
  edges: DependencyEdge[];
}

/** A node in the dependency graph */
export interface DependencyNode {
  id: string;
  name: string;
  type: 'profile' | 'extension' | 'valueset' | 'codesystem';
  url: string;
}

/** An edge in the dependency graph */
export interface DependencyEdge {
  from: string;
  to: string;
  type: 'baseDefinition' | 'type' | 'binding' | 'extension';
}
