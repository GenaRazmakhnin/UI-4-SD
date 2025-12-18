import { Badge, Tooltip, ActionIcon } from '@mantine/core';
import {
  IconPlus,
  IconEdit,
  IconLock,
  IconTarget,
  IconLink,
  IconCut,
  IconX,
  IconAlertTriangle,
  IconInfoCircle,
  IconCheck,
} from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';

interface InheritanceIndicatorProps {
  element: ElementNode;
}

export function InheritanceIndicator({ element }: InheritanceIndicatorProps) {
  if (element.isModified) {
    return (
      <Tooltip label="Modified from base definition">
        <Badge
          color="blue"
          size="xs"
          variant="light"
          leftSection={<IconEdit size={12} />}
        >
          MOD
        </Badge>
      </Tooltip>
    );
  }

  return null;
}

interface ConstraintIndicatorsProps {
  element: ElementNode;
}

export function ConstraintIndicators({ element }: ConstraintIndicatorsProps) {
  const indicators = [];

  if (element.binding) {
    indicators.push(
      <Tooltip
        key="binding"
        label={`Binding: ${element.binding.valueSet} (${element.binding.strength})`}
      >
        <ActionIcon size="xs" color="orange" variant="subtle">
          <IconLink size={14} />
        </ActionIcon>
      </Tooltip>,
    );
  }

  if (element.slicing) {
    indicators.push(
      <Tooltip key="slicing" label="Sliced element">
        <ActionIcon size="xs" color="grape" variant="subtle">
          <IconCut size={14} />
        </ActionIcon>
      </Tooltip>,
    );
  }

  if (indicators.length === 0) return null;

  return <div style={{ display: 'flex', gap: 4 }}>{indicators}</div>;
}

interface CardinalityBadgeProps {
  min: number;
  max: string;
  isModified?: boolean;
}

export function CardinalityBadge({
  min,
  max,
  isModified,
}: CardinalityBadgeProps) {
  return (
    <Badge
      size="xs"
      variant={isModified ? 'filled' : 'outline'}
      color={isModified ? 'blue' : 'gray'}
    >
      {min}..{max}
    </Badge>
  );
}

interface FlagIndicatorsProps {
  element: ElementNode;
}

export function FlagIndicators({ element }: FlagIndicatorsProps) {
  return (
    <div style={{ display: 'flex', gap: 4 }}>
      {element.mustSupport && (
        <Tooltip label="Must Support">
          <Badge size="xs" color="cyan" variant="filled">
            MS
          </Badge>
        </Tooltip>
      )}
      {element.isModifier && (
        <Tooltip label="Is Modifier">
          <Badge size="xs" color="red" variant="filled">
            MOD
          </Badge>
        </Tooltip>
      )}
      {element.isSummary && (
        <Tooltip label="Is Summary">
          <Badge size="xs" color="gray" variant="filled">
            Σ
          </Badge>
        </Tooltip>
      )}
    </div>
  );
}
