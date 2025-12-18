import type {
  ElementNode,
  ExportResult,
  Extension,
  Package,
  Profile,
  SearchFilters,
  SearchResult,
  ValidationResult,
  ValueSet,
  ValueSetExpansion,
} from '@shared/types';
import { apiClient } from '../client';

/**
 * Real API implementation that connects to the backend
 * This will be implemented once the backend API is ready
 */
export const realApi = {
  profiles: {
    async list(): Promise<Profile[]> {
      return apiClient.get<Profile[]>('/api/profiles');
    },

    async get(id: string): Promise<Profile> {
      return apiClient.get<Profile>(`/api/profiles/${id}`);
    },

    async create(data: Partial<Profile>): Promise<Profile> {
      return apiClient.post<Profile>('/api/profiles', data);
    },

    async update(id: string, data: Partial<Profile>): Promise<Profile> {
      return apiClient.patch<Profile>(`/api/profiles/${id}`, data);
    },

    async delete(id: string): Promise<void> {
      return apiClient.delete<void>(`/api/profiles/${id}`);
    },

    async updateElement(
      profileId: string,
      elementPath: string,
      updates: Partial<ElementNode>
    ): Promise<Profile> {
      return apiClient.patch<Profile>(
        `/api/profiles/${profileId}/elements/${encodeURIComponent(elementPath)}`,
        updates
      );
    },

    async addSlice(
      profileId: string,
      elementPath: string,
      slice: {
        sliceName: string;
        min: number;
        max: string;
        short?: string;
      }
    ): Promise<Profile> {
      return apiClient.post<Profile>(
        `/api/profiles/${profileId}/elements/${encodeURIComponent(elementPath)}/slices`,
        slice
      );
    },
  },

  packages: {
    async list(): Promise<Package[]> {
      return apiClient.get<Package[]>('/api/packages');
    },

    async search(query: string): Promise<Package[]> {
      return apiClient.get<Package[]>(`/api/packages/search?q=${query}`);
    },

    async install(packageId: string): Promise<Package> {
      return apiClient.post<Package>(`/api/packages/${packageId}/install`);
    },

    async uninstall(packageId: string): Promise<void> {
      return apiClient.post<void>(`/api/packages/${packageId}/uninstall`);
    },
  },

  search: {
    async resources(query: string, filters?: SearchFilters): Promise<SearchResult[]> {
      const params = new URLSearchParams({ q: query });
      if (filters?.type) {
        for (const type of filters.type) {
          params.append('type', type);
        }
      }
      return apiClient.get<SearchResult[]>(`/api/search/resources?${params}`);
    },

    async extensions(query: string, filters?: { package?: string[] }): Promise<Extension[]> {
      const params = new URLSearchParams({ q: query });
      if (filters?.package) {
        for (const pkg of filters.package) {
          params.append('package', pkg);
        }
      }
      return apiClient.get<Extension[]>(`/api/search/extensions?${params}`);
    },

    async valueSets(query: string, options?: { codeSystem?: string[] }): Promise<ValueSet[]> {
      const params = new URLSearchParams({ q: query });
      if (options?.codeSystem) {
        for (const system of options.codeSystem) {
          params.append('codeSystem', system);
        }
      }
      return apiClient.get<ValueSet[]>(`/api/search/valuesets?${params}`);
    },
  },

  terminology: {
    async expand(valueSetUrl: string): Promise<ValueSetExpansion> {
      return apiClient.get<ValueSetExpansion>(
        `/api/terminology/expand?url=${encodeURIComponent(valueSetUrl)}`
      );
    },
  },

  validation: {
    async validate(profileId: string): Promise<ValidationResult> {
      return apiClient.post<ValidationResult>(`/api/validation/validate/${profileId}`);
    },
  },

  export: {
    async toSD(profileId: string): Promise<ExportResult> {
      return apiClient.get<ExportResult>(`/api/export/${profileId}/sd`);
    },

    async toFSH(profileId: string): Promise<ExportResult> {
      return apiClient.get<ExportResult>(`/api/export/${profileId}/fsh`);
    },
  },

  undo: {
    async canUndo(profileId: string): Promise<boolean> {
      return apiClient.get<boolean>(`/api/undo/${profileId}/can-undo`);
    },

    async canRedo(profileId: string): Promise<boolean> {
      return apiClient.get<boolean>(`/api/undo/${profileId}/can-redo`);
    },

    async undo(profileId: string): Promise<Profile> {
      return apiClient.post<Profile>(`/api/undo/${profileId}/undo`);
    },

    async redo(profileId: string): Promise<Profile> {
      return apiClient.post<Profile>(`/api/undo/${profileId}/redo`);
    },
  },
};
