import {
  ActionIcon,
  Badge,
  Button,
  Drawer,
  Group,
  ScrollArea,
  Stack,
  Text,
  Timeline,
  Tooltip,
} from '@mantine/core';
import {
  IconArrowBackUp,
  IconArrowForwardUp,
  IconPlayerTrackNext,
  IconTrash,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import type { Operation, OperationType } from '../lib/types';
import {
  $currentHistoryIndex,
  $historyViewerOpen,
  $operationHistory,
  $redoStack,
  historyClearRequested,
  historyPositionChanged,
  historyViewerClosed,
} from '../model';
import styles from './HistoryViewer.module.css';

const OPERATION_COLORS: Record<OperationType, string> = {
  cardinality: 'blue',
  binding: 'grape',
  flags: 'green',
  'type-constraint': 'orange',
  slice: 'cyan',
  extension: 'pink',
  general: 'gray',
};

const OPERATION_LABELS: Record<OperationType, string> = {
  cardinality: 'Cardinality',
  binding: 'Binding',
  flags: 'Flags',
  'type-constraint': 'Type Constraint',
  slice: 'Slice',
  extension: 'Extension',
  general: 'Change',
};

function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - timestamp;

  // Less than 1 minute ago
  if (diff < 60 * 1000) {
    return 'Just now';
  }

  // Less than 1 hour ago
  if (diff < 60 * 60 * 1000) {
    const minutes = Math.floor(diff / (60 * 1000));
    return `${minutes}m ago`;
  }

  // Same day
  if (date.toDateString() === now.toDateString()) {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }

  // Different day
  return date.toLocaleDateString([], { month: 'short', day: 'numeric' });
}

interface HistoryItemProps {
  operation: Operation;
  index: number;
  currentIndex: number;
  onJump: (index: number) => void;
}

function HistoryItem({ operation, index, currentIndex, onJump }: HistoryItemProps) {
  const isCurrent = index === currentIndex - 1;
  const isPast = index < currentIndex - 1;

  return (
    <Timeline.Item
      bullet={
        isCurrent ? (
          <IconPlayerTrackNext size={12} />
        ) : isPast ? (
          <IconArrowBackUp size={12} />
        ) : (
          <IconArrowForwardUp size={12} />
        )
      }
      className={isCurrent ? styles.currentItem : undefined}
    >
      <Group justify="space-between" wrap="nowrap">
        <Stack gap={4}>
          <Group gap="xs">
            <Badge size="xs" color={OPERATION_COLORS[operation.type]} variant="light">
              {OPERATION_LABELS[operation.type]}
            </Badge>
            <Text size="xs" c="dimmed">
              {formatTimestamp(operation.timestamp)}
            </Text>
          </Group>
          <Text size="sm">{operation.description}</Text>
          {operation.elementPath && (
            <Text size="xs" c="dimmed" ff="monospace">
              {operation.elementPath}
            </Text>
          )}
        </Stack>
        {!isCurrent && (
          <Tooltip label={`Jump to this point`}>
            <ActionIcon size="sm" variant="subtle" onClick={() => onJump(index + 1)}>
              <IconPlayerTrackNext size={14} />
            </ActionIcon>
          </Tooltip>
        )}
      </Group>
    </Timeline.Item>
  );
}

export function HistoryViewer() {
  const [isOpen, history, redoStack, currentIndex] = useUnit([
    $historyViewerOpen,
    $operationHistory,
    $redoStack,
    $currentHistoryIndex,
  ]);

  const handleClose = () => {
    historyViewerClosed();
  };

  const handleClear = () => {
    if (confirm('Clear all history? This cannot be undone.')) {
      historyClearRequested();
    }
  };

  const handleJump = (index: number) => {
    historyPositionChanged(index);
  };

  // Combine past and future operations for display
  const allOperations = [...history];
  const hasHistory = history.length > 0 || redoStack.length > 0;

  return (
    <Drawer
      opened={isOpen}
      onClose={handleClose}
      position="right"
      size="md"
      title={
        <Group>
          <Text fw={600}>Operation History</Text>
          <Badge size="sm" variant="light">
            {history.length} operations
          </Badge>
        </Group>
      }
    >
      <Stack h="100%">
        {!hasHistory ? (
          <Stack align="center" justify="center" h={200}>
            <IconArrowBackUp size={48} color="var(--mantine-color-dimmed)" />
            <Text c="dimmed" ta="center">
              No operations yet.
              <br />
              Changes you make will appear here.
            </Text>
          </Stack>
        ) : (
          <>
            <ScrollArea flex={1} offsetScrollbars>
              <Timeline active={currentIndex - 1} bulletSize={24} lineWidth={2}>
                {allOperations.map((operation, index) => (
                  <HistoryItem
                    key={operation.id}
                    operation={operation}
                    index={index}
                    currentIndex={currentIndex}
                    onJump={handleJump}
                  />
                ))}
              </Timeline>
            </ScrollArea>

            <Group justify="flex-end" pt="md">
              <Button
                variant="subtle"
                color="red"
                size="xs"
                leftSection={<IconTrash size={14} />}
                onClick={handleClear}
              >
                Clear History
              </Button>
            </Group>
          </>
        )}
      </Stack>
    </Drawer>
  );
}
