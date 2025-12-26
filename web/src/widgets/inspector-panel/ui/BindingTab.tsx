import { BindingEditor } from '@features/binding-editor';
import { Alert, Stack, Text, Title } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import { IconAlertCircle } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { $browserOpen, browserClosed, bindingChanged } from '@features/binding-editor';
import { ValueSetBrowser } from '@features/binding-editor';

interface BindingTabProps {
  element: ElementNode;
}

export function BindingTab({ element }: BindingTabProps) {
  const canBind = canHaveBinding(element);

  if (!canBind) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="gray">
        This element type cannot have terminology bindings. Only code, Coding, CodeableConcept,
        Quantity, and string elements can be bound to ValueSets.
      </Alert>
    );
  }

  return (
    <Stack gap="lg">
      {/* Binding Configuration */}
      <section>
        <Title order={6} mb="sm">
          Binding Configuration
        </Title>
        {!element.binding && (
          <Text size="sm" c="dimmed" mb="md">
            No binding configured. Add a terminology binding to constrain the allowed values for
            this element.
          </Text>
        )}
        <BindingEditor element={element} />
      </section>

      {/* ValueSet Browser Modal */}
      <ValueSetBrowser
        opened={useUnit($browserOpen)}
        onClose={() => browserClosed()}
        onValueSetSelected={(url) => {
          bindingChanged({
            elementId: element.id,
            binding: {
              strength: 'required', // Default to required
              valueSet: url,
            },
          });
          browserClosed();
        }}
      />
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

  const bindableTypes = ['code', 'Coding', 'CodeableConcept', 'Quantity', 'string', 'uri'];
  return element.type.some((t) => bindableTypes.includes(t.code));
}
