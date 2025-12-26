import { api } from '@shared/api';
import type { ElementNode } from '@shared/types';
import { $profileContext, $selectedElement } from '@widgets/element-tree';
import { profileChanged } from '@pages/editor/model';
import { createEffect, createEvent, sample, combine } from 'effector';
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
 * Handle flag changes - use real profile context
 */
sample({
  clock: flagChanged,
  source: combine($selectedElement, $profileContext),
  filter: ([element, context]): element is [ElementNode, NonNullable<typeof context>] =>
    element !== null && context !== null,
  fn: ([element, context], { flag, value }) => {
    const updates: Partial<ElementNode> = {
      [flag]: value,
    };

    return {
      profileId: context.profileId,
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
      // Fire profileChanged to mark as dirty
      profileChanged();
    } else {
      throw new Error(`Validation failed: ${validation.errors.join(', ')}`);
    }
  }),
});
