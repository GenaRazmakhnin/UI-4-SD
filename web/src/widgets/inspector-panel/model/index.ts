import { $selectedElement } from '@widgets/element-tree';
import { createEvent, createStore, sample } from 'effector';

/**
 * Active tab in inspector panel
 */
export const $activeTab = createStore<string>('constraints');

/**
 * Change active tab
 */
export const tabChanged = createEvent<string>();

$activeTab.on(tabChanged, (_, tab) => tab);

/**
 * Reset to constraints tab when element changes
 */
sample({
  clock: $selectedElement,
  filter: (element) => element !== null,
  fn: () => 'constraints',
  target: $activeTab,
});

/**
 * Panel width (for resizing)
 */
export const $panelWidth = createStore<number>(400);

export const panelWidthChanged = createEvent<number>();

$panelWidth.on(panelWidthChanged, (_, width) => {
  // Clamp width between 300 and 800
  return Math.max(300, Math.min(800, width));
});
