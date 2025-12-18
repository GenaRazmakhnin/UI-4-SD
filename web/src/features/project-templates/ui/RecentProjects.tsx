import { ActionIcon, Badge, Button, Card, Group, Menu, Stack, Text, Tooltip } from '@mantine/core';
import {
  IconClock,
  IconDotsVertical,
  IconFolder,
  IconFolderOpen,
  IconPlus,
  IconTrash,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import type { RecentProject } from '../lib/types';
import {
  $recentProjects,
  dialogOpened,
  recentProjectOpened,
  recentProjectRemoved,
  recentProjectsCleared,
} from '../model';
import styles from './RecentProjects.module.css';

function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp;

  const minutes = Math.floor(diff / (60 * 1000));
  const hours = Math.floor(diff / (60 * 60 * 1000));
  const days = Math.floor(diff / (24 * 60 * 60 * 1000));

  if (minutes < 1) return 'Just now';
  if (minutes < 60) return `${minutes}m ago`;
  if (hours < 24) return `${hours}h ago`;
  if (days < 7) return `${days}d ago`;

  return new Date(timestamp).toLocaleDateString();
}

interface ProjectCardProps {
  project: RecentProject;
  onOpen: (id: string) => void;
  onRemove: (id: string) => void;
}

function ProjectCard({ project, onOpen, onRemove }: ProjectCardProps) {
  return (
    <Card
      className={styles.projectCard}
      padding="sm"
      radius="md"
      withBorder
      onClick={() => onOpen(project.id)}
    >
      <Group justify="space-between" wrap="nowrap">
        <Group gap="sm" wrap="nowrap">
          <IconFolder size={24} className={styles.folderIcon} />
          <div>
            <Text fw={500} size="sm" truncate maw={200}>
              {project.name}
            </Text>
            <Group gap="xs">
              <Badge size="xs" variant="light">
                FHIR {project.fhirVersion}
              </Badge>
              {project.packageId && (
                <Text size="xs" c="dimmed" ff="monospace" truncate maw={150}>
                  {project.packageId}
                </Text>
              )}
            </Group>
          </div>
        </Group>

        <Group gap="xs" wrap="nowrap">
          <Tooltip label={formatRelativeTime(project.lastOpened)}>
            <Group gap={4}>
              <IconClock size={12} style={{ opacity: 0.5 }} />
              <Text size="xs" c="dimmed">
                {formatRelativeTime(project.lastOpened)}
              </Text>
            </Group>
          </Tooltip>

          <Menu position="bottom-end" shadow="md" withinPortal>
            <Menu.Target>
              <ActionIcon variant="subtle" size="sm" onClick={(e) => e.stopPropagation()}>
                <IconDotsVertical size={14} />
              </ActionIcon>
            </Menu.Target>
            <Menu.Dropdown>
              <Menu.Item
                leftSection={<IconFolderOpen size={14} />}
                onClick={(e) => {
                  e.stopPropagation();
                  onOpen(project.id);
                }}
              >
                Open Project
              </Menu.Item>
              <Menu.Divider />
              <Menu.Item
                leftSection={<IconTrash size={14} />}
                color="red"
                onClick={(e) => {
                  e.stopPropagation();
                  onRemove(project.id);
                }}
              >
                Remove from Recents
              </Menu.Item>
            </Menu.Dropdown>
          </Menu>
        </Group>
      </Group>
    </Card>
  );
}

interface RecentProjectsProps {
  maxItems?: number;
  showClearAll?: boolean;
  showNewButton?: boolean;
}

export function RecentProjects({
  maxItems = 5,
  showClearAll = true,
  showNewButton = true,
}: RecentProjectsProps) {
  const recentProjects = useUnit($recentProjects);

  const handleOpen = (id: string) => {
    recentProjectOpened(id);
  };

  const handleRemove = (id: string) => {
    recentProjectRemoved(id);
  };

  const handleClearAll = () => {
    if (confirm('Remove all recent projects?')) {
      recentProjectsCleared();
    }
  };

  const handleNewProject = () => {
    dialogOpened();
  };

  const displayedProjects = recentProjects.slice(0, maxItems);

  return (
    <Stack gap="md">
      <Group justify="space-between">
        <Text fw={600}>Recent Projects</Text>
        <Group gap="xs">
          {showClearAll && recentProjects.length > 0 && (
            <Button variant="subtle" size="xs" color="gray" onClick={handleClearAll}>
              Clear All
            </Button>
          )}
          {showNewButton && (
            <Button size="xs" leftSection={<IconPlus size={14} />} onClick={handleNewProject}>
              New Project
            </Button>
          )}
        </Group>
      </Group>

      {displayedProjects.length === 0 ? (
        <Card padding="xl" radius="md" withBorder>
          <Stack align="center" gap="sm">
            <IconFolder size={48} style={{ opacity: 0.3 }} />
            <Text c="dimmed" ta="center">
              No recent projects.
              <br />
              Create a new project to get started.
            </Text>
            {showNewButton && (
              <Button
                variant="light"
                leftSection={<IconPlus size={16} />}
                onClick={handleNewProject}
              >
                Create New Project
              </Button>
            )}
          </Stack>
        </Card>
      ) : (
        <Stack gap="xs">
          {displayedProjects.map((project) => (
            <ProjectCard
              key={project.id}
              project={project}
              onOpen={handleOpen}
              onRemove={handleRemove}
            />
          ))}
        </Stack>
      )}

      {recentProjects.length > maxItems && (
        <Text size="xs" c="dimmed" ta="center">
          +{recentProjects.length - maxItems} more projects
        </Text>
      )}
    </Stack>
  );
}
