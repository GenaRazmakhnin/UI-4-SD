export interface ExportResult {
  format: 'json' | 'xml' | 'fsh';
  content: string;
  filename: string;
}

/** Export metadata from the backend */
export interface ExportMetadata {
  resourceId: string;
  name: string;
  url: string;
  fhirVersion: string;
  filename: string;
  contentType: string;
  etag: string;
  persistedPath?: string;
}

/** SD export response */
export interface SdExportResponse {
  content: unknown;
  metadata: ExportMetadata;
  diagnostics?: ExportDiagnostic[];
}

/** FSH export response */
export interface FshExportResponse {
  content: string;
  metadata: ExportMetadata;
  diagnostics?: ExportDiagnostic[];
}

/** Preview response */
export interface PreviewResponse {
  content: string;
  format: 'sd' | 'fsh';
  highlighting?: SyntaxHighlighting;
  diagnostics?: ExportDiagnostic[];
}

/** Syntax highlighting metadata */
export interface SyntaxHighlighting {
  language: string;
  tokens: HighlightToken[];
}

/** Syntax highlighting token */
export interface HighlightToken {
  line: number;
  startColumn: number;
  endColumn: number;
  tokenType: string;
}

/** Export diagnostic */
export interface ExportDiagnostic {
  severity: 'error' | 'warning' | 'info';
  code: string;
  message: string;
  path?: string;
}

/** Bulk export response */
export interface BulkExportResponse {
  projectId: string;
  files: ExportedFile[];
  summary: ExportSummary;
  diagnostics?: ResourceDiagnostic[];
}

/** Single exported file */
export interface ExportedFile {
  resourceId: string;
  name: string;
  path: string;
  format: string;
  content: string;
  isBase64: boolean;
}

/** Export summary */
export interface ExportSummary {
  totalResources: number;
  successCount: number;
  failedCount: number;
  skippedCount: number;
  formats: string[];
}

/** Resource-level diagnostic */
export interface ResourceDiagnostic {
  resourceId: string;
  name: string;
  diagnostics: ExportDiagnostic[];
}

/** Export options */
export interface SdExportOptions {
  format?: 'differential' | 'snapshot' | 'both';
  pretty?: boolean;
  persist?: boolean;
  force?: boolean;
}

export interface FshExportOptions {
  persist?: boolean;
  force?: boolean;
}

export interface BulkExportOptions {
  format?: 'sd' | 'fsh' | 'both';
  structure?: 'flat' | 'packaged';
  pretty?: boolean;
}

export interface PreviewOptions {
  format?: 'sd' | 'fsh';
  highlight?: boolean;
}

// === Import Types ===

/** Import content format */
export type ImportFormat = 'json' | 'fsh';

/** Request to import a profile from SD or FSH */
export interface ImportProfileRequest {
  /** Content format */
  format: ImportFormat;
  /** The content to import */
  content: string;
  /** Whether to replace existing profile or create new */
  replace?: boolean;
}

/** Import diagnostic from backend */
export interface ImportDiagnostic {
  severity: 'error' | 'warning' | 'info';
  code: string;
  message: string;
  path?: string;
}

/** Import response from backend */
export interface ImportProfileResponse {
  /** Imported profile details */
  profile: {
    id: string;
    name: string;
    url: string;
    status: string;
    baseDefinition: string;
  };
  /** Import diagnostics */
  diagnostics: ImportDiagnostic[];
}
