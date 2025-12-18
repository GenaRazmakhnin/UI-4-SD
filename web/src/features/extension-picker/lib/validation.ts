import type { ElementNode, Extension, ExtensionContextValidation } from '@shared/types';

/**
 * Validates if an extension can be applied to a given element based on context rules
 */
export function validateExtensionContext(
  extension: Extension,
  element: ElementNode
): ExtensionContextValidation {
  // If no context rules, the extension can be applied anywhere (shouldn't happen in practice)
  if (!extension.context || extension.context.length === 0) {
    return {
      isValid: true,
      isWarning: false,
    };
  }

  // Check each context rule - if any matches, the extension is valid
  for (const context of extension.context) {
    const result = validateSingleContext(context.type, context.expression, element);
    if (result.isValid) {
      return result;
    }
  }

  // None of the context rules matched
  return {
    isValid: false,
    isWarning: false,
    message: `This extension is not allowed on ${element.path}. Valid contexts: ${extension.context
      .map((c) => c.expression)
      .join(', ')}`,
  };
}

/**
 * Validates a single context rule against an element
 */
function validateSingleContext(
  contextType: 'element' | 'fhirpath' | 'extension',
  expression: string,
  element: ElementNode
): ExtensionContextValidation {
  switch (contextType) {
    case 'element':
      return validateElementContext(expression, element);

    case 'fhirpath':
      // FHIRPath validation would require a FHIRPath evaluator
      // For now, treat as a warning that manual verification is needed
      return {
        isValid: true,
        isWarning: true,
        message: `This extension uses FHIRPath context (${expression}). Please verify it matches your use case.`,
      };

    case 'extension':
      return validateExtensionContext_Extension(expression, element);

    default:
      return {
        isValid: true,
        isWarning: true,
        message: `Unknown context type. Please verify this extension is appropriate.`,
      };
  }
}

/**
 * Validates element context - checks if the element path matches the context expression
 */
function validateElementContext(
  expression: string,
  element: ElementNode
): ExtensionContextValidation {
  const elementPath = element.path;

  // Exact match
  if (elementPath === expression) {
    return { isValid: true, isWarning: false };
  }

  // Check if element is a child of the context resource
  // e.g., "Patient.name" matches context "Patient"
  if (elementPath.startsWith(`${expression}.`)) {
    return { isValid: true, isWarning: false };
  }

  // Check if element's resource type matches
  // e.g., "Patient.identifier" has resource type "Patient"
  const resourceType = elementPath.split('.')[0];
  if (resourceType === expression) {
    return { isValid: true, isWarning: false };
  }

  // Check for base resource type "Resource" which matches everything
  if (expression === 'Resource') {
    return { isValid: true, isWarning: false };
  }

  // Check for Element context (matches any element)
  if (expression === 'Element') {
    return { isValid: true, isWarning: false };
  }

  // Check for data type matches
  // e.g., "HumanName.family" context expression "HumanName.family" or "HumanName"
  const pathParts = elementPath.split('.');
  const exprParts = expression.split('.');

  // Check if the last part of the path matches the context
  // This handles data type extensions like "HumanName.family" in "Patient.name.family"
  if (pathParts.length >= exprParts.length) {
    const pathTail = pathParts.slice(-exprParts.length).join('.');
    if (pathTail === expression) {
      return { isValid: true, isWarning: false };
    }
  }

  return {
    isValid: false,
    isWarning: false,
    message: `Element ${elementPath} does not match context ${expression}`,
  };
}

/**
 * Validates extension context - for extensions that can only be applied to other extensions
 */
function validateExtensionContext_Extension(
  expression: string,
  element: ElementNode
): ExtensionContextValidation {
  // Check if current element is an extension element
  const pathParts = element.path.split('.');
  const isExtensionElement = pathParts.some((part) => part === 'extension');

  if (!isExtensionElement) {
    return {
      isValid: false,
      isWarning: false,
      message: `This extension can only be applied to other extensions`,
    };
  }

  // If expression is specified, it would be the URL of the parent extension
  // For now, allow any extension-to-extension context
  return {
    isValid: true,
    isWarning: true,
    message: `This extension extends another extension. Verify the parent extension matches: ${expression}`,
  };
}

/**
 * Gets a human-readable description of where an extension can be used
 */
export function getContextDescription(extension: Extension): string {
  if (!extension.context || extension.context.length === 0) {
    return 'Can be used anywhere';
  }

  if (extension.context.length === 1) {
    const ctx = extension.context[0];
    return `Can be used on: ${ctx.expression}`;
  }

  const contexts = extension.context.map((c) => c.expression);
  if (contexts.length <= 3) {
    return `Can be used on: ${contexts.join(', ')}`;
  }

  return `Can be used on: ${contexts.slice(0, 3).join(', ')} and ${contexts.length - 3} more`;
}

/**
 * Checks if an extension is likely relevant for a given element
 * Returns a relevance score (0-100)
 */
export function calculateExtensionRelevance(extension: Extension, element: ElementNode): number {
  let score = 0;

  // Check context match
  const validation = validateExtensionContext(extension, element);
  if (validation.isValid && !validation.isWarning) {
    score += 50; // Perfect match
  } else if (validation.isValid && validation.isWarning) {
    score += 25; // Possible match
  } else {
    return 0; // Not valid
  }

  // Boost score if extension name suggests it's for this resource type
  const resourceType = element.path.split('.')[0].toLowerCase();
  if (extension.name.toLowerCase().includes(resourceType)) {
    score += 20;
  }

  // Boost score for common/popular extensions (based on package)
  if (extension.package === 'hl7.fhir.r4.core') {
    score += 15;
  } else if (extension.package?.includes('us.core')) {
    score += 10;
  }

  // Boost score for active/published extensions
  if (extension.status === 'active') {
    score += 10;
  }

  // Penalize experimental extensions
  if (extension.experimental) {
    score -= 5;
  }

  return Math.min(100, Math.max(0, score));
}
