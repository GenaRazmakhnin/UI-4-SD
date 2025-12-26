import { api } from '@shared/api';
import type { ElementNode } from '@shared/types';
import { combine, createEffect, createEvent, createStore, sample } from 'effector';
import { persist } from 'effector-storage/local';
import { getAvailableActions, QUICK_ACTIONS } from '../lib/actions';
import type { QuickAction } from '../lib/types';
import {
  DEFAULT_PREFERENCES,
  type QuickActionId,
  type QuickConstraintsPreferences,
  type RecentItem,
} from '../lib/types';
import { wizardOpened as openSlicingWizard } from '../../slicing-wizard/model';
import { browserOpened as openBindingBrowser } from '../../binding-editor/model';
import { pickerOpened as openExtensionPicker } from '../../extension-picker/model';

// ============================================================================
// Stores
// ============================================================================

/**
 * User preferences (favorites, recents, settings)
 */
export const $preferences = createStore<QuickConstraintsPreferences>(DEFAULT_PREFERENCES);

/**
 * Currently pending action (for confirmation dialog)
 */
export const $pendingAction = createStore<{
  actionId: QuickActionId;
  element: ElementNode;
} | null>(null);

/**
 * Context menu state
 */
export const $contextMenuState = createStore<{
  isOpen: boolean;
  position: { x: number; y: number };
  element: ElementNode | null;
}>({
  isOpen: false,
  position: { x: 0, y: 0 },
  element: null,
});

/**
 * Shortcut help overlay open state
 */
export const $shortcutHelpOpen = createStore(false);

/**
 * Local copy of selected element (populated via connectToElementTree)
 */
export const $localSelectedElement = createStore<ElementNode | null>(null);

// ============================================================================
// Derived Stores
// ============================================================================

/**
 * Available actions for currently selected element
 */
export const $availableActions = $localSelectedElement.map((element): QuickAction[] => {
  if (!element) return [];
  return getAvailableActions(element);
});

/**
 * Favorite actions for current element (filtered by availability)
 */
export const $favoriteActions = combine(
  $localSelectedElement,
  $preferences,
  (element, prefs): (QuickAction | undefined)[] => {
    if (!element) return [];
    return prefs.favoriteActions
      .map((id) => QUICK_ACTIONS.find((a) => a.id === id))
      .filter((action) => action && action.isAvailable(element));
  }
);

/**
 * Recent ValueSets
 */
export const $recentValueSets = $preferences.map((p) => p.recentValueSets);

/**
 * Recent Extensions
 */
export const $recentExtensions = $preferences.map((p) => p.recentExtensions);

// ============================================================================
// Events
// ============================================================================

/**
 * Execute a quick action
 */
export const actionTriggered = createEvent<QuickActionId>();

/**
 * Confirm pending action
 */
export const actionConfirmed = createEvent();

/**
 * Cancel pending action
 */
export const actionCancelled = createEvent();

/**
 * Toggle favorite status of an action
 */
export const favoriteToggled = createEvent<QuickActionId>();

/**
 * Add recent item (ValueSet or Extension)
 */
export const recentItemAdded = createEvent<RecentItem>();

/**
 * Clear recent items
 */
export const recentItemsCleared = createEvent<'valueset' | 'extension'>();

/**
 * Update preferences
 */
export const preferencesUpdated = createEvent<Partial<QuickConstraintsPreferences>>();

/**
 * Open context menu
 */
export const contextMenuOpened = createEvent<{
  position: { x: number; y: number };
  element: ElementNode;
}>();

/**
 * Close context menu
 */
export const contextMenuClosed = createEvent();

/**
 * Internal events for execution logic
 */
const actionValidated = createEvent<{
  actionId: QuickActionId;
  element: ElementNode;
  needsConfirmation: boolean;
}>();

/**
 * Toggle shortcut help overlay
 */
export const shortcutHelpToggled = createEvent();

// ============================================================================
// Effects
// ============================================================================

/**
 * Execute cardinality change
 */
export const updateCardinalityFx = createEffect(
  async ({
    profileId,
    elementPath,
    min,
    max,
  }: {
    profileId: string;
    elementPath: string;
    min?: number;
    max?: string;
  }) => {
    return await api.profiles.updateElement(profileId, elementPath, { min, max });
  }
);

/**
 * Toggle element flag
 */
export const toggleFlagFx = createEffect(
  async ({
    profileId,
    elementPath,
    flag,
    value,
  }: {
    profileId: string;
    elementPath: string;
    flag: 'mustSupport' | 'isModifier' | 'isSummary';
    value: boolean;
  }) => {
    return await api.profiles.updateElement(profileId, elementPath, { [flag]: value });
  }
);

// ============================================================================
// Store Updates
// ============================================================================

// Toggle favorites
$preferences.on(favoriteToggled, (prefs, actionId) => {
  const favorites = prefs.favoriteActions.includes(actionId)
    ? prefs.favoriteActions.filter((id) => id !== actionId)
    : [...prefs.favoriteActions, actionId];
  return { ...prefs, favoriteActions: favorites };
});

// Add recent item
$preferences.on(recentItemAdded, (prefs, item) => {
  const key = item.type === 'valueset' ? 'recentValueSets' : 'recentExtensions';
  const existing = prefs[key].filter((i) => i.url !== item.url);
  const updated = [item, ...existing].slice(0, prefs.maxRecentItems);
  return { ...prefs, [key]: updated };
});

// Clear recent items
$preferences.on(recentItemsCleared, (prefs, type) => {
  const key = type === 'valueset' ? 'recentValueSets' : 'recentExtensions';
  return { ...prefs, [key]: [] };
});

// Update preferences
$preferences.on(preferencesUpdated, (prefs, updates) => ({
  ...prefs,
  ...updates,
}));

// Context menu state
$contextMenuState.on(contextMenuOpened, (_, { position, element }) => ({
  isOpen: true,
  position,
  element,
}));

$contextMenuState.on(contextMenuClosed, (state) => ({
  ...state,
  isOpen: false,
}));

// Shortcut help overlay
$shortcutHelpOpen.on(shortcutHelpToggled, (open) => !open);

// Pending action management
$pendingAction.on(actionCancelled, () => null);
$pendingAction.on(actionConfirmed, () => null);

// ============================================================================
// Action Execution Logic
// ============================================================================

/**
 * Event to update local selected element from element-tree
 */
export const elementSelectionChanged = createEvent<ElementNode | null>();

// Connect local store to element selection
$localSelectedElement.on(elementSelectionChanged, (_, element) => element);

/**
 * Handle action trigger - check if confirmation needed
 */
sample({
  clock: actionTriggered,
  source: $localSelectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, actionId) => {
    const action = QUICK_ACTIONS.find((a) => a.id === actionId);
    return {
      actionId,
      element: element!,
      needsConfirmation: action?.requiresConfirmation ?? false,
    };
  },
  target: actionValidated,
});

// Set pending action if confirmation needed
sample({
  clock: actionTriggered,
  source: combine($localSelectedElement, $preferences, (element, prefs) => ({ element, prefs })),
  filter: ({ element, prefs }, actionId) => {
    if (!element) return false;
    const action = QUICK_ACTIONS.find((a) => a.id === actionId);
    return (action?.requiresConfirmation ?? false) && !prefs.skipConfirmation;
  },
  fn: ({ element }, actionId) => ({ actionId, element: element! }),
  target: $pendingAction,
});

// Execute cardinality actions
sample({
  clock: actionTriggered,
  source: $localSelectedElement,
  filter: (element, actionId) =>
    element !== null &&
    ['make-required', 'make-optional', 'allow-multiple', 'make-prohibited'].includes(actionId),
  fn: (element, actionId) => {
    const updates: { min?: number; max?: string } = {};
    switch (actionId) {
      case 'make-required':
        updates.min = 1;
        break;
      case 'make-optional':
        updates.min = 0;
        break;
      case 'allow-multiple':
        updates.max = '*';
        break;
      case 'make-prohibited':
        updates.max = '0';
        break;
    }
    return {
      profileId: 'current-profile', // TODO: Get from context
      elementPath: element!.path,
      ...updates,
    };
  },
  target: updateCardinalityFx,
});

// Execute flag toggle actions
sample({
  clock: actionTriggered,
  source: $localSelectedElement,
  filter: (element, actionId) =>
    element !== null &&
    ['toggle-must-support', 'toggle-is-modifier', 'toggle-is-summary'].includes(actionId),
  fn: (element, actionId) => {
    const flagMap: Record<string, 'mustSupport' | 'isModifier' | 'isSummary'> = {
      'toggle-must-support': 'mustSupport',
      'toggle-is-modifier': 'isModifier',
      'toggle-is-summary': 'isSummary',
    };
    const flag = flagMap[actionId]!;
    const currentValue = element![flag] ?? false;
    return {
      profileId: 'current-profile', // TODO: Get from context
      elementPath: element!.path,
      flag,
      value: !currentValue,
    };
  },
  target: toggleFlagFx,
});

// Trigger Slicing Wizard
sample({
  clock: actionTriggered,
  source: $localSelectedElement,
  filter: (element, actionId) => element !== null && actionId === 'create-slice',
  fn: (element) => ({ element: element! }),
  target: openSlicingWizard,
});

// Trigger Binding Browser
sample({
  clock: actionTriggered,
  source: $localSelectedElement,
  filter: (element, actionId) => element !== null && actionId === 'set-binding',
  target: openBindingBrowser,
});

// Trigger Extension Picker
sample({
  clock: actionTriggered,
  source: $localSelectedElement,
  filter: (element, actionId) => element !== null && actionId === 'add-extension',
  target: openExtensionPicker,
});

// TODO: Implement other complex actions (constrain-type, add-pattern, add-invariant)
// These would typically open search modals/wizards

// ============================================================================
// Persistence
// ============================================================================

persist({
  store: $preferences,
  key: 'quick-constraints-preferences',
});

// ============================================================================
// Pending States
// ============================================================================

export const $isExecuting = updateCardinalityFx.pending;
