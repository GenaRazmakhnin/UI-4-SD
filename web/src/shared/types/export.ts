export interface ExportResult {
  format: 'json' | 'xml' | 'fsh';
  content: string;
  filename: string;
}
