import { $selectedElement } from '@widgets/element-tree';
import { useUnit } from 'effector-react';
import { useCallback, useEffect, useRef } from 'react';
import { actionTriggered, shortcutHelpToggled } from '../model';
import { QUICK_ACTIONS } from './actions';

/**
 * Hook to register keyboard shortcuts for quick actions
 *
 * Uses chord shortcuts: Ctrl+K followed by a letter
 * - Ctrl+K, R: Make required
 * - Ctrl+K, O: Make optional
 * - Ctrl+K, M: Allow multiple
 * - Ctrl+K, S: Toggle mustSupport
 * - Ctrl+K, B: Set binding
 * - Ctrl+K, E: Add extension
 * - Ctrl+K, L: Create slice
 * - Ctrl+K, T: Constrain type
 * - Ctrl+K, ?: Show shortcut help
 */
export function useQuickActionShortcuts() {
  const selectedElement = useUnit($selectedElement);
  const chordStarted = useRef(false);
  const chordTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);

  const resetChord = useCallback(() => {
    chordStarted.current = false;
    if (chordTimeout.current) {
      clearTimeout(chordTimeout.current);
      chordTimeout.current = null;
    }
  }, []);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Don't trigger in input fields
      const target = event.target as HTMLElement;
      if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
        return;
      }

      const isModifierPressed = event.ctrlKey || event.metaKey;

      // Start chord with Ctrl+K
      if (isModifierPressed && event.key === 'k') {
        event.preventDefault();
        chordStarted.current = true;

        // Reset chord after 2 seconds if no follow-up
        chordTimeout.current = setTimeout(() => {
          resetChord();
        }, 2000);

        return;
      }

      // Handle chord completion
      if (chordStarted.current) {
        event.preventDefault();
        resetChord();

        // Show help with ?
        if (event.key === '?' || event.key === '/') {
          shortcutHelpToggled();
          return;
        }

        // No element selected
        if (!selectedElement) {
          return;
        }

        // Find action by shortcut
        const key = event.key.toLowerCase();
        const action = QUICK_ACTIONS.find(
          (a) => a.shortcut?.toLowerCase() === key && a.isAvailable(selectedElement)
        );

        if (action) {
          actionTriggered(action.id);
        }
      }
    };

    document.addEventListener('keydown', handleKeyDown);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      resetChord();
    };
  }, [selectedElement, resetChord]);
}
