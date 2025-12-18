import {
  ActionIcon,
  Badge,
  Checkbox,
  Group,
  Select,
  Stack,
  Text,
  Textarea,
  TextInput,
  Tooltip,
} from '@mantine/core';
import type { FhirVersion } from '@shared/types';
import { IconInfoCircle, IconTrash } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { $configErrors, $projectConfig, configUpdated, dependencyRemoved } from '../model';

const FHIR_VERSIONS: { value: FhirVersion; label: string }[] = [
  { value: '4.0.1', label: 'FHIR R4 (4.0.1)' },
  { value: '4.3.0', label: 'FHIR R4B (4.3.0)' },
  { value: '5.0.0', label: 'FHIR R5 (5.0.0)' },
];

export function ProjectConfigForm() {
  const [config, errors] = useUnit([$projectConfig, $configErrors]);

  return (
    <Stack gap="md">
      {/* Project Name */}
      <TextInput
        label="Project Name"
        description="A human-readable name for your project"
        placeholder="My FHIR Profile"
        required
        value={config.name}
        onChange={(e) => configUpdated({ name: e.currentTarget.value })}
        error={errors.name}
      />

      {/* Canonical Base URL */}
      <TextInput
        label="Canonical Base URL"
        description="The base URL for all canonical URLs in this project"
        placeholder="http://example.org/fhir"
        required
        value={config.canonicalBase}
        onChange={(e) => configUpdated({ canonicalBase: e.currentTarget.value })}
        error={errors.canonicalBase}
        rightSection={
          <Tooltip
            label="This URL will be used as the base for all profile, extension, and ValueSet URLs"
            multiline
            w={250}
          >
            <IconInfoCircle size={16} style={{ opacity: 0.5 }} />
          </Tooltip>
        }
      />

      {/* FHIR Version */}
      <Select
        label="FHIR Version"
        description="The FHIR specification version"
        data={FHIR_VERSIONS}
        value={config.fhirVersion}
        onChange={(value) => value && configUpdated({ fhirVersion: value as FhirVersion })}
        required
      />

      {/* Package ID */}
      <TextInput
        label="Package ID"
        description="NPM-style package identifier (e.g., org.example.myig)"
        placeholder="org.example.myig"
        required
        value={config.packageId}
        onChange={(e) => configUpdated({ packageId: e.currentTarget.value })}
        error={errors.packageId}
      />

      {/* Version */}
      <TextInput
        label="Version"
        description="Semantic version number (e.g., 1.0.0)"
        placeholder="0.1.0"
        required
        value={config.version}
        onChange={(e) => configUpdated({ version: e.currentTarget.value })}
        error={errors.version}
      />

      {/* Description (Optional) */}
      <Textarea
        label="Description"
        description="Optional description of the project"
        placeholder="Describe your project..."
        value={config.description || ''}
        onChange={(e) => configUpdated({ description: e.currentTarget.value })}
        minRows={2}
      />

      {/* Publisher (Optional) */}
      <TextInput
        label="Publisher"
        description="Organization or individual publishing this project"
        placeholder="Your Organization"
        value={config.publisher || ''}
        onChange={(e) => configUpdated({ publisher: e.currentTarget.value })}
      />

      {/* Dependencies */}
      {config.dependencies.length > 0 && (
        <div>
          <Text size="sm" fw={500} mb="xs">
            Dependencies
          </Text>
          <Stack gap="xs">
            {config.dependencies.map((dep) => (
              <Group key={dep.packageId} justify="space-between" p="xs" bg="gray.0">
                <div>
                  <Group gap="xs">
                    <Text size="sm" fw={500}>
                      {dep.name}
                    </Text>
                    <Badge size="xs" variant="light">
                      {dep.version}
                    </Badge>
                  </Group>
                  <Text size="xs" c="dimmed">
                    {dep.packageId}
                  </Text>
                </div>
                <Tooltip label="Remove dependency">
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    size="sm"
                    onClick={() => dependencyRemoved(dep.packageId)}
                  >
                    <IconTrash size={14} />
                  </ActionIcon>
                </Tooltip>
              </Group>
            ))}
          </Stack>
        </div>
      )}

      {/* Git Option */}
      <Checkbox
        label="Initialize Git repository"
        description="Create a Git repository with initial commit"
        checked={config.initGit}
        onChange={(e) => configUpdated({ initGit: e.currentTarget.checked })}
      />
    </Stack>
  );
}
