/**
 * FHIR Terminology Types
 */

export interface ValueSet {
  url: string;
  name: string;
  title?: string;
  status: 'draft' | 'active' | 'retired' | 'unknown';
  description?: string;
  publisher?: string;
  version?: string;
  compose?: ValueSetCompose;
  expansion?: ValueSetExpansion;
}

export interface ValueSetCompose {
  include: ValueSetInclude[];
  exclude?: ValueSetInclude[];
}

export interface ValueSetInclude {
  system: string;
  version?: string;
  concept?: ValueSetConcept[];
  filter?: ValueSetFilter[];
}

export interface ValueSetConcept {
  code: string;
  display?: string;
  designation?: Designation[];
}

export interface ValueSetFilter {
  property: string;
  op: string;
  value: string;
}

export interface Designation {
  language?: string;
  use?: Coding;
  value: string;
}

export interface Coding {
  system?: string;
  version?: string;
  code?: string;
  display?: string;
}

export interface ValueSetExpansion {
  total?: number;
  offset?: number;
  contains?: ValueSetExpansionContains[];
  error?: string;
}

export interface ValueSetExpansionContains {
  system?: string;
  abstract?: boolean;
  inactive?: boolean;
  version?: string;
  code: string;
  display?: string;
  designation?: Designation[];
  contains?: ValueSetExpansionContains[];
}

export interface CodeSystem {
  url: string;
  name: string;
  title?: string;
  status: 'draft' | 'active' | 'retired' | 'unknown';
  description?: string;
  publisher?: string;
  version?: string;
  content: 'not-present' | 'example' | 'fragment' | 'complete' | 'supplement';
  concept?: CodeSystemConcept[];
}

export interface CodeSystemConcept {
  code: string;
  display?: string;
  definition?: string;
  designation?: Designation[];
  property?: ConceptProperty[];
  concept?: CodeSystemConcept[];
}

export interface ConceptProperty {
  code: string;
  value: string | number | boolean | Coding;
}
