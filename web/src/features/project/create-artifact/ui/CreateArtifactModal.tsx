import { useCreateArtifact } from '@entities/file-tree';
import {
  Button,
  Divider,
  Group,
  Loader,
  Modal,
  SegmentedControl,
  Select,
  Stack,
  Text,
  Textarea,
  TextInput,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { api } from '@shared/api';
import type { ArtifactKind, BaseResource, CreatedArtifact } from '@shared/types';
import { IconPlus } from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { useMemo } from 'react';

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
  content: string;
}

// Fallback list if API fails
const fallbackBaseResources = [
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

  // Fetch base resources from the API
  const { data: baseResources, isLoading: isLoadingResources } = useQuery({
    queryKey: ['baseResources'],
    queryFn: () => api.search.baseResources(),
    staleTime: 1000 * 60 * 10, // Cache for 10 minutes
  });

  // Convert base resources to Select data format
  const baseResourceOptions = useMemo(() => {
    if (baseResources && baseResources.length > 0) {
      return baseResources.map((r: BaseResource) => ({
        value: r.name,
        label: r.title || r.name,
        description: r.description,
      }));
    }
    // Fallback to static list
    return fallbackBaseResources.map((name) => ({ value: name, label: name }));
  }, [baseResources]);

  const form = useForm<FormValues>({
    initialValues: {
      kind: 'profile',
      name: '',
      id: '',
      baseResource: 'Patient',
      description: '',
      context: '',
      purpose: '',
      content: '',
    },
    validate: {
      name: (value, values) => {
        // Name is optional if content is provided for extension/valueset
        if (values.kind !== 'profile' && values.content.trim()) {
          return null;
        }
        return !value.trim() ? 'Name is required' : null;
      },
      baseResource: (value, values) =>
        values.kind === 'profile' && !value ? 'Select base resource' : null,
      content: (value, values) => {
        if (values.kind !== 'profile' && value.trim()) {
          try {
            JSON.parse(value);
            return null;
          } catch {
            return 'Invalid JSON format';
          }
        }
        return null;
      },
    },
  });

  const inferredId = useMemo(
    () => form.values.id.trim() || slugify(form.values.name),
    [form.values]
  );

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

    // Try to extract name/id from content if provided
    let name = values.name.trim();
    let id = inferredId;
    if (!name && values.content.trim() && values.kind !== 'profile') {
      try {
        const parsed = JSON.parse(values.content);
        name = parsed.name || parsed.title || parsed.id || 'Imported Resource';
        id = parsed.id || slugify(name);
      } catch {
        // Already validated
      }
    }

    const payload = {
      kind: values.kind,
      name,
      id,
      baseResource: values.kind === 'profile' ? values.baseResource : undefined,
      description: values.description.trim() || undefined,
      context: values.kind === 'extension' ? values.context.trim() || undefined : undefined,
      purpose: values.kind === 'valueset' ? values.purpose.trim() || undefined : undefined,
      content:
        values.kind !== 'profile' && values.content.trim() ? values.content.trim() : undefined,
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
              data={baseResourceOptions}
              searchable
              placeholder={isLoadingResources ? 'Loading resources...' : 'Select base resource'}
              rightSection={isLoadingResources ? <Loader size="xs" /> : undefined}
              disabled={isLoadingResources}
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

          {form.values.kind !== 'profile' && (
            <>
              <Divider
                label={
                  <Text size="xs" c="dimmed">
                    Or paste existing resource JSON
                  </Text>
                }
                labelPosition="center"
              />
              <Textarea
                label="Resource JSON content"
                description="Paste a valid FHIR resource JSON. Name and ID will be extracted automatically if left empty above."
                placeholder={`{\n  "resourceType": "${form.values.kind === 'extension' ? 'StructureDefinition' : 'ValueSet'}",\n  "id": "my-${form.values.kind}",\n  "name": "My${form.values.kind === 'extension' ? 'Extension' : 'ValueSet'}",\n  ...\n}`}
                autosize
                minRows={6}
                maxRows={12}
                styles={{ input: { fontFamily: 'monospace', fontSize: 12 } }}
                {...form.getInputProps('content')}
              />
            </>
          )}

          <Text size="sm" c="dimmed">
            File path preview:{' '}
            <Text span fw={600}>
              {pathPreview}
            </Text>
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
