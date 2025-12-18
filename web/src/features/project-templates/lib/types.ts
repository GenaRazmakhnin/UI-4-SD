import type { FhirVersion } from '@shared/types';

/**
 * Project template definition
 */
export interface ProjectTemplate {
  id: string;
  name: string;
  description: string;
  icon: string;
  fhirVersion: FhirVersion;
  dependencies: PackageDependency[];
  structure: TemplateStructure;
  category: TemplateCategory;
  tags: string[];
}

export type TemplateCategory = 'blank' | 'implementation-guide' | 'regional' | 'custom';

/**
 * Package dependency
 */
export interface PackageDependency {
  packageId: string;
  version: string;
  name: string;
  description?: string;
}

/**
 * Template structure preview
 */
export interface TemplateStructure {
  profiles: string[];
  extensions: string[];
  valueSets: string[];
  codeSystems: string[];
}

/**
 * Project configuration form data
 */
export interface ProjectConfig {
  name: string;
  canonicalBase: string;
  fhirVersion: FhirVersion;
  packageId: string;
  version: string;
  description?: string;
  publisher?: string;
  dependencies: PackageDependency[];
  initGit: boolean;
}

/**
 * Validation errors for project config
 */
export interface ProjectConfigErrors {
  name?: string;
  canonicalBase?: string;
  packageId?: string;
  version?: string;
}

/**
 * Recent project entry
 */
export interface RecentProject {
  id: string;
  name: string;
  path: string;
  fhirVersion: FhirVersion;
  lastOpened: number;
  packageId?: string;
}

/**
 * Wizard step
 */
export type WizardStep = 'template' | 'configure' | 'review';

/**
 * Default project config
 */
export const DEFAULT_PROJECT_CONFIG: ProjectConfig = {
  name: '',
  canonicalBase: 'http://example.org/fhir',
  fhirVersion: '4.0.1',
  packageId: '',
  version: '0.1.0',
  description: '',
  publisher: '',
  dependencies: [],
  initGit: true,
};
