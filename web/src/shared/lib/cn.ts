import { type ClassValue, clsx } from 'clsx';

/**
 * Combines multiple class names, supporting conditional classes
 * @example cn('base', condition && 'conditional', styles.module)
 */
export function cn(...inputs: ClassValue[]) {
  return clsx(inputs);
}
