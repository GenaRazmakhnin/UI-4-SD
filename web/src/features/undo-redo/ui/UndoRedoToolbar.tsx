import { ActionIcon, Group, Tooltip } from '@mantine/core';
import { IconArrowBackUp, IconArrowForwardUp, IconHistory } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import {
  $canRedo,
  $canUndo,
  $redoPending,
  $undoPending,
  historyViewerToggled,
  redoTriggered,
  undoTriggered,
} from '../model';

interface UndoRedoToolbarProps {
  showHistoryButton?: boolean;
}

export function UndoRedoToolbar({ showHistoryButton = true }: UndoRedoToolbarProps) {
  const [canUndo, canRedo, undoPending, redoPending] = useUnit([
    $canUndo,
    $canRedo,
    $undoPending,
    $redoPending,
  ]);

  const handleUndo = () => {
    undoTriggered();
  };

  const handleRedo = () => {
    redoTriggered();
  };

  const handleHistoryToggle = () => {
    historyViewerToggled();
  };

  return (
    <Group gap="xs">
      <Tooltip label="Undo (Ctrl+Z)" position="bottom" withArrow disabled={!canUndo}>
        <ActionIcon
          variant="subtle"
          color="gray"
          onClick={handleUndo}
          disabled={!canUndo}
          loading={undoPending}
          aria-label="Undo"
        >
          <IconArrowBackUp size={18} />
        </ActionIcon>
      </Tooltip>

      <Tooltip label="Redo (Ctrl+Shift+Z)" position="bottom" withArrow disabled={!canRedo}>
        <ActionIcon
          variant="subtle"
          color="gray"
          onClick={handleRedo}
          disabled={!canRedo}
          loading={redoPending}
          aria-label="Redo"
        >
          <IconArrowForwardUp size={18} />
        </ActionIcon>
      </Tooltip>

      {showHistoryButton && (
        <Tooltip label="View history" position="bottom" withArrow>
          <ActionIcon
            variant="subtle"
            color="gray"
            onClick={handleHistoryToggle}
            aria-label="View history"
          >
            <IconHistory size={18} />
          </ActionIcon>
        </Tooltip>
      )}
    </Group>
  );
}
