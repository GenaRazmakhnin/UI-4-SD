/**
 * Extension context types that define where an extension can be used
 */
export type ExtensionContextType = 'fhirpath' | 'element' | 'extension';

/**
 * Extension context definition
 * Defines where an extension is allowed to be used
 */
export interface ExtensionContext {
  type: ExtensionContextType;
  expression: string;
}

/**
 * Extension definition structure
 * Represents a FHIR StructureDefinition of type Extension
 */
export interface Extension {
  id: string;
  url: string;
  name: string;
  title: string;
  status: 'draft' | 'active' | 'retired';
  description?: string;
  version?: string;
  publisher?: string;
  package?: string;

  /**
   * Context rules defining where this extension can be used
   */
  context: ExtensionContext[];

  /**
   * Cardinality of the extension
   */
  min?: number;
  max?: string;

  /**
   * Value type(s) for the extension
   * For simple extensions, this defines what type of value can be stored
   */
  valueTypes?: string[];

  /**
   * Whether this is a complex extension (has sub-extensions)
   */
  isComplex: boolean;

  /**
   * Date when the extension was last updated
   */
  date?: string;

  /**
   * Experimental flag
   */
  experimental?: boolean;

  /**
   * FHIR version
   */
  fhirVersion?: string;
}

/**
 * Extension context validation result
 */
export interface ExtensionContextValidation {
  isValid: boolean;
  isWarning: boolean;
  message?: string;
}

/**
 * Extension usage tracking for recent/favorites
 */
export interface ExtensionUsage {
  extensionUrl: string;
  lastUsed: string;
  useCount: number;
  isFavorite: boolean;
}
