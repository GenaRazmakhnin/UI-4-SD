import { notifications } from '@mantine/notifications';
import { IconAlertTriangle } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { createElement, useEffect, useRef } from 'react';
import { $historyConfig, $historyDepthWarning, $operationHistory } from '../model';

/**
 * Hook to show warning notifications when history is approaching or at limit
 */
export function useHistoryLimitWarning() {
  const [warning, historyLength, config] = useUnit([
    $historyDepthWarning,
    $operationHistory.map((h) => h.length),
    $historyConfig,
  ]);

  const lastWarning = useRef<string | null>(null);

  useEffect(() => {
    // Only show warning once per state change
    if (warning === lastWarning.current) {
      return;
    }
    lastWarning.current = warning;

    if (warning === 'approaching-limit') {
      notifications.show({
        title: 'History Limit Warning',
        message: `History is approaching the limit (${historyLength}/${config.maxDepth}). Oldest operations will be removed.`,
        icon: createElement(IconAlertTriangle, { size: 16 }),
        color: 'yellow',
        autoClose: 5000,
      });
    }

    if (warning === 'limit-reached') {
      notifications.show({
        title: 'History Limit Reached',
        message: `History limit reached (${config.maxDepth}). Oldest operations are being removed.`,
        icon: createElement(IconAlertTriangle, { size: 16 }),
        color: 'orange',
        autoClose: 5000,
      });
    }
  }, [warning, historyLength, config.maxDepth]);
}
