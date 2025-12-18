import { Badge, Button, Group, Stack, Text, Title } from '@mantine/core';
import { IconCheck, IconSparkles } from '@tabler/icons-react';
import { useState } from 'react';
import { SLICING_TEMPLATES } from '../lib/templates';
import { templateApplied } from '../model';

interface TemplateSelectorProps {
  onSkip: () => void;
}

export function TemplateSelector({ onSkip }: TemplateSelectorProps) {
  const [selectedTemplate, setSelectedTemplate] = useState<string | null>(null);

  const handleApplyTemplate = () => {
    if (selectedTemplate) {
      templateApplied(selectedTemplate);
      onSkip(); // Move to next step
    }
  };

  return (
    <Stack gap="lg">
      <div>
        <Group gap="xs" mb="xs">
          <IconSparkles size={20} style={{ color: 'var(--mantine-color-yellow-6)' }} />
          <Title order={4}>Quick Start with Template</Title>
        </Group>
        <Text size="sm" c="dimmed">
          Choose a common slicing pattern to auto-configure discriminators, rules, and suggested
          slices. You can customize everything in the next steps.
        </Text>
      </div>

      {/* Template Cards */}
      <Stack gap="sm">
        {SLICING_TEMPLATES.map((template) => (
          <div
            key={template.id}
            onClick={() => setSelectedTemplate(template.id)}
            style={{
              padding: '16px',
              background:
                selectedTemplate === template.id
                  ? 'var(--mantine-color-blue-0)'
                  : 'var(--mantine-color-gray-0)',
              border: `2px solid ${
                selectedTemplate === template.id
                  ? 'var(--mantine-color-blue-6)'
                  : 'var(--mantine-color-gray-3)'
              }`,
              borderRadius: 'var(--mantine-radius-md)',
              cursor: 'pointer',
              transition: 'all 0.2s',
            }}
            onMouseEnter={(e) => {
              if (selectedTemplate !== template.id) {
                e.currentTarget.style.background = 'var(--mantine-color-gray-1)';
              }
            }}
            onMouseLeave={(e) => {
              if (selectedTemplate !== template.id) {
                e.currentTarget.style.background = 'var(--mantine-color-gray-0)';
              }
            }}
          >
            <Group justify="space-between" mb="xs">
              <Group gap="xs">
                <Text size="sm" fw={600}>
                  {template.name}
                </Text>
                {selectedTemplate === template.id && (
                  <IconCheck size={16} style={{ color: 'var(--mantine-color-blue-6)' }} />
                )}
              </Group>
              <Group gap="xs">
                <Badge size="sm" variant="light">
                  {template.discriminators.length} discriminator
                  {template.discriminators.length !== 1 ? 's' : ''}
                </Badge>
                <Badge size="sm" variant="light" color="green">
                  {template.suggestedSlices?.length || 0} slice
                  {template.suggestedSlices?.length !== 1 ? 's' : ''}
                </Badge>
              </Group>
            </Group>

            <Text size="xs" c="dimmed" mb="sm">
              {template.description}
            </Text>

            <Group gap="xs">
              {template.discriminators.map((disc, index) => (
                <Badge key={index} size="xs" variant="outline">
                  {disc.type}: {disc.path}
                </Badge>
              ))}
            </Group>
          </div>
        ))}
      </Stack>

      {/* Actions */}
      <Group justify="space-between">
        <Button variant="subtle" onClick={onSkip}>
          Skip - Configure Manually
        </Button>
        <Button
          leftSection={<IconSparkles size={16} />}
          onClick={handleApplyTemplate}
          disabled={!selectedTemplate}
        >
          Use Template
        </Button>
      </Group>
    </Stack>
  );
}
