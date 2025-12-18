import type { SlicingDiscriminator, SlicingRules } from '@shared/types';

export interface SlicingTemplate {
  id: string;
  name: string;
  description: string;
  discriminators: SlicingDiscriminator[];
  rules: SlicingRules['rules'];
  ordered: boolean;
  suggestedSlices?: Array<{
    name: string;
    min: number;
    max: string;
    description?: string;
  }>;
}

/**
 * Common slicing templates for FHIR resources
 */
export const SLICING_TEMPLATES: SlicingTemplate[] = [
  {
    id: 'extension-by-url',
    name: 'Extension by URL',
    description: 'Slice extensions by their URL. Most common extension slicing pattern.',
    discriminators: [
      {
        type: 'value',
        path: 'url',
      },
    ],
    rules: 'open',
    ordered: false,
    suggestedSlices: [
      {
        name: 'myExtension',
        min: 0,
        max: '1',
        description: 'Custom extension slice',
      },
    ],
  },
  {
    id: 'coding-by-system',
    name: 'Coding by System',
    description:
      'Slice codings by their code system. Used for CodeableConcept with multiple coding systems.',
    discriminators: [
      {
        type: 'value',
        path: 'system',
      },
    ],
    rules: 'open',
    ordered: false,
    suggestedSlices: [
      {
        name: 'snomedCoding',
        min: 0,
        max: '1',
        description: 'SNOMED CT coding',
      },
      {
        name: 'loincCoding',
        min: 0,
        max: '1',
        description: 'LOINC coding',
      },
    ],
  },
  {
    id: 'identifier-by-system',
    name: 'Identifier by System',
    description:
      'Slice identifiers by their system. Used to specify which identifier types are allowed.',
    discriminators: [
      {
        type: 'value',
        path: 'system',
      },
    ],
    rules: 'open',
    ordered: false,
    suggestedSlices: [
      {
        name: 'mrn',
        min: 0,
        max: '1',
        description: 'Medical Record Number',
      },
      {
        name: 'ssn',
        min: 0,
        max: '1',
        description: 'Social Security Number',
      },
    ],
  },
  {
    id: 'reference-by-type',
    name: 'Reference by Type',
    description:
      'Slice references by their resource type. Used to constrain which resource types can be referenced.',
    discriminators: [
      {
        type: 'type',
        path: '$this',
      },
    ],
    rules: 'closed',
    ordered: false,
    suggestedSlices: [
      {
        name: 'patientReference',
        min: 0,
        max: '1',
        description: 'Reference to Patient',
      },
      {
        name: 'practitionerReference',
        min: 0,
        max: '1',
        description: 'Reference to Practitioner',
      },
    ],
  },
  {
    id: 'profile-constraint',
    name: 'Profile Constraint',
    description: 'Slice by profile URL to specify different profiles for the same element.',
    discriminators: [
      {
        type: 'profile',
        path: '$this',
      },
    ],
    rules: 'open',
    ordered: false,
    suggestedSlices: [
      {
        name: 'customProfile',
        min: 0,
        max: '*',
        description: 'Custom profile slice',
      },
    ],
  },
  {
    id: 'pattern-match',
    name: 'Pattern Match',
    description: 'Slice by matching a fixed pattern. Used for complex discriminators.',
    discriminators: [
      {
        type: 'pattern',
        path: 'code',
      },
    ],
    rules: 'open',
    ordered: false,
    suggestedSlices: [
      {
        name: 'fixedCode',
        min: 0,
        max: '1',
        description: 'Element with fixed code pattern',
      },
    ],
  },
  {
    id: 'exists-check',
    name: 'Existence Check',
    description: 'Slice based on whether an element exists. Used for optional sub-elements.',
    discriminators: [
      {
        type: 'exists',
        path: 'extension',
      },
    ],
    rules: 'open',
    ordered: false,
    suggestedSlices: [
      {
        name: 'withExtension',
        min: 0,
        max: '*',
        description: 'Element with extension',
      },
      {
        name: 'withoutExtension',
        min: 0,
        max: '*',
        description: 'Element without extension',
      },
    ],
  },
];

/**
 * Get common discriminator paths for an element type
 */
export function getCommonDiscriminatorPaths(elementPath: string): string[] {
  // Extract the base type from the path
  const pathParts = elementPath.split('.');
  const lastPart = pathParts[pathParts.length - 1];

  // Common paths for different element types
  const commonPaths: Record<string, string[]> = {
    extension: ['url', 'value[x]'],
    identifier: ['system', 'use', 'type.coding.system'],
    coding: ['system', 'code'],
    codeableConcept: ['coding.system', 'coding.code', 'text'],
    reference: ['reference', 'type', 'identifier.system'],
    contactPoint: ['system', 'use'],
    address: ['use', 'type'],
    humanName: ['use'],
    quantity: ['system', 'code', 'unit'],
  };

  // Check if path ends with a known type
  for (const [type, paths] of Object.entries(commonPaths)) {
    if (lastPart.toLowerCase().includes(type.toLowerCase())) {
      return paths;
    }
  }

  // Default common paths
  return ['url', 'system', 'code', 'type', 'use'];
}

/**
 * Validate discriminator path
 */
export function validateDiscriminatorPath(
  path: string,
  type: SlicingDiscriminator['type']
): { valid: boolean; error?: string } {
  if (!path || path.trim() === '') {
    return { valid: false, error: 'Path cannot be empty' };
  }

  // Special validation for $this path
  if (path === '$this') {
    if (type !== 'type' && type !== 'profile') {
      return {
        valid: false,
        error: '$this path can only be used with type or profile discriminators',
      };
    }
    return { valid: true };
  }

  // Basic FHIRPath validation
  if (path.includes('..') || path.startsWith('.') || path.endsWith('.')) {
    return { valid: false, error: 'Invalid path format' };
  }

  return { valid: true };
}

/**
 * Get discriminator type description
 */
export function getDiscriminatorTypeDescription(type: SlicingDiscriminator['type']): string {
  const descriptions = {
    value: 'The slices are differentiated by the value of the nominated element',
    exists: 'The slices are differentiated by the presence or absence of the nominated element',
    pattern:
      'The slices are differentiated by conformance of the nominated element to a specified pattern',
    type: 'The slices are differentiated by type of the nominated element',
    profile:
      'The slices are differentiated by conformance of the nominated element to a specified profile',
  };
  return descriptions[type];
}

/**
 * Get slicing rules description
 */
export function getSlicingRulesDescription(rules: SlicingRules['rules']): string {
  const descriptions = {
    closed: 'No additional slices are allowed beyond those defined in this profile',
    open: 'Additional slices are allowed beyond those defined (most common)',
    openAtEnd: 'Additional slices are allowed, but only at the end of the list',
  };
  return descriptions[rules];
}

/**
 * Generate slice name from discriminator
 */
export function generateSliceName(discriminator: SlicingDiscriminator, value?: string): string {
  if (value) {
    // Extract meaningful part from URL or system
    const parts = value.split('/');
    const last = parts[parts.length - 1];
    return last.replace(/[^a-zA-Z0-9]/g, '');
  }

  // Generate based on discriminator type
  return `${discriminator.type}Slice`;
}
