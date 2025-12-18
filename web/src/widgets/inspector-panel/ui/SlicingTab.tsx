import { SlicingWizard } from '@features/slicing-wizard';
import { Alert, Badge, Button, Group, Stack, Text, Title } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import { IconAlertCircle, IconPlus } from '@tabler/icons-react';
import { useState } from 'react';

interface SlicingTabProps {
  element: ElementNode;
}

export function SlicingTab({ element }: SlicingTabProps) {
  const [wizardOpen, setWizardOpen] = useState(false);
  const canSlice = element.max === '*' || Number(element.max) > 1;

  if (!canSlice) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="gray">
        This element cannot be sliced because its maximum cardinality is 1. Only elements with max
        &gt; 1 or max = * can be sliced.
      </Alert>
    );
  }

  const hasSlicing = !!element.slicing;
  const slices = element.children.filter((c) => c.sliceName);

  return (
    <Stack gap="lg">
      {/* Slicing Status */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Slicing Status</Title>
          {hasSlicing ? (
            <Badge color="green">Sliced</Badge>
          ) : (
            <Badge color="gray">Not Sliced</Badge>
          )}
        </Group>

        {!hasSlicing ? (
          <Button leftSection={<IconPlus size={16} />} onClick={() => setWizardOpen(true)}>
            Create Slicing
          </Button>
        ) : (
          <Stack gap="sm">
            {/* Slicing Rules */}
            <div>
              <Text size="sm" fw={500} mb={4}>
                Discriminator
              </Text>
              <Stack gap={4}>
                {element.slicing.discriminator.map((d, i) => (
                  <Text key={i} size="xs" c="dimmed">
                    {d.type} @ {d.path}
                  </Text>
                ))}
              </Stack>
            </div>

            <Group gap="lg">
              <div>
                <Text size="sm" fw={500}>
                  Rules
                </Text>
                <Badge size="sm" variant="light">
                  {element.slicing.rules}
                </Badge>
              </div>

              <div>
                <Text size="sm" fw={500}>
                  Ordered
                </Text>
                <Badge size="sm" variant="light" color={element.slicing.ordered ? 'blue' : 'gray'}>
                  {element.slicing.ordered ? 'Yes' : 'No'}
                </Badge>
              </div>
            </Group>

            {element.slicing.description && (
              <div>
                <Text size="sm" fw={500} mb={4}>
                  Description
                </Text>
                <Text size="xs" c="dimmed">
                  {element.slicing.description}
                </Text>
              </div>
            )}
          </Stack>
        )}
      </section>

      {/* Slices List */}
      {hasSlicing && slices.length > 0 && (
        <section>
          <Group justify="space-between" mb="sm">
            <Title order={6}>Slices ({slices.length})</Title>
            <Button size="xs" variant="light" leftSection={<IconPlus size={14} />} disabled>
              Add Slice
            </Button>
          </Group>

          <Stack gap="xs">
            {slices.map((slice) => (
              <div
                key={slice.id}
                style={{
                  padding: '8px',
                  background: 'var(--mantine-color-gray-0)',
                  borderRadius: '4px',
                }}
              >
                <Text size="sm" fw={500}>
                  {slice.sliceName}
                </Text>
                <Text size="xs" c="dimmed">
                  {slice.min}..{slice.max}
                </Text>
              </div>
            ))}
          </Stack>
        </section>
      )}

      {/* Slicing Wizard */}
      <SlicingWizard element={element} opened={wizardOpen} onClose={() => setWizardOpen(false)} />
    </Stack>
  );
}
