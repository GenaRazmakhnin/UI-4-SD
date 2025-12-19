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
  /** Package identifier (name@version) */
  id: string;
  name: string;
  version: string;
  description?: string;
  fhirVersion: string;
  publisher?: string;
  /** Whether the package is already installed */
  installed: boolean;
  /** Installed version if different from this version */
  installedVersion?: string;
  downloadCount?: number;
}

export type PackageInstallStatus =
  | 'idle'
  | 'installing'
  | 'extracting'
  | 'indexing'
  | 'installed'
  | 'error';

export interface PackageInstallProgress {
  packageId: string;
  status: PackageInstallStatus;
  progress: number;
  message?: string;
  error?: string;
  downloadedBytes?: number;
  totalBytes?: number;
}

// SSE Event Types from Backend
export type InstallEventType =
  | 'start'
  | 'progress'
  | 'extracting'
  | 'indexing'
  | 'complete'
  | 'error';

export interface InstallEventStart {
  type: 'start';
  package_id: string;
  total_bytes?: number;
}

export interface InstallEventProgress {
  type: 'progress';
  package_id: string;
  downloaded_bytes: number;
  total_bytes?: number;
  percentage: number;
}

export interface InstallEventExtracting {
  type: 'extracting';
  package_id: string;
}

export interface InstallEventIndexing {
  type: 'indexing';
  package_id: string;
}

export interface InstallEventComplete {
  type: 'complete';
  package: Package;
}

export interface InstallEventError {
  type: 'error';
  package_id: string;
  message: string;
  code: string;
}

export type InstallEvent =
  | InstallEventStart
  | InstallEventProgress
  | InstallEventExtracting
  | InstallEventIndexing
  | InstallEventComplete
  | InstallEventError;

// Polling-based install job types
export type InstallJobStatus =
  | 'pending'
  | 'downloading'
  | 'extracting'
  | 'indexing'
  | 'completed'
  | 'failed';

export interface InstallJob {
  jobId: string;
  packageId: string;
  status: InstallJobStatus;
  progress: number;
  message?: string;
  downloadedBytes?: number;
  totalBytes?: number;
  error?: string;
  package?: Package;
  createdAt: string;
  updatedAt: string;
}

// Search response with facets (matches backend)
export interface SearchResponseWithFacets<T> {
  results: T[];
  total_count: number;
  facets?: FacetsDto;
}

export interface FacetsDto {
  resource_types: Record<string, number>;
  packages: Record<string, number>;
}
