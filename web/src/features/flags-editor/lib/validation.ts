import type { ElementNode } from '@shared/types';

export interface FlagValidation {
  isValid: boolean;
  errors: string[];
  warnings: string[];
}

/**
 * Validate flag changes
 */
export function validateFlags(element: ElementNode, updates: Partial<ElementNode>): FlagValidation {
  const errors: string[] = [];
  const warnings: string[] = [];

  // IsModifier validation
  if (updates.isModifier !== undefined) {
    if (updates.isModifier && !element.isModifier) {
      warnings.push(
        'Adding isModifier flag changes the semantics of the resource. Use with caution.'
      );
    }
  }

  // MustSupport validation
  if (updates.mustSupport !== undefined) {
    if (updates.mustSupport && element.min === 0) {
      warnings.push(
        'Setting MustSupport on optional element (min=0). ' +
          'This means systems must be capable of handling it, but it is not required in every instance.'
      );
    }

    if (updates.mustSupport && !element.isModified) {
      warnings.push(
        'Setting MustSupport without constraining the element is unusual. ' +
          'Consider adding cardinality or type constraints.'
      );
    }
  }

  return {
    isValid: errors.length === 0,
    errors,
    warnings,
  };
}

/**
 * Check if setting MS without constraints is suspicious
 */
export function isSuspiciousMustSupport(element: ElementNode): boolean {
  return element.mustSupport === true && element.min === 0 && !element.isModified;
}

/**
 * Get recommended flags based on element state
 */
export function getRecommendedFlags(element: ElementNode): Partial<ElementNode> {
  const recommendations: Partial<ElementNode> = {};

  // Recommend MS for required elements
  if (element.min >= 1 && !element.mustSupport) {
    recommendations.mustSupport = true;
  }

  return recommendations;
}
