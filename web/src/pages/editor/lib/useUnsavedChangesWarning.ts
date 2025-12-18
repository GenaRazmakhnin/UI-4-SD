import { useBlocker } from '@tanstack/react-router';
import { useUnit } from 'effector-react';
import { useEffect } from 'react';
import { $hasUnsavedChanges } from '../model';

/**
 * Hook to warn users when navigating away with unsaved changes.
 * Uses both browser beforeunload and TanStack Router blocker.
 */
export function useUnsavedChangesWarning() {
  const hasUnsavedChanges = useUnit($hasUnsavedChanges);

  // Browser beforeunload event (for page reload/close)
  useEffect(() => {
    const handleBeforeUnload = (e: BeforeUnloadEvent) => {
      if (hasUnsavedChanges) {
        e.preventDefault();
        // Chrome requires returnValue to be set
        e.returnValue = '';
        return '';
      }
    };

    window.addEventListener('beforeunload', handleBeforeUnload);
    return () => window.removeEventListener('beforeunload', handleBeforeUnload);
  }, [hasUnsavedChanges]);

  // TanStack Router blocker (for in-app navigation)
  useBlocker({
    condition: hasUnsavedChanges,
    withResolver: true,
  });

  return { hasUnsavedChanges };
}
