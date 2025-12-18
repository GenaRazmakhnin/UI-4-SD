import type { ElementNode, TypeConstraint } from '@shared/types';
import { isSubtype } from './type-hierarchy';

export interface TypeValidation {
  isValid: boolean;
  errors: string[];
  warnings: string[];
}

/**
 * Validate type constraint changes
 */
export function validateTypeConstraints(
  element: ElementNode,
  newTypes: TypeConstraint[],
): TypeValidation {
  const errors: string[] = [];
  const warnings: string[] = [];

  // Must have at least one type
  if (newTypes.length === 0) {
    errors.push('Element must have at least one allowed type');
  }

  // Check each type against base definition
  const baseTypes = getBaseAllowedTypes(element);
  newTypes.forEach((newType) => {
    if (!isValidTypeRestriction(newType.code, baseTypes)) {
      errors.push(
        `Type "${newType.code}" is not allowed in the base definition`,
      );
    }

    // Validate profile URLs
    if (newType.profile) {
      newType.profile.forEach((profileUrl) => {
        if (!isValidProfileUrl(profileUrl)) {
          errors.push(`Invalid profile URL: ${profileUrl}`);
        }
      });
    }
  });

  // Warnings for common issues
  if (newTypes.length > 1 && newTypes.every((t) => t.profile && t.profile.length > 0)) {
    warnings.push(
      'Multiple types with profiles may make the element harder to implement',
    );
  }

  return {
    isValid: errors.length === 0,
    errors,
    warnings,
  };
}

/**
 * Check if type restriction is valid
 */
function isValidTypeRestriction(
  typeCode: string,
  baseTypes: TypeConstraint[],
): boolean {
  // Check if type code is allowed in base
  const baseTypeCodes = baseTypes.map((t) => t.code);

  if (baseTypeCodes.includes(typeCode)) {
    return true;
  }

  // Check if it's a subtype (e.g., Integer is subtype of decimal)
  return baseTypeCodes.some((baseType) => isSubtype(typeCode, baseType));
}

/**
 * Validate profile URL format
 */
function isValidProfileUrl(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

/**
 * Get base allowed types (would come from base definition in real implementation)
 */
function getBaseAllowedTypes(element: ElementNode): TypeConstraint[] {
  // In real implementation, fetch from base definition
  return element.type || [{ code: 'string' }];
}

/**
 * Get recommended type constraints based on common patterns
 */
export function getRecommendedTypeConstraints(
  element: ElementNode,
): TypeConstraint[] | null {
  const path = element.path.toLowerCase();

  // Recommend specific profiles for common patterns
  if (path.includes('identifier')) {
    return [
      {
        code: 'Identifier',
        profile: ['http://hl7.org/fhir/StructureDefinition/Identifier'],
      },
    ];
  }

  if (path.includes('reference')) {
    return [
      {
        code: 'Reference',
        targetProfile: ['http://hl7.org/fhir/StructureDefinition/Patient'],
      },
    ];
  }

  return null;
}
