import { projectSelected, useCreateProject } from '@entities/project';
import { Badge, Button, Group, Modal, Select, Stack, Text, Textarea, TextInput } from '@mantine/core';
import { useForm } from '@mantine/form';
import type { FhirVersion, Project } from '@shared/types';
import { IconAlertCircle, IconPlus } from '@tabler/icons-react';
import { useMemo, useState } from 'react';

const FHIR_VERSION_OPTIONS: { value: FhirVersion; label: string }[] = [
  { value: '4.0.1', label: 'FHIR R4 (4.0.1)' },
  { value: '4.3.0', label: 'FHIR R4B (4.3.0)' },
  { value: '5.0.0', label: 'FHIR R5 (5.0.0)' },
];

const TEMPLATE_OPTIONS = [
  { value: 'blank', label: 'Blank project' },
  { value: 'implementation-guide', label: 'Implementation guide' },
  { value: 'regional', label: 'Regional starter' },
  { value: 'research', label: 'Research starter' },
];

interface CreateProjectModalProps {
  opened: boolean;
  onClose: () => void;
  onCreated?: (project: Project) => void;
}

export function CreateProjectModal({ opened, onClose, onCreated }: CreateProjectModalProps) {
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const { mutateAsync, isPending, reset } = useCreateProject();

  const form = useForm({
    initialValues: {
      name: '',
      fhirVersion: '4.0.1' as FhirVersion,
      templateId: 'blank',
      description: '',
    },
    validate: {
      name: (value) => (value.trim().length === 0 ? 'Project name is required' : null),
      fhirVersion: (value) => (!value ? 'Select a FHIR version' : null),
    },
  });

  const templateHint = useMemo(() => {
    switch (form.values.templateId) {
      case 'implementation-guide':
        return 'Includes basic IG scaffolding and publisher defaults.';
      case 'regional':
        return 'Preloads IPS-aligned packages for regional work.';
      case 'research':
        return 'Adds research-oriented profiles for pilots.';
      default:
        return 'Start with a clean slate.';
    }
  }, [form.values.templateId]);

  const handleClose = () => {
    if (isPending) return;
    form.reset();
    setErrorMessage(null);
    reset();
    onClose();
  };

  const handleSubmit = form.onSubmit(async (values) => {
    try {
      const project = await mutateAsync({
        name: values.name.trim(),
        fhirVersion: values.fhirVersion,
        templateId: values.templateId || undefined,
        description: values.description.trim() || undefined,
      });

      projectSelected(project);
      onCreated?.(project);
      form.reset();
      setErrorMessage(null);
      reset();
      onClose();
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Failed to create project');
    }
  });

  return (
    <Modal
      opened={opened}
      onClose={handleClose}
      title="Create a project"
      centered
      radius="lg"
      overlayProps={{ opacity: 0.08, blur: 4 }}
    >
      <form onSubmit={handleSubmit}>
        <Stack gap="md">
          <TextInput
            label="Project name"
            placeholder="Care pathways IG"
            required
            data-autofocus
            {...form.getInputProps('name')}
          />

          <Select
            label="FHIR version"
            placeholder="Select version"
            data={FHIR_VERSION_OPTIONS}
            allowDeselect={false}
            required
            {...form.getInputProps('fhirVersion')}
          />

          <Select
            label="Template (optional)"
            placeholder="Choose a starting point"
            data={TEMPLATE_OPTIONS}
            clearable
            description={templateHint}
            {...form.getInputProps('templateId')}
          />

          <Textarea
            label="Description"
            placeholder="What are you building?"
            minRows={2}
            autosize
            {...form.getInputProps('description')}
          />

          {errorMessage && (
            <Group gap="xs">
              <IconAlertCircle size={16} color="var(--mantine-color-red-6)" />
              <Text size="sm" c="red.6">
                {errorMessage}
              </Text>
            </Group>
          )}

          <Group justify="space-between">
            <Group gap="xs">
              <Badge variant="light" color="gray">
                Auto-saves locally
              </Badge>
              <Badge variant="light" color="gray">
                Switch anytime
              </Badge>
            </Group>

            <Group gap="xs">
              <Button variant="subtle" onClick={handleClose} disabled={isPending}>
                Cancel
              </Button>
              <Button
                type="submit"
                leftSection={<IconPlus size={16} />}
                loading={isPending}
                radius="md"
              >
                Create project
              </Button>
            </Group>
          </Group>
        </Stack>
      </form>
    </Modal>
  );
}
