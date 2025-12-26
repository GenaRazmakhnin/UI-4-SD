import { Alert, Badge, Code, Divider, Group, Stack, Text, Title } from '@mantine/core';
import { IconCheck, IconInfoCircle } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { getDiscriminatorTypeDescription, getSlicingRulesDescription } from '../lib/templates';
import { $wizardState } from '../model';

export function Step4Review() {
  const wizardState = useUnit($wizardState);

  return (
    <Stack gap="lg">
      <div>
        <Title order={4} mb="xs">
          Review Slicing Configuration
        </Title>
        <Text size="sm" c="dimmed">
          Review your slicing configuration before applying. This will modify the structure
          definition.
        </Text>
      </div>

      {/* Element Path */}
      <div>
        <Text size="sm" fw={500} mb="xs">
          Target Element
        </Text>
        <Code block>{wizardState.elementPath}</Code>
      </div>

      {/* Discriminators */}
      <div>
        <Group justify="space-between" mb="xs">
          <Text size="sm" fw={500}>
            Discriminators
          </Text>
          <Badge size="sm" variant="light">
            {wizardState.discriminators.length}
          </Badge>
        </Group>
        <Stack gap="sm">
          {wizardState.discriminators.map((disc, index) => (
            <div
              key={index}
              style={{
                padding: '12px',
                background: 'var(--mantine-color-gray-0)',
                borderRadius: 'var(--mantine-radius-sm)',
                borderLeft: '3px solid var(--mantine-color-blue-6)',
              }}
            >
              <Group gap="xs" mb={4}>
                <Badge size="sm">{disc.type}</Badge>
                <Code>{disc.path}</Code>
              </Group>
              <Text size="xs" c="dimmed">
                {getDiscriminatorTypeDescription(disc.type)}
              </Text>
            </div>
          ))}
        </Stack>
      </div>

      <Divider />

      {/* Slicing Rules */}
      <div>
        <Text size="sm" fw={500} mb="sm">
          Slicing Rules
        </Text>
        <Stack gap="sm">
          <Group>
            <Text size="sm" c="dimmed" style={{ width: '100px' }}>
              Rules:
            </Text>
            <Badge variant="light">{wizardState.rules}</Badge>
            <Text size="xs" c="dimmed">
              {getSlicingRulesDescription(wizardState.rules)}
            </Text>
          </Group>
          <Group>
            <Text size="sm" c="dimmed" style={{ width: '100px' }}>
              Ordered:
            </Text>
            <Badge variant="light" color={wizardState.ordered ? 'blue' : 'gray'}>
              {wizardState.ordered ? 'Yes' : 'No'}
            </Badge>
            {wizardState.ordered && (
              <Text size="xs" c="dimmed">
                Slices must appear in the specified order
              </Text>
            )}
          </Group>
          {wizardState.description && (
            <div>
              <Text size="sm" c="dimmed" mb={4}>
                Description:
              </Text>
              <Text size="sm" style={{ fontStyle: 'italic' }}>
                "{wizardState.description}"
              </Text>
            </div>
          )}
        </Stack>
      </div>

      <Divider />

      {/* Slices */}
      <div>
        <Group justify="space-between" mb="xs">
          <Text size="sm" fw={500}>
            Slices to Create
          </Text>
          <Badge size="sm" variant="light">
            {wizardState.slices.length}
          </Badge>
        </Group>
        <Stack gap="xs">
          {wizardState.slices.map((slice, index) => (
            <div
              key={index}
              style={{
                padding: '12px',
                background: 'var(--mantine-color-green-0)',
                borderRadius: 'var(--mantine-radius-md)',
                border: '1px solid var(--mantine-color-green-2)',
              }}
            >
              <Group justify="space-between" mb={4}>
                <Group gap="xs">
                  <Text size="sm" fw={700} c="green.9">
                    {slice.name}
                  </Text>
                  <Badge size="sm" variant="filled" color="green">
                    {slice.min}..{slice.max}
                  </Badge>
                </Group>
              </Group>
              {slice.description && (
                <Text size="xs" c="dimmed">
                  {slice.description}
                </Text>
              )}
            </div>
          ))}
        </Stack>
      </div>

      {/* Impact Summary */}
      <Alert icon={<IconCheck size={16} />} color="green">
        <Text size="sm" fw={500} mb="xs">
          What will happen:
        </Text>
        <Stack gap={4}>
          <Text size="xs">
            ✓ Slicing definition will be added to <Code>{wizardState.elementPath}</Code>
          </Text>
          <Text size="xs">
            ✓ {wizardState.slices.length} slice{wizardState.slices.length !== 1 ? 's' : ''} will be
            created
          </Text>
          <Text size="xs">✓ Each slice can be individually constrained in the element tree</Text>
          <Text size="xs">✓ Validators will check instances against discriminator rules</Text>
        </Stack>
      </Alert>

      {/* Warnings */}
      {wizardState.ordered && (
        <Alert icon={<IconInfoCircle size={16} />} color="yellow">
          <Text size="xs">
            <strong>Note:</strong> Ordered slicing requires instances to maintain the specified
            order, which may be difficult for some implementations.
          </Text>
        </Alert>
      )}

      {wizardState.rules === 'closed' && (
        <Alert icon={<IconInfoCircle size={16} />} color="yellow">
          <Text size="xs">
            <strong>Note:</strong> Closed slicing prevents any additional slices beyond those
            defined here. Consider using "open" for more flexibility.
          </Text>
        </Alert>
      )}

      {/* Documentation Link */}
      <Alert color="blue" variant="light">
        <Text size="xs">
          Learn more about FHIR slicing:{' '}
          <a
            href="https://www.hl7.org/fhir/profiling.html#slicing"
            target="_blank"
            rel="noopener noreferrer"
          >
            FHIR Profiling - Slicing
          </a>
        </Text>
      </Alert>
    </Stack>
  );
}
