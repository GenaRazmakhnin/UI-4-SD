import { ExtensionPicker } from '@features/extension-picker';
import { Alert, Badge, Button, Group, Stack, Text, Title } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import { IconInfoCircle, IconPlus } from '@tabler/icons-react';
import { useState } from 'react';

interface ExtensionsTabProps {
  element: ElementNode;
}

export function ExtensionsTab({ element }: ExtensionsTabProps) {
  const [pickerOpen, setPickerOpen] = useState(false);

  // TODO: Get actual extensions from element
  // For now, showing placeholder
  const hasExtensions = false;
  const extensions: any[] = [];

  return (
    <Stack gap="lg">
      {/* Info alert */}
      <Alert variant="light" color="blue" icon={<IconInfoCircle size={16} />}>
        <Text size="sm">
          Extensions allow you to add additional data elements beyond what is defined in the base
          FHIR specification. They are a key mechanism for adapting FHIR to specific use cases.
        </Text>
      </Alert>

      {/* Add Extension Button */}
      <Group>
        <Button leftSection={<IconPlus size={16} />} onClick={() => setPickerOpen(true)}>
          Add Extension
        </Button>
      </Group>

      {/* Extensions List */}
      {hasExtensions ? (
        <section>
          <Title order={6} mb="sm">
            Current Extensions
          </Title>
          <Stack gap="sm">
            {extensions.map((ext: any, index: number) => (
              <Group key={index} p="sm" style={{ border: '1px solid #e0e0e0', borderRadius: 4 }}>
                <Badge>{ext.url}</Badge>
                <Text size="sm">{ext.description}</Text>
              </Group>
            ))}
          </Stack>
        </section>
      ) : (
        <Alert variant="light" color="gray">
          <Text size="sm" c="dimmed">
            No extensions have been added to this element yet. Click "Add Extension" to get started.
          </Text>
        </Alert>
      )}

      {/* Extension Picker Modal */}
      <ExtensionPicker opened={pickerOpen} onClose={() => setPickerOpen(false)} element={element} />
    </Stack>
  );
}
