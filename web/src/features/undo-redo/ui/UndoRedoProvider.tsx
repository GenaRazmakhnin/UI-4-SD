import { useHistoryLimitWarning, useUndoRedoNotifications, useUndoRedoShortcuts } from '../lib';
import { HistoryViewer } from './HistoryViewer';

interface UndoRedoProviderProps {
  children: React.ReactNode;
}

/**
 * Provider component that sets up undo/redo functionality
 * - Registers keyboard shortcuts
 * - Shows notifications on undo/redo
 * - Shows history limit warnings
 * - Renders the HistoryViewer drawer
 */
export function UndoRedoProvider({ children }: UndoRedoProviderProps) {
  // Register keyboard shortcuts
  useUndoRedoShortcuts();

  // Show notifications on undo/redo
  useUndoRedoNotifications();

  // Show warnings when approaching history limit
  useHistoryLimitWarning();

  return (
    <>
      {children}
      <HistoryViewer />
    </>
  );
}
