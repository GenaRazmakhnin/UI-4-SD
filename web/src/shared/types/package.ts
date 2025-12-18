export interface Package {
  id: string;
  name: string;
  version: string;
  description?: string;
  fhirVersion: string;
  installed: boolean;
  size: string;
  dependencies?: PackageDependency[];
  // Additional fields for package browser
  publisher?: string;
  license?: string;
  homepage?: string;
  repository?: string;
  canonical?: string;
  downloadCount?: number;
  publishedDate?: string;
  latestVersion?: string;
  hasUpdate?: boolean;
  resourceCounts?: PackageResourceCounts;
  versions?: PackageVersion[];
}

export interface PackageDependency {
  packageId?: string;
  name: string;
  version: string;
  isInstalled?: boolean;
}

export interface PackageResourceCounts {
  profiles: number;
  extensions: number;
  valueSets: number;
  codeSystems: number;
  searchParameters: number;
  operationDefinitions: number;
  capabilityStatements: number;
  total: number;
}

export interface PackageVersion {
  version: string;
  fhirVersion: string;
  publishedDate: string;
  size: string;
}

export interface PackageResource {
  id: string;
  url: string;
  name: string;
  title?: string;
  description?: string;
  resourceType: string;
  status?: string;
  version?: string;
}

export interface PackageSearchResult {
  name: string;
  description?: string;
  fhirVersion: string;
  version: string;
  publisher?: string;
  downloadCount?: number;
}

export type PackageInstallStatus = 'idle' | 'installing' | 'installed' | 'error';

export interface PackageInstallProgress {
  packageId: string;
  status: PackageInstallStatus;
  progress: number;
  message?: string;
  error?: string;
}
