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

/**
 * Quick fix action that can be applied to resolve a diagnostic
 */
export interface QuickFix {
  /**
   * Unique identifier for this fix
   */
  id: string;

  /**
   * Display label for the fix action
   */
  label: string;

  /**
   * Description of what the fix will do
   */
  description?: string;

  /**
   * Type of fix action
   */
  kind: 'replace' | 'insert' | 'delete' | 'refactor';

  /**
   * Preview of the changes (optional)
   */
  preview?: string;

  /**
   * Whether this fix is preferred (primary)
   */
  isPreferred?: boolean;
}

/**
 * Diagnostic message with additional metadata for the diagnostics panel
 */
export interface Diagnostic {
  /**
   * Unique identifier for this diagnostic
   */
  id: string;

  /**
   * Severity level
   */
  severity: ValidationSeverity;

  /**
   * Diagnostic code (e.g., 'FHIR-001', 'CARD-002')
   */
  code: string;

  /**
   * Human-readable message
   */
  message: string;

  /**
   * Element path this diagnostic relates to
   */
  elementPath?: string;

  /**
   * Source of the diagnostic
   */
  source?: string;

  /**
   * Available quick fixes for this diagnostic
   */
  quickFixes?: QuickFix[];

  /**
   * Timestamp when the diagnostic was created
   */
  timestamp?: string;

  /**
   * Whether this diagnostic is new (for highlighting)
   */
  isNew?: boolean;

  /**
   * Whether this diagnostic has been fixed
   */
  isFixed?: boolean;

  /**
   * Related diagnostics (for grouped issues)
   */
  relatedDiagnostics?: string[];
}
