import type {
  BaseResource,
  BulkExportOptions,
  BulkExportResponse,
  ElementNode,
  ElementSearchResult,
  ExportResult,
  Extension,
  FshExportOptions,
  FshExportResponse,
  ImportProfileRequest,
  ImportProfileResponse,
  InstallEvent,
  InstallJob,
  Package,
  PackageResource,
  PackageSearchResult,
  PreviewOptions,
  PreviewResponse,
  Profile,
  SdExportOptions,
  SdExportResponse,
  SearchFilters,
  SearchResponseWithFacets,
  SearchResult,
  ValidationResult,
  ValueSet,
  ValueSetExpansion,
} from '@shared/types';
import { apiClient } from '../client';
import { projectsApi } from '../projects';

/**
 * Real API implementation that connects to the backend
 */
export const realApi = {
  projects: projectsApi,

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

    /** Import a profile from SD JSON or FSH content */
    async import(
      projectId: string,
      profileId: string,
      request: ImportProfileRequest
    ): Promise<ImportProfileResponse> {
      return apiClient.post<ImportProfileResponse>(
        `/api/projects/${projectId}/profiles/${encodeURIComponent(profileId)}/import`,
        request
      );
    },
  },

  packages: {
    async list(): Promise<Package[]> {
      return apiClient.get<Package[]>('/api/packages');
    },

    async getInstalledPackages(): Promise<Package[]> {
      const packages = await apiClient.get<Package[]>('/api/packages');
      return packages.filter((pkg) => pkg.installed);
    },

    async get(packageId: string): Promise<Package> {
      return apiClient.get<Package>(`/api/packages/${encodeURIComponent(packageId)}`);
    },

    async getResources(
      packageId: string,
      options?: { type?: string; query?: string }
    ): Promise<PackageResource[]> {
      const params = new URLSearchParams();
      if (options?.query) params.append('q', options.query);
      if (options?.type) params.append('resource_type', options.type);
      params.append('package', packageId);

      const response = await apiClient.get<SearchResponseWithFacets<PackageResource>>(
        `/api/search/resources?${params}`
      );
      return response.results;
    },

    async search(query: string): Promise<Package[]> {
      return apiClient.get<Package[]>(`/api/packages/search?q=${encodeURIComponent(query)}`);
    },

    /** Search the FHIR registry for packages with FHIR version filtering */
    async searchRegistry(
      query: string,
      options?: {
        fhirVersion?: string;
        sortBy?: 'relevance' | 'downloads' | 'date';
        limit?: number;
      }
    ): Promise<PackageSearchResult[]> {
      const params = new URLSearchParams({ q: query });
      if (options?.fhirVersion) params.append('fhirVersion', options.fhirVersion);
      if (options?.sortBy) params.append('sortBy', options.sortBy);
      if (options?.limit) params.append('limit', options.limit.toString());

      return apiClient.get<PackageSearchResult[]>(`/api/packages/search?${params}`);
    },

    /** Install package (returns immediately, blocks until complete) */
    async install(packageId: string): Promise<Package> {
      return apiClient.post<Package>(`/api/packages/${encodeURIComponent(packageId)}/install`);
    },

    /** Start package installation job (for polling-based progress) */
    async startInstall(packageId: string): Promise<InstallJob> {
      return apiClient.post<InstallJob>(
        `/api/packages/${encodeURIComponent(packageId)}/install/start`
      );
    },

    /** Get install job status (for polling) */
    async getInstallJob(jobId: string): Promise<InstallJob> {
      return apiClient.get<InstallJob>(`/api/packages/jobs/${encodeURIComponent(jobId)}`);
    },

    /** Install with SSE progress (legacy, may not work in all environments) */
    async installWithProgress(
      packageId: string,
      onEvent: (event: InstallEvent) => void,
      options?: { signal?: AbortSignal }
    ): Promise<void> {
      return apiClient.streamSSE<InstallEvent>(
        `/api/packages/${encodeURIComponent(packageId)}/install`,
        onEvent,
        options
      );
    },

    async installVersion(packageId: string, version: string): Promise<Package> {
      const fullId = `${packageId}@${version}`;
      return apiClient.post<Package>(`/api/packages/${encodeURIComponent(fullId)}/install`);
    },

    async uninstall(packageId: string): Promise<void> {
      return apiClient.post<void>(`/api/packages/${encodeURIComponent(packageId)}/uninstall`);
    },

    async update(packageId: string): Promise<Package> {
      // Update means installing the latest version
      // The package ID without version will install latest
      const nameOnly = packageId.split('@')[0];
      return apiClient.post<Package>(`/api/packages/${encodeURIComponent(nameOnly)}/install`);
    },
  },

  search: {
    async resources(
      query: string,
      filters?: SearchFilters
    ): Promise<SearchResponseWithFacets<SearchResult>> {
      const params = new URLSearchParams();
      if (query) params.append('q', query);
      if (filters?.type) {
        for (const type of filters.type) {
          params.append('resource_type', type);
        }
      }
      if (filters?.package) {
        for (const pkg of filters.package) {
          params.append('package', pkg);
        }
      }
      return apiClient.get<SearchResponseWithFacets<SearchResult>>(
        `/api/search/resources?${params}`
      );
    },

    async extensions(
      query: string,
      filters?: {
        package?: string[];
        context?: string;
        contextPath?: string;
        fhirVersion?: string;
      }
    ): Promise<SearchResponseWithFacets<Extension>> {
      const params = new URLSearchParams();
      if (query) params.append('q', query);
      if (filters?.package) {
        for (const pkg of filters.package) {
          params.append('package', pkg);
        }
      }
      if (filters?.context) params.append('context', filters.context);
      if (filters?.contextPath) params.append('context_path', filters.contextPath);
      if (filters?.fhirVersion) params.append('fhir_version', filters.fhirVersion);

      return apiClient.get<SearchResponseWithFacets<Extension>>(`/api/search/extensions?${params}`);
    },

    async valueSets(
      query: string,
      options?: {
        package?: string[];
        system?: string;
        fhirVersion?: string;
      }
    ): Promise<SearchResponseWithFacets<ValueSet>> {
      const params = new URLSearchParams();
      if (query) params.append('q', query);
      if (options?.package) {
        for (const pkg of options.package) {
          params.append('package', pkg);
        }
      }
      if (options?.system) params.append('system', options.system);
      if (options?.fhirVersion) params.append('fhir_version', options.fhirVersion);

      return apiClient.get<SearchResponseWithFacets<ValueSet>>(`/api/search/valuesets?${params}`);
    },

    async profiles(
      query: string,
      options?: {
        package?: string[];
        baseType?: string;
        derivation?: 'constraint' | 'specialization';
        fhirVersion?: string;
      }
    ): Promise<SearchResponseWithFacets<SearchResult>> {
      const params = new URLSearchParams();
      if (query) params.append('q', query);
      if (options?.package) {
        for (const pkg of options.package) {
          params.append('package', pkg);
        }
      }
      if (options?.baseType) params.append('base_type', options.baseType);
      if (options?.derivation) params.append('derivation', options.derivation);
      if (options?.fhirVersion) params.append('fhir_version', options.fhirVersion);

      return apiClient.get<SearchResponseWithFacets<SearchResult>>(
        `/api/search/profiles?${params}`
      );
    },

    async baseResources(options?: {
      query?: string;
      package?: string[];
      fhirVersion?: string;
      limit?: number;
    }): Promise<BaseResource[]> {
      const params = new URLSearchParams();
      if (options?.query) params.append('q', options.query);
      if (options?.package) {
        for (const pkg of options.package) {
          params.append('package', pkg);
        }
      }
      if (options?.fhirVersion) params.append('fhir_version', options.fhirVersion);
      if (options?.limit) params.append('limit', options.limit.toString());

      const response = await apiClient.get<SearchResponseWithFacets<BaseResource>>(
        `/api/search/base-resources?${params}`
      );
      return response.results;
    },

    /** Search elements within a profile */
    async elements(
      profileId: string,
      options?: {
        query?: string;
        limit?: number;
      }
    ): Promise<ElementSearchResult[]> {
      const params = new URLSearchParams();
      params.append('profile_id', profileId);
      if (options?.query) params.append('q', options.query);
      if (options?.limit) params.append('limit', options.limit.toString());

      const response = await apiClient.get<SearchResponseWithFacets<ElementSearchResult>>(
        `/api/search/elements?${params}`
      );
      return response.results;
    },
  },

  terminology: {
    async expand(valueSetUrl: string): Promise<ValueSetExpansion> {
      return apiClient.get<ValueSetExpansion>(
        `/api/terminology/expand?url=${encodeURIComponent(valueSetUrl)}`
      );
    },
  },

  // Project-scoped validation APIs
  validation: {
    /** Validate a profile within a project */
    async validate(projectId: string, profileId: string): Promise<ValidationResult> {
      return apiClient.post<ValidationResult>(
        `/api/projects/${projectId}/profiles/${profileId}/validate`
      );
    },

    /** Quick structural validation */
    async quickValidate(projectId: string, profileId: string): Promise<ValidationResult> {
      return apiClient.post<ValidationResult>(
        `/api/projects/${projectId}/profiles/${profileId}/validate/quick`
      );
    },

    /** Get cached validation results */
    async getResults(projectId: string, profileId: string): Promise<ValidationResult> {
      return apiClient.get<ValidationResult>(
        `/api/projects/${projectId}/profiles/${profileId}/validation`
      );
    },

    /** Apply a quick fix */
    async applyFix(projectId: string, profileId: string, fixId: string): Promise<Profile> {
      return apiClient.post<Profile>(`/api/projects/${projectId}/profiles/${profileId}/apply-fix`, {
        fixId,
      });
    },
  },

  // Fetch the original input IT resource
  resources: {
    async getInputIt(projectId: string, profileId: string): Promise<unknown> {
      return apiClient.get<unknown>(
        `/api/projects/${projectId}/profiles/${encodeURIComponent(profileId)}/input-it`
      );
    },
  },

  // Project-scoped export APIs
  export: {
    /** Export profile as StructureDefinition JSON */
    async toSD(
      projectId: string,
      profileId: string,
      options?: SdExportOptions
    ): Promise<SdExportResponse> {
      const params = new URLSearchParams();
      if (options?.format) params.append('format', options.format);
      if (options?.pretty) params.append('pretty', 'true');
      if (options?.persist) params.append('persist', 'true');
      if (options?.force) params.append('force', 'true');

      const queryString = params.toString();
      const url = `/api/projects/${projectId}/profiles/${profileId}/export/sd${queryString ? `?${queryString}` : ''}`;
      return apiClient.get<SdExportResponse>(url);
    },

    /** Export profile as FHIR Shorthand */
    async toFSH(
      projectId: string,
      profileId: string,
      options?: FshExportOptions
    ): Promise<FshExportResponse> {
      const params = new URLSearchParams();
      if (options?.persist) params.append('persist', 'true');
      if (options?.force) params.append('force', 'true');

      const queryString = params.toString();
      const url = `/api/projects/${projectId}/profiles/${profileId}/export/fsh${queryString ? `?${queryString}` : ''}`;
      return apiClient.get<FshExportResponse>(url);
    },

    /** Preview profile without downloading */
    async preview(
      projectId: string,
      profileId: string,
      options?: PreviewOptions
    ): Promise<PreviewResponse> {
      const params = new URLSearchParams();
      if (options?.format) params.append('format', options.format);
      if (options?.highlight !== undefined) params.append('highlight', String(options.highlight));

      const queryString = params.toString();
      const url = `/api/projects/${projectId}/profiles/${profileId}/preview${queryString ? `?${queryString}` : ''}`;
      return apiClient.get<PreviewResponse>(url);
    },

    /** Bulk export all profiles in a project */
    async bulkExport(projectId: string, options?: BulkExportOptions): Promise<BulkExportResponse> {
      const params = new URLSearchParams();
      if (options?.format) params.append('format', options.format);
      if (options?.structure) params.append('structure', options.structure);
      if (options?.pretty) params.append('pretty', 'true');

      const queryString = params.toString();
      const url = `/api/projects/${projectId}/export${queryString ? `?${queryString}` : ''}`;
      return apiClient.get<BulkExportResponse>(url);
    },

    /** Download packaged export as ZIP (returns blob URL) */
    async downloadPackage(projectId: string, options?: BulkExportOptions): Promise<string> {
      const params = new URLSearchParams();
      params.append('structure', 'packaged');
      if (options?.format) params.append('format', options.format);
      if (options?.pretty) params.append('pretty', 'true');

      const response = await fetch(`/api/projects/${projectId}/export?${params}`, {
        headers: { Accept: 'application/zip' },
      });

      if (!response.ok) {
        throw new Error(`Export failed: ${response.statusText}`);
      }

      const blob = await response.blob();
      return URL.createObjectURL(blob);
    },

    // Legacy methods for backwards compatibility
    async toSDLegacy(profileId: string): Promise<ExportResult> {
      return apiClient.get<ExportResult>(`/api/export/${profileId}/sd`);
    },

    async toFSHLegacy(profileId: string): Promise<ExportResult> {
      return apiClient.get<ExportResult>(`/api/export/${profileId}/fsh`);
    },
  },

  // Project-scoped undo/redo APIs
  history: {
    /** Undo last operation */
    async undo(projectId: string, profileId: string): Promise<Profile> {
      return apiClient.post<Profile>(`/api/projects/${projectId}/profiles/${profileId}/undo`);
    },

    /** Redo next operation */
    async redo(projectId: string, profileId: string): Promise<Profile> {
      return apiClient.post<Profile>(`/api/projects/${projectId}/profiles/${profileId}/redo`);
    },

    /** Get operation history */
    async getHistory(
      projectId: string,
      profileId: string
    ): Promise<{ operations: HistoryOperation[]; currentIndex: number }> {
      return apiClient.get<{ operations: HistoryOperation[]; currentIndex: number }>(
        `/api/projects/${projectId}/profiles/${profileId}/history`
      );
    },

    /** Jump to specific history point */
    async gotoHistory(projectId: string, profileId: string, index: number): Promise<Profile> {
      return apiClient.post<Profile>(
        `/api/projects/${projectId}/profiles/${profileId}/history/goto`,
        { index }
      );
    },
  },

  // Legacy undo API for backwards compatibility
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

/** History operation type */
export interface HistoryOperation {
  id: string;
  description: string;
  timestamp: string;
  type: 'add' | 'update' | 'delete' | 'move';
}
