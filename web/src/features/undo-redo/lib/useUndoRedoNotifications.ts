import { notifications } from '@mantine/notifications';
import { IconArrowBackUp, IconArrowForwardUp, IconCheck, IconX } from '@tabler/icons-react';
import { createElement, useEffect } from 'react';
import { redoFx, undoFx } from '../model';

/**
 * Hook to show toast notifications for undo/redo operations
 */
export function useUndoRedoNotifications() {
  useEffect(() => {
    // Subscribe to undo success
    const unsubUndoSuccess = undoFx.done.watch(() => {
      notifications.show({
        title: 'Undo',
        message: 'Action undone successfully',
        icon: createElement(IconArrowBackUp, { size: 16 }),
        color: 'blue',
        autoClose: 2000,
      });
    });

    // Subscribe to undo failure
    const unsubUndoFail = undoFx.fail.watch(({ error }) => {
      notifications.show({
        title: 'Undo Failed',
        message: error instanceof Error ? error.message : 'Failed to undo action',
        icon: createElement(IconX, { size: 16 }),
        color: 'red',
        autoClose: 4000,
      });
    });

    // Subscribe to redo success
    const unsubRedoSuccess = redoFx.done.watch(() => {
      notifications.show({
        title: 'Redo',
        message: 'Action redone successfully',
        icon: createElement(IconArrowForwardUp, { size: 16 }),
        color: 'teal',
        autoClose: 2000,
      });
    });

    // Subscribe to redo failure
    const unsubRedoFail = redoFx.fail.watch(({ error }) => {
      notifications.show({
        title: 'Redo Failed',
        message: error instanceof Error ? error.message : 'Failed to redo action',
        icon: createElement(IconX, { size: 16 }),
        color: 'red',
        autoClose: 4000,
      });
    });

    return () => {
      unsubUndoSuccess();
      unsubUndoFail();
      unsubRedoSuccess();
      unsubRedoFail();
    };
  }, []);
}
