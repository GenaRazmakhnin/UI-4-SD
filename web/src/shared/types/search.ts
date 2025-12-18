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
