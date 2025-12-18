import type { FhirVersion } from './profile';
import type { PackageDependency } from './package';

export interface Project {
  id: string;
  name: string;
  fhirVersion: FhirVersion;
  templateId?: string;
  description?: string;
  packageId?: string;
  canonicalBase?: string;
  version?: string;
  publisher?: string;
  path?: string;
  createdAt?: string;
  updatedAt?: string;
  lastOpenedAt?: string;
  dependencies?: PackageDependency[];
}

export interface CreateProjectInput {
  name: string;
  fhirVersion: FhirVersion;
  templateId?: string;
  description?: string;
  packageId?: string;
  canonicalBase?: string;
  version?: string;
  publisher?: string;
  dependencies?: PackageDependency[];
}

export interface UpdateProjectInput {
  name?: string;
  description?: string;
  canonicalBase?: string;
  packageId?: string;
  version?: string;
  publisher?: string;
  dependencies?: PackageDependency[];
  lastOpenedAt?: string;
}
