import { api } from '@shared/api';
import type { ElementNode } from '@shared/types';
import { $profileContext, $selectedElement } from '@widgets/element-tree';
import { profileChanged } from '@pages/editor/model';
import { createEffect, createEvent, createStore, sample, combine } from 'effector';
import { validateCardinality } from '../lib/validation';

// Events
export const cardinalityChanged = createEvent<{
  elementPath: string;
  min: number;
  max: string;
}>();

export const cardinalityEditCancelled = createEvent();

// Stores
export const $isEditingCardinality = createStore(false);

export const $cardinalityValidation = createStore({
  minError: null as string | null,
  maxError: null as string | null,
  isValid: true,
});

// Effects
const applyCardinalityFx = createEffect(
  async ({
    projectId,
    profileId,
    elementPath,
    min,
    max,
  }: {
    projectId: string;
    profileId: string;
    elementPath: string;
    min: number;
    max: string;
  }) => {
    return await api.profiles.updateElement(profileId, elementPath, {
      min,
      max,
    });
  }
);

// Logic - combine profile context with selected element for real IDs
sample({
  clock: cardinalityChanged,
  source: combine($selectedElement, $profileContext),
  filter: ([element, context]): element is [ElementNode, NonNullable<typeof context>] =>
    element !== null && context !== null,
  fn: ([element, context], { min, max }) => ({
    projectId: context.projectId,
    profileId: context.profileId,
    elementPath: element.path,
    min,
    max,
    element,
  }),
  target: createEffect(async ({ projectId, profileId, elementPath, min, max, element }) => {
    // Validate first
    const validation = validateCardinality(min, max, element.min, element.max);

    if (validation.isValid) {
      await applyCardinalityFx({
        projectId,
        profileId,
        elementPath,
        min,
        max,
      });
      // Fire profileChanged to mark as dirty
      profileChanged();
    } else {
      throw new Error('Validation failed');
    }
  }),
});

// Update editing state
$isEditingCardinality
  .on(cardinalityChanged, () => true)
  .on([applyCardinalityFx.done, cardinalityEditCancelled], () => false);
