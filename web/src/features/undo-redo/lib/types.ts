/**
 * Types for the undo/redo system
 */

export interface Operation {
  id: string;
  type: OperationType;
  description: string;
  timestamp: number;
  elementPath?: string;
  details?: Record<string, unknown>;
}

export type OperationType =
  | 'cardinality'
  | 'binding'
  | 'flags'
  | 'type-constraint'
  | 'slice'
  | 'extension'
  | 'general';

export interface HistoryConfig {
  maxDepth: number;
  warnAtDepth: number;
  persistOnSave: boolean;
}

export const DEFAULT_HISTORY_CONFIG: HistoryConfig = {
  maxDepth: 100,
  warnAtDepth: 80,
  persistOnSave: true,
};
