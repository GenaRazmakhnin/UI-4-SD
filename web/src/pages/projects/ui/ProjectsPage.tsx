import { projectSelected, useProjects } from '@entities/project';
import { CreateProjectModal } from '@features/project/create-project';
import {
  ActionIcon,
  Badge,
  Button,
  Card,
  Container,
  Group,
  SimpleGrid,
  Skeleton,
  Stack,
  Text,
  Title,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { navigation } from '@shared/lib/navigation';
import type { Project } from '@shared/types';
import {
  IconAlertCircle,
  IconArrowRight,
  IconCalendarClock,
  IconFolders,
  IconPencil,
  IconPlus,
  IconRefresh,
} from '@tabler/icons-react';
import { formatDistanceToNow } from 'date-fns';

interface ProjectCardProps {
  project: Project;
  onOpen: (project: Project) => void;
  onEdit: (project: Project) => void;
}

function ProjectCard({ project, onOpen, onEdit }: ProjectCardProps) {
  return (
    <Card withBorder padding="lg" radius="lg" shadow="sm" style={{ transition: 'all 150ms ease' }}>
      <Stack gap="sm">
        <Group justify="space-between" align="flex-start">
          <Stack gap={2}>
            <Text fw={600}>{project.name}</Text>
            <Text size="xs" c="dimmed" ff="monospace">
              {project.id}
            </Text>
          </Stack>
          <Group gap="xs">
            <Badge variant="light">FHIR {project.fhirVersion}</Badge>
            <ActionIcon
              variant="subtle"
              radius="lg"
              aria-label={`Edit project ${project.name}`}
              onClick={() => onEdit(project)}
            >
              <IconPencil size={16} />
            </ActionIcon>
          </Group>
        </Group>

        {project.description && (
          <Text size="sm" c="dimmed" lineClamp={2}>
            {project.description}
          </Text>
        )}

        <Group justify="space-between" align="center">
          <Group gap={6}>
            <IconCalendarClock size={14} style={{ opacity: 0.6 }} />
            <Text size="xs" c="dimmed">
              {project.lastOpenedAt
                ? `Opened ${formatDistanceToNow(new Date(project.lastOpenedAt), {
                    addSuffix: true,
                  })}`
                : project.updatedAt
                  ? `Updated ${formatDistanceToNow(new Date(project.updatedAt), {
                      addSuffix: true,
                    })}`
                  : 'Fresh project'}
            </Text>
          </Group>
          <Button
            size="xs"
            variant="light"
            rightSection={<IconArrowRight size={14} />}
            onClick={() => onOpen(project)}
          >
            Open
          </Button>
        </Group>
      </Stack>
    </Card>
  );
}

export function ProjectsPage() {
  const { data, isLoading, isError, refetch } = useProjects();
  const [isCreateOpen, { open: openCreate, close: closeCreate }] = useDisclosure(false);
  const projects = data ?? [];

  const handleOpenProject = (project: Project) => {
    const now = new Date().toISOString();
    const withTimestamp = { ...project, lastOpenedAt: now };
    projectSelected(withTimestamp);
    navigation.toProjectFiles(project.id);
  };

  const handleEditProject = (project: Project) => {
    projectSelected(project);
    navigation.toProject(project.id);
  };

  const handleCreated = (project: Project) => {
    handleOpenProject(project);
    closeCreate();
  };

  const renderSkeletons = () => (
    <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
      {Array.from({ length: 6 }).map((_, index) => (
        <Card key={index} padding="lg" radius="lg" withBorder shadow="sm">
          <Stack gap="sm">
            <Skeleton height={16} width="60%" />
            <Skeleton height={12} width="40%" />
            <Skeleton height={10} width="50%" />
            <Skeleton height={10} width="70%" />
          </Stack>
        </Card>
      ))}
    </SimpleGrid>
  );

  return (
    <Container size="xl" py="xl">
      <Stack gap="lg">
        <Group justify="space-between" align="flex-start">
          <Stack gap={4}>
            <Title order={2}>Projects</Title>
            <Text c="dimmed">Browse, create, and switch between your FHIR projects.</Text>
          </Stack>
          <Group gap="sm">
            <ActionIcon variant="filled" size="lg" radius="lg" onClick={openCreate}>
              <IconPlus size={18} />
            </ActionIcon>
          </Group>
        </Group>

        {isError && (
          <Card padding="md" radius="lg" withBorder>
            <Group justify="space-between">
              <Group gap="sm">
                <IconAlertCircle size={18} color="var(--mantine-color-red-6)" />
                <div>
                  <Text fw={600}>Could not load projects</Text>
                  <Text size="sm" c="dimmed">
                    Check your connection or retry in a moment.
                  </Text>
                </div>
              </Group>
              <ActionIcon variant="light" onClick={() => refetch()}>
                <IconRefresh size={16} />
              </ActionIcon>
            </Group>
          </Card>
        )}

        {isLoading && renderSkeletons()}

        {!isLoading && projects.length === 0 && !isError && (
          <Card padding="xl" radius="lg" withBorder>
            <Stack align="center" gap="sm">
              <IconFolders size={40} style={{ opacity: 0.4 }} />
              <Text fw={600}>No projects yet</Text>
              <Text size="sm" c="dimmed" ta="center">
                Start a new project to begin authoring profiles and value sets.
              </Text>
              <ActionIcon variant="filled" size="lg" radius="lg" onClick={openCreate}>
                <IconPlus size={18} />
              </ActionIcon>
            </Stack>
          </Card>
        )}

        {!isLoading && projects.length > 0 && (
          <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
            {projects.map((project) => (
              <ProjectCard
                key={project.id}
                project={project}
                onOpen={handleOpenProject}
                onEdit={handleEditProject}
              />
            ))}
          </SimpleGrid>
        )}
      </Stack>

      <CreateProjectModal opened={isCreateOpen} onClose={closeCreate} onCreated={handleCreated} />
    </Container>
  );
}
