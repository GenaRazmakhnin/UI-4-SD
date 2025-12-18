import { Alert, Radio, Stack, Switch, Text, Textarea, Title } from '@mantine/core';
import type { SlicingRules } from '@shared/types';
import { IconInfoCircle } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { getSlicingRulesDescription } from '../lib/templates';
import { $wizardState, rulesChanged } from '../model';

export function Step2Rules() {
  const wizardState = useUnit($wizardState);

  const handleRulesChange = (rules: SlicingRules['rules']) => {
    rulesChanged({ rules });
  };

  const handleOrderedChange = (ordered: boolean) => {
    rulesChanged({ ordered });
  };

  const handleDescriptionChange = (description: string) => {
    rulesChanged({ description });
  };

  return (
    <Stack gap="lg">
      <div>
        <Title order={4} mb="xs">
          Slicing Rules
        </Title>
        <Text size="sm" c="dimmed">
          Define how additional slices beyond those explicitly defined should be handled, and
          whether slice order matters.
        </Text>
      </div>

      {/* Slicing Rules */}
      <div>
        <Text size="sm" fw={500} mb="sm">
          Slicing Rules
        </Text>
        <Radio.Group value={wizardState.rules} onChange={handleRulesChange}>
          <Stack gap="md">
            <Radio
              value="open"
              label="Open (Recommended)"
              description={getSlicingRulesDescription('open')}
            />
            <Radio
              value="closed"
              label="Closed"
              description={getSlicingRulesDescription('closed')}
            />
            <Radio
              value="openAtEnd"
              label="Open at End"
              description={getSlicingRulesDescription('openAtEnd')}
            />
          </Stack>
        </Radio.Group>

        <Alert icon={<IconInfoCircle size={16} />} color="blue" mt="md">
          <Text size="xs">
            <strong>Recommendation:</strong> Use "open" unless you have a specific reason to
            restrict additional slices. Closed slicing can make profiles less flexible for
            implementers.
          </Text>
        </Alert>
      </div>

      {/* Ordered Flag */}
      <div>
        <Text size="sm" fw={500} mb="sm">
          Slice Ordering
        </Text>
        <Switch
          label="Slices must appear in specified order"
          description="When enabled, slices must appear in the exact order defined in the profile"
          checked={wizardState.ordered}
          onChange={(event) => handleOrderedChange(event.currentTarget.checked)}
        />

        {wizardState.ordered && (
          <Alert icon={<IconInfoCircle size={16} />} color="yellow" mt="sm">
            <Text size="xs">
              Ordered slicing adds an additional constraint that may be difficult for implementers
              to satisfy. Use only when order is semantically meaningful.
            </Text>
          </Alert>
        )}
      </div>

      {/* Description */}
      <div>
        <Text size="sm" fw={500} mb="sm">
          Slicing Description (Optional)
        </Text>
        <Textarea
          placeholder="Describe the purpose and usage of this slicing..."
          description="Provide context to help implementers understand why and how to use these slices"
          value={wizardState.description}
          onChange={(e) => handleDescriptionChange(e.currentTarget.value)}
          rows={4}
          maxLength={500}
        />
        {wizardState.description && (
          <Text size="xs" c="dimmed" ta="right" mt={4}>
            {wizardState.description.length}/500
          </Text>
        )}
      </div>

      {/* Examples */}
      <Alert color="grape" variant="light">
        <Text size="xs" fw={500} mb="xs">
          Common Use Cases:
        </Text>
        <Stack gap={4}>
          <Text size="xs">
            • <strong>Open + Unordered:</strong> Most common - allows flexibility
          </Text>
          <Text size="xs">
            • <strong>Closed + Unordered:</strong> Strict control over allowed slices
          </Text>
          <Text size="xs">
            • <strong>Open + Ordered:</strong> Specific order required, additional slices OK
          </Text>
          <Text size="xs">
            • <strong>Closed + Ordered:</strong> Complete control (rarely needed)
          </Text>
        </Stack>
      </Alert>
    </Stack>
  );
}
