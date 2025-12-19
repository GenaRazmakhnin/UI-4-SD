import {
  $expandedPaths,
  $flattenedTree,
  $searchQuery,
  $selectedNode,
  $selectedPath,
  collapseAll,
  expandAll,
  type FlattenedTreeNode,
  nodeSelected,
  pathToggled,
  searchChanged,
  selectPath,
  treeLoaded,
  useProjectTree,
} from '@entities/file-tree';
import { Alert, Badge, Button, Group, Skeleton, Stack, Text, Title } from '@mantine/core';
import { useDebouncedValue, useDisclosure } from '@mantine/hooks';
import { notifications } from '@mantine/notifications';
import { navigation } from '@shared/lib/navigation';
import type { ProjectTreeNode } from '@shared/types';
import {
  IconAlertCircle,
  IconBolt,
  IconCloudDownload,
  IconCloudUpload,
  IconHierarchy3,
  IconRefresh,
  IconRoute,
  IconSparkles,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { createElement, useCallback, useEffect, useMemo, useState } from 'react';
import { useTreeKeyboard } from '../lib/useTreeKeyboard';
import { DetailsPanel } from './DetailsPanel';
import { ImportModal } from './ImportModal';
import styles from './ProjectExplorer.module.css';
import { ProjectTree } from './ProjectTree';
import { ProjectTreeToolbar } from './ProjectTreeToolbar';

interface ProjectExplorerProps {
  projectId: string;
  initialPath?: string;
  onCreateArtifact?: () => void;
}

export function ProjectExplorer({
  projectId,
  initialPath,
  onCreateArtifact,
}: ProjectExplorerProps) {
  const { data, isLoading, isError, isFetching, refetch } = useProjectTree(projectId);
  const rows = useUnit($flattenedTree);
  const expanded = useUnit($expandedPaths);
  const selectedNode = useUnit($selectedNode);
  const selectedPath = useUnit($selectedPath);
  const searchValue = useUnit($searchQuery);

  const [searchInput, setSearchInput] = useState(searchValue);
  const [debouncedSearch] = useDebouncedValue(searchInput, 250);
  const [importModalOpened, { open: openImportModal, close: closeImportModal }] =
    useDisclosure(false);

  // Load tree data when query resolves
  useEffect(() => {
    if (data) {
      treeLoaded({ projectId, nodes: data });
    }
  }, [data, projectId]);

  // Apply initial deep-link selection/search
  useEffect(() => {
    if (initialPath) {
      selectPath(initialPath);
    }
  }, [initialPath, data]);

  // Sync search input to store
  useEffect(() => {
    if (debouncedSearch !== searchValue) {
      searchChanged(debouncedSearch);
    }
  }, [debouncedSearch, searchValue]);

  // Keep input in sync if store changes from elsewhere
  useEffect(() => {
    setSearchInput(searchValue);
  }, [searchValue]);

  const handleSelect = useCallback((node: ProjectTreeNode) => {
    nodeSelected(node);
    // Just select the node to show info in DetailsPanel
    // User clicks "Open in Profile Editor" button to navigate
  }, []);

  const handleToggle = useCallback((path: string) => {
    pathToggled(path);
  }, []);

  const handleImport = useCallback(() => {
    openImportModal();
  }, [openImportModal]);

  const handleImported = useCallback(
    (profileId: string) => {
      // Navigate to the newly imported profile in the editor
      navigation.toEditor(projectId, profileId);
    },
    [projectId]
  );

  const handleExportSelected = useCallback(() => {
    if (!selectedNode) return;
    notifications.show({
      title: 'Export',
      message: `Exporting ${selectedNode.name} (mock).`,
      icon: createElement(IconCloudDownload, { size: 16 }),
      color: 'teal',
    });
  }, [selectedNode]);

  const handleExportAll = useCallback(() => {
    notifications.show({
      title: 'Export project',
      message: 'Bulk export to IR/SD/FSH coming soon.',
      icon: createElement(IconCloudDownload, { size: 16 }),
      color: 'teal',
    });
  }, []);

  useTreeKeyboard(handleSelect);

  const treeRowProps = useMemo(
    () => ({
      rows,
      expanded,
      selectedPath,
      onSelect: handleSelect,
      onToggle: handleToggle,
    }),
    [rows, expanded, selectedPath, handleSelect, handleToggle]
  );

  const totalFiles = useMemo(
    () => rows.filter((row: FlattenedTreeNode) => row.node.kind === 'file').length,
    [rows]
  );

  return (
    <Stack gap="md" className={styles.container}>
      <Stack gap="xs" className={styles.heading}>
        <Group justify="space-between" align="flex-start">
          <Title order={3}>Project Explorer</Title>
        </Group>
        <Text size="sm" c="dimmed">
          Browse Implementation (IR), StructureDefinitions (SD), and FSH assets for this project.
        </Text>
      </Stack>

      <Group justify="space-between" align="center" className={styles.headerActions}>
        <Group gap="xs">
          {onCreateArtifact && (
            <Button
              size="sm"
              variant="light"
              leftSection={<IconSparkles size={14} />}
              onClick={onCreateArtifact}
            >
              New artifact
            </Button>
          )}
          <Button
            size="sm"
            variant="light"
            leftSection={<IconCloudUpload size={14} />}
            onClick={handleImport}
          >
            Import
          </Button>
          <Button
            size="sm"
            variant="light"
            disabled={!selectedNode}
            leftSection={<IconCloudDownload size={14} />}
            onClick={handleExportSelected}
          >
            Export selected
          </Button>
          <Button
            size="sm"
            variant="subtle"
            leftSection={<IconCloudDownload size={14} />}
            onClick={handleExportAll}
          >
            Export project
          </Button>
        </Group>
        <Group gap="xs">
          <Badge variant="light" leftSection={<IconHierarchy3 size={14} />} size="sm">
            {rows.length} nodes
          </Badge>
          <Badge variant="light" leftSection={<IconBolt size={14} />} size="sm">
            {totalFiles} files
          </Badge>
        </Group>
      </Group>

      <ProjectTreeToolbar
        searchValue={searchInput}
        onSearchChange={setSearchInput}
        onExpandAll={() => expandAll()}
        onCollapseAll={() => collapseAll()}
        onRefresh={() => refetch()}
        loading={isFetching}
      />

      <div className={styles.split}>
        <div className={styles.treeSurface}>
          {isError && (
            <Alert
              color="red"
              title="Could not load project tree"
              icon={<IconAlertCircle size={16} />}
              variant="light"
              mb="xs"
              className={styles.alert}
              rightSection={
                <button className={styles.refreshButton} onClick={() => refetch()}>
                  <IconRefresh size={14} />
                  <span>Retry</span>
                </button>
              }
            >
              Check your connection or try again in a moment.
            </Alert>
          )}

          {isLoading ? (
            <div className={styles.loadingPane}>
              <Skeleton height={32} radius="md" />
              <Skeleton height={32} radius="md" />
              <Skeleton height={32} radius="md" />
              <Skeleton height={32} radius="md" />
              <Skeleton height={32} radius="md" />
            </div>
          ) : rows.length === 0 ? (
            <div className={styles.emptyState}>
              <IconRoute size={42} stroke={1.3} />
              <Text fw={600}>Nothing to show yet</Text>
              <Text size="sm" c="dimmed" ta="center">
                No files were returned for this project. Try refreshing or switch to a different
                project.
              </Text>
            </div>
          ) : (
            <ProjectTree {...treeRowProps} />
          )}
        </div>

        <div className={styles.detailsSurface}>
          <DetailsPanel
            node={selectedNode}
            isLoading={isLoading}
            fallbackPath={rows[0]?.node.path}
            onOpenProfile={(profileId) => navigation.toEditor(projectId, profileId)}
          />
        </div>
      </div>

      <ImportModal
        opened={importModalOpened}
        onClose={closeImportModal}
        projectId={projectId}
        onImported={handleImported}
      />
    </Stack>
  );
}
