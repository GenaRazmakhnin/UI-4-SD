export interface SearchResult {
  id: string;
  url: string;
  name: string;
  title: string;
  description?: string;
  type: 'resource' | 'extension' | 'valueset' | 'profile';
  package?: string;
}

export interface SearchFilters {
  type?: string[];
  package?: string[];
  fhirVersion?: string[];
}

/** Element search result */
export interface ElementSearchResult {
  /** Element path (e.g., "Patient.name") */
  path: string;
  /** Short description */
  short?: string;
  /** Full definition */
  definition?: string;
  /** Element types */
  types: string[];
  /** Minimum cardinality */
  min?: number;
  /** Maximum cardinality */
  max?: string;
}

/** Base FHIR resource type for profile creation. */
export interface BaseResource {
  /** Resource type name (e.g., "Patient", "Observation") */
  name: string;
  /** Canonical URL for the resource definition */
  url: string;
  /** Human-readable title */
  title?: string;
  /** Description of the resource */
  description?: string;
  /** Source package name */
  packageName: string;
  /** Source package version */
  packageVersion: string;
}
