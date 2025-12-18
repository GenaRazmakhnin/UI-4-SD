import { api } from '@shared/api';
import type { ElementNode } from '@shared/types';
import { $selectedElement } from '@widgets/element-tree';
import { createEffect, createEvent, sample } from 'effector';
import { validateFlags } from '../lib/validation';

/**
 * Flag changed event
 */
export const flagChanged = createEvent<{
  elementId: string;
  flag: 'mustSupport' | 'isModifier' | 'isSummary';
  value: boolean;
  reason?: string;
}>();

/**
 * Update element flag effect
 */
const updateFlagFx = createEffect(
  async ({
    profileId,
    elementPath,
    updates,
  }: {
    profileId: string;
    elementPath: string;
    updates: Partial<ElementNode>;
  }) => {
    return await api.profiles.updateElement(profileId, elementPath, updates);
  }
);

/**
 * Handle flag changes
 */
sample({
  clock: flagChanged,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { flag, value }) => {
    const updates: Partial<ElementNode> = {
      [flag]: value,
    };

    return {
      profileId: 'current-profile', // TODO: Get from profile context
      elementPath: element.path,
      updates,
      element,
    };
  },
  target: createEffect(async ({ profileId, elementPath, updates, element }) => {
    // Validate first
    const validation = validateFlags(element, updates);

    if (validation.isValid) {
      await updateFlagFx({
        profileId,
        elementPath,
        updates,
      });
    } else {
      throw new Error(`Validation failed: ${validation.errors.join(', ')}`);
    }
  }),
});
