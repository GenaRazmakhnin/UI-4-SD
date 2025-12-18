import { createStore, createEvent, createEffect, sample } from 'effector';
import { $selectedElement } from '@widgets/element-tree';
import { api } from '@shared/api';
import { validateCardinality } from '../lib/validation';
import type { ElementNode } from '@shared/types';

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
    profileId,
    elementPath,
    min,
    max,
  }: {
    profileId: string;
    elementPath: string;
    min: number;
    max: string;
  }) => {
    return await api.profiles.updateElement(profileId, elementPath, {
      min,
      max,
    });
  },
);

// Logic
sample({
  clock: cardinalityChanged,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { min, max }) => ({
    profileId: 'current-profile', // TODO: Get from profile context
    elementPath: element.path,
    min,
    max,
    element,
  }),
  target: createEffect(
    async ({ profileId, elementPath, min, max, element }) => {
      // Validate first
      const validation = validateCardinality(min, max, element.min, element.max);

      if (validation.isValid) {
        await applyCardinalityFx({
          profileId,
          elementPath,
          min,
          max,
        });
      } else {
        throw new Error('Validation failed');
      }
    },
  ),
});

// Update editing state
$isEditingCardinality
  .on(cardinalityChanged, () => true)
  .on([applyCardinalityFx.done, cardinalityEditCancelled], () => false);
