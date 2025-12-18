import { ActionIcon, Badge, Button, Code, Group, Menu, Stack, Text, Tooltip } from '@mantine/core';
import type { Diagnostic, QuickFix } from '@shared/types';
import {
  IconAlertCircle,
  IconAlertTriangle,
  IconCheck,
  IconChevronRight,
  IconDotsVertical,
  IconInfoCircle,
  IconTool,
  IconX,
} from '@tabler/icons-react';
import { diagnosticClicked, diagnosticDismissed, quickFixApplied } from '../model';
import styles from './DiagnosticItem.module.css';

interface DiagnosticItemProps {
  diagnostic: Diagnostic;
  selected?: boolean;
  showElementPath?: boolean;
}

export function DiagnosticItem({
  diagnostic,
  selected = false,
  showElementPath = true,
}: DiagnosticItemProps) {
  const handleClick = () => {
    diagnosticClicked(diagnostic);
  };

  const handleDismiss = (e: React.MouseEvent) => {
    e.stopPropagation();
    diagnosticDismissed(diagnostic.id);
  };

  const handleApplyFix = (e: React.MouseEvent, fix: QuickFix) => {
    e.stopPropagation();
    quickFixApplied({ diagnosticId: diagnostic.id, fix });
  };

  const severityIcon = getSeverityIcon(diagnostic.severity);
  const severityColor = getSeverityColor(diagnostic.severity);

  return (
    <div
      className={`${styles.item} ${selected ? styles.selected : ''} ${diagnostic.isNew ? styles.isNew : ''} ${diagnostic.isFixed ? styles.isFixed : ''}`}
      onClick={handleClick}
      role="button"
      tabIndex={0}
      onKeyPress={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          handleClick();
        }
      }}
    >
      <Group gap="sm" wrap="nowrap" align="flex-start">
        {/* Severity Icon */}
        <div
          className={styles.iconWrapper}
          style={{ color: `var(--mantine-color-${severityColor}-6)` }}
        >
          {severityIcon}
        </div>

        {/* Content */}
        <Stack gap={4} className={styles.content}>
          {/* Message */}
          <Text size="sm" className={diagnostic.isFixed ? styles.fixedText : ''}>
            {diagnostic.message}
          </Text>

          {/* Metadata */}
          <Group gap="xs">
            <Code size="xs">{diagnostic.code}</Code>

            {showElementPath && diagnostic.elementPath && (
              <Group gap={4}>
                <IconChevronRight size={12} />
                <Text size="xs" c="dimmed" className={styles.path}>
                  {diagnostic.elementPath}
                </Text>
              </Group>
            )}

            {diagnostic.source && (
              <Badge size="xs" variant="light">
                {diagnostic.source}
              </Badge>
            )}
          </Group>

          {/* Quick Fixes */}
          {diagnostic.quickFixes && diagnostic.quickFixes.length > 0 && !diagnostic.isFixed && (
            <Group gap="xs" mt={4}>
              {diagnostic.quickFixes.map((fix, index) => (
                <Button
                  key={fix.id || index}
                  size="compact-xs"
                  variant={fix.isPreferred ? 'light' : 'subtle'}
                  leftSection={<IconTool size={12} />}
                  onClick={(e) => handleApplyFix(e, fix)}
                >
                  {fix.label}
                </Button>
              ))}
            </Group>
          )}
        </Stack>

        {/* Actions */}
        <Group gap={4} className={styles.actions}>
          {diagnostic.isFixed ? (
            <Badge size="sm" color="green" variant="light" leftSection={<IconCheck size={12} />}>
              Fixed
            </Badge>
          ) : (
            <Menu position="bottom-end" withinPortal>
              <Menu.Target>
                <ActionIcon
                  variant="subtle"
                  size="sm"
                  onClick={(e) => e.stopPropagation()}
                  aria-label="More actions"
                >
                  <IconDotsVertical size={14} />
                </ActionIcon>
              </Menu.Target>

              <Menu.Dropdown>
                <Menu.Item leftSection={<IconX size={14} />} onClick={handleDismiss}>
                  Dismiss
                </Menu.Item>

                {diagnostic.quickFixes?.map((fix, index) => (
                  <Menu.Item
                    key={fix.id || index}
                    leftSection={<IconTool size={14} />}
                    onClick={(e) => {
                      e.stopPropagation();
                      handleApplyFix(e as any, fix);
                    }}
                  >
                    {fix.label}
                  </Menu.Item>
                ))}
              </Menu.Dropdown>
            </Menu>
          )}
        </Group>
      </Group>
    </div>
  );
}

function getSeverityIcon(severity: Diagnostic['severity']) {
  switch (severity) {
    case 'error':
      return <IconAlertCircle size={18} />;
    case 'warning':
      return <IconAlertTriangle size={18} />;
    case 'info':
      return <IconInfoCircle size={18} />;
    default:
      return <IconInfoCircle size={18} />;
  }
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
