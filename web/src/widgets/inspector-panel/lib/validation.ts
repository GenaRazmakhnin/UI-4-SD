import type { ElementNode, ValidationMessage } from '@shared/types';

/**
 * Get validation messages for an element
 */
export function getElementValidation(
  element: ElementNode,
  allMessages: ValidationMessage[],
): ValidationMessage[] {
  return allMessages.filter(
    (msg) =>
      msg.path === element.path || msg.path.startsWith(`${element.path}.`),
  );
}

/**
 * Group validation messages by severity
 */
export function groupMessagesBySeverity(messages: ValidationMessage[]) {
  return {
    errors: messages.filter((m) => m.severity === 'error'),
    warnings: messages.filter((m) => m.severity === 'warning'),
    info: messages.filter((m) => m.severity === 'info'),
  };
}

/**
 * Get highest severity for an element
 */
export function getHighestSeverity(
  messages: ValidationMessage[],
): 'error' | 'warning' | 'info' | null {
  if (messages.some((m) => m.severity === 'error')) return 'error';
  if (messages.some((m) => m.severity === 'warning')) return 'warning';
  if (messages.some((m) => m.severity === 'info')) return 'info';
  return null;
}
