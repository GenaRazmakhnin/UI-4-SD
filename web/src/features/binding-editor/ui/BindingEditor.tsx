import { Stack, Select, TextInput, Alert, Text } from '@mantine/core';
import { IconInfoCircle } from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';

interface BindingEditorProps {
  element: ElementNode;
}

export function BindingEditor({ element }: BindingEditorProps) {
  return (
    <div>
      <Alert icon={<IconInfoCircle size={16} />} color="blue" variant="light" mb="md">
        <Text size="xs">
          Placeholder: Binding editor will be implemented in task UI-09
        </Text>
      </Alert>

      <Stack gap="sm">
        <Select
          label="Strength"
          value={element.binding?.strength}
          data={['required', 'extensible', 'preferred', 'example']}
          disabled
        />
        <TextInput
          label="ValueSet"
          value={element.binding?.valueSet || ''}
          disabled
        />
      </Stack>
    </div>
  );
}
