import { CardinalityEditor } from '@features/cardinality-editor';
import { FlagsEditor } from '@features/flags-editor';
import { TypeConstraintEditor } from '@features/type-constraint-editor';
import { Divider, Stack, Textarea, Title } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import { elementFieldChanged } from '@widgets/element-tree';
import { profileChanged } from '@pages/editor/model';
import { useState, useCallback } from 'react';

interface ConstraintsTabProps {
  element: ElementNode;
}

export function ConstraintsTab({ element }: ConstraintsTabProps) {
  const [short, setShort] = useState(element.short || '');
  const [definition, setDefinition] = useState(element.definition || '');
  const [comment, setComment] = useState(element.comment || '');

  const handleFieldChange = useCallback(
    (field: string, value: string) => {
      elementFieldChanged({
        elementPath: element.path,
        field,
        value,
      });
      profileChanged();
    },
    [element.path]
  );

  const handleShortChange = (value: string) => {
    setShort(value);
    handleFieldChange('short', value);
  };

  const handleDefinitionChange = (value: string) => {
    setDefinition(value);
    handleFieldChange('definition', value);
  };

  const handleCommentChange = (value: string) => {
    setComment(value);
    handleFieldChange('comment', value);
  };

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
            value={short}
            maxLength={254}
            placeholder="Brief description..."
            onChange={(e) => handleShortChange(e.currentTarget.value)}
          />

          <Textarea
            label="Definition"
            value={definition}
            rows={4}
            placeholder="Detailed definition..."
            onChange={(e) => handleDefinitionChange(e.currentTarget.value)}
          />

          <Textarea
            label="Comment"
            value={comment}
            rows={3}
            placeholder="Additional comments..."
            onChange={(e) => handleCommentChange(e.currentTarget.value)}
          />
        </Stack>
      </section>
    </Stack>
  );
}
