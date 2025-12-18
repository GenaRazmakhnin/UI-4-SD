import type { BindingConstraint } from '@shared/types';

export interface BindingValidation {
  isValid: boolean;
  error?: string;
  warnings: string[];
}

/**
 * Validate binding strength change
 */
export function canChangeBindingStrength(
  baseStrength: BindingConstraint['strength'] | undefined,
  newStrength: BindingConstraint['strength']
): BindingValidation {
  const warnings: string[] = [];

  // If no base binding, any strength is allowed
  if (!baseStrength) {
    return { isValid: true, warnings };
  }

  // Binding strength hierarchy (stronger to weaker)
  const strengthOrder = ['required', 'extensible', 'preferred', 'example'];
  const baseIndex = strengthOrder.indexOf(baseStrength);
  const newIndex = strengthOrder.indexOf(newStrength);

  // Cannot weaken binding strength
  if (newIndex > baseIndex) {
    return {
      isValid: false,
      error: `Cannot weaken binding strength from "${baseStrength}" to "${newStrength}". Profiles can only strengthen bindings.`,
      warnings,
    };
  }

  // Warn if strengthening significantly
  if (newIndex < baseIndex) {
    warnings.push(
      `Strengthening binding from "${baseStrength}" to "${newStrength}". ` +
        `Ensure this aligns with your use case requirements.`
    );
  }

  return { isValid: true, warnings };
}

/**
 * Get description for binding strength
 */
export function getBindingStrengthDescription(strength: BindingConstraint['strength']): string {
  const descriptions = {
    required:
      'REQUIRED: Codes SHALL be from the specified value set. This is the strictest binding.',
    extensible:
      'EXTENSIBLE: Codes SHALL be from the specified value set if applicable. ' +
      'If no suitable code exists, an alternative code may be used.',
    preferred:
      'PREFERRED: Codes SHOULD be from the specified value set for interoperability, ' +
      'but alternative codes are allowed.',
    example:
      'EXAMPLE: Codes MAY be from the specified value set. This is the weakest binding, ' +
      'used for examples only.',
  };

  return descriptions[strength];
}

/**
 * Validate ValueSet URL format
 */
export function isValidValueSetUrl(url: string): boolean {
  if (!url) return false;

  try {
    const parsedUrl = new URL(url);
    // ValueSet URLs should be HTTP(S)
    return parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:';
  } catch {
    return false;
  }
}

/**
 * Get recommended binding strength based on element criticality
 */
export function getRecommendedBindingStrength(element: {
  mustSupport?: boolean;
  min: number;
}): BindingConstraint['strength'] {
  // Must Support required elements should have strong bindings
  if (element.mustSupport && element.min >= 1) {
    return 'required';
  }

  // Must Support optional elements
  if (element.mustSupport) {
    return 'extensible';
  }

  // Optional elements
  return 'preferred';
}
