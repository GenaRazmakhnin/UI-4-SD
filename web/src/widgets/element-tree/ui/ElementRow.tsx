import { Tooltip } from '@mantine/core';
import { cn } from '@shared/lib';
import type { ElementNode } from '@shared/types';
import { IconChevronRight } from '@tabler/icons-react';
import { memo } from 'react';
import type { RowComponentProps } from 'react-window';
import styles from './ElementRow.module.css';
import {
  CardinalityBadge,
  ConstraintIndicators,
  FlagIndicators,
  InheritanceIndicator,
} from './indicators';

export interface ElementRowData {
  elements: ElementNode[];
  expandedPaths: Set<string>;
  selectedId: string | null;
  onSelect: (element: ElementNode) => void;
  onToggle: (path: string) => void;
}

type ElementRowProps = RowComponentProps<ElementRowData>;

export const ElementRow = memo(({ index, style, ariaAttributes, ...rowProps }: ElementRowProps) => {
  const { elements, expandedPaths, selectedId, onSelect, onToggle } = rowProps;
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
      {...ariaAttributes}
      role="treeitem"
      tabIndex={0}
      aria-selected={isSelected}
      className={cn(styles.row, {
        [styles.selected]: isSelected,
        [styles.modified]: isModified,
      })}
      onClick={() => onSelect(element)}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onSelect(element);
        }
      }}
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
