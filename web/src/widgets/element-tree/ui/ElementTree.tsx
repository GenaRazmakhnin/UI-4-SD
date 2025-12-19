import { Text } from '@mantine/core';
import { IconListTree } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useMemo } from 'react';
import { List } from 'react-window';
import {
  $expandedPaths,
  $flattenedElements,
  $selectedElementId,
  $sliceViews,
  elementSelected,
  pathToggled,
  sliceViewChanged,
} from '../model';
import { ElementRow } from './ElementRow';
import styles from './ElementRow.module.css';
import { ElementTreeToolbar } from './ElementTreeToolbar';

export function ElementTree() {
  const elements = useUnit($flattenedElements);
  const expandedPaths = useUnit($expandedPaths);
  const selectedId = useUnit($selectedElementId);
  const sliceViews = useUnit($sliceViews);

  const rowProps = useMemo(
    () => ({
      elements,
      expandedPaths,
      selectedId,
      sliceViews,
      onSelect: elementSelected,
      onToggle: pathToggled,
      onSliceChange: sliceViewChanged,
    }),
    [elements, expandedPaths, selectedId, sliceViews]
  );

  return (
    <div className={styles.container}>
      <ElementTreeToolbar />
      <div className={styles.treeContent}>
        {elements.length === 0 ? (
          <div className={styles.emptyState}>
            <IconListTree size={48} stroke={1.5} />
            <Text size="sm" c="dimmed">
              No elements to display
            </Text>
          </div>
        ) : (
          <List
            defaultHeight={600}
            rowCount={elements.length}
            rowHeight={32}
            rowComponent={ElementRow}
            rowProps={rowProps}
            style={{ height: '100%', width: '100%' }}
          ></List>
        )}
      </div>
    </div>
  );
}
