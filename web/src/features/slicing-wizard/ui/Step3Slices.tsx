import {
  ActionIcon,
  Alert,
  Badge,
  Button,
  Group,
  NumberInput,
  Select,
  Stack,
  Text,
  TextInput,
  Title,
} from '@mantine/core';
import { IconAlertCircle, IconEdit, IconPlus, IconTrash } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState } from 'react';
import { $wizardState, sliceAdded, sliceRemoved, sliceUpdated } from '../model';

export function Step3Slices() {
  const wizardState = useUnit($wizardState);
  const [sliceName, setSliceName] = useState('');
  const [sliceMin, setSliceMin] = useState<number>(0);
  const [sliceMax, setSliceMax] = useState<string>('1');
  const [sliceDescription, setSliceDescription] = useState('');
  const [editingIndex, setEditingIndex] = useState<number | null>(null);
  const [nameError, setNameError] = useState<string | null>(null);

  const validateSliceName = (name: string): boolean => {
    if (!name || !/^[a-zA-Z][a-zA-Z0-9]*$/.test(name)) {
      setNameError('Slice name must start with a letter and contain only letters and numbers');
      return false;
    }

    // Check for duplicates (excluding current edit)
    const isDuplicate = wizardState.slices.some(
      (s, index) => s.name === name && index !== editingIndex
    );
    if (isDuplicate) {
      setNameError('Slice name must be unique');
      return false;
    }

    setNameError(null);
    return true;
  };

  const handleAddSlice = () => {
    if (!validateSliceName(sliceName)) {
      return;
    }

    if (editingIndex !== null) {
      // Update existing slice
      sliceUpdated({
        index: editingIndex,
        updates: {
          name: sliceName,
          min: sliceMin,
          max: sliceMax,
          description: sliceDescription || undefined,
        },
      });
      setEditingIndex(null);
    } else {
      // Add new slice
      sliceAdded({
        name: sliceName,
        min: sliceMin,
        max: sliceMax,
        description: sliceDescription || undefined,
      });
    }

    // Reset form
    setSliceName('');
    setSliceMin(0);
    setSliceMax('1');
    setSliceDescription('');
    setNameError(null);
  };

  const handleEditSlice = (index: number) => {
    const slice = wizardState.slices[index];
    setSliceName(slice.name);
    setSliceMin(slice.min);
    setSliceMax(slice.max);
    setSliceDescription(slice.description || '');
    setEditingIndex(index);
    setNameError(null);
  };

  const handleCancelEdit = () => {
    setSliceName('');
    setSliceMin(0);
    setSliceMax('1');
    setSliceDescription('');
    setEditingIndex(null);
    setNameError(null);
  };

  const handleRemoveSlice = (index: number) => {
    sliceRemoved(index);
    if (editingIndex === index) {
      handleCancelEdit();
    }
  };

  return (
    <Stack gap="lg">
      <div>
        <Title order={4} mb="xs">
          Define Slices
        </Title>
        <Text size="sm" c="dimmed">
          Create named slices with their cardinality constraints. Each slice represents a specific
          variant of the element being sliced.
        </Text>
      </div>

      {/* Existing Slices */}
      {wizardState.slices.length > 0 && (
        <div>
          <Text size="sm" fw={500} mb="xs">
            Defined Slices ({wizardState.slices.length})
          </Text>
          <Stack gap="xs">
            {wizardState.slices.map((slice, index) => (
              <Group
                key={index}
                justify="space-between"
                p="sm"
                style={{
                  background:
                    editingIndex === index
                      ? 'var(--mantine-color-yellow-0)'
                      : 'var(--mantine-color-gray-0)',
                  borderRadius: 'var(--mantine-radius-sm)',
                  border: `1px solid ${
                    editingIndex === index
                      ? 'var(--mantine-color-yellow-3)'
                      : 'var(--mantine-color-gray-3)'
                  }`,
                }}
              >
                <div style={{ flex: 1 }}>
                  <Group gap="xs" mb={4}>
                    <Text size="sm" fw={600}>
                      {slice.name}
                    </Text>
                    <Badge size="sm" variant="light">
                      {slice.min}..{slice.max}
                    </Badge>
                    {editingIndex === index && (
                      <Badge size="sm" color="yellow">
                        Editing
                      </Badge>
                    )}
                  </Group>
                  {slice.description && (
                    <Text size="xs" c="dimmed">
                      {slice.description}
                    </Text>
                  )}
                </div>
                <Group gap={4}>
                  <ActionIcon
                    variant="subtle"
                    color="blue"
                    onClick={() => handleEditSlice(index)}
                    disabled={editingIndex !== null && editingIndex !== index}
                    aria-label="Edit slice"
                  >
                    <IconEdit size={16} />
                  </ActionIcon>
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    onClick={() => handleRemoveSlice(index)}
                    aria-label="Remove slice"
                  >
                    <IconTrash size={16} />
                  </ActionIcon>
                </Group>
              </Group>
            ))}
          </Stack>
        </div>
      )}

      {/* Add/Edit Slice Form */}
      <div>
        <Text size="sm" fw={500} mb="sm">
          {editingIndex !== null ? 'Edit Slice' : 'Add New Slice'}
        </Text>

        <Stack gap="md">
          {/* Slice Name */}
          <TextInput
            label="Slice Name"
            description="Must start with a letter and contain only letters and numbers"
            placeholder="e.g., mySlice, identifier1"
            value={sliceName}
            onChange={(e) => {
              setSliceName(e.currentTarget.value);
              setNameError(null);
            }}
            error={nameError}
            required
          />

          {/* Cardinality */}
          <Group grow align="flex-start">
            <NumberInput
              label="Minimum"
              description="Minimum occurrences"
              value={sliceMin}
              onChange={(value) => setSliceMin(Number(value) || 0)}
              min={0}
              required
            />
            <Select
              label="Maximum"
              description="Maximum occurrences"
              data={[
                { value: '0', label: '0' },
                { value: '1', label: '1' },
                { value: '2', label: '2' },
                { value: '3', label: '3' },
                { value: '5', label: '5' },
                { value: '10', label: '10' },
                { value: '*', label: '* (unbounded)' },
              ]}
              value={sliceMax}
              onChange={(value) => value && setSliceMax(value)}
              required
            />
          </Group>

          {/* Slice Description */}
          <TextInput
            label="Description (Optional)"
            description="Brief explanation of what this slice represents"
            placeholder="e.g., Medical Record Number identifier"
            value={sliceDescription}
            onChange={(e) => setSliceDescription(e.currentTarget.value)}
          />

          {/* Action Buttons */}
          <Group>
            <Button
              leftSection={editingIndex !== null ? <IconEdit size={16} /> : <IconPlus size={16} />}
              onClick={handleAddSlice}
              disabled={!sliceName}
            >
              {editingIndex !== null ? 'Update Slice' : 'Add Slice'}
            </Button>
            {editingIndex !== null && (
              <Button variant="subtle" onClick={handleCancelEdit}>
                Cancel Edit
              </Button>
            )}
          </Group>
        </Stack>
      </div>

      {/* Validation Warning */}
      {wizardState.slices.length === 0 && (
        <Alert icon={<IconAlertCircle size={16} />} color="yellow">
          <Text size="sm">
            At least one slice is required to proceed. Slices are the named variants that your
            discriminator rules will identify.
          </Text>
        </Alert>
      )}

      {/* Help Text */}
      <Alert color="blue" variant="light">
        <Text size="xs">
          <strong>Tip:</strong> Give slices meaningful names that describe their purpose. For
          example, when slicing identifiers by system, use names like "mrn", "ssn", or
          "driverLicense".
        </Text>
      </Alert>
    </Stack>
  );
}
