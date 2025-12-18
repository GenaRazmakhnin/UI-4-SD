import { useUnit } from 'effector-react';
import { useEffect } from 'react';
import { $canRedo, $canUndo, redoTriggered, undoTriggered } from '../model';

/**
 * Hook to register keyboard shortcuts for undo/redo
 *
 * - Ctrl+Z / Cmd+Z: Undo
 * - Ctrl+Shift+Z / Cmd+Shift+Z: Redo
 * - Ctrl+Y / Cmd+Y: Redo (alternative)
 */
export function useUndoRedoShortcuts() {
  const [canUndo, canRedo] = useUnit([$canUndo, $canRedo]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Check for modifier key (Ctrl on Windows/Linux, Cmd on Mac)
      const isModifierPressed = event.ctrlKey || event.metaKey;

      if (!isModifierPressed) {
        return;
      }

      // Don't trigger when typing in input fields
      const target = event.target as HTMLElement;
      if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
        return;
      }

      // Ctrl+Z / Cmd+Z - Undo
      if (event.key === 'z' && !event.shiftKey) {
        event.preventDefault();
        if (canUndo) {
          undoTriggered();
        }
        return;
      }

      // Ctrl+Shift+Z / Cmd+Shift+Z - Redo
      if (event.key === 'z' && event.shiftKey) {
        event.preventDefault();
        if (canRedo) {
          redoTriggered();
        }
        return;
      }

      // Ctrl+Y / Cmd+Y - Redo (alternative)
      if (event.key === 'y') {
        event.preventDefault();
        if (canRedo) {
          redoTriggered();
        }
        return;
      }
    };

    document.addEventListener('keydown', handleKeyDown);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [canUndo, canRedo]);
}
