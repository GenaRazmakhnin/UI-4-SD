export type ProfileStatus = 'draft' | 'active' | 'retired';
export type FhirVersion = '4.0.1' | '4.3.0' | '5.0.0';
export type DerivationType = 'constraint' | 'specialization';

export interface Profile {
  id: string;
  url: string;
  name: string;
  title: string;
  status: ProfileStatus;
  fhirVersion: FhirVersion;
  baseDefinition: string;
  derivation: DerivationType;
  description?: string;
  elements: ElementNode[];
  isDirty: boolean;
  publisher?: string;
  contact?: ContactDetail[];
  copyright?: string;
}

export interface ElementNode {
  id: string;
  path: string;
  sliceName?: string;
  min: number;
  max: string;
  type?: TypeConstraint[];
  binding?: BindingConstraint;
  slicing?: SlicingRules;
  mustSupport?: boolean;
  isModifier?: boolean;
  isSummary?: boolean;
  short?: string;
  definition?: string;
  comment?: string;
  isModified: boolean;
  children: ElementNode[];
}

export interface TypeConstraint {
  code: string;
  profile?: string[];
  targetProfile?: string[];
  aggregation?: ('contained' | 'referenced' | 'bundled')[];
}

export interface BindingConstraint {
  strength: 'required' | 'extensible' | 'preferred' | 'example';
  valueSet: string;
  description?: string;
}

export interface SlicingRules {
  discriminator: SlicingDiscriminator[];
  rules: 'open' | 'closed' | 'openAtEnd';
  ordered: boolean;
  description?: string;
}

export interface SlicingDiscriminator {
  type: 'value' | 'exists' | 'pattern' | 'type' | 'profile';
  path: string;
}

export interface ContactDetail {
  name?: string;
  telecom?: ContactPoint[];
}

export interface ContactPoint {
  system: 'phone' | 'fax' | 'email' | 'pager' | 'url' | 'sms' | 'other';
  value: string;
  use?: 'home' | 'work' | 'temp' | 'old' | 'mobile';
}
