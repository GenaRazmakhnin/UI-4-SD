import {
  ActionIcon,
  Badge,
  Button,
  Collapse,
  Group,
  Paper,
  Switch,
  Tabs,
  Text,
  TextInput,
  Tooltip,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import type { ValidationSeverity } from '@shared/types';
import {
  IconAlertCircle,
  IconAlertTriangle,
  IconCheck,
  IconChevronDown,
  IconChevronUp,
  IconFilter,
  IconInfoCircle,
  IconRefresh,
  IconSearch,
  IconTrash,
  IconX,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState } from 'react';
import {
  $errorCount,
  $filters,
  $hasNewDiagnostics,
  $infoCount,
  $isValidating,
  $totalCount,
  $warningCount,
  diagnosticsCleared,
  filterCleared,
  filtersChanged,
  markAllAsRead,
  validateProfileFx,
} from '../model';
import { DiagnosticsList } from './DiagnosticsList';
import styles from './DiagnosticsPanel.module.css';

interface DiagnosticsPanelProps {
  /**
   * Profile ID to validate
   */
  profileId?: string;

  /**
   * Height of the panel (when expanded)
   */
  height?: number | string;

  /**
   * Whether the panel is collapsible
   * @default true
   */
  collapsible?: boolean;

  /**
   * Initial collapsed state
   * @default false
   */
  defaultCollapsed?: boolean;
}

export function DiagnosticsPanel({
  profileId,
  height = 300,
  collapsible = true,
  defaultCollapsed = false,
}: DiagnosticsPanelProps) {
  const [collapsed, { toggle: toggleCollapse }] = useDisclosure(!defaultCollapsed);
  const [activeTab, setActiveTab] = useState<string | null>('all');
  const [showFilters, setShowFilters] = useState(false);

  const errorCount = useUnit($errorCount);
  const warningCount = useUnit($warningCount);
  const infoCount = useUnit($infoCount);
  const totalCount = useUnit($totalCount);
  const isValidating = useUnit($isValidating);
  const hasNewDiagnostics = useUnit($hasNewDiagnostics);
  const filters = useUnit($filters);

  // Handle tab change to set severity filter
  const handleTabChange = (tab: string | null) => {
    setActiveTab(tab);

    if (tab === 'all') {
      filtersChanged({ severity: undefined });
    } else if (tab === 'errors') {
      filtersChanged({ severity: ['error'] });
    } else if (tab === 'warnings') {
      filtersChanged({ severity: ['warning'] });
    } else if (tab === 'info') {
      filtersChanged({ severity: ['info'] });
    }
  };

  // Handle validate
  const handleValidate = () => {
    if (profileId) {
      validateProfileFx(profileId);
    }
  };

  // Handle clear
  const handleClear = () => {
    diagnosticsCleared();
  };

  // Handle mark as read
  const handleMarkAsRead = () => {
    markAllAsRead();
  };

  // Handle search change
  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    filtersChanged({ searchQuery: e.currentTarget.value || undefined });
  };

  // Handle show fixed toggle
  const handleShowFixedToggle = (checked: boolean) => {
    filtersChanged({ showFixed: checked });
  };

  return (
    <Paper className={styles.panel} shadow="sm" withBorder>
      {/* Header */}
      <div className={styles.header}>
        <Group gap="sm" wrap="nowrap">
          {/* Title with counts */}
          <Group gap="xs">
            <Text size="sm" fw={600}>
              Diagnostics
            </Text>

            {totalCount > 0 && (
              <Badge size="sm" variant="filled" color={errorCount > 0 ? 'red' : 'orange'}>
                {totalCount}
              </Badge>
            )}

            {hasNewDiagnostics && (
              <Tooltip label="New issues detected">
                <Badge size="xs" variant="dot" color="blue">
                  New
                </Badge>
              </Tooltip>
            )}
          </Group>

          {/* Actions */}
          <Group gap="xs" ml="auto">
            {/* Search */}
            <TextInput
              placeholder="Search..."
              size="xs"
              leftSection={<IconSearch size={14} />}
              value={filters.searchQuery || ''}
              onChange={handleSearchChange}
              className={styles.searchInput}
              rightSection={
                filters.searchQuery && (
                  <ActionIcon
                    size="xs"
                    variant="subtle"
                    onClick={() => filtersChanged({ searchQuery: undefined })}
                  >
                    <IconX size={12} />
                  </ActionIcon>
                )
              }
            />

            {/* Filter toggle */}
            <Tooltip label="Filters">
              <ActionIcon
                variant={showFilters ? 'filled' : 'subtle'}
                size="sm"
                onClick={() => setShowFilters(!showFilters)}
              >
                <IconFilter size={16} />
              </ActionIcon>
            </Tooltip>

            {/* Validate */}
            {profileId && (
              <Tooltip label="Validate profile">
                <ActionIcon
                  variant="subtle"
                  size="sm"
                  onClick={handleValidate}
                  loading={isValidating}
                >
                  <IconRefresh size={16} />
                </ActionIcon>
              </Tooltip>
            )}

            {/* Mark as read */}
            {hasNewDiagnostics && (
              <Tooltip label="Mark all as read">
                <ActionIcon variant="subtle" size="sm" onClick={handleMarkAsRead}>
                  <IconCheck size={16} />
                </ActionIcon>
              </Tooltip>
            )}

            {/* Clear */}
            <Tooltip label="Clear all">
              <ActionIcon
                variant="subtle"
                size="sm"
                color="red"
                onClick={handleClear}
                disabled={totalCount === 0}
              >
                <IconTrash size={16} />
              </ActionIcon>
            </Tooltip>

            {/* Collapse toggle */}
            {collapsible && (
              <ActionIcon variant="subtle" size="sm" onClick={toggleCollapse}>
                {collapsed ? <IconChevronUp size={16} /> : <IconChevronDown size={16} />}
              </ActionIcon>
            )}
          </Group>
        </Group>

        {/* Filters row */}
        <Collapse in={showFilters}>
          <Group gap="md" mt="sm">
            <Switch
              label="Show fixed issues"
              size="xs"
              checked={filters.showFixed || false}
              onChange={(e) => handleShowFixedToggle(e.currentTarget.checked)}
            />

            {(filters.searchQuery || filters.showFixed) && (
              <Button
                size="compact-xs"
                variant="subtle"
                leftSection={<IconX size={12} />}
                onClick={() => filterCleared()}
              >
                Clear filters
              </Button>
            )}
          </Group>
        </Collapse>
      </div>

      {/* Content */}
      <Collapse in={collapsed}>
        <Tabs value={activeTab} onChange={handleTabChange} className={styles.tabs}>
          <Tabs.List>
            <Tabs.Tab
              value="all"
              rightSection={
                totalCount > 0 ? (
                  <Badge size="xs" variant="filled" color="gray">
                    {totalCount}
                  </Badge>
                ) : null
              }
            >
              All
            </Tabs.Tab>

            <Tabs.Tab
              value="errors"
              leftSection={<IconAlertCircle size={14} />}
              rightSection={
                errorCount > 0 ? (
                  <Badge size="xs" variant="filled" color="red">
                    {errorCount}
                  </Badge>
                ) : null
              }
            >
              Errors
            </Tabs.Tab>

            <Tabs.Tab
              value="warnings"
              leftSection={<IconAlertTriangle size={14} />}
              rightSection={
                warningCount > 0 ? (
                  <Badge size="xs" variant="filled" color="orange">
                    {warningCount}
                  </Badge>
                ) : null
              }
            >
              Warnings
            </Tabs.Tab>

            <Tabs.Tab
              value="info"
              leftSection={<IconInfoCircle size={14} />}
              rightSection={
                infoCount > 0 ? (
                  <Badge size="xs" variant="filled" color="blue">
                    {infoCount}
                  </Badge>
                ) : null
              }
            >
              Info
            </Tabs.Tab>
          </Tabs.List>

          <Tabs.Panel value="all">
            <DiagnosticsList height={height} />
          </Tabs.Panel>

          <Tabs.Panel value="errors">
            <DiagnosticsList height={height} emptyMessage="No errors" />
          </Tabs.Panel>

          <Tabs.Panel value="warnings">
            <DiagnosticsList height={height} emptyMessage="No warnings" />
          </Tabs.Panel>

          <Tabs.Panel value="info">
            <DiagnosticsList height={height} emptyMessage="No info messages" />
          </Tabs.Panel>
        </Tabs>
      </Collapse>

      {/* Collapsed summary */}
      {!collapsed && totalCount > 0 && (
        <Group gap="md" p="xs" className={styles.collapsedSummary}>
          {errorCount > 0 && (
            <Group gap={4}>
              <IconAlertCircle size={14} color="var(--mantine-color-red-6)" />
              <Text size="xs" c="red">
                {errorCount} error{errorCount !== 1 ? 's' : ''}
              </Text>
            </Group>
          )}

          {warningCount > 0 && (
            <Group gap={4}>
              <IconAlertTriangle size={14} color="var(--mantine-color-orange-6)" />
              <Text size="xs" c="orange">
                {warningCount} warning{warningCount !== 1 ? 's' : ''}
              </Text>
            </Group>
          )}

          {infoCount > 0 && (
            <Group gap={4}>
              <IconInfoCircle size={14} color="var(--mantine-color-blue-6)" />
              <Text size="xs" c="blue">
                {infoCount} info
              </Text>
            </Group>
          )}
        </Group>
      )}
    </Paper>
  );
}
