import { memo } from 'react';
import { IconChevronRight } from '@tabler/icons-react';
import { ActionIcon, Tooltip } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import {
  InheritanceIndicator,
  ConstraintIndicators,
  CardinalityBadge,
  FlagIndicators,
} from './indicators';
import styles from './ElementRow.module.css';
import { cn } from '@shared/lib';

export interface ElementRowData {
  elements: ElementNode[];
  expandedPaths: Set<string>;
  selectedId: string | null;
  onSelect: (element: ElementNode) => void;
  onToggle: (path: string) => void;
}

interface ElementRowProps {
  index: number;
  style: React.CSSProperties;
  data: ElementRowData;
}

export const ElementRow = memo(({ index, style, data }: ElementRowProps) => {
  const { elements, expandedPaths, selectedId, onSelect, onToggle } = data;
  const element = elements[index];

  if (!element) return null;

  const depth = element.path.split('.').length - 1;
  const hasChildren = element.children.length > 0;
  const isExpanded = expandedPaths.has(element.path);
  const isSelected = selectedId === element.id;
  const isModified = element.isModified;

  // Get the last part of the path for display
  const pathParts = element.path.split('.');
  const displayName = element.sliceName
    ? `${pathParts[pathParts.length - 1]}:${element.sliceName}`
    : pathParts[pathParts.length - 1];

  return (
    <div
      style={style}
      className={cn(styles.row, {
        [styles.selected]: isSelected,
        [styles.modified]: isModified,
      })}
      onClick={() => onSelect(element)}
    >
      {/* Indentation */}
      <div style={{ width: depth * 20 }} />

      {/* Expand/Collapse Button */}
      {hasChildren ? (
        <button
          type="button"
          className={cn(styles.expandButton, {
            [styles.expanded]: isExpanded,
          })}
          onClick={(e) => {
            e.stopPropagation();
            onToggle(element.path);
          }}
          aria-label={isExpanded ? 'Collapse' : 'Expand'}
        >
          <IconChevronRight size={14} />
        </button>
      ) : (
        <div style={{ width: 20 }} />
      )}

      {/* Element Path */}
      <Tooltip label={element.path} openDelay={500}>
        <span className={styles.path}>{displayName}</span>
      </Tooltip>

      {/* Cardinality Badge */}
      <CardinalityBadge min={element.min} max={element.max} />

      {/* Inheritance Indicator */}
      <InheritanceIndicator element={element} />

      {/* Flag Indicators */}
      <FlagIndicators element={element} />

      {/* Constraint Indicators */}
      <ConstraintIndicators element={element} />
    </div>
  );
});

ElementRow.displayName = 'ElementRow';
