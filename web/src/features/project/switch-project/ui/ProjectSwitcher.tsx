import { $currentProject, projectSelected, useProjects } from '@entities/project';
import { CreateProjectModal } from '@features/project/create-project';
import {
  ActionIcon,
  Badge,
  Button,
  Group,
  Loader,
  Menu,
  Stack,
  Text,
  Tooltip,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { navigation } from '@shared/lib/navigation';
import type { Project } from '@shared/types';
import { IconArrowBackUp, IconChevronDown, IconPlus, IconRefresh } from '@tabler/icons-react';
import { formatDistanceToNow } from 'date-fns';
import { useUnit } from 'effector-react';
import { useMemo } from 'react';

export function ProjectSwitcher() {
  const currentProject = useUnit($currentProject);
  const { data, isLoading, isError, refetch } = useProjects();
  const [isCreateOpen, { open: openCreate, close: closeCreate }] = useDisclosure(false);

  const sortedProjects = useMemo(() => {
    if (!data) return [];
    return [...data].sort((a, b) => {
      const aTime = a.lastOpenedAt ? new Date(a.lastOpenedAt).getTime() : 0;
      const bTime = b.lastOpenedAt ? new Date(b.lastOpenedAt).getTime() : 0;
      return bTime - aTime;
    });
  }, [data]);

  const handleSelect = (projectId: string, project?: Project) => {
    const projectData = project ?? sortedProjects.find((p) => p.id === projectId);
    if (projectData) {
      const now = new Date().toISOString();
      projectSelected({ ...projectData, lastOpenedAt: now });
    }
    navigation.toProjectFiles(projectId);
  };

  return (
    <>
      <Menu position="bottom-end" width={320} withinPortal shadow="md">
        <Menu.Target>
          <Button
            variant="light"
            size="sm"
            rightSection={<IconChevronDown size={14} />}
            leftSection={<IconArrowBackUp size={14} />}
            loading={isLoading}
          >
            {currentProject ? currentProject.name : 'Switch project'}
          </Button>
        </Menu.Target>
        <Menu.Dropdown>
          <Menu.Label>Recent projects</Menu.Label>

          {isLoading && (
            <Menu.Item>
              <Group gap="xs">
                <Loader size="xs" />
                <Text size="sm">Loading projects…</Text>
              </Group>
            </Menu.Item>
          )}

          {isError && (
            <Menu.Item leftSection={<IconRefresh size={14} />} onClick={() => refetch()}>
              Retry loading projects
            </Menu.Item>
          )}

          {!isLoading && sortedProjects.length === 0 && (
            <Menu.Item disabled c="dimmed">
              No projects yet
            </Menu.Item>
          )}

          {sortedProjects.map((project) => (
            <Menu.Item key={project.id} onClick={() => handleSelect(project.id)}>
              <Stack gap={2}>
                <Group gap="xs">
                  <Text fw={600} size="sm">
                    {project.name}
                  </Text>
                  <Badge size="xs" variant="light">
                    FHIR {project.fhirVersion}
                  </Badge>
                </Group>
                <Group gap="xs">
                  <Text size="xs" c="dimmed" ff="monospace">
                    {project.id}
                  </Text>
                  {project.lastOpenedAt && (
                    <Tooltip label={new Date(project.lastOpenedAt).toLocaleString()}>
                      <Text size="xs" c="dimmed">
                        {formatDistanceToNow(new Date(project.lastOpenedAt), { addSuffix: true })}
                      </Text>
                    </Tooltip>
                  )}
                </Group>
              </Stack>
            </Menu.Item>
          ))}

          <Menu.Divider />

          <Menu.Item leftSection={<IconPlus size={14} />} onClick={openCreate}>
            New project
          </Menu.Item>
          <Menu.Item
            leftSection={<IconRefresh size={14} />}
            onClick={() => refetch()}
            disabled={isLoading}
          >
            Refresh list
          </Menu.Item>
        </Menu.Dropdown>
      </Menu>

      <CreateProjectModal
        opened={isCreateOpen}
        onClose={closeCreate}
        onCreated={(project) => {
          closeCreate();
          handleSelect(project.id, project);
        }}
      />
    </>
  );
}
