import type { PackageDependency } from './package';
import type { FhirVersion } from './profile';

export type ProjectStatus = 'draft' | 'review' | 'published' | 'archived';

export interface Project {
  id: string;
  name: string;
  fhirVersion: FhirVersion;
  canonicalBase: string;
  status: ProjectStatus;
  version: string;
  description?: string;
  publisher?: string;
  createdAt: string;
  modifiedAt: string;
  dependencies: PackageDependency[];
  // UI-specific fields (not from backend API)
  lastOpenedAt?: string;
  templateId?: string;
  packageId?: string;
  path?: string;
}

export interface CreateProjectInput {
  id: string;
  name: string;
  canonicalBase: string;
  fhirVersion?: FhirVersion;
  description?: string;
  publisher?: string;
  dependencies?: PackageDependency[];
}

export interface UpdateProjectInput {
  name?: string;
  description?: string;
  publisher?: string;
  version?: string;
  status?: ProjectStatus;
  dependencies?: PackageDependency[];
}
