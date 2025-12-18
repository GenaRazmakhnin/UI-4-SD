import { Paper, Tabs } from '@mantine/core';
import { $selectedElement } from '@widgets/element-tree';
import { useUnit } from 'effector-react';
import { BindingTab } from './BindingTab';
import { ConstraintsTab } from './ConstraintsTab';
import { ElementHeader } from './ElementHeader';
import { EmptyState } from './EmptyState';
import { ExtensionsTab } from './ExtensionsTab';
import styles from './InspectorPanel.module.css';
import { MetadataTab } from './MetadataTab';
import { SlicingTab } from './SlicingTab';

export function InspectorPanel() {
  const selectedElement = useUnit($selectedElement);

  if (!selectedElement) {
    return <EmptyState />;
  }

  return (
    <Paper className={styles.panel} shadow="sm" withBorder>
      <div className={styles.header}>
        <ElementHeader element={selectedElement} />
      </div>

      <Tabs
        defaultValue="constraints"
        className={styles.tabs}
        classNames={{
          root: styles.tabsRoot,
          list: styles.tabsList,
          panel: styles.tabsPanel,
        }}
      >
        <Tabs.List>
          <Tabs.Tab value="constraints">Constraints</Tabs.Tab>
          <Tabs.Tab value="binding">Binding</Tabs.Tab>
          <Tabs.Tab value="extensions">Extensions</Tabs.Tab>
          <Tabs.Tab value="slicing" disabled={!canSlice(selectedElement)}>
            Slicing
          </Tabs.Tab>
          <Tabs.Tab value="metadata">Metadata</Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="constraints" className={styles.scrollablePanel}>
          <ConstraintsTab element={selectedElement} />
        </Tabs.Panel>

        <Tabs.Panel value="binding" className={styles.scrollablePanel}>
          <BindingTab element={selectedElement} />
        </Tabs.Panel>

        <Tabs.Panel value="extensions" className={styles.scrollablePanel}>
          <ExtensionsTab element={selectedElement} />
        </Tabs.Panel>

        <Tabs.Panel value="slicing" className={styles.scrollablePanel}>
          <SlicingTab element={selectedElement} />
        </Tabs.Panel>

        <Tabs.Panel value="metadata" className={styles.scrollablePanel}>
          <MetadataTab element={selectedElement} />
        </Tabs.Panel>
      </Tabs>
    </Paper>
  );
}

/**
 * Check if an element can be sliced
 */
function canSlice(element: { max: string }): boolean {
  // Elements with max > 1 can be sliced
  return element.max === '*' || Number(element.max) > 1;
}
