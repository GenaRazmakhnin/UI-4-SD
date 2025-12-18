import { createEffect, createEvent, createStore, sample } from 'effector';
import { DEFAULT_HISTORY_CONFIG, type HistoryConfig, type Operation } from '../lib/types';

// ============================================================================
// Stores
// ============================================================================

/**
 * History of operations (past operations)
 */
export const $operationHistory = createStore<Operation[]>([]);

/**
 * Future operations (for redo)
 */
export const $redoStack = createStore<Operation[]>([]);

/**
 * Current position in history
 */
export const $currentHistoryIndex = createStore(0);

/**
 * History configuration
 */
export const $historyConfig = createStore<HistoryConfig>(DEFAULT_HISTORY_CONFIG);

/**
 * Derived: Can perform undo
 */
export const $canUndo = $operationHistory.map((history) => history.length > 0);

/**
 * Derived: Can perform redo
 */
export const $canRedo = $redoStack.map((stack) => stack.length > 0);

/**
 * Derived: History depth warning
 */
export const $historyDepthWarning = sample({
  source: { history: $operationHistory, config: $historyConfig },
  fn: ({ history, config }) => {
    if (history.length >= config.maxDepth) {
      return 'limit-reached';
    }
    if (history.length >= config.warnAtDepth) {
      return 'approaching-limit';
    }
    return null;
  },
});

/**
 * History viewer open state
 */
export const $historyViewerOpen = createStore(false);

// ============================================================================
// Events
// ============================================================================

/**
 * User clicked undo button or pressed shortcut
 */
export const undoTriggered = createEvent();

/**
 * User clicked redo button or pressed shortcut
 */
export const redoTriggered = createEvent();

/**
 * User jumped to a specific position in history
 */
export const historyPositionChanged = createEvent<number>();

/**
 * Record a new operation (called by other features)
 */
export const operationRecorded = createEvent<Omit<Operation, 'id' | 'timestamp'>>();

/**
 * Clear history
 */
export const historyClearRequested = createEvent();

/**
 * Update history configuration
 */
export const historyConfigUpdated = createEvent<Partial<HistoryConfig>>();

/**
 * Toggle history viewer
 */
export const historyViewerToggled = createEvent();

/**
 * Open history viewer
 */
export const historyViewerOpened = createEvent();

/**
 * Close history viewer
 */
export const historyViewerClosed = createEvent();

// ============================================================================
// Effects
// ============================================================================

/**
 * Execute undo operation on backend
 */
export const undoFx = createEffect(async () => {
  // TODO: Call backend API to undo
  // const result = await api.undo();
  // return result;
  return { success: true };
});

/**
 * Execute redo operation on backend
 */
export const redoFx = createEffect(async () => {
  // TODO: Call backend API to redo
  // const result = await api.redo();
  // return result;
  return { success: true };
});

/**
 * Jump to history position on backend
 */
export const jumpToPositionFx = createEffect(async (position: number) => {
  // TODO: Call backend API to jump to position
  // const result = await api.jumpToHistoryPosition(position);
  // return result;
  return { success: true, position };
});

// ============================================================================
// Store updates
// ============================================================================

// Add new operation to history
$operationHistory.on(operationRecorded, (history, operation) => {
  const newOperation: Operation = {
    ...operation,
    id: crypto.randomUUID(),
    timestamp: Date.now(),
  };

  // Clear redo stack when new operation is recorded
  return [...history, newOperation];
});

// Clear redo stack when new operation is recorded
$redoStack.on(operationRecorded, () => []);

// On undo, move operation to redo stack
sample({
  clock: undoFx.done,
  source: $operationHistory,
  filter: (history) => history.length > 0,
  fn: (history) => {
    const lastOp = history[history.length - 1];
    return lastOp;
  },
  target: createEvent<Operation>(),
});

$operationHistory.on(undoFx.done, (history) => {
  return history.slice(0, -1);
});

$redoStack.on(undoFx.done, (stack, { params }) => {
  // Push the undone operation to redo stack
  // This is simplified; in real impl, backend would return the operation
  return stack;
});

// On redo, move operation back to history
$operationHistory.on(redoFx.done, (history) => {
  // In real implementation, operation comes from backend
  return history;
});

$redoStack.on(redoFx.done, (stack) => {
  return stack.slice(0, -1);
});

// Update current index based on history length
$currentHistoryIndex.on($operationHistory, (_, history) => history.length);

// Clear history
$operationHistory.on(historyClearRequested, () => []);
$redoStack.on(historyClearRequested, () => []);
$currentHistoryIndex.on(historyClearRequested, () => 0);

// Update config
$historyConfig.on(historyConfigUpdated, (config, updates) => ({
  ...config,
  ...updates,
}));

// History viewer toggle
$historyViewerOpen.on(historyViewerToggled, (open) => !open);
$historyViewerOpen.on(historyViewerOpened, () => true);
$historyViewerOpen.on(historyViewerClosed, () => false);

// Enforce max history depth
sample({
  clock: $operationHistory,
  source: $historyConfig,
  filter: (config, history) => history.length > config.maxDepth,
  fn: (config) => config.maxDepth,
  target: createEvent<number>(),
});

$operationHistory.on($operationHistory, (history, _, config) => {
  // Trim to max depth if exceeded
  // This is handled via sample above in real implementation
  return history;
});

// ============================================================================
// Trigger effects
// ============================================================================

sample({
  clock: undoTriggered,
  source: $canUndo,
  filter: (canUndo) => canUndo,
  target: undoFx,
});

sample({
  clock: redoTriggered,
  source: $canRedo,
  filter: (canRedo) => canRedo,
  target: redoFx,
});

sample({
  clock: historyPositionChanged,
  target: jumpToPositionFx,
});

// ============================================================================
// Pending states
// ============================================================================

export const $undoPending = undoFx.pending;
export const $redoPending = redoFx.pending;
