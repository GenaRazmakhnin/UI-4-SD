export interface CardinalityValidation {
  minError?: string;
  maxError?: string;
  isValid: boolean;
}

/**
 * Validate cardinality constraints
 */
export function validateCardinality(
  min: number,
  max: string,
  baseMin: number,
  baseMax: string,
): CardinalityValidation {
  const errors: CardinalityValidation = { isValid: true };

  // Rule 1: Min must be non-negative
  if (min < 0) {
    errors.minError = 'Minimum must be ≥ 0';
    errors.isValid = false;
  }

  // Rule 2: Min must be ≥ base min (cannot loosen constraint)
  if (min < baseMin) {
    errors.minError = `Minimum must be ≥ ${baseMin} (base minimum)`;
    errors.isValid = false;
  }

  // Rule 3: Max must be valid format
  if (max !== '*' && (isNaN(Number(max)) || !Number.isInteger(Number(max)))) {
    errors.maxError = 'Maximum must be a number or "*"';
    errors.isValid = false;
  }

  // Rule 4: Max must be ≤ base max (cannot loosen constraint)
  if (max !== '*') {
    const maxNum = Number(max);
    const baseMaxNum = baseMax === '*' ? Infinity : Number(baseMax);
    if (maxNum > baseMaxNum) {
      errors.maxError = `Maximum must be ≤ ${baseMax} (base maximum)`;
      errors.isValid = false;
    }
  } else if (baseMax !== '*') {
    errors.maxError = `Cannot set to "*" when base maximum is ${baseMax}`;
    errors.isValid = false;
  }

  // Rule 5: Min must be ≤ max
  if (max !== '*') {
    const maxNum = Number(max);
    if (min > maxNum) {
      errors.minError = 'Minimum cannot exceed maximum';
      errors.isValid = false;
    }
  }

  return errors;
}

/**
 * Get impact message for cardinality change
 */
export function getImpactMessage(
  min: number,
  max: string,
  baseMin: number,
  baseMax: string,
): string | null {
  // Required → Optional
  if (baseMin >= 1 && min === 0) {
    return '⚠️ This will make the element optional (was required)';
  }

  // Optional → Required
  if (baseMin === 0 && min >= 1) {
    return '✅ This will make the element required (was optional)';
  }

  // Single → Multiple
  if (baseMax === '1' && max === '*') {
    return '📋 This will allow multiple values (was single value)';
  }

  // Multiple → Single
  if (baseMax === '*' && max === '1') {
    return '🔒 This will restrict to single value (was multiple)';
  }

  // Tightened range
  if (
    min > baseMin ||
    (max !== '*' && baseMax !== '*' && Number(max) < Number(baseMax))
  ) {
    return `🎯 Constraint tightened from ${baseMin}..${baseMax} to ${min}..${max}`;
  }

  return null;
}

/**
 * Parse max value to number for validation
 */
export function parseMaxToNumber(max: string): number | undefined {
  if (max === '*') return undefined;
  const num = Number(max);
  return isNaN(num) ? undefined : num;
}
