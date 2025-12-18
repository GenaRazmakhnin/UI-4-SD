import { useEffect, useRef } from 'react';
import { useUnit } from 'effector-react';
import { $currentProject, $lastProjectId, fetchProjectFx } from '../model';

/**
 * Best-effort restore of the last opened project from localStorage.
 */
export function useRestoreLastProject() {
  const [lastProjectId, currentProject] = useUnit([$lastProjectId, $currentProject]);
  const attemptedRef = useRef(false);

  useEffect(() => {
    if (attemptedRef.current) return;
    if (!currentProject && lastProjectId) {
      fetchProjectFx(lastProjectId);
      attemptedRef.current = true;
    }
  }, [currentProject, lastProjectId]);
}
