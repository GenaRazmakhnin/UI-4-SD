import { useCreateArtifact } from '@entities/file-tree';
import {
  Button,
  Group,
  Modal,
  SegmentedControl,
  Select,
  Stack,
  Text,
  TextInput,
  Textarea,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { IconPlus } from '@tabler/icons-react';
import { useMemo } from 'react';
import type { ArtifactKind, CreatedArtifact } from '@shared/types';

interface CreateArtifactModalProps {
  opened: boolean;
  onClose: () => void;
  projectId: string;
  onCreated?: (payload: { projectId: string; artifact: CreatedArtifact }) => void;
}

interface FormValues {
  kind: ArtifactKind;
  name: string;
  id: string;
  baseResource: string;
  description: string;
  context: string;
  purpose: string;
}

const baseResources = [
  'Patient',
  'Observation',
  'Condition',
  'Medication',
  'AllergyIntolerance',
  'Practitioner',
  'Encounter',
  'Coverage',
  'Claim',
];

const slugify = (value: string) =>
  value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/(^-|-$)+/g, '');

export function CreateArtifactModal({
  opened,
  onClose,
  projectId,
  onCreated,
}: CreateArtifactModalProps) {
  const createArtifact = useCreateArtifact();

  const form = useForm<FormValues>({
    initialValues: {
      kind: 'profile',
      name: '',
      id: '',
      baseResource: 'Patient',
      description: '',
      context: '',
      purpose: '',
    },
    validate: {
      name: (value) => (!value.trim() ? 'Name is required' : null),
      baseResource: (value, values) =>
        values.kind === 'profile' && !value ? 'Select base resource' : null,
    },
  });

  const inferredId = useMemo(() => form.values.id.trim() || slugify(form.values.name), [form.values]);

  const pathPreview = useMemo(() => {
    const folder =
      form.values.kind === 'valueset'
        ? 'SD/value-sets'
        : form.values.kind === 'extension'
          ? 'SD/extensions'
          : 'IR/input/profiles';
    return `${folder}/${inferredId || 'new-artifact'}.json`;
  }, [form.values.kind, inferredId]);

  const handleSubmit = form.onSubmit(async (values) => {
    if (!projectId) {
      form.setFieldError('name', 'Project is required');
      return;
    }

    const payload = {
      kind: values.kind,
      name: values.name.trim(),
      id: inferredId,
      baseResource: values.kind === 'profile' ? values.baseResource : undefined,
      description: values.description.trim() || undefined,
      context: values.kind === 'extension' ? values.context.trim() || undefined : undefined,
      purpose: values.kind === 'valueset' ? values.purpose.trim() || undefined : undefined,
    };

    try {
      const artifact = await createArtifact.mutateAsync({ projectId, input: payload });
      if (artifact) {
        onCreated?.({ projectId, artifact });
      }
      form.reset();
      onClose();
    } catch (error) {
      // Surface error on form for now
      form.setFieldError('name', (error as Error).message);
    }
  });

  const disabled = createArtifact.isPending;

  return (
    <Modal opened={opened} onClose={onClose} title="New project artifact" radius="lg" size="lg">
      <form onSubmit={handleSubmit}>
        <Stack gap="md">
          <SegmentedControl
            value={form.values.kind}
            onChange={(value) => form.setFieldValue('kind', value as ArtifactKind)}
            data={[
              { label: 'Profile', value: 'profile' },
              { label: 'Extension', value: 'extension' },
              { label: 'ValueSet', value: 'valueset' },
            ]}
          />

          <Group grow>
            <TextInput
              label="Name"
              placeholder="e.g., Vital Signs Profile"
              required
              {...form.getInputProps('name')}
              onChange={(event) => {
                form.getInputProps('name').onChange(event);
                if (!form.values.id) {
                  form.setFieldValue('id', slugify(event.currentTarget.value));
                }
              }}
            />
            <TextInput
              label="ID (slug)"
              placeholder="vital-signs-profile"
              {...form.getInputProps('id')}
            />
          </Group>

          {form.values.kind === 'profile' && (
            <Select
              label="Base resource"
              data={baseResources}
              searchable
              placeholder="Select base resource"
              {...form.getInputProps('baseResource')}
            />
          )}

          {form.values.kind === 'extension' && (
            <TextInput
              label="Context (optional)"
              placeholder="e.g., Patient.identifier"
              {...form.getInputProps('context')}
            />
          )}

          {form.values.kind === 'valueset' && (
            <TextInput
              label="Purpose (optional)"
              placeholder="e.g., Codes for device alert categories"
              {...form.getInputProps('purpose')}
            />
          )}

          <Textarea
            label="Description"
            placeholder="Short summary of this artifact"
            autosize
            minRows={2}
            {...form.getInputProps('description')}
          />

          <Text size="sm" c="dimmed">
            File path preview: <Text span fw={600}>{pathPreview}</Text>
          </Text>

          <Group justify="flex-end">
            <Button variant="light" onClick={onClose}>
              Cancel
            </Button>
            <Button
              type="submit"
              leftSection={<IconPlus size={16} />}
              loading={disabled}
              disabled={disabled}
            >
              Create
            </Button>
          </Group>
        </Stack>
      </form>
    </Modal>
  );
}
