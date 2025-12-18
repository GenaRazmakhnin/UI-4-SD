import { projectSelected, useProject, useUpdateProject } from '@entities/project';
import { usePackages } from '@entities/package/api/queries';
import {
  Alert,
  Badge,
  Button,
  Card,
  Container,
  Divider,
  Group,
  MultiSelect,
  Skeleton,
  Stack,
  Text,
  Textarea,
  TextInput,
  Title,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { navigation } from '@shared/lib/navigation';
import {
  IconAlertCircle,
  IconAlertTriangle,
  IconArrowLeft,
  IconClockHour4,
  IconHash,
  IconPackages,
  IconSquareRoundedCheck,
} from '@tabler/icons-react';
import { formatDistanceToNow } from 'date-fns';
import { useEffect, useMemo, useRef, useState } from 'react';
import { useParams } from '@tanstack/react-router';

export function ProjectDetailsPage() {
  const { projectId } = useParams({ from: '/projects/$projectId' });
  const { data, isLoading, isError, refetch } = useProject(projectId);
  const { data: packages } = usePackages();
  const { mutateAsync: updateProject, isPending: isSaving } = useUpdateProject(projectId);
  const [error, setError] = useState<string | null>(null);
  const lastLoadedIdRef = useRef<string | null>(null);

  const packageOptions = useMemo(() => {
    console.log('1111', packages);
    if (!packages || packages.length === 0) return [];
    console.log('aaa', packages);
    return packages
      .filter((pkg) => pkg.id && pkg.name)
      .map((pkg) => ({
        value: pkg.id,
        label: pkg.name,
        description: `v${pkg.version} • ${pkg.fhirVersion}`,
      }));
  }, [packages]);

  const form = useForm({
    initialValues: {
      name: '',
      description: '',
      canonicalBase: '',
      packageId: '',
      version: '',
      publisher: '',
      dependencies: [] as string[],
    },
    validate: {
      name: (value) => (value.trim().length === 0 ? 'Name is required' : null),
    },
  });

  useEffect(() => {
    if (data && data.id !== lastLoadedIdRef.current) {
      lastLoadedIdRef.current = data.id;
      projectSelected(data);
      form.setValues({
        name: data.name,
        description: data.description || '',
        canonicalBase: data.canonicalBase || '',
        packageId: data.packageId || '',
        version: data.version || '',
        publisher: data.publisher || '',
        dependencies:
          data.dependencies
            ?.map((dep) => dep.packageId || dep.name)
            .filter((pkgId): pkgId is string => Boolean(pkgId)) || [],
      });
    }
  }, [data, form]);

  const handleSave = form.onSubmit(async (values) => {
    if (!data) return;
    setError(null);
    const dependencies = values.dependencies.map((pkgId) => {
      const match = (packages ?? []).find((pkg) => pkg.id === pkgId);
      return {
        packageId: pkgId,
        version: match?.version ?? 'latest',
        name: match?.name ?? pkgId,
      };
    });

    try {
      const updated = await updateProject({
        name: values.name.trim(),
        description: values.description.trim() || undefined,
        canonicalBase: values.canonicalBase.trim() || undefined,
        packageId: values.packageId.trim() || undefined,
        version: values.version.trim() || undefined,
        publisher: values.publisher.trim() || undefined,
        dependencies,
      });
      projectSelected(updated);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save project');
    }
  });

  if (isLoading) {
    return (
      <Container size="md" py="xl">
        <Card padding="xl" radius="lg" withBorder shadow="sm">
          <Stack gap="md">
            <Skeleton height={24} width="40%" />
            <Skeleton height={12} width="60%" />
            <Skeleton height={12} width="50%" />
            <Skeleton height={12} width="70%" />
          </Stack>
        </Card>
      </Container>
    );
  }
  if (isError || !data) {
    return (
      <Container size="md" py="xl">
        <Card padding="lg" radius="lg" withBorder>
          <Stack gap="sm">
            <Group gap="xs">
              <IconAlertTriangle size={18} color="var(--mantine-color-red-6)" />
              <Text fw={600}>Could not load project</Text>
            </Group>
            <Text size="sm" c="dimmed">
              We couldn&apos;t restore this project. Try again or go back to your projects list.
            </Text>
            <Group>
              <Button variant="light" leftSection={<IconArrowLeft size={16} />} onClick={() => navigation.toProjects()}>
                Back to projects
              </Button>
              <Button onClick={() => refetch()}>Retry</Button>
            </Group>
          </Stack>
        </Card>
      </Container>
    );
  }

  return (
    <Container size="md" py="xl">
      <Stack gap="md">
            <Group justify="space-between" align="flex-start">
              <Stack gap={6}>
                <Title order={2}>{data.name}</Title>
                <Group gap="xs">
                  <Badge variant="light">FHIR {data.fhirVersion}</Badge>
                  {data.templateId && <Badge variant="light">{data.templateId}</Badge>}
                  {data.version && (
                    <Badge variant="light" color="gray">
                      {data.version}
                    </Badge>
                  )}
                </Group>
              </Stack>
              <Group gap="xs">
                <Button variant="light" onClick={() => navigation.toProjectFiles(data.id)}>
                  Files
                </Button>
                <Button
                  variant="subtle"
                  leftSection={<IconArrowLeft size={16} />}
                  onClick={() => navigation.toProjects()}
                >
                  Back to projects
                </Button>
              </Group>
            </Group>

        <Card withBorder radius="lg" padding="lg">
          <Stack gap="sm">
            <Group gap="xs">
              <IconHash size={14} style={{ opacity: 0.6 }} />
              <Text size="sm" fw={600}>
                Project ID
              </Text>
              <Text size="sm" ff="monospace">
                {data.id}
              </Text>
            </Group>

            {data.description && (
              <Text size="sm" c="dimmed">
                {data.description}
              </Text>
            )}

            <Divider my="sm" />

            <Group gap="sm">
              <IconClockHour4 size={14} style={{ opacity: 0.6 }} />
              <Text size="sm" c="dimmed">
                {data.lastOpenedAt
                  ? `Opened ${formatDistanceToNow(new Date(data.lastOpenedAt), { addSuffix: true })}`
                  : 'Not opened yet'}
              </Text>
            </Group>

            {data.updatedAt && (
              <Text size="sm" c="dimmed">
                Updated {formatDistanceToNow(new Date(data.updatedAt), { addSuffix: true })}
              </Text>
            )}
          </Stack>
        </Card>

        <Card withBorder radius="lg" padding="lg" shadow="sm">
          <Stack gap="md">
            <Group justify="space-between">
              <Group gap="xs">
                <IconPackages size={18} />
                <Text fw={600}>Project settings</Text>
              </Group>
              <Button type="submit" form="project-settings" loading={isSaving} variant="filled">
                Save changes
              </Button>
            </Group>

            {error && (
              <Alert icon={<IconAlertCircle size={16} />} color="red" variant="light">
                {error}
              </Alert>
            )}

            <form id="project-settings" onSubmit={handleSave}>
              <Stack gap="md">
                <TextInput
                  label="Project name"
                  placeholder="My IG"
                  required
                  {...form.getInputProps('name')}
                />

                <Textarea
                  label="Description"
                  placeholder="Add a short summary for teammates"
                  minRows={2}
                  autosize
                  {...form.getInputProps('description')}
                />

                <TextInput
                  label="Canonical base"
                  placeholder="http://example.org/fhir"
                  {...form.getInputProps('canonicalBase')}
                />

                <Group grow>
                  <TextInput
                    label="Package ID"
                    placeholder="org.example.myig"
                    {...form.getInputProps('packageId')}
                  />
                  <TextInput
                    label="Version"
                    placeholder="0.1.0"
                    {...form.getInputProps('version')}
                  />
                </Group>

                <TextInput
                  label="Publisher"
                  placeholder="FHIR Builders"
                  {...form.getInputProps('publisher')}
                />

                <MultiSelect
                  label="Package dependencies"
                  placeholder="Select packages"
                  searchable
                  data={packageOptions}
                  nothingFoundMessage="No packages"
                  {...form.getInputProps('dependencies')}
                />
              </Stack>
            </form>

            {form.values.dependencies.length > 0 && (
              <Stack gap="xs">
                <Text size="sm" fw={600}>
                  Current dependencies
                </Text>
                <Group gap="xs" wrap="wrap">
                  {form.values.dependencies.map((pkgId) => {
                    const pkg = packages?.find((p) => p.id === pkgId);
                    const label = pkg?.name || pkgId;
                    const version = pkg?.version;
                    return (
                      <Badge
                        key={pkgId}
                        variant="light"
                        leftSection={<IconSquareRoundedCheck size={12} />}
                        style={{ whiteSpace: 'normal' }}
                      >
                        {label}
                        {version ? ` • v${version}` : ''}
                      </Badge>
                    );
                  })}
                </Group>
              </Stack>
            )}
          </Stack>
        </Card>
      </Stack>
    </Container>
  );
}
