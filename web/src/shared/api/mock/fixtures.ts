import type {
  Profile,
  ElementNode,
  Package,
  ValidationResult,
} from '@shared/types';

// Mock Profile: US Core Patient (Simple)
export const usCorePatient: Profile = {
  id: 'us-core-patient',
  url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient',
  name: 'USCorePatientProfile',
  title: 'US Core Patient Profile',
  status: 'active',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
  derivation: 'constraint',
  description:
    'Defines constraints and extensions on the Patient resource for the minimal set of data to query and retrieve patient demographic information.',
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
  const paths = [
    'identifier',
    'name',
    'telecom',
    'address',
    'contact',
    'communication',
  ];

  while (elementCount < count) {
    for (const path of paths) {
      if (elementCount >= count) return [root];
      root.children.push({
        id: `Patient.${path}[${elementCount}]`,
        path: `Patient.${path}`,
        min: 0,
        max: '*',
        isModified: Math.random() > 0.7,
        children: [],
      });
      elementCount++;
    }
  }

  return [root];
}

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
        message:
          'Cardinality constraint violation: min (2) cannot be greater than max (1)',
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
      description:
        'Demographics and other administrative information about an individual',
      type: 'resource' as const,
    },
    {
      id: 'observation',
      url: 'http://hl7.org/fhir/StructureDefinition/Observation',
      name: 'Observation',
      title: 'Observation Resource',
      description: 'Measurements and simple assertions',
      type: 'resource' as const,
    },
  ],
  extensions: [
    {
      id: 'patient-birthPlace',
      url: 'http://hl7.org/fhir/StructureDefinition/patient-birthPlace',
      name: 'birthPlace',
      title: 'Birth Place',
      description: 'The registered place of birth of the patient',
      type: 'extension' as const,
    },
  ],
  valueSets: [
    {
      id: 'administrative-gender',
      url: 'http://hl7.org/fhir/ValueSet/administrative-gender',
      name: 'AdministrativeGender',
      title: 'Administrative Gender',
      description: 'The gender of a person used for administrative purposes',
      type: 'valueset' as const,
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
export const mockUndoStack: Record<string, unknown[]> = {};
export const mockRedoStack: Record<string, unknown[]> = {};

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
