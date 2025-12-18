# Task: Mock Data Layer for Parallel Development

## ✅ Implementation Status: COMPLETED

**Date Completed**: 2025-12-18

### Summary of Implementation

All requirements have been successfully implemented:

- ✅ **R1**: Mock API Implementation - Complete mock API with all endpoints (profiles, packages, search, validation, export, undo/redo)
- ✅ **R2**: Mock Data Fixtures - Realistic FHIR profiles including US Core Patient, Observation with slicing, and large profile (500+ elements)
- ✅ **R3**: Shared TypeScript Types - Complete type definitions for Profile, Package, Validation, Search, and Export
- ✅ **R4**: Development Mode Toggle - Environment-based API switching with `.env` configuration
- ✅ **R5**: Mock State Persistence - localStorage persistence for profiles and undo/redo stacks
- ✅ **R6**: TanStack Query Integration - Query hooks for profiles, packages, and validation
- ✅ **R7**: Mock Error Simulation - Realistic error scenarios with configurable probability
- ✅ **R8**: Documentation - Comprehensive README with usage examples and troubleshooting

### Files Created

**Type Definitions** (`web/src/shared/types/`):
- `profile.ts` - Profile, ElementNode, and related types
- `package.ts` - Package and dependency types
- `validation.ts` - Validation result and message types
- `search.ts` - Search result and filter types
- `export.ts` - Export result types
- `index.ts` - Barrel export for all types

**Mock API** (`web/src/shared/api/mock/`):
- `index.ts` - Complete mock API implementation
- `fixtures.ts` - Mock data (profiles, packages, search results, validation)
- `utils.ts` - Delay simulation, error simulation, logging
- `errors.ts` - Error classes and error simulator
- `persistence.ts` - localStorage persistence utilities
- `README.md` - Comprehensive documentation

**Real API** (`web/src/shared/api/real/`):
- `index.ts` - Real API stub (placeholder for backend integration)

**Configuration**:
- Updated `web/src/shared/config/env.ts` - Added mock API toggle and feature flags
- Updated `web/src/shared/api/index.ts` - API facade with mock/real switching
- Updated `web/.env.development` - Mock API enabled by default
- Updated `web/.env.example` - Documentation for environment variables

**TanStack Query Hooks**:
- `web/src/entities/profile/api/queries.ts` - Profile query hooks
- `web/src/entities/package/api/queries.ts` - Package query hooks
- `web/src/features/validate-profile/api/queries.ts` - Validation hooks

### Key Decisions

1. **Mock API Toggle**: Implemented via `VITE_USE_MOCK_API` environment variable for easy switching
2. **Data Persistence**: Using localStorage to maintain state across page refreshes
3. **Error Simulation**: 5% random error rate for network errors, 2% for rate limiting
4. **Network Delays**: Realistic delays (100-2000ms) based on operation type
5. **Type Safety**: All types are strictly typed with TypeScript for compile-time safety
6. **FSD Architecture**: Followed Feature-Sliced Design for query hooks organization

### Usage

To use the mock API:
```bash
# In .env.development
VITE_USE_MOCK_API=true
```

To switch to real backend:
```bash
VITE_USE_MOCK_API=false
VITE_API_BASE_URL=http://localhost:8080
```

### Next Steps

The mock data layer is ready for use. UI components can now be developed independently using:
```typescript
import { useProfiles, useProfile } from '@entities/profile/api/queries';
import { usePackages } from '@entities/package/api/queries';
```

---

## 📋 Description

Create a comprehensive mock data layer that enables UI development to proceed independently of backend implementation. This includes mock API implementations, realistic fixtures, shared TypeScript types, and development mode toggles that allow seamless switching between mock and real backends.

**Reference**: IMPLEMENTATION_PLAN.md Section 20 "Parallel Development Strategy"

## 🎯 Context from Implementation Plan

This implements the parallel development strategy with:
- **Mock Data Layer** (20.1): Complete mock API matching backend contracts
- **Shared Types** (20.2): TypeScript types shared between frontend and backend teams
- **Development Workflow** (20.3): Environment-based API toggling for local development
- **Testing Infrastructure** (20.4): Mock data for testing without backend dependencies
- **Technology Stack** (15.2): TanStack Query for data fetching with mock provider

## 📐 Requirements

### R1: Mock API Implementation

**Complete Mock API Layer**:
```typescript
// web/src/shared/api/mock/index.ts
import { Profile, Package, SearchResult, ValidationResult, ExportResult } from '@shared/types';
import * as fixtures from './fixtures';
import { simulateDelay, simulateError } from './utils';

export const mockApi = {
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
      if (!profile) {
        throw new Error(`Profile ${id} not found`);
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
      const index = fixtures.mockProfiles.findIndex(p => p.id === id);
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
  },

  packages: {
    async list(): Promise<Package[]> {
      await simulateDelay(200, 400);
      return fixtures.mockPackages;
    },

    async search(query: string): Promise<Package[]> {
      await simulateDelay(100, 300);
      return fixtures.mockPackages.filter(pkg =>
        pkg.name.toLowerCase().includes(query.toLowerCase()) ||
        pkg.description?.toLowerCase().includes(query.toLowerCase())
      );
    },

    async install(packageId: string): Promise<Package> {
      await simulateDelay(1000, 2000); // Installation takes longer
      const pkg = fixtures.mockPackages.find(p => p.id === packageId);
      if (!pkg) {
        throw new Error(`Package ${packageId} not found`);
      }
      pkg.installed = true;
      return pkg;
    },

    async uninstall(packageId: string): Promise<void> {
      await simulateDelay(500, 1000);
      const pkg = fixtures.mockPackages.find(p => p.id === packageId);
      if (!pkg) {
        throw new Error(`Package ${packageId} not found`);
      }
      pkg.installed = false;
    },
  },

  search: {
    async resources(query: string, filters?: SearchFilters): Promise<SearchResult[]> {
      await simulateDelay(150, 350);
      return fixtures.mockSearchResults.resources
        .filter(r => matchesQuery(r, query))
        .filter(r => matchesFilters(r, filters));
    },

    async extensions(query: string): Promise<SearchResult[]> {
      await simulateDelay(150, 350);
      return fixtures.mockSearchResults.extensions
        .filter(e => matchesQuery(e, query));
    },

    async valueSets(query: string): Promise<SearchResult[]> {
      await simulateDelay(150, 350);
      return fixtures.mockSearchResults.valueSets
        .filter(vs => matchesQuery(vs, query));
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
      return fixtures.mockUndoStack[profileId]?.length > 0;
    },

    async canRedo(profileId: string): Promise<boolean> {
      await simulateDelay(50, 100);
      return fixtures.mockRedoStack[profileId]?.length > 0;
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

function matchesQuery(item: any, query: string): boolean {
  const lowerQuery = query.toLowerCase();
  return (
    item.name?.toLowerCase().includes(lowerQuery) ||
    item.title?.toLowerCase().includes(lowerQuery) ||
    item.description?.toLowerCase().includes(lowerQuery)
  );
}

function matchesFilters(item: any, filters?: SearchFilters): boolean {
  if (!filters) return true;
  // Implement filter matching logic
  return true;
}
```

**Mock Utilities**:
```typescript
// web/src/shared/api/mock/utils.ts

/**
 * Simulates network delay with random jitter
 */
export async function simulateDelay(minMs: number, maxMs: number): Promise<void> {
  const delay = Math.floor(Math.random() * (maxMs - minMs) + minMs);
  await new Promise(resolve => setTimeout(resolve, delay));
}

/**
 * Simulates random errors for testing error handling
 */
export function simulateError(probability: number): boolean {
  return Math.random() < probability;
}

/**
 * Logs mock API calls in development mode
 */
export function logMockCall(method: string, endpoint: string, data?: any): void {
  if (import.meta.env.DEV) {
    console.log(`[Mock API] ${method} ${endpoint}`, data || '');
  }
}
```

### R2: Mock Data Fixtures

**Complete Mock Fixtures**:
```typescript
// web/src/shared/api/mock/fixtures.ts
import { Profile, ElementNode, Package, ValidationResult } from '@shared/types';

// Mock Profile: US Core Patient (Simple)
export const usCore Patient: Profile = {
  id: 'us-core-patient',
  url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient',
  name: 'USCorePatientProfile',
  title: 'US Core Patient Profile',
  status: 'active',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
  derivation: 'constraint',
  description: 'Defines constraints and extensions on the Patient resource for the minimal set of data to query and retrieve patient demographic information.',
  elements: [
    {
      id: 'Patient',
      path: 'Patient',
      min: 0,
      max: '*',
      isModified: false,
      children: [
        {
          id: 'Patient.identifier',
          path: 'Patient.identifier',
          min: 1,
          max: '*',
          short: 'An identifier for this patient',
          definition: 'An identifier for this patient.',
          mustSupport: true,
          isModified: true,
          children: [],
        },
        {
          id: 'Patient.name',
          path: 'Patient.name',
          min: 1,
          max: '*',
          short: 'A name associated with the patient',
          mustSupport: true,
          isModified: true,
          children: [
            {
              id: 'Patient.name.family',
              path: 'Patient.name.family',
              min: 1,
              max: '1',
              short: 'Family name',
              mustSupport: true,
              isModified: true,
              children: [],
            },
            {
              id: 'Patient.name.given',
              path: 'Patient.name.given',
              min: 1,
              max: '*',
              short: 'Given names',
              mustSupport: true,
              isModified: true,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.gender',
          path: 'Patient.gender',
          min: 1,
          max: '1',
          short: 'male | female | other | unknown',
          mustSupport: true,
          isModified: true,
          binding: {
            strength: 'required',
            valueSet: 'http://hl7.org/fhir/ValueSet/administrative-gender',
          },
          children: [],
        },
      ],
    },
  ],
  isDirty: false,
};

// Mock Profile: With Slicing (Observation)
export const observationWithSlicing: Profile = {
  id: 'observation-with-slicing',
  url: 'http://example.org/StructureDefinition/observation-with-slicing',
  name: 'ObservationWithSlicing',
  title: 'Observation with Component Slicing',
  status: 'draft',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Observation',
  derivation: 'constraint',
  elements: [
    {
      id: 'Observation',
      path: 'Observation',
      min: 0,
      max: '*',
      isModified: false,
      children: [
        {
          id: 'Observation.component',
          path: 'Observation.component',
          min: 2,
          max: '*',
          short: 'Component observations (sliced)',
          slicing: {
            discriminator: [{ type: 'pattern', path: 'code' }],
            rules: 'open',
            ordered: false,
          },
          isModified: true,
          children: [
            {
              id: 'Observation.component:systolic',
              path: 'Observation.component',
              sliceName: 'systolic',
              min: 1,
              max: '1',
              short: 'Systolic blood pressure',
              isModified: true,
              children: [],
            },
            {
              id: 'Observation.component:diastolic',
              path: 'Observation.component',
              sliceName: 'diastolic',
              min: 1,
              max: '1',
              short: 'Diastolic blood pressure',
              isModified: true,
              children: [],
            },
          ],
        },
      ],
    },
  ],
  isDirty: false,
};

// Mock Profile: Large Profile (500+ elements)
export const largeProfile: Profile = {
  id: 'large-profile',
  url: 'http://example.org/StructureDefinition/large-profile',
  name: 'LargeProfile',
  title: 'Large Profile for Performance Testing',
  status: 'draft',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
  derivation: 'constraint',
  elements: generateLargeElementTree(500),
  isDirty: false,
};

// Generate large element tree for performance testing
function generateLargeElementTree(count: number): ElementNode[] {
  const root: ElementNode = {
    id: 'Patient',
    path: 'Patient',
    min: 0,
    max: '*',
    isModified: false,
    children: [],
  };

  let elementCount = 1;
  const paths = ['identifier', 'name', 'telecom', 'address', 'contact', 'communication'];

  while (elementCount < count) {
    paths.forEach(path => {
      if (elementCount >= count) return;
      root.children.push({
        id: `Patient.${path}[${elementCount}]`,
        path: `Patient.${path}`,
        min: 0,
        max: '*',
        isModified: Math.random() > 0.7,
        children: [],
      });
      elementCount++;
    });
  }

  return [root];
}

// Mock Packages
export const mockPackages: Package[] = [
  {
    id: 'hl7.fhir.r4.core',
    name: 'hl7.fhir.r4.core',
    version: '4.0.1',
    description: 'FHIR R4 Core Package',
    fhirVersion: '4.0.1',
    installed: true,
    size: '45.2 MB',
  },
  {
    id: 'hl7.fhir.us.core',
    name: 'hl7.fhir.us.core',
    version: '6.1.0',
    description: 'US Core Implementation Guide',
    fhirVersion: '4.0.1',
    installed: true,
    size: '12.8 MB',
  },
  {
    id: 'hl7.fhir.uv.ipa',
    name: 'hl7.fhir.uv.ipa',
    version: '1.0.0',
    description: 'International Patient Access',
    fhirVersion: '4.0.1',
    installed: false,
    size: '3.4 MB',
  },
];

// Mock Validation Results
export const mockValidationResults: Record<string, ValidationResult> = {
  'us-core-patient': {
    isValid: true,
    errors: [],
    warnings: [],
    info: [
      {
        severity: 'info',
        message: 'Profile is valid and ready for use',
        path: '',
      },
    ],
  },
  'observation-with-slicing': {
    isValid: false,
    errors: [
      {
        severity: 'error',
        message: 'Cardinality constraint violation: min (2) cannot be greater than max (1)',
        path: 'Observation.component',
        line: 42,
      },
    ],
    warnings: [
      {
        severity: 'warning',
        message: 'Slicing discriminator may be ambiguous',
        path: 'Observation.component',
        line: 40,
      },
    ],
    info: [],
  },
};

export const defaultValidationResult: ValidationResult = {
  isValid: true,
  errors: [],
  warnings: [],
  info: [],
};

// Mock Search Results
export const mockSearchResults = {
  resources: [
    {
      id: 'patient',
      url: 'http://hl7.org/fhir/StructureDefinition/Patient',
      name: 'Patient',
      title: 'Patient Resource',
      description: 'Demographics and other administrative information about an individual',
      type: 'resource',
    },
    {
      id: 'observation',
      url: 'http://hl7.org/fhir/StructureDefinition/Observation',
      name: 'Observation',
      title: 'Observation Resource',
      description: 'Measurements and simple assertions',
      type: 'resource',
    },
  ],
  extensions: [
    {
      id: 'patient-birthPlace',
      url: 'http://hl7.org/fhir/StructureDefinition/patient-birthPlace',
      name: 'birthPlace',
      title: 'Birth Place',
      description: 'The registered place of birth of the patient',
      type: 'extension',
    },
  ],
  valueSets: [
    {
      id: 'administrative-gender',
      url: 'http://hl7.org/fhir/ValueSet/administrative-gender',
      name: 'AdministrativeGender',
      title: 'Administrative Gender',
      description: 'The gender of a person used for administrative purposes',
      type: 'valueset',
    },
  ],
};

// Export mock profiles as array and by ID
export const mockProfiles: Profile[] = [
  usCorePatient,
  observationWithSlicing,
  largeProfile,
];

export const mockProfilesById: Record<string, Profile> = {
  [usCorePatient.id]: usCorePatient,
  [observationWithSlicing.id]: observationWithSlicing,
  [largeProfile.id]: largeProfile,
};

// Mock SD/FSH Export
export const mockSDExport: Record<string, string> = {
  'us-core-patient': JSON.stringify(usCorePatient, null, 2),
};

export const mockFSHExport: Record<string, string> = {
  'us-core-patient': `
Profile: USCorePatientProfile
Parent: Patient
Id: us-core-patient
Title: "US Core Patient Profile"
Description: "Defines constraints and extensions on the Patient resource..."
* identifier 1..* MS
* name 1..* MS
* name.family 1..1 MS
* name.given 1..* MS
* gender 1..1 MS
`.trim(),
};

// Mock undo/redo stacks
export const mockUndoStack: Record<string, any[]> = {};
export const mockRedoStack: Record<string, any[]> = {};

// Factory function for creating mock profiles
export function createMockProfile(overrides?: Partial<Profile>): Profile {
  return {
    id: `profile-${Date.now()}`,
    url: `http://example.org/StructureDefinition/profile-${Date.now()}`,
    name: `CustomProfile${Date.now()}`,
    title: 'Custom Profile',
    status: 'draft',
    fhirVersion: '4.0.1',
    baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
    derivation: 'constraint',
    elements: [],
    isDirty: false,
    ...overrides,
  };
}
```

### R3: Shared TypeScript Types

**Complete Type Definitions**:
```typescript
// web/src/shared/types/profile.ts
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
```

**Additional Types**:
```typescript
// web/src/shared/types/package.ts
export interface Package {
  id: string;
  name: string;
  version: string;
  description?: string;
  fhirVersion: string;
  installed: boolean;
  size: string;
  dependencies?: PackageDependency[];
}

export interface PackageDependency {
  name: string;
  version: string;
}

// web/src/shared/types/validation.ts
export type ValidationSeverity = 'error' | 'warning' | 'info';

export interface ValidationMessage {
  severity: ValidationSeverity;
  message: string;
  path: string;
  line?: number;
  column?: number;
}

export interface ValidationResult {
  isValid: boolean;
  errors: ValidationMessage[];
  warnings: ValidationMessage[];
  info: ValidationMessage[];
}

// web/src/shared/types/search.ts
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

// web/src/shared/types/export.ts
export interface ExportResult {
  format: 'json' | 'xml' | 'fsh';
  content: string;
  filename: string;
}
```

### R4: Development Mode Toggle

**Environment Configuration**:
```typescript
// web/src/shared/config/env.ts
export const config = {
  // API configuration
  USE_MOCK_API: import.meta.env.VITE_USE_MOCK_API === 'true',
  API_BASE_URL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000',

  // Development flags
  IS_DEV: import.meta.env.DEV,
  IS_PROD: import.meta.env.PROD,

  // Feature flags
  ENABLE_UNDO_REDO: import.meta.env.VITE_ENABLE_UNDO_REDO !== 'false',
  ENABLE_FSH_EXPORT: import.meta.env.VITE_ENABLE_FSH_EXPORT !== 'false',

  // Performance
  VIRTUALIZATION_THRESHOLD: Number(import.meta.env.VITE_VIRTUALIZATION_THRESHOLD) || 100,
  DEBOUNCE_MS: Number(import.meta.env.VITE_DEBOUNCE_MS) || 300,
} as const;
```

**API Provider with Toggle**:
```typescript
// web/src/shared/api/index.ts
import { config } from '@shared/config/env';
import { mockApi } from './mock';
import { realApi } from './real';

export const api = config.USE_MOCK_API ? mockApi : realApi;

// Export types for use in components
export type Api = typeof api;

// Log which API is being used
if (config.IS_DEV) {
  console.log(`[API] Using ${config.USE_MOCK_API ? 'MOCK' : 'REAL'} API`);
}
```

**Environment Variables** (`.env.development`):
```bash
# Use mock API for local development
VITE_USE_MOCK_API=true

# Backend API URL (when not using mocks)
VITE_API_BASE_URL=http://localhost:3000

# Feature flags
VITE_ENABLE_UNDO_REDO=true
VITE_ENABLE_FSH_EXPORT=true

# Performance tuning
VITE_VIRTUALIZATION_THRESHOLD=100
VITE_DEBOUNCE_MS=300
```

### R5: Mock State Persistence

**LocalStorage Persistence**:
```typescript
// web/src/shared/api/mock/persistence.ts
import { Profile } from '@shared/types';

const STORAGE_KEY_PREFIX = 'mock-api';

export const persistence = {
  /**
   * Save profiles to localStorage
   */
  saveProfiles(profiles: Profile[]): void {
    try {
      localStorage.setItem(
        `${STORAGE_KEY_PREFIX}:profiles`,
        JSON.stringify(profiles)
      );
    } catch (error) {
      console.error('[Mock Persistence] Failed to save profiles:', error);
    }
  },

  /**
   * Load profiles from localStorage
   */
  loadProfiles(): Profile[] | null {
    try {
      const data = localStorage.getItem(`${STORAGE_KEY_PREFIX}:profiles`);
      return data ? JSON.parse(data) : null;
    } catch (error) {
      console.error('[Mock Persistence] Failed to load profiles:', error);
      return null;
    }
  },

  /**
   * Clear all mock data
   */
  clear(): void {
    const keys = Object.keys(localStorage).filter(key =>
      key.startsWith(STORAGE_KEY_PREFIX)
    );
    keys.forEach(key => localStorage.removeItem(key));
  },

  /**
   * Save undo/redo stacks
   */
  saveUndoStack(profileId: string, stack: any[]): void {
    try {
      localStorage.setItem(
        `${STORAGE_KEY_PREFIX}:undo:${profileId}`,
        JSON.stringify(stack)
      );
    } catch (error) {
      console.error('[Mock Persistence] Failed to save undo stack:', error);
    }
  },

  loadUndoStack(profileId: string): any[] {
    try {
      const data = localStorage.getItem(`${STORAGE_KEY_PREFIX}:undo:${profileId}`);
      return data ? JSON.parse(data) : [];
    } catch (error) {
      console.error('[Mock Persistence] Failed to load undo stack:', error);
      return [];
    }
  },
};

// Auto-save on window unload
if (typeof window !== 'undefined') {
  window.addEventListener('beforeunload', () => {
    // Auto-save will be triggered by individual stores
  });
}
```

### R6: TanStack Query Integration

**Query Hooks with Mock Support**:
```typescript
// web/src/entities/profile/api/queries.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@shared/api';
import type { Profile } from '@shared/types';

/**
 * Query keys for profile data
 */
export const profileKeys = {
  all: ['profiles'] as const,
  lists: () => [...profileKeys.all, 'list'] as const,
  list: (filters: string) => [...profileKeys.lists(), { filters }] as const,
  details: () => [...profileKeys.all, 'detail'] as const,
  detail: (id: string) => [...profileKeys.details(), id] as const,
};

/**
 * Fetch all profiles
 */
export function useProfiles() {
  return useQuery({
    queryKey: profileKeys.lists(),
    queryFn: () => api.profiles.list(),
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

/**
 * Fetch single profile by ID
 */
export function useProfile(id: string) {
  return useQuery({
    queryKey: profileKeys.detail(id),
    queryFn: () => api.profiles.get(id),
    enabled: !!id,
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Create new profile
 */
export function useCreateProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: Partial<Profile>) => api.profiles.create(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: profileKeys.lists() });
    },
  });
}

/**
 * Update profile
 */
export function useUpdateProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<Profile> }) =>
      api.profiles.update(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: profileKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: profileKeys.lists() });
    },
  });
}

/**
 * Delete profile
 */
export function useDeleteProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => api.profiles.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: profileKeys.lists() });
    },
  });
}

/**
 * Update element in profile
 */
export function useUpdateElement() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      profileId,
      elementPath,
      updates,
    }: {
      profileId: string;
      elementPath: string;
      updates: Partial<ElementNode>;
    }) => api.profiles.updateElement(profileId, elementPath, updates),
    onSuccess: (_, { profileId }) => {
      queryClient.invalidateQueries({ queryKey: profileKeys.detail(profileId) });
    },
  });
}
```

### R7: Mock Error Simulation

**Realistic Error Scenarios**:
```typescript
// web/src/shared/api/mock/errors.ts

export class MockApiError extends Error {
  constructor(
    message: string,
    public statusCode: number,
    public details?: any
  ) {
    super(message);
    this.name = 'MockApiError';
  }
}

/**
 * Simulates various error scenarios
 */
export const errorSimulator = {
  /**
   * Random network error (5% chance)
   */
  networkError(): void {
    if (Math.random() < 0.05) {
      throw new MockApiError('Network error', 500);
    }
  },

  /**
   * Rate limiting error
   */
  rateLimitError(): void {
    if (Math.random() < 0.02) {
      throw new MockApiError('Rate limit exceeded', 429, {
        retryAfter: 60,
      });
    }
  },

  /**
   * Validation error
   */
  validationError(field: string, message: string): MockApiError {
    return new MockApiError('Validation failed', 422, {
      field,
      message,
    });
  },

  /**
   * Not found error
   */
  notFoundError(resource: string, id: string): MockApiError {
    return new MockApiError(`${resource} not found`, 404, {
      resource,
      id,
    });
  },

  /**
   * Unauthorized error
   */
  unauthorizedError(): MockApiError {
    return new MockApiError('Unauthorized', 401);
  },
};
```

### R8: Documentation and Developer Tools

**Mock API Documentation**:
```typescript
// web/src/shared/api/mock/README.md
/**
 * # Mock API Layer
 *
 * ## Overview
 * The mock API layer enables UI development without backend dependencies.
 *
 * ## Usage
 *
 * ### Enable Mock API
 * Set in `.env.development`:
 * ```
 * VITE_USE_MOCK_API=true
 * ```
 *
 * ### Switch to Real API
 * Set in `.env.development`:
 * ```
 * VITE_USE_MOCK_API=false
 * VITE_API_BASE_URL=http://localhost:3000
 * ```
 *
 * ## Mock Data
 *
 * ### Available Profiles
 * - `us-core-patient`: Simple US Core Patient profile
 * - `observation-with-slicing`: Observation with component slicing
 * - `large-profile`: 500+ elements for performance testing
 *
 * ### Customizing Mock Data
 * Edit `web/src/shared/api/mock/fixtures.ts`
 *
 * ### Adding New Mock Profiles
 * ```typescript
 * import { createMockProfile } from '@shared/api/mock/fixtures';
 *
 * const myProfile = createMockProfile({
 *   name: 'MyCustomProfile',
 *   title: 'My Custom Profile',
 *   // ... other overrides
 * });
 * ```
 *
 * ## Simulated Behaviors
 *
 * - **Delays**: 100-600ms random latency
 * - **Errors**: 5% random failure rate
 * - **Persistence**: Saves to localStorage
 * - **Validation**: Mock validation results
 *
 * ## Testing Error States
 *
 * The mock API randomly simulates errors. To test specific error scenarios:
 * ```typescript
 * import { errorSimulator } from '@shared/api/mock/errors';
 *
 * throw errorSimulator.validationError('name', 'Name is required');
 * ```
 */
```

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] Mock API implements all backend endpoints (profiles, packages, search, validation, export, undo/redo)
- [ ] Mock data includes 3+ realistic profiles (simple, sliced, large)
- [ ] Mock data includes all FHIR packages (us-core, uv-ipa, r4-core)
- [ ] Mock data includes validation results (valid, errors, warnings)
- [ ] Mock data includes search results (resources, extensions, valueSets)
- [ ] Environment toggle switches between mock and real API
- [ ] TypeScript types are shared and consistent
- [ ] Mock factories generate valid data on demand
- [ ] CRUD operations work correctly (create, read, update, delete)
- [ ] Element updates modify profile tree correctly

### Performance Requirements
- [ ] Mock API responses complete in 100-600ms (simulated latency)
- [ ] Large profile (500+ elements) loads in <1s
- [ ] localStorage persistence completes in <50ms
- [ ] No memory leaks from mock data
- [ ] Mock data size stays under 10MB in localStorage

### Development Experience
- [ ] `.env` toggle works without code changes
- [ ] Mock API logs calls in development mode
- [ ] Clear error messages when switching APIs
- [ ] Mock data persists across page refreshes
- [ ] Easy to reset mock data (clear localStorage)
- [ ] Documentation explains mock API usage
- [ ] Examples show how to add custom mock data

### Testing Requirements
- [ ] Mock API works with TanStack Query hooks
- [ ] Mock API works with Effector stores
- [ ] Mock delays are configurable
- [ ] Mock error rate is configurable
- [ ] Mock data can be seeded for tests
- [ ] Mock state can be reset between tests
- [ ] Storybook stories work with mock data

### Integration Requirements
- [ ] Real API has same interface as mock API
- [ ] Switching from mock to real requires no code changes
- [ ] Type definitions match backend OpenAPI spec
- [ ] Mock validation results match real validation format
- [ ] Mock export format matches real export format

## 🔗 Dependencies

### Required Tasks
- **UI 01**: React App Scaffold - Vite configuration and project structure
- **UI 02**: App Initialization & Routing - TanStack Query provider setup

### Integration Points
- **All UI Tasks**: All components depend on mock API during development
- **Backend API**: Mock API must match backend contract exactly
- **Type System**: Shared types ensure frontend/backend compatibility

## 📚 API Contract

**Profile List**:
```typescript
GET /api/profiles
Response: Profile[]
```

**Profile Get**:
```typescript
GET /api/profiles/:id
Response: Profile
```

**Profile Create**:
```typescript
POST /api/profiles
Body: Partial<Profile>
Response: Profile
```

**Profile Update**:
```typescript
PUT /api/profiles/:id
Body: Partial<Profile>
Response: Profile
```

**Profile Delete**:
```typescript
DELETE /api/profiles/:id
Response: void
```

**Element Update**:
```typescript
PATCH /api/profiles/:id/elements/:path
Body: Partial<ElementNode>
Response: Profile
```

## 🧪 Testing Examples

**Mock API Test**:
```typescript
// web/src/shared/api/mock/__tests__/mock-api.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { mockApi } from '../index';
import * as fixtures from '../fixtures';

describe('Mock API', () => {
  beforeEach(() => {
    // Reset fixtures to initial state
    fixtures.mockProfiles.length = 0;
    fixtures.mockProfiles.push(
      fixtures.usCorePatient,
      fixtures.observationWithSlicing
    );
  });

  describe('profiles.list', () => {
    it('returns all profiles', async () => {
      const profiles = await mockApi.profiles.list();
      expect(profiles).toHaveLength(2);
      expect(profiles[0].id).toBe('us-core-patient');
    });

    it('simulates delay', async () => {
      const start = Date.now();
      await mockApi.profiles.list();
      const duration = Date.now() - start;
      expect(duration).toBeGreaterThan(100);
      expect(duration).toBeLessThan(1000);
    });
  });

  describe('profiles.get', () => {
    it('returns profile by ID', async () => {
      const profile = await mockApi.profiles.get('us-core-patient');
      expect(profile.name).toBe('USCorePatientProfile');
    });

    it('throws error for unknown profile', async () => {
      await expect(mockApi.profiles.get('unknown')).rejects.toThrow(
        'Profile unknown not found'
      );
    });
  });

  describe('profiles.update', () => {
    it('updates profile', async () => {
      const updated = await mockApi.profiles.update('us-core-patient', {
        title: 'Updated Title',
      });
      expect(updated.title).toBe('Updated Title');
      expect(updated.isDirty).toBe(true);
    });
  });

  describe('profiles.updateElement', () => {
    it('updates element in tree', async () => {
      const updated = await mockApi.profiles.updateElement(
        'us-core-patient',
        'Patient.name',
        { min: 2 }
      );

      const element = findElement(updated.elements, 'Patient.name');
      expect(element?.min).toBe(2);
      expect(element?.isModified).toBe(true);
    });
  });
});

function findElement(elements: ElementNode[], path: string): ElementNode | null {
  for (const el of elements) {
    if (el.path === path) return el;
    const found = findElement(el.children, path);
    if (found) return found;
  }
  return null;
}
```

**TanStack Query Integration Test**:
```typescript
// web/src/entities/profile/api/__tests__/queries.test.tsx
import { describe, it, expect } from 'vitest';
import { renderHook, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useProfiles, useProfile, useUpdateProfile } from '../queries';

const queryClient = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

const wrapper = ({ children }) => (
  <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
);

describe('Profile Queries', () => {
  it('useProfiles fetches profiles', async () => {
    const { result } = renderHook(() => useProfiles(), { wrapper });

    await waitFor(() => expect(result.current.isSuccess).toBe(true));

    expect(result.current.data).toBeDefined();
    expect(result.current.data!.length).toBeGreaterThan(0);
  });

  it('useProfile fetches single profile', async () => {
    const { result } = renderHook(() => useProfile('us-core-patient'), {
      wrapper,
    });

    await waitFor(() => expect(result.current.isSuccess).toBe(true));

    expect(result.current.data?.name).toBe('USCorePatientProfile');
  });

  it('useUpdateProfile mutates profile', async () => {
    const { result } = renderHook(() => useUpdateProfile(), { wrapper });

    result.current.mutate({
      id: 'us-core-patient',
      data: { title: 'New Title' },
    });

    await waitFor(() => expect(result.current.isSuccess).toBe(true));

    expect(result.current.data?.title).toBe('New Title');
  });
});
```

## 📖 Related Documentation

- **IMPLEMENTATION_PLAN.md Section 20**: Parallel Development Strategy
- **IMPLEMENTATION_PLAN.md Section 15.2**: TanStack Query for data fetching
- **IMPLEMENTATION_PLAN.md Section 17**: UI State Model (Effector)
- **Backend API Specification**: Will define real API contract
- **TanStack Query Docs**: https://tanstack.com/query/latest
- **Effector Docs**: https://effector.dev/

## 🎨 Priority

🔴 **Critical** - Enables parallel development of UI and backend

## ⏱️ Estimated Complexity

**Medium** - 1 week (40 hours)

### Breakdown:
- Mock API implementation: 8 hours
- Mock data fixtures: 12 hours
- Type definitions: 4 hours
- TanStack Query integration: 8 hours
- Testing & documentation: 8 hours
