import { ActionIcon, Group, TextInput, Tooltip } from '@mantine/core';
import { IconArrowsMinimize, IconArrowsMaximize, IconRefresh, IconSearch } from '@tabler/icons-react';
import styles from './ProjectExplorer.module.css';

interface ProjectTreeToolbarProps {
  searchValue: string;
  onSearchChange: (value: string) => void;
  onExpandAll: () => void;
  onCollapseAll: () => void;
  onRefresh: () => void;
  loading?: boolean;
}

export function ProjectTreeToolbar({
  searchValue,
  onSearchChange,
  onExpandAll,
  onCollapseAll,
  onRefresh,
  loading,
}: ProjectTreeToolbarProps) {
  return (
    <Group className={styles.toolbar} justify="space-between" wrap="nowrap" gap="sm">
      <TextInput
        placeholder="Search by name, path, or id"
        leftSection={<IconSearch size={16} />}
        value={searchValue}
        onChange={(event) => onSearchChange(event.currentTarget.value)}
        className={styles.search}
        radius="md"
        size="sm"
        aria-label="Search project files"
      />

      <Group gap={6} wrap="nowrap">
        <Tooltip label="Expand all folders" withinPortal>
          <ActionIcon variant="light" size="lg" radius="md" onClick={onExpandAll}>
            <IconArrowsMaximize size={16} />
          </ActionIcon>
        </Tooltip>

        <Tooltip label="Collapse to roots" withinPortal>
          <ActionIcon variant="light" size="lg" radius="md" onClick={onCollapseAll}>
            <IconArrowsMinimize size={16} />
          </ActionIcon>
        </Tooltip>

        <Tooltip label="Refresh tree" withinPortal>
          <ActionIcon
            variant="filled"
            color="blue"
            size="lg"
            radius="md"
            onClick={onRefresh}
            disabled={loading}
            aria-busy={loading}
          >
            <IconRefresh size={16} className={loading ? styles.spin : undefined} />
          </ActionIcon>
        </Tooltip>
      </Group>
    </Group>
  );
}
