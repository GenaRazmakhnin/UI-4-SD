import { projectSelected, useProjects } from '@entities/project';
import { CreateProjectModal } from '@features/project/create-project';
import {
  ActionIcon,
  Badge,
  Button,
  Card,
  Group,
  Paper,
  SimpleGrid,
  Skeleton,
  Stack,
  Text,
  Title,
  ThemeIcon,
  Box,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { navigation } from '@shared/lib/navigation';
import type { Project } from '@shared/types';
import {
  IconAlertCircle,
  IconArrowRight,
  IconCalendarClock,
  IconFolders,
  IconPackage,
  IconPencil,
  IconPlus,
  IconRefresh,
  IconFileDescription,
  IconSparkles,
  IconClock,
} from '@tabler/icons-react';
import { formatDistanceToNow } from 'date-fns';
import styles from './ProjectsPage.module.css';

interface ProjectCardProps {
  project: Project;
  onOpen: (project: Project) => void;
  onEdit: (project: Project) => void;
  featured?: boolean;
}

function ProjectCard({ project, onOpen, onEdit, featured }: ProjectCardProps) {
  return (
    <Card
      className={styles.projectCard}
      data-featured={featured || undefined}
      withBorder
      padding="lg"
      radius="md"
    >
      <Stack gap="sm">
        <Group justify="space-between" align="flex-start">
          <Stack gap={2}>
            <Text fw={600} size="md">
              {project.name}
            </Text>
            <Text size="xs" c="dimmed" ff="monospace">
              {project.id.slice(0, 8)}...
            </Text>
          </Stack>
          <Group gap="xs">
            <Badge variant="light" size="sm">
              FHIR {project.fhirVersion}
            </Badge>
            <ActionIcon
              variant="subtle"
              radius="md"
              size="sm"
              aria-label={`Edit project ${project.name}`}
              onClick={(e) => {
                e.stopPropagation();
                onEdit(project);
              }}
            >
              <IconPencil size={14} />
            </ActionIcon>
          </Group>
        </Group>

        {project.description && (
          <Text size="sm" c="dimmed" lineClamp={2}>
            {project.description}
          </Text>
        )}

        <Group justify="space-between" align="center" mt="auto">
          <Group gap={6}>
            <IconClock size={12} style={{ opacity: 0.5 }} />
            <Text size="xs" c="dimmed">
              {project.lastOpenedAt
                ? formatDistanceToNow(new Date(project.lastOpenedAt), {
                  addSuffix: true,
                })
                : project.modifiedAt
                  ? formatDistanceToNow(new Date(project.modifiedAt), {
                    addSuffix: true,
                  })
                  : 'Just created'}
            </Text>
          </Group>
          <Button
            size="xs"
            variant="light"
            rightSection={<IconArrowRight size={12} />}
            onClick={() => onOpen(project)}
          >
            Open
          </Button>
        </Group>
      </Stack>
    </Card>
  );
}

interface StatCardProps {
  icon: React.ReactNode;
  label: string;
  value: string | number;
  color: string;
}

function StatCard({ icon, label, value, color }: StatCardProps) {
  return (
    <Paper className={styles.statCard} withBorder p="md" radius="md">
      <Group gap="sm">
        <ThemeIcon variant="light" size="lg" radius="md" color={color}>
          {icon}
        </ThemeIcon>
        <div>
          <Text size="xl" fw={700} lh={1}>
            {value}
          </Text>
          <Text size="xs" c="dimmed">
            {label}
          </Text>
        </div>
      </Group>
    </Paper>
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

  // Get recent projects (opened in last 7 days)
  const recentProjects = projects
    .filter((p) => p.lastOpenedAt)
    .sort((a, b) => new Date(b.lastOpenedAt!).getTime() - new Date(a.lastOpenedAt!).getTime())
    .slice(0, 3);

  const allProjects = projects.sort((a, b) => {
    const aDate = a.modifiedAt || a.createdAt;
    const bDate = b.modifiedAt || b.createdAt;
    return new Date(bDate).getTime() - new Date(aDate).getTime();
  });

  const renderSkeletons = () => (
    <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
      {Array.from({ length: 6 }).map((_, index) => (
        <Card key={index} padding="lg" radius="md" withBorder>
          <Stack gap="sm">
            <Skeleton height={18} width="60%" />
            <Skeleton height={12} width="40%" />
            <Skeleton height={14} width="80%" />
            <Skeleton height={10} width="50%" />
          </Stack>
        </Card>
      ))}
    </SimpleGrid>
  );

  return (
    <div className={styles.container}>
      {/* Header */}
      <div className={styles.header}>
        <div>
          <Title order={2} fw={600}>
            Projects
          </Title>
          <Text c="dimmed" size="sm">
            Create and manage your FHIR implementation guides
          </Text>
        </div>
        <Button
          leftSection={<IconPlus size={16} />}
          onClick={openCreate}
          radius="md"
        >
          New Project
        </Button>
      </div>

      {/* Stats */}
      {!isLoading && projects.length > 0 && (
        <SimpleGrid cols={{ base: 2, sm: 4 }} spacing="md" className={styles.stats}>
          <StatCard
            icon={<IconFolders size={18} />}
            label="Total Projects"
            value={projects.length}
            color="blue"
          />
          <StatCard
            icon={<IconFileDescription size={18} />}
            label="Profiles"
            value="—"
            color="green"
          />
          <StatCard
            icon={<IconPackage size={18} />}
            label="Packages"
            value="—"
            color="violet"
          />
          <StatCard
            icon={<IconSparkles size={18} />}
            label="Extensions"
            value="—"
            color="orange"
          />
        </SimpleGrid>
      )}

      {/* Error State */}
      {isError && (
        <Card padding="md" radius="md" withBorder className={styles.errorCard}>
          <Group justify="space-between">
            <Group gap="sm">
              <IconAlertCircle size={18} className={styles.errorIcon} />
              <div>
                <Text fw={600}>Could not load projects</Text>
                <Text size="sm" c="dimmed">
                  Check your connection and try again
                </Text>
              </div>
            </Group>
            <ActionIcon variant="light" onClick={() => refetch()}>
              <IconRefresh size={16} />
            </ActionIcon>
          </Group>
        </Card>
      )}

      {/* Loading State */}
      {isLoading && renderSkeletons()}

      {/* Empty State */}
      {!isLoading && projects.length === 0 && !isError && (
        <Card padding="xl" radius="md" withBorder className={styles.emptyCard}>
          <Stack align="center" gap="md">
            <ThemeIcon size={64} radius="xl" variant="light" color="gray">
              <IconFolders size={32} />
            </ThemeIcon>
            <div style={{ textAlign: 'center' }}>
              <Text fw={600} size="lg">
                No projects yet
              </Text>
              <Text size="sm" c="dimmed" maw={300}>
                Start your first project to begin authoring FHIR profiles and extensions
              </Text>
            </div>
            <Button
              leftSection={<IconPlus size={16} />}
              onClick={openCreate}
              radius="md"
              size="md"
            >
              Create Project
            </Button>
          </Stack>
        </Card>
      )}

      {/* Recent Projects */}
      {!isLoading && recentProjects.length > 0 && (
        <section className={styles.section}>
          <Group justify="space-between" mb="md">
            <Group gap="xs">
              <IconCalendarClock size={18} style={{ opacity: 0.6 }} />
              <Text fw={600}>Recently Opened</Text>
            </Group>
          </Group>
          <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
            {recentProjects.map((project) => (
              <ProjectCard
                key={project.id}
                project={project}
                onOpen={handleOpenProject}
                onEdit={handleEditProject}
                featured
              />
            ))}
          </SimpleGrid>
        </section>
      )}

      {/* All Projects */}
      {!isLoading && allProjects.length > 0 && (
        <section className={styles.section}>
          <Group justify="space-between" mb="md">
            <Group gap="xs">
              <IconFolders size={18} style={{ opacity: 0.6 }} />
              <Text fw={600}>All Projects</Text>
            </Group>
            <Text size="xs" c="dimmed">
              {allProjects.length} project{allProjects.length !== 1 ? 's' : ''}
            </Text>
          </Group>
          <SimpleGrid cols={{ base: 1, sm: 2, lg: 3 }} spacing="md">
            {allProjects.map((project) => (
              <ProjectCard
                key={project.id}
                project={project}
                onOpen={handleOpenProject}
                onEdit={handleEditProject}
              />
            ))}
          </SimpleGrid>
        </section>
      )}

      <CreateProjectModal opened={isCreateOpen} onClose={closeCreate} onCreated={handleCreated} />
    </div>
  );
}
