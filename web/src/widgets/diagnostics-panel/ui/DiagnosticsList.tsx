import { Accordion, Badge, Group, ScrollArea, Stack, Text } from '@mantine/core';
import type { Diagnostic } from '@shared/types';
import { IconChevronRight } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { $groupedDiagnostics, $selectedDiagnosticId } from '../model';
import { DiagnosticItem } from './DiagnosticItem';
import styles from './DiagnosticsList.module.css';

interface DiagnosticsListProps {
  /**
   * Height of the scrollable area
   */
  height?: number | string;

  /**
   * Whether to show grouped view
   * @default true
   */
  grouped?: boolean;

  /**
   * Empty state message
   */
  emptyMessage?: string;
}

export function DiagnosticsList({
  height = 400,
  grouped = true,
  emptyMessage = 'No issues found',
}: DiagnosticsListProps) {
  const groupedDiagnostics = useUnit($groupedDiagnostics);
  const selectedDiagnosticId = useUnit($selectedDiagnosticId);

  // Flatten for ungrouped view
  const allDiagnostics = groupedDiagnostics.flatMap((g) => g.diagnostics);

  if (allDiagnostics.length === 0) {
    return (
      <div className={styles.emptyState} style={{ height }}>
        <Text size="sm" c="dimmed">
          {emptyMessage}
        </Text>
      </div>
    );
  }

  if (!grouped) {
    return (
      <ScrollArea h={height} className={styles.scrollArea}>
        <Stack gap="xs" p="sm">
          {allDiagnostics.map((diagnostic) => (
            <DiagnosticItem
              key={diagnostic.id}
              diagnostic={diagnostic}
              selected={selectedDiagnosticId === diagnostic.id}
              showElementPath
            />
          ))}
        </Stack>
      </ScrollArea>
    );
  }

  return (
    <ScrollArea h={height} className={styles.scrollArea}>
      <Accordion
        multiple
        defaultValue={groupedDiagnostics.map((g) => g.path)}
        classNames={{
          item: styles.accordionItem,
          control: styles.accordionControl,
          content: styles.accordionContent,
        }}
      >
        {groupedDiagnostics.map((group) => (
          <Accordion.Item key={group.path} value={group.path}>
            <Accordion.Control>
              <Group gap="xs" wrap="nowrap">
                <Text size="sm" fw={500} className={styles.groupPath}>
                  {group.path}
                </Text>

                <Group gap={4}>
                  {getSeverityCounts(group.diagnostics).map(
                    ({ severity, count }) =>
                      count > 0 && (
                        <Badge
                          key={severity}
                          size="sm"
                          variant="light"
                          color={getSeverityColor(severity)}
                        >
                          {count}
                        </Badge>
                      )
                  )}
                </Group>
              </Group>
            </Accordion.Control>

            <Accordion.Panel>
              <Stack gap="xs">
                {group.diagnostics.map((diagnostic) => (
                  <DiagnosticItem
                    key={diagnostic.id}
                    diagnostic={diagnostic}
                    selected={selectedDiagnosticId === diagnostic.id}
                    showElementPath={false}
                  />
                ))}
              </Stack>
            </Accordion.Panel>
          </Accordion.Item>
        ))}
      </Accordion>
    </ScrollArea>
  );
}

/**
 * Get severity counts for a group of diagnostics
 */
function getSeverityCounts(diagnostics: Diagnostic[]) {
  const counts = {
    error: 0,
    warning: 0,
    info: 0,
  };

  for (const d of diagnostics) {
    if (!d.isFixed) {
      counts[d.severity]++;
    }
  }

  return [
    { severity: 'error' as const, count: counts.error },
    { severity: 'warning' as const, count: counts.warning },
    { severity: 'info' as const, count: counts.info },
  ];
}

function getSeverityColor(severity: Diagnostic['severity']): string {
  switch (severity) {
    case 'error':
      return 'red';
    case 'warning':
      return 'orange';
    case 'info':
      return 'blue';
    default:
      return 'gray';
  }
}
