import {
  Alert,
  Badge,
  Button,
  Code,
  Group,
  NumberInput,
  Stack,
  Text,
  TextInput,
} from '@mantine/core';
import type { ElementNode } from '@shared/types';
import { IconArrowBackUp, IconInfoCircle } from '@tabler/icons-react';
import { useState } from 'react';
import { getImpactMessage, parseMaxToNumber, validateCardinality } from '../lib/validation';
import { cardinalityChanged } from '../model';
import styles from './CardinalityEditor.module.css';

interface CardinalityEditorProps {
  element: ElementNode;
  onClose?: () => void;
}

export function CardinalityEditor({ element, onClose }: CardinalityEditorProps) {
  const [min, setMin] = useState(element.min);
  const [max, setMax] = useState(element.max);

  // Use element's own values as "base" for now
  // In a real implementation, this would come from the base definition
  const baseMin = element.min;
  const baseMax = element.max;
  const isModified = min !== baseMin || max !== baseMax;

  // Validation
  const validation = validateCardinality(min, max, baseMin, baseMax);
  const minError = validation.minError;
  const maxError = validation.maxError;
  const hasErrors = !!minError || !!maxError;

  // Impact preview
  const impact = getImpactMessage(min, max, baseMin, baseMax);

  const handleApply = () => {
    if (!hasErrors) {
      cardinalityChanged({
        elementPath: element.path,
        min,
        max,
      });
      onClose?.();
    }
  };

  const handleReset = () => {
    setMin(baseMin);
    setMax(baseMax);
  };

  return (
    <Stack gap="md" className={styles.container}>
      {/* Header */}
      <Group justify="space-between">
        <Text size="sm" fw={600}>
          Cardinality
        </Text>
        {isModified && (
          <Badge size="sm" color="blue">
            Modified
          </Badge>
        )}
      </Group>

      {/* Baseline Display */}
      <Group gap="xs" className={styles.baseline}>
        <Text size="xs" c="dimmed">
          Base:
        </Text>
        <Code className={styles.baselineValue}>
          {baseMin}..{baseMax}
        </Code>
      </Group>

      {/* Min/Max Inputs */}
      <Group grow>
        <NumberInput
          label="Minimum"
          value={min}
          onChange={(val) => setMin(Number(val))}
          min={0}
          max={parseMaxToNumber(max)}
          error={minError}
          description={`Must be ≥ ${baseMin}`}
          classNames={{ input: isModified ? styles.modified : undefined }}
        />
        <TextInput
          label="Maximum"
          value={max}
          onChange={(e) => setMax(e.target.value)}
          error={maxError}
          description="Enter number or '*'"
          placeholder="* or number"
          classNames={{ input: isModified ? styles.modified : undefined }}
        />
      </Group>

      {/* Quick Presets */}
      <div>
        <Text size="xs" c="dimmed" mb="xs">
          Quick Presets:
        </Text>
        <Group gap="xs">
          <Button
            size="xs"
            variant="light"
            onClick={() => {
              setMin(0);
              setMax('1');
            }}
          >
            0..1 (Optional)
          </Button>
          <Button
            size="xs"
            variant="light"
            onClick={() => {
              setMin(1);
              setMax('1');
            }}
          >
            1..1 (Required)
          </Button>
          <Button
            size="xs"
            variant="light"
            onClick={() => {
              setMin(0);
              setMax('*');
            }}
          >
            0..* (Any)
          </Button>
          <Button
            size="xs"
            variant="light"
            onClick={() => {
              setMin(1);
              setMax('*');
            }}
          >
            1..* (At least one)
          </Button>
        </Group>
      </div>

      {/* Impact Preview */}
      {impact && (
        <Alert icon={<IconInfoCircle size={16} />} color="blue" variant="light">
          <Text size="sm">{impact}</Text>
        </Alert>
      )}

      {/* Actions */}
      {onClose && (
        <Group justify="flex-end" gap="xs">
          {isModified && (
            <Button
              size="sm"
              variant="subtle"
              leftSection={<IconArrowBackUp size={16} />}
              onClick={handleReset}
            >
              Reset to Base
            </Button>
          )}
          <Button size="sm" variant="default" onClick={onClose}>
            Cancel
          </Button>
          <Button size="sm" onClick={handleApply} disabled={hasErrors}>
            Apply
          </Button>
        </Group>
      )}
    </Stack>
  );
}
