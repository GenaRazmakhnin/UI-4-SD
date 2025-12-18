// UI Components

// Types
export type { HistoryConfig, Operation, OperationType } from './lib';
// Hooks
export {
  DEFAULT_HISTORY_CONFIG,
  useHistoryLimitWarning,
  useUndoRedoNotifications,
  useUndoRedoShortcuts,
} from './lib';

// Model (stores, events, effects)
export {
  $canRedo,
  $canUndo,
  $currentHistoryIndex,
  $historyConfig,
  $historyDepthWarning,
  $historyViewerOpen,
  // Stores
  $operationHistory,
  $redoPending,
  $redoStack,
  $undoPending,
  historyClearRequested,
  historyConfigUpdated,
  historyPositionChanged,
  historyViewerClosed,
  historyViewerOpened,
  historyViewerToggled,
  jumpToPositionFx,
  operationRecorded,
  redoFx,
  redoTriggered,
  // Effects
  undoFx,
  // Events
  undoTriggered,
} from './model';
export { HistoryViewer } from './ui/HistoryViewer';
export { UndoRedoProvider } from './ui/UndoRedoProvider';
export { UndoRedoToolbar } from './ui/UndoRedoToolbar';
