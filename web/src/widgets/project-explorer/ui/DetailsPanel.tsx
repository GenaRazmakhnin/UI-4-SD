import {
  Alert,
  Badge,
  Button,
  Divider,
  Group,
  Skeleton,
  Stack,
  Text,
  Code,
  ScrollArea,
} from '@mantine/core';
import type { ProjectTreeNode } from '@shared/types';
import { IconHierarchy3, IconInfoCircle, IconRoute } from '@tabler/icons-react';
import styles from './ProjectExplorer.module.css';

interface DetailsPanelProps {
  node: ProjectTreeNode | null;
  isLoading: boolean;
  fallbackPath?: string;
  onOpenProfile?: (profileId: string) => void;
}

export function DetailsPanel({ node, isLoading, fallbackPath, onOpenProfile }: DetailsPanelProps) {
  if (isLoading) {
    return (
      <Stack gap="sm">
        <Skeleton height={12} width="60%" />
        <Skeleton height={12} width="40%" />
        <Skeleton height={10} width="70%" />
        <Skeleton height={10} width="80%" />
      </Stack>
    );
  }

  if (!node) {
    return (
      <Stack gap={8} align="center" justify="center" className={styles.placeholder}>
        <IconHierarchy3 size={36} stroke={1.4} />
        <Text fw={600}>Select a file</Text>
        <Text size="sm" c="dimmed" ta="center">
          {fallbackPath
            ? `Choose a node to inspect metadata and open editors.`
            : 'Tree is empty for this project.'}
        </Text>
      </Stack>
    );
  }

  const rootColor = node.root === 'SD' ? 'blue' : node.root === 'IR' ? 'teal' : 'grape';
  const kindColor = resourceKindColor(node.resourceKind);

  const mockContent =
    node.kind === 'file'
      ? JSON.stringify(
          {
            path: node.path,
            resourceId: node.resourceId,
            resourceType: node.resourceType,
            canonicalUrl: node.canonicalUrl,
            note: 'File preview uses mock data until backend file content API is available.',
          },
          null,
          2
        )
      : null;

  return (
    <Stack gap="sm">
      <Group justify="space-between" align="flex-start">
        <Stack gap={2}>
          <Text fw={700}>{node.name}</Text>
          <Text size="sm" c="dimmed" ff="monospace">
            {node.path}
          </Text>
        </Stack>
        <Group gap={6}>
          {node.kind === 'file' && (
            <Badge color={kindColor} variant="light">
              {resourceKindLabel(node)}
            </Badge>
          )}
          <Badge color={rootColor} variant="light">
            {node.root}
          </Badge>
        </Group>
      </Group>

      <Divider />

      <Stack gap={6}>
        <Group gap={6}>
          <IconInfoCircle size={16} className={styles.mutedIcon} />
          <Text size="sm">
            {node.kind === 'folder'
              ? 'Folder'
              : node.resourceType || node.resourceKind || 'File'}
          </Text>
        </Group>

        {node.resourceId && (
          <Group gap={6}>
            <IconRoute size={16} className={styles.mutedIcon} />
            <Text size="sm" fw={600}>
              {node.resourceId}
            </Text>
          </Group>
        )}

        {node.canonicalUrl && (
          <Text size="sm" c="dimmed" className={styles.canonical}>
            {node.canonicalUrl}
          </Text>
        )}
      </Stack>

      {node.kind === 'file' && node.resourceKind && node.resourceKind !== 'profile' && (
        <Alert color="gray" variant="light" icon={<IconInfoCircle size={16} />}>
          {node.resourceKind === 'extension'
            ? 'Extension editor not implemented yet. You can review metadata while we build it.'
            : node.resourceKind === 'valueset'
              ? 'ValueSet details are read-only here. A dedicated editor is coming soon.'
              : 'This resource type is read-only in the project explorer for now.'}
        </Alert>
      )}

      {node.resourceKind === 'profile' && node.root === 'IR' && node.resourceId && (
        <>
          <Divider />
          <Button variant="light" size="sm" onClick={() => onOpenProfile?.(node.resourceId)}>
            Open in Profile Editor
          </Button>
        </>
      )}

      {mockContent && (
        <>
          <Divider />
          <Stack gap={6}>
            <Text size="sm" fw={600}>
              File preview
            </Text>
            <ScrollArea h={180} scrollbarSize={6} type="auto">
              <Code block>{mockContent}</Code>
            </ScrollArea>
          </Stack>
        </>
      )}
    </Stack>
  );
}

function resourceKindColor(kind?: ProjectTreeNode['resourceKind']) {
  switch (kind) {
    case 'profile':
      return 'blue';
    case 'valueset':
      return 'violet';
    case 'codesystem':
      return 'grape';
    case 'extension':
      return 'cyan';
    case 'instance':
      return 'teal';
    default:
      return 'gray';
  }
}

function resourceKindLabel(node: ProjectTreeNode) {
  if (node.kind === 'folder') return node.root;
  if (node.resourceKind) {
    return formatResourceKind(node.resourceKind);
  }
  if (node.resourceType) return node.resourceType;
  return 'File';
}

function formatResourceKind(kind: ProjectTreeNode['resourceKind']) {
  switch (kind) {
    case 'valueset':
      return 'ValueSet';
    case 'codesystem':
      return 'CodeSystem';
    case 'operation':
      return 'Operation';
    case 'mapping':
      return 'Mapping';
    case 'instance':
      return 'Instance';
    case 'example':
      return 'Example';
    case 'extension':
      return 'Extension';
    case 'profile':
      return 'Profile';
    default:
      return `${(kind || 'File').charAt(0).toUpperCase()}${(kind || 'File').slice(1)}`;
  }
}
