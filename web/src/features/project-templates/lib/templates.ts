import type { ProjectTemplate } from './types';

/**
 * Available project templates
 */
export const PROJECT_TEMPLATES: ProjectTemplate[] = [
  {
    id: 'blank',
    name: 'Blank Project',
    description: 'Start from scratch with an empty project. Perfect for custom implementations.',
    icon: 'IconFile',
    fhirVersion: '4.0.1',
    dependencies: [
      {
        packageId: 'hl7.fhir.r4.core',
        version: '4.0.1',
        name: 'FHIR R4 Core',
        description: 'Base FHIR R4 specification',
      },
    ],
    structure: {
      profiles: [],
      extensions: [],
      valueSets: [],
      codeSystems: [],
    },
    category: 'blank',
    tags: ['starter', 'empty', 'basic'],
  },
  {
    id: 'us-core',
    name: 'US Core Based',
    description:
      'Build on top of US Core Implementation Guide. Includes common US healthcare profiles.',
    icon: 'IconFlag',
    fhirVersion: '4.0.1',
    dependencies: [
      {
        packageId: 'hl7.fhir.r4.core',
        version: '4.0.1',
        name: 'FHIR R4 Core',
        description: 'Base FHIR R4 specification',
      },
      {
        packageId: 'hl7.fhir.us.core',
        version: '6.1.0',
        name: 'US Core',
        description: 'US Core Implementation Guide',
      },
    ],
    structure: {
      profiles: ['USCorePatient', 'USCorePractitioner', 'USCoreOrganization'],
      extensions: ['USCoreRace', 'USCoreEthnicity', 'USCoreBirthSex'],
      valueSets: ['USCoreConditionCode', 'USCoreProcedureCode'],
      codeSystems: [],
    },
    category: 'regional',
    tags: ['us', 'united-states', 'uscdi', 'onc'],
  },
  {
    id: 'ipa',
    name: 'International Patient Access',
    description:
      'Based on IPA Implementation Guide. Designed for international patient data exchange.',
    icon: 'IconWorld',
    fhirVersion: '4.0.1',
    dependencies: [
      {
        packageId: 'hl7.fhir.r4.core',
        version: '4.0.1',
        name: 'FHIR R4 Core',
        description: 'Base FHIR R4 specification',
      },
      {
        packageId: 'hl7.fhir.uv.ipa',
        version: '1.0.0',
        name: 'International Patient Access',
        description: 'IPA Implementation Guide',
      },
    ],
    structure: {
      profiles: ['IPAPatient', 'IPACondition', 'IPAMedicationRequest'],
      extensions: [],
      valueSets: [],
      codeSystems: [],
    },
    category: 'implementation-guide',
    tags: ['international', 'patient-access', 'smart'],
  },
  {
    id: 'mcode',
    name: 'mCODE Oncology',
    description:
      'Minimal Common Oncology Data Elements. For cancer-related healthcare applications.',
    icon: 'IconHeartbeat',
    fhirVersion: '4.0.1',
    dependencies: [
      {
        packageId: 'hl7.fhir.r4.core',
        version: '4.0.1',
        name: 'FHIR R4 Core',
        description: 'Base FHIR R4 specification',
      },
      {
        packageId: 'hl7.fhir.us.core',
        version: '6.1.0',
        name: 'US Core',
        description: 'US Core Implementation Guide',
      },
      {
        packageId: 'hl7.fhir.us.mcode',
        version: '3.0.0',
        name: 'mCODE',
        description: 'Minimal Common Oncology Data Elements',
      },
    ],
    structure: {
      profiles: ['CancerPatient', 'PrimaryCancerCondition', 'TNMStageGroup'],
      extensions: ['HistologyMorphologyBehavior'],
      valueSets: ['CancerStagingSystemVS', 'PrimaryCancerConditionVS'],
      codeSystems: [],
    },
    category: 'implementation-guide',
    tags: ['oncology', 'cancer', 'mcode', 'specialty'],
  },
  {
    id: 'smart-app',
    name: 'SMART on FHIR App',
    description: 'Template for building SMART on FHIR applications with proper authorization.',
    icon: 'IconLock',
    fhirVersion: '4.0.1',
    dependencies: [
      {
        packageId: 'hl7.fhir.r4.core',
        version: '4.0.1',
        name: 'FHIR R4 Core',
        description: 'Base FHIR R4 specification',
      },
      {
        packageId: 'hl7.fhir.uv.smart-app-launch',
        version: '2.1.0',
        name: 'SMART App Launch',
        description: 'SMART App Launch Framework',
      },
    ],
    structure: {
      profiles: [],
      extensions: [],
      valueSets: [],
      codeSystems: [],
    },
    category: 'implementation-guide',
    tags: ['smart', 'oauth', 'app', 'authorization'],
  },
  {
    id: 'fhir-r5',
    name: 'FHIR R5 Project',
    description:
      'Start with the latest FHIR R5 specification. For forward-looking implementations.',
    icon: 'IconSparkles',
    fhirVersion: '5.0.0',
    dependencies: [
      {
        packageId: 'hl7.fhir.r5.core',
        version: '5.0.0',
        name: 'FHIR R5 Core',
        description: 'Base FHIR R5 specification',
      },
    ],
    structure: {
      profiles: [],
      extensions: [],
      valueSets: [],
      codeSystems: [],
    },
    category: 'blank',
    tags: ['r5', 'latest', 'future'],
  },
];

/**
 * Get template by ID
 */
export function getTemplate(id: string): ProjectTemplate | undefined {
  return PROJECT_TEMPLATES.find((t) => t.id === id);
}

/**
 * Get templates by category
 */
export function getTemplatesByCategory(category: ProjectTemplate['category']): ProjectTemplate[] {
  return PROJECT_TEMPLATES.filter((t) => t.category === category);
}

/**
 * Search templates
 */
export function searchTemplates(query: string): ProjectTemplate[] {
  const q = query.toLowerCase();
  return PROJECT_TEMPLATES.filter(
    (t) =>
      t.name.toLowerCase().includes(q) ||
      t.description.toLowerCase().includes(q) ||
      t.tags.some((tag) => tag.includes(q))
  );
}
