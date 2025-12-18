import type {
  ElementNode,
  Extension,
  Package,
  Profile,
  ValidationResult,
  ValueSet,
  ValueSetExpansion,
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
  const paths = ['identifier', 'name', 'telecom', 'address', 'contact', 'communication'];

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

// Mock Extensions
export const mockExtensions: Extension[] = [
  {
    id: 'patient-birthPlace',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-birthPlace',
    name: 'birthPlace',
    title: 'Birth Place',
    status: 'active',
    description:
      "The registered place of birth of the patient. A sytem may use the address.text if they don't store the birthPlace address in discrete elements.",
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['Address'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-birthTime',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-birthTime',
    name: 'birthTime',
    title: 'Birth Time',
    status: 'active',
    description:
      'The time of day that the patient was born. This includes the date to ensure that the timezone information can be communicated effectively.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient.birthDate',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['dateTime'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-mothersMaidenName',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-mothersMaidenName',
    name: 'mothersMaidenName',
    title: 'Mothers Maiden Name',
    status: 'active',
    description:
      "Mother's maiden (unmarried) name, commonly collected to help verify patient identity.",
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['string'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-nationality',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-nationality',
    name: 'nationality',
    title: 'Nationality',
    status: 'active',
    description:
      'The nationality of the patient. This is a complex extension that includes both a code and a period.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '*',
    isComplex: true,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-disability',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-disability',
    name: 'disability',
    title: 'Disability',
    status: 'active',
    description:
      'A code that identifies the disability or disabilities that affect how the person functions in everyday life.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '*',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-religion',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-religion',
    name: 'religion',
    title: 'Religion',
    status: 'active',
    description: "The patient's professed religious affiliations.",
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-cadavericDonor',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-cadavericDonor',
    name: 'cadavericDonor',
    title: 'Cadaveric Donor',
    status: 'active',
    description:
      'Flag indicating whether the patient authorized the donation of body parts after death.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['boolean'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'us-core-race',
    url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-race',
    name: 'race',
    title: 'US Core Race Extension',
    status: 'active',
    description:
      'Concepts classifying the person into a named category of humans sharing common history, traits, geographical origin or nationality. The race codes used to represent these concepts are based upon the CDC Race and Ethnicity Code Set.',
    publisher: 'HL7 US Realm Steering Committee',
    package: 'hl7.fhir.us.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
      {
        type: 'element',
        expression: 'RelatedPerson',
      },
      {
        type: 'element',
        expression: 'Practitioner',
      },
      {
        type: 'element',
        expression: 'Person',
      },
    ],
    min: 0,
    max: '1',
    isComplex: true,
    fhirVersion: '4.0.1',
    date: '2020-07-21T00:00:00+00:00',
    experimental: false,
  },
  {
    id: 'us-core-ethnicity',
    url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity',
    name: 'ethnicity',
    title: 'US Core Ethnicity Extension',
    status: 'active',
    description:
      'Concepts classifying the person into a named category of humans sharing a common real or presumed heritage, history, ancestry, or country of origin. The ethnicity codes used to represent these concepts are based upon the CDC Race and Ethnicity Code Set.',
    publisher: 'HL7 US Realm Steering Committee',
    package: 'hl7.fhir.us.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
      {
        type: 'element',
        expression: 'RelatedPerson',
      },
      {
        type: 'element',
        expression: 'Practitioner',
      },
      {
        type: 'element',
        expression: 'Person',
      },
    ],
    min: 0,
    max: '1',
    isComplex: true,
    fhirVersion: '4.0.1',
    date: '2020-07-21T00:00:00+00:00',
    experimental: false,
  },
  {
    id: 'us-core-birthsex',
    url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex',
    name: 'birthsex',
    title: 'US Core Birth Sex Extension',
    status: 'active',
    description:
      "A code classifying the person's sex assigned at birth as specified by the Office of the National Coordinator for Health IT (ONC).",
    publisher: 'HL7 US Realm Steering Committee',
    package: 'hl7.fhir.us.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['code'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2020-07-21T00:00:00+00:00',
    experimental: false,
  },
  {
    id: 'observation-bodyPosition',
    url: 'http://hl7.org/fhir/StructureDefinition/observation-bodyPosition',
    name: 'bodyPosition',
    title: 'Body Position',
    status: 'active',
    description: 'The position of the body when the observation was made.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Observation',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'observation-delta',
    url: 'http://hl7.org/fhir/StructureDefinition/observation-delta',
    name: 'delta',
    title: 'Observation Delta',
    status: 'active',
    description:
      'The qualitative change in the value relative to the previous measurement. Usually only recorded if the change is clinically significant.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Observation',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'resource-effectivePeriod',
    url: 'http://hl7.org/fhir/StructureDefinition/resource-effectivePeriod',
    name: 'effectivePeriod',
    title: 'Effective Period',
    status: 'active',
    description:
      'The period during which the resource content was or is planned to be in active use. Allows establishing a transition period from one resource to another.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Resource',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['Period'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'humanname-fathers-family',
    url: 'http://hl7.org/fhir/StructureDefinition/humanname-fathers-family',
    name: 'fathersFamily',
    title: "Father's Family Name",
    status: 'active',
    description:
      'Indicates the family name of the father. Useful in cultures where the family name is derived from both parents.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'HumanName.family',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['string'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'humanname-mothers-family',
    url: 'http://hl7.org/fhir/StructureDefinition/humanname-mothers-family',
    name: 'mothersFamily',
    title: "Mother's Family Name",
    status: 'active',
    description:
      'Indicates the family name of the mother. Useful in cultures where the family name is derived from both parents.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'HumanName.family',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['string'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
];

// Mock ValueSets
export const mockValueSets: ValueSet[] = [
  {
    url: 'http://hl7.org/fhir/ValueSet/administrative-gender',
    name: 'AdministrativeGender',
    title: 'Administrative Gender',
    status: 'active',
    description: 'The gender of a person used for administrative purposes',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://hl7.org/fhir/administrative-gender',
        },
      ],
    },
  },
  {
    url: 'http://hl7.org/fhir/ValueSet/marital-status',
    name: 'MaritalStatus',
    title: 'Marital Status Codes',
    status: 'active',
    description:
      'This value set defines the set of codes that can be used to indicate the marital status of a person',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        },
      ],
    },
  },
  {
    url: 'http://hl7.org/fhir/ValueSet/observation-status',
    name: 'ObservationStatus',
    title: 'Observation Status',
    status: 'active',
    description: 'Codes providing the status of an observation',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://hl7.org/fhir/observation-status',
        },
      ],
    },
  },
  {
    url: 'http://hl7.org/fhir/ValueSet/contact-point-system',
    name: 'ContactPointSystem',
    title: 'Contact Point System',
    status: 'active',
    description: 'Telecommunications form for contact point',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://hl7.org/fhir/contact-point-system',
        },
      ],
    },
  },
  {
    url: 'http://loinc.org/vs/LL715-4',
    name: 'LaboratoryTestResults',
    title: 'Laboratory Test Results',
    status: 'active',
    description: 'LOINC codes for laboratory test results',
    publisher: 'Regenstrief Institute',
    compose: {
      include: [
        {
          system: 'http://loinc.org',
        },
      ],
    },
  },
  {
    url: 'http://snomed.info/sct/ValueSet/clinical-findings',
    name: 'ClinicalFindings',
    title: 'SNOMED CT Clinical Findings',
    status: 'active',
    description: 'SNOMED CT Clinical Findings',
    publisher: 'SNOMED International',
    compose: {
      include: [
        {
          system: 'http://snomed.info/sct',
        },
      ],
    },
  },
];

// Mock ValueSet Expansions
export const mockValueSetExpansions: Record<string, ValueSetExpansion> = {
  'http://hl7.org/fhir/ValueSet/administrative-gender': {
    total: 4,
    contains: [
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'male',
        display: 'Male',
      },
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'female',
        display: 'Female',
      },
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'other',
        display: 'Other',
      },
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'unknown',
        display: 'Unknown',
      },
    ],
  },
  'http://hl7.org/fhir/ValueSet/marital-status': {
    total: 8,
    contains: [
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'A',
        display: 'Annulled',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'D',
        display: 'Divorced',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'I',
        display: 'Interlocutory',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'L',
        display: 'Legally Separated',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'M',
        display: 'Married',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'P',
        display: 'Polygamous',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'S',
        display: 'Never Married',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'W',
        display: 'Widowed',
      },
    ],
  },
  'http://hl7.org/fhir/ValueSet/observation-status': {
    total: 7,
    contains: [
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'registered',
        display: 'Registered',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'preliminary',
        display: 'Preliminary',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'final',
        display: 'Final',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'amended',
        display: 'Amended',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'corrected',
        display: 'Corrected',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'cancelled',
        display: 'Cancelled',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'entered-in-error',
        display: 'Entered in Error',
      },
    ],
  },
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
export const mockProfiles: Profile[] = [usCorePatient, observationWithSlicing, largeProfile];

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
