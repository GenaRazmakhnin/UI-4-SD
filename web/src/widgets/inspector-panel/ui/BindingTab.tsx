import { Stack, Title, Button, Alert, Text } from '@mantine/core';
import { IconAlertCircle, IconSearch } from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';
import { BindingEditor } from '@features/binding-editor';

interface BindingTabProps {
  element: ElementNode;
}

export function BindingTab({ element }: BindingTabProps) {
  const canBind = canHaveBinding(element);

  if (!canBind) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="gray">
        This element type cannot have terminology bindings. Only code, Coding,
        CodeableConcept, Quantity, and string elements can be bound to
        ValueSets.
      </Alert>
    );
  }

  if (!element.binding) {
    return (
      <Stack gap="md">
        <Text size="sm" c="dimmed">
          No binding configured for this element.
        </Text>
        <Button
          leftSection={<IconSearch size={16} />}
          disabled
        >
          Add Binding
        </Button>
      </Stack>
    );
  }

  return (
    <Stack gap="lg">
      {/* Binding Configuration */}
      <section>
        <Title order={6} mb="sm">
          Binding Configuration
        </Title>
        <BindingEditor element={element} />
      </section>

      {/* ValueSet Details */}
      {element.binding.valueSet && (
        <section>
          <Title order={6} mb="sm">
            ValueSet Details
          </Title>
          <Stack gap="xs">
            <Text size="sm" fw={500}>
              {element.binding.valueSet}
            </Text>
            {element.binding.description && (
              <Text size="xs" c="dimmed">
                {element.binding.description}
              </Text>
            )}
          </Stack>
        </section>
      )}

      {/* Binding Strength Info */}
      <Alert color="blue" variant="light">
        <Text size="xs">
          <strong>{element.binding.strength}:</strong>{' '}
          {getBindingStrengthDescription(element.binding.strength)}
        </Text>
      </Alert>
    </Stack>
  );
}

/**
 * Check if element can have a terminology binding
 */
function canHaveBinding(element: ElementNode): boolean {
  if (!element.type || element.type.length === 0) {
    return false;
  }

  const bindableTypes = [
    'code',
    'Coding',
    'CodeableConcept',
    'Quantity',
    'string',
    'uri',
  ];
  return element.type.some((t) => bindableTypes.includes(t.code));
}

/**
 * Get description for binding strength
 */
function getBindingStrengthDescription(strength: string): string {
  switch (strength) {
    case 'required':
      return 'To be conformant, the concept in this element SHALL be from the specified value set.';
    case 'extensible':
      return 'To be conformant, the concept in this element SHALL be from the specified value set if any of the codes within the value set can apply to the concept being communicated.';
    case 'preferred':
      return 'Instances are encouraged to draw from the specified codes for interoperability purposes but are not required to do so.';
    case 'example':
      return 'Instances are not expected or even encouraged to draw from the specified value set.';
    default:
      return '';
  }
}
