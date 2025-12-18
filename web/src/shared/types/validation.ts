export type ValidationSeverity = 'error' | 'warning' | 'info';

export interface ValidationMessage {
  severity: ValidationSeverity;
  message: string;
  path: string;
  line?: number;
  column?: number;
}

export interface ValidationResult {
  isValid: boolean;
  errors: ValidationMessage[];
  warnings: ValidationMessage[];
  info: ValidationMessage[];
}
