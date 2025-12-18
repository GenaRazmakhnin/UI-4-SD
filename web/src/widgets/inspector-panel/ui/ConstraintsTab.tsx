import { CardinalityEditor } from '@features/cardinality-editor';
import { FlagsEditor } from '@features/flags-editor';
import { TypeConstraintEditor } from '@features/type-constraint-editor';
import { Divider, Stack, Textarea, Title } from '@mantine/core';
import type { ElementNode } from '@shared/types';

interface ConstraintsTabProps {
  element: ElementNode;
}

export function ConstraintsTab({ element }: ConstraintsTabProps) {
  return (
    <Stack gap="lg">
      {/* Cardinality Section */}
      <section>
        <Title order={6} mb="sm">
          Cardinality
        </Title>
        <CardinalityEditor element={element} />
      </section>

      <Divider />

      {/* Type Constraints Section */}
      <section>
        <Title order={6} mb="sm">
          Type Constraints
        </Title>
        <TypeConstraintEditor element={element} />
      </section>

      <Divider />

      {/* Flags Section */}
      <section>
        <Title order={6} mb="sm">
          Flags
        </Title>
        <FlagsEditor element={element} />
      </section>

      <Divider />

      {/* Documentation Section */}
      <section>
        <Title order={6} mb="sm">
          Documentation
        </Title>
        <Stack gap="sm">
          <Textarea
            label="Short Description"
            value={element.short || ''}
            maxLength={254}
            placeholder="Brief description..."
            disabled
          />

          <Textarea
            label="Definition"
            value={element.definition || ''}
            rows={4}
            placeholder="Detailed definition..."
            disabled
          />

          <Textarea
            label="Comment"
            value={element.comment || ''}
            rows={3}
            placeholder="Additional comments..."
            disabled
          />
        </Stack>
      </section>
    </Stack>
  );
}
