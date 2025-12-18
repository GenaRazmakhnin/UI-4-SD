import { api } from '@shared/api';
import type { ElementNode, SlicingDiscriminator, SlicingRules } from '@shared/types';
import { $selectedElement } from '@widgets/element-tree';
import { createEffect, createEvent, createStore, sample } from 'effector';

/**
 * Wizard state
 */
export interface WizardState {
  currentStep: number;
  elementPath: string;
  discriminators: SlicingDiscriminator[];
  rules: SlicingRules['rules'];
  ordered: boolean;
  description: string;
  slices: Array<{
    name: string;
    min: number;
    max: string;
    description?: string;
  }>;
}

/**
 * Initial wizard state
 */
const initialState: WizardState = {
  currentStep: 0,
  elementPath: '',
  discriminators: [],
  rules: 'open',
  ordered: false,
  description: '',
  slices: [],
};

/**
 * Events
 */
export const wizardOpened = createEvent<{ element: ElementNode }>();
export const wizardClosed = createEvent();
export const stepChanged = createEvent<number>();
export const discriminatorAdded = createEvent<SlicingDiscriminator>();
export const discriminatorRemoved = createEvent<number>();
export const rulesChanged = createEvent<{
  rules?: SlicingRules['rules'];
  ordered?: boolean;
  description?: string;
}>();
export const sliceAdded = createEvent<{
  name: string;
  min: number;
  max: string;
  description?: string;
}>();
export const sliceRemoved = createEvent<number>();
export const sliceUpdated = createEvent<{
  index: number;
  updates: Partial<{
    name: string;
    min: number;
    max: string;
    description: string;
  }>;
}>();
export const templateApplied = createEvent<string>();
export const wizardReset = createEvent();

/**
 * Stores
 */
export const $wizardState = createStore<WizardState>(initialState);
export const $wizardOpen = createStore<boolean>(false);
export const $canProceed = createStore<boolean>(false);

/**
 * Apply slicing effect
 */
export const applySlicingFx = createEffect(
  async ({
    profileId,
    elementPath,
    slicing,
    slices,
  }: {
    profileId: string;
    elementPath: string;
    slicing: SlicingRules;
    slices: WizardState['slices'];
  }) => {
    // First, add the slicing definition
    await api.profiles.updateElement(profileId, elementPath, { slicing });

    // Then, create each slice
    for (const slice of slices) {
      await api.profiles.addSlice(profileId, elementPath, {
        sliceName: slice.name,
        min: slice.min,
        max: slice.max,
        short: slice.description,
      });
    }

    return { success: true };
  }
);

/**
 * Handle wizard opened
 */
$wizardOpen.on(wizardOpened, () => true).on(wizardClosed, () => false);

sample({
  clock: wizardOpened,
  fn: ({ element }) => ({
    ...initialState,
    elementPath: element.path,
  }),
  target: $wizardState,
});

/**
 * Handle step changes
 */
$wizardState.on(stepChanged, (state, step) => ({
  ...state,
  currentStep: step,
}));

/**
 * Handle discriminators
 */
$wizardState.on(discriminatorAdded, (state, discriminator) => ({
  ...state,
  discriminators: [...state.discriminators, discriminator],
}));

$wizardState.on(discriminatorRemoved, (state, index) => ({
  ...state,
  discriminators: state.discriminators.filter((_, i) => i !== index),
}));

/**
 * Handle rules changes
 */
$wizardState.on(rulesChanged, (state, changes) => ({
  ...state,
  rules: changes.rules ?? state.rules,
  ordered: changes.ordered ?? state.ordered,
  description: changes.description ?? state.description,
}));

/**
 * Handle slices
 */
$wizardState.on(sliceAdded, (state, slice) => ({
  ...state,
  slices: [...state.slices, slice],
}));

$wizardState.on(sliceRemoved, (state, index) => ({
  ...state,
  slices: state.slices.filter((_, i) => i !== index),
}));

$wizardState.on(sliceUpdated, (state, { index, updates }) => ({
  ...state,
  slices: state.slices.map((slice, i) => (i === index ? { ...slice, ...updates } : slice)),
}));

/**
 * Handle template applied
 */
sample({
  clock: templateApplied,
  source: $wizardState,
  fn: (state, templateId) => {
    const { SLICING_TEMPLATES } = require('../lib/templates');
    const template = SLICING_TEMPLATES.find((t) => t.id === templateId);

    if (!template) return state;

    return {
      ...state,
      discriminators: template.discriminators,
      rules: template.rules,
      ordered: template.ordered,
      slices: template.suggestedSlices || [],
    };
  },
  target: $wizardState,
});

/**
 * Handle wizard reset/close
 */
$wizardState.on(wizardReset, () => initialState).on(wizardClosed, () => initialState);

/**
 * Calculate if wizard can proceed to next step
 */
$canProceed.on($wizardState, (_, state) => {
  switch (state.currentStep) {
    case 0: // Discriminator step
      return state.discriminators.length > 0;
    case 1: // Rules step
      return true; // Rules have defaults
    case 2: // Slices step
      return state.slices.length > 0;
    case 3: // Review step
      return true;
    default:
      return false;
  }
});

/**
 * Apply slicing when wizard completes
 */
sample({
  clock: applySlicingFx.doneData,
  target: wizardClosed,
});
