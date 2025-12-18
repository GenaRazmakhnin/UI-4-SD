import { Alert, Button, Divider, Group, Stack, Text, TextInput, Title } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import { IconInfoCircle, IconPlus } from '@tabler/icons-react';

interface MetadataTabProps {
  element: ElementNode;
}

export function MetadataTab({ element }: MetadataTabProps) {
  return (
    <Stack gap="lg">
      <Alert icon={<IconInfoCircle size={16} />} color="blue" variant="light">
        <Text size="xs">
          Metadata features (aliases, mappings, constraints, examples) will be fully implemented in
          future tasks.
        </Text>
      </Alert>

      {/* Aliases */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Aliases</Title>
          <Button size="xs" variant="light" leftSection={<IconPlus size={14} />} disabled>
            Add Alias
          </Button>
        </Group>

        <TextInput placeholder="No aliases defined" disabled />
      </section>

      <Divider />

      {/* Mappings */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Mappings</Title>
          <Button size="xs" variant="light" leftSection={<IconPlus size={14} />} disabled>
            Add Mapping
          </Button>
        </Group>

        <TextInput placeholder="No mappings defined" disabled />
      </section>

      <Divider />

      {/* Constraints */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Constraints</Title>
          <Button size="xs" variant="light" leftSection={<IconPlus size={14} />} disabled>
            Add Constraint
          </Button>
        </Group>

        <TextInput placeholder="No constraints defined" disabled />
      </section>

      <Divider />

      {/* Examples */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Examples</Title>
          <Button size="xs" variant="light" leftSection={<IconPlus size={14} />} disabled>
            Add Example
          </Button>
        </Group>

        <TextInput placeholder="No examples defined" disabled />
      </section>
    </Stack>
  );
}
