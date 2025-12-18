import { useUnit } from 'effector-react';
import { FixedSizeList } from 'react-window';
import { Text } from '@mantine/core';
import { IconFileTree } from '@tabler/icons-react';
import {
  $flattenedElements,
  $expandedPaths,
  $selectedElementId,
  elementSelected,
  pathToggled,
} from '../model';
import { ElementRow } from './ElementRow';
import { ElementTreeToolbar } from './ElementTreeToolbar';
import styles from './ElementRow.module.css';

export function ElementTree() {
  const elements = useUnit($flattenedElements);
  const expandedPaths = useUnit($expandedPaths);
  const selectedId = useUnit($selectedElementId);

  return (
    <div className={styles.container}>
      <ElementTreeToolbar />
      <div className={styles.treeContent}>
        {elements.length === 0 ? (
          <div className={styles.emptyState}>
            <IconFileTree size={48} stroke={1.5} />
            <Text size="sm" c="dimmed">
              No elements to display
            </Text>
          </div>
        ) : (
          <FixedSizeList
            height={600}
            itemCount={elements.length}
            itemSize={32}
            width="100%"
            itemData={{
              elements,
              expandedPaths,
              selectedId,
              onSelect: elementSelected,
              onToggle: pathToggled,
            }}
          >
            {ElementRow}
          </FixedSizeList>
        )}
      </div>
    </div>
  );
}
