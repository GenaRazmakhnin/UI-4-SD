import type {
  ElementNode,
  ExportResult,
  Package,
  Profile,
  CreateProjectInput,
  UpdateProjectInput,
  Project,
  ProjectTreeNode,
  CreateArtifactInput,
  CreatedArtifact,
  SearchFilters,
  SearchResult,
  ValidationResult,
  ProjectResourceMetadata,
} from '@shared/types';
import * as fixtures from './fixtures';
import { simulateDelay, simulateError } from './utils';

// Helper functions
function findElementByPath(elements: ElementNode[], path: string): ElementNode | null {
  for (const element of elements) {
    if (element.path === path) return element;
    if (element.children.length > 0) {
      const found = findElementByPath(element.children, path);
      if (found) return found;
    }
  }
  return null;
}

function matchesQuery(item: SearchResult, query: string): boolean {
  const lowerQuery = query.toLowerCase();
  return (
    item.name?.toLowerCase().includes(lowerQuery) ||
    item.title?.toLowerCase().includes(lowerQuery) ||
    item.description?.toLowerCase().includes(lowerQuery) ||
    false
  );
}

function matchesFilters(item: SearchResult, filters?: SearchFilters): boolean {
  if (!filters) return true;
  // Implement filter matching logic
  return true;
}

export const mockApi = {
  projects: {
    async list(): Promise<Project[]> {
      await simulateDelay(200, 400);
      if (simulateError(0.05)) {
        throw new Error('Failed to load projects');
      }
      return [...fixtures.mockProjects];
    },

    async get(id: string): Promise<Project> {
      await simulateDelay(150, 300);
      const project = fixtures.mockProjectsById[id];
      if (!project) {
        throw new Error(`Project ${id} not found`);
      }

      const lastOpenedAt = new Date().toISOString();
      project.lastOpenedAt = lastOpenedAt;
      project.updatedAt = project.updatedAt || lastOpenedAt;

      return { ...project };
    },

  async create(input: CreateProjectInput): Promise<Project> {
    await simulateDelay(250, 500);
    if (simulateError(0.08)) {
      throw new Error('Failed to create project');
    }

      const project = fixtures.createMockProject({
        name: input.name.trim(),
        fhirVersion: input.fhirVersion,
        templateId: input.templateId,
        description: input.description,
        packageId: input.packageId,
        canonicalBase: input.canonicalBase,
        version: input.version,
        publisher: input.publisher,
        dependencies: input.dependencies,
      });

      return { ...project };
    },

    async update(id: string, payload: UpdateProjectInput): Promise<Project> {
      await simulateDelay(200, 400);
      const project = fixtures.mockProjectsById[id];
      if (!project) {
        throw new Error(`Project ${id} not found`);
      }

      const updated: Project = {
        ...project,
        ...payload,
        dependencies: payload.dependencies ?? project.dependencies ?? [],
        updatedAt: new Date().toISOString(),
      };

      fixtures.mockProjectsById[id] = updated;
      const index = fixtures.mockProjects.findIndex((p) => p.id === id);
      if (index !== -1) {
        fixtures.mockProjects[index] = updated;
      }

      return { ...updated };
    },

    async tree(id: string): Promise<ProjectTreeNode[]> {
      await simulateDelay(120, 260);
      const tree = fixtures.mockProjectTrees[id] ?? fixtures.mockProjectTrees.default;
      return fixtures.cloneProjectTree(tree);
    },

    async resource(projectId: string, resourceId: string): Promise<ProjectResourceMetadata> {
      await simulateDelay(120, 260);
      const tree = fixtures.mockProjectTrees[projectId] ?? fixtures.mockProjectTrees.default;
      const node = fixtures.findResourceNodeById(tree, resourceId);
      if (!node || node.kind !== 'file') {
        throw new Error(`Resource ${resourceId} not found`);
      }
      return fixtures.toResourceMetadata(projectId, node);
    },

    async createArtifact(projectId: string, input: CreateArtifactInput): Promise<CreatedArtifact> {
      await simulateDelay(150, 320);
      const tree = fixtures.mockProjectTrees[projectId] ?? fixtures.mockProjectTrees.default;
      const result = fixtures.addMockArtifact(projectId, tree, input);
      return { ...result };
    },
  },

  profiles: {
    async list(): Promise<Profile[]> {
      await simulateDelay(200, 400);
      if (simulateError(0.05)) {
        throw new Error('Failed to fetch profiles');
      }
      return fixtures.mockProfiles;
    },

    async get(id: string): Promise<Profile> {
      await simulateDelay(150, 300);
      const profile = fixtures.mockProfilesById[id];
      // Return default profile if not found
      if (!profile) {
        return fixtures.defaultProfile;
      }
      return profile;
    },

    async create(data: Partial<Profile>): Promise<Profile> {
      await simulateDelay(300, 600);
      const newProfile = fixtures.createMockProfile(data);
      fixtures.mockProfiles.push(newProfile);
      return newProfile;
    },

    async update(id: string, data: Partial<Profile>): Promise<Profile> {
      await simulateDelay(250, 500);
      const profile = fixtures.mockProfilesById[id];
      if (!profile) {
        throw new Error(`Profile ${id} not found`);
      }
      Object.assign(profile, data);
      profile.isDirty = true;
      return profile;
    },

    async delete(id: string): Promise<void> {
      await simulateDelay(200, 400);
      const index = fixtures.mockProfiles.findIndex((p) => p.id === id);
      if (index === -1) {
        throw new Error(`Profile ${id} not found`);
      }
      fixtures.mockProfiles.splice(index, 1);
    },

    async updateElement(
      profileId: string,
      elementPath: string,
      updates: Partial<ElementNode>
    ): Promise<Profile> {
      await simulateDelay(100, 250);
      const profile = fixtures.mockProfilesById[profileId];
      if (!profile) {
        throw new Error(`Profile ${profileId} not found`);
      }

      // Find and update element in tree
      const element = findElementByPath(profile.elements, elementPath);
      if (!element) {
        throw new Error(`Element ${elementPath} not found`);
      }

      Object.assign(element, updates);
      element.isModified = true;
      profile.isDirty = true;

      return profile;
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
      await simulateDelay(150, 300);
      const profile = fixtures.mockProfilesById[profileId];
      if (!profile) {
        throw new Error(`Profile ${profileId} not found`);
      }

      // Find parent element
      const element = findElementByPath(profile.elements, elementPath);
      if (!element) {
        throw new Error(`Element ${elementPath} not found`);
      }

      // Create new slice element
      const sliceElement: ElementNode = {
        id: `${element.id}:${slice.sliceName}`,
        path: element.path,
        sliceName: slice.sliceName,
        min: slice.min,
        max: slice.max,
        short: slice.short,
        isModified: true,
        children: [],
      };

      // Add to parent's children
      element.children.push(sliceElement);
      profile.isDirty = true;

      return profile;
    },
  },

  packages: {
    async list(): Promise<Package[]> {
      await simulateDelay(200, 400);
      return fixtures.mockPackages;
    },

    async get(packageId: string): Promise<Package> {
      await simulateDelay(150, 300);
      const pkg = fixtures.mockPackages.find((p) => p.id === packageId);
      if (!pkg) {
        throw new Error(`Package ${packageId} not found`);
      }
      return pkg;
    },

    async search(query: string): Promise<Package[]> {
      await simulateDelay(100, 300);
      return fixtures.mockPackages.filter(
        (pkg) =>
          pkg.name.toLowerCase().includes(query.toLowerCase()) ||
          pkg.description?.toLowerCase().includes(query.toLowerCase())
      );
    },

    async searchRegistry(
      query: string,
      options?: { fhirVersion?: string; sortBy?: 'relevance' | 'downloads' | 'date' }
    ): Promise<Package[]> {
      await simulateDelay(300, 600);
      let results = fixtures.mockPackages.filter(
        (pkg) =>
          pkg.name.toLowerCase().includes(query.toLowerCase()) ||
          pkg.description?.toLowerCase().includes(query.toLowerCase()) ||
          pkg.publisher?.toLowerCase().includes(query.toLowerCase())
      );

      if (options?.fhirVersion) {
        results = results.filter((pkg) => pkg.fhirVersion === options.fhirVersion);
      }

      if (options?.sortBy === 'downloads') {
        results.sort((a, b) => (b.downloadCount || 0) - (a.downloadCount || 0));
      } else if (options?.sortBy === 'date') {
        results.sort((a, b) => {
          const dateA = a.publishedDate ? new Date(a.publishedDate).getTime() : 0;
          const dateB = b.publishedDate ? new Date(b.publishedDate).getTime() : 0;
          return dateB - dateA;
        });
      }

      return results;
    },

    async install(packageId: string): Promise<Package> {
      await simulateDelay(1000, 2000); // Installation takes longer
      const pkg = fixtures.mockPackages.find((p) => p.id === packageId);
      if (!pkg) {
        throw new Error(`Package ${packageId} not found`);
      }
      pkg.installed = true;
      return pkg;
    },

    async installVersion(packageId: string, version: string): Promise<Package> {
      await simulateDelay(1000, 2000);
      const pkg = fixtures.mockPackages.find((p) => p.id === packageId);
      if (!pkg) {
        throw new Error(`Package ${packageId} not found`);
      }
      pkg.installed = true;
      pkg.version = version;
      return pkg;
    },

    async uninstall(packageId: string): Promise<void> {
      await simulateDelay(500, 1000);
      const pkg = fixtures.mockPackages.find((p) => p.id === packageId);
      if (!pkg) {
        throw new Error(`Package ${packageId} not found`);
      }
      pkg.installed = false;
    },

    async update(packageId: string): Promise<Package> {
      await simulateDelay(1500, 3000);
      const pkg = fixtures.mockPackages.find((p) => p.id === packageId);
      if (!pkg) {
        throw new Error(`Package ${packageId} not found`);
      }
      if (pkg.latestVersion) {
        pkg.version = pkg.latestVersion;
        pkg.hasUpdate = false;
      }
      return pkg;
    },

    async getResources(
      packageId: string,
      options?: { type?: string; query?: string }
    ): Promise<import('@shared/types').PackageResource[]> {
      await simulateDelay(200, 400);
      let resources = fixtures.mockPackageResources[packageId] || [];

      if (options?.type) {
        resources = resources.filter((r) => r.resourceType === options.type);
      }

      if (options?.query) {
        const lowerQuery = options.query.toLowerCase();
        resources = resources.filter(
          (r) =>
            r.name.toLowerCase().includes(lowerQuery) ||
            r.title?.toLowerCase().includes(lowerQuery) ||
            r.description?.toLowerCase().includes(lowerQuery)
        );
      }

      return resources;
    },

    async getInstalledPackages(): Promise<Package[]> {
      await simulateDelay(100, 200);
      return fixtures.mockPackages.filter((pkg) => pkg.installed);
    },
  },

  search: {
    async resources(query: string, filters?: SearchFilters): Promise<SearchResult[]> {
      await simulateDelay(150, 350);
      return fixtures.mockSearchResults.resources
        .filter((r) => matchesQuery(r, query))
        .filter((r) => matchesFilters(r, filters));
    },

    async extensions(
      query: string,
      filters?: { package?: string[] }
    ): Promise<import('@shared/types').Extension[]> {
      await simulateDelay(150, 350);
      const lowerQuery = query.toLowerCase();
      return fixtures.mockExtensions.filter((ext) => {
        const matchesText =
          !query ||
          ext.name?.toLowerCase().includes(lowerQuery) ||
          ext.title?.toLowerCase().includes(lowerQuery) ||
          ext.description?.toLowerCase().includes(lowerQuery);

        const matchesPackage =
          !filters?.package ||
          filters.package.length === 0 ||
          filters.package.includes(ext.package || '');

        return matchesText && matchesPackage;
      });
    },

    async valueSets(
      query: string,
      options?: { codeSystem?: string[] }
    ): Promise<import('@shared/types').ValueSet[]> {
      await simulateDelay(150, 350);
      const lowerQuery = query.toLowerCase();
      return fixtures.mockValueSets.filter((vs) => {
        const matchesText =
          !query ||
          vs.name?.toLowerCase().includes(lowerQuery) ||
          vs.title?.toLowerCase().includes(lowerQuery) ||
          vs.description?.toLowerCase().includes(lowerQuery);

        const matchesCodeSystem =
          !options?.codeSystem ||
          options.codeSystem.length === 0 ||
          vs.compose?.include.some((inc) => options.codeSystem?.includes(inc.system));

        return matchesText && matchesCodeSystem;
      });
    },
  },

  terminology: {
    async expand(valueSetUrl: string): Promise<import('@shared/types').ValueSetExpansion> {
      await simulateDelay(300, 800);

      // Return cached expansion if available
      if (fixtures.mockValueSetExpansions[valueSetUrl]) {
        return fixtures.mockValueSetExpansions[valueSetUrl];
      }

      // If not found, return empty expansion
      throw new Error(`ValueSet ${valueSetUrl} not found or cannot be expanded`);
    },
  },

  validation: {
    async validate(profileId: string): Promise<ValidationResult> {
      await simulateDelay(500, 1000); // Validation takes longer
      if (simulateError(0.05)) {
        throw new Error('Validation service unavailable');
      }
      return fixtures.mockValidationResults[profileId] || fixtures.defaultValidationResult;
    },
  },

  export: {
    async toSD(profileId: string): Promise<ExportResult> {
      await simulateDelay(300, 600);
      return {
        format: 'json',
        content: fixtures.mockSDExport[profileId] || '{}',
        filename: `${profileId}.json`,
      };
    },

    async toFSH(profileId: string): Promise<ExportResult> {
      await simulateDelay(300, 600);
      return {
        format: 'fsh',
        content: fixtures.mockFSHExport[profileId] || '',
        filename: `${profileId}.fsh`,
      };
    },
  },

  undo: {
    async canUndo(profileId: string): Promise<boolean> {
      await simulateDelay(50, 100);
      return (fixtures.mockUndoStack[profileId]?.length ?? 0) > 0;
    },

    async canRedo(profileId: string): Promise<boolean> {
      await simulateDelay(50, 100);
      return (fixtures.mockRedoStack[profileId]?.length ?? 0) > 0;
    },

    async undo(profileId: string): Promise<Profile> {
      await simulateDelay(100, 200);
      // Implement undo logic with mock stacks
      return fixtures.mockProfilesById[profileId];
    },

    async redo(profileId: string): Promise<Profile> {
      await simulateDelay(100, 200);
      // Implement redo logic with mock stacks
      return fixtures.mockProfilesById[profileId];
    },
  },
};
