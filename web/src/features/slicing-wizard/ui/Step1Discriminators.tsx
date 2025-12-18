import {
  ActionIcon,
  Alert,
  Badge,
  Button,
  Group,
  Select,
  Stack,
  Text,
  TextInput,
  Title,
} from '@mantine/core';
import type { SlicingDiscriminator } from '@shared/types';
import { IconAlertCircle, IconPlus, IconTrash } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState } from 'react';
import {
  getCommonDiscriminatorPaths,
  getDiscriminatorTypeDescription,
  validateDiscriminatorPath,
} from '../lib/templates';
import { $wizardState, discriminatorAdded, discriminatorRemoved } from '../model';

const DISCRIMINATOR_TYPES = [
  { value: 'value', label: 'Value' },
  { value: 'exists', label: 'Exists' },
  { value: 'pattern', label: 'Pattern' },
  { value: 'type', label: 'Type' },
  { value: 'profile', label: 'Profile' },
];

export function Step1Discriminators() {
  const wizardState = useUnit($wizardState);
  const [selectedType, setSelectedType] = useState<SlicingDiscriminator['type']>('value');
  const [pathInput, setPathInput] = useState('');
  const [pathError, setPathError] = useState<string | null>(null);

  // Get common paths for suggestions
  const commonPaths = getCommonDiscriminatorPaths(wizardState.elementPath);

  const handleAddDiscriminator = () => {
    // Validate path
    const validation = validateDiscriminatorPath(pathInput, selectedType);
    if (!validation.valid) {
      setPathError(validation.error || 'Invalid path');
      return;
    }

    // Add discriminator
    discriminatorAdded({
      type: selectedType,
      path: pathInput,
    });

    // Reset form
    setPathInput('');
    setPathError(null);
  };

  const handlePathSelect = (path: string) => {
    setPathInput(path);
    setPathError(null);
  };

  const handleRemove = (index: number) => {
    discriminatorRemoved(index);
  };

  return (
    <Stack gap="lg">
      <div>
        <Title order={4} mb="xs">
          Configure Discriminators
        </Title>
        <Text size="sm" c="dimmed">
          Discriminators define how slices are distinguished from each other. Choose the element
          path and type that will differentiate your slices.
        </Text>
      </div>

      {/* Existing Discriminators */}
      {wizardState.discriminators.length > 0 && (
        <div>
          <Text size="sm" fw={500} mb="xs">
            Current Discriminators ({wizardState.discriminators.length})
          </Text>
          <Stack gap="xs">
            {wizardState.discriminators.map((disc, index) => (
              <Group
                key={index}
                justify="space-between"
                p="sm"
                style={{
                  background: 'var(--mantine-color-blue-0)',
                  borderRadius: 'var(--mantine-radius-sm)',
                  border: '1px solid var(--mantine-color-blue-2)',
                }}
              >
                <div>
                  <Group gap="xs" mb={4}>
                    <Badge size="sm" variant="light">
                      {disc.type}
                    </Badge>
                    <Text size="sm" fw={500} style={{ fontFamily: 'monospace' }}>
                      {disc.path}
                    </Text>
                  </Group>
                  <Text size="xs" c="dimmed">
                    {getDiscriminatorTypeDescription(disc.type)}
                  </Text>
                </div>
                <ActionIcon
                  color="red"
                  variant="subtle"
                  onClick={() => handleRemove(index)}
                  aria-label="Remove discriminator"
                >
                  <IconTrash size={16} />
                </ActionIcon>
              </Group>
            ))}
          </Stack>
        </div>
      )}

      {/* Add New Discriminator */}
      <div>
        <Text size="sm" fw={500} mb="sm">
          Add Discriminator
        </Text>

        <Stack gap="md">
          {/* Discriminator Type */}
          <Select
            label="Discriminator Type"
            description={
              selectedType ? getDiscriminatorTypeDescription(selectedType) : 'Select a type'
            }
            data={DISCRIMINATOR_TYPES}
            value={selectedType}
            onChange={(value) => value && setSelectedType(value as SlicingDiscriminator['type'])}
            required
          />

          {/* Path Input */}
          <TextInput
            label="Discriminator Path"
            description="FHIRPath expression pointing to the discriminating element"
            placeholder="e.g., url, system, code"
            value={pathInput}
            onChange={(e) => {
              setPathInput(e.currentTarget.value);
              setPathError(null);
            }}
            error={pathError}
            required
          />

          {/* Common Paths Suggestions */}
          {commonPaths.length > 0 && !pathInput && (
            <div>
              <Text size="xs" c="dimmed" mb="xs">
                Common paths for this element:
              </Text>
              <Group gap="xs">
                {commonPaths.map((path) => (
                  <Button
                    key={path}
                    size="xs"
                    variant="light"
                    onClick={() => handlePathSelect(path)}
                  >
                    {path}
                  </Button>
                ))}
              </Group>
            </div>
          )}

          {/* Special Path Helper */}
          {(selectedType === 'type' || selectedType === 'profile') && (
            <Alert icon={<IconAlertCircle size={16} />} color="blue">
              <Text size="xs">
                For {selectedType} discriminators, you can use <code>$this</code> to refer to the
                element itself.
              </Text>
            </Alert>
          )}

          {/* Add Button */}
          <Button
            leftSection={<IconPlus size={16} />}
            onClick={handleAddDiscriminator}
            disabled={!pathInput || !selectedType}
          >
            Add Discriminator
          </Button>
        </Stack>
      </div>

      {/* Validation Warning */}
      {wizardState.discriminators.length === 0 && (
        <Alert icon={<IconAlertCircle size={16} />} color="yellow">
          <Text size="sm">
            At least one discriminator is required to proceed. Discriminators tell FHIR how to
            distinguish between different slices.
          </Text>
        </Alert>
      )}

      {/* Help Text */}
      <Alert color="blue" variant="light">
        <Text size="xs">
          <strong>Tip:</strong> Most slicing uses a single discriminator, but you can add multiple
          discriminators for complex scenarios. They will be evaluated in order.
        </Text>
      </Alert>
    </Stack>
  );
}
