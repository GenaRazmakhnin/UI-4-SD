import { api } from '@shared/api';
import type { BindingConstraint, ElementNode, ValueSet, ValueSetExpansion } from '@shared/types';
import { $selectedElement } from '@widgets/element-tree';
import { createEffect, createEvent, createStore, sample } from 'effector';

/**
 * Binding changed
 */
export const bindingChanged = createEvent<{
  elementId: string;
  binding: BindingConstraint;
}>();

/**
 * Remove binding
 */
export const removeBinding = createEvent<{
  elementId: string;
}>();

/**
 * Search ValueSets effect
 */
export const searchValueSetsFx = createEffect(
  async ({ query, codeSystemFilter }: { query: string; codeSystemFilter: string | null }) => {
    const results = await api.search.valueSets(query, {
      codeSystem: codeSystemFilter ? [codeSystemFilter] : undefined,
    });

    return results;
  }
);

/**
 * Fetch ValueSet expansion effect
 */
export const fetchExpansionFx = createEffect(async ({ valueSetUrl }: { valueSetUrl: string }) => {
  try {
    const expansion = await api.terminology.expand(valueSetUrl);
    return { valueSetUrl, expansion };
  } catch (error) {
    return {
      valueSetUrl,
      expansion: {
        error: error instanceof Error ? error.message : 'Failed to expand ValueSet',
      } as ValueSetExpansion,
    };
  }
});

/**
 * Search results store
 */
export const $searchResults = createStore<ValueSet[]>([]);
export const $searchLoading = searchValueSetsFx.pending;

$searchResults.on(searchValueSetsFx.doneData, (_, results) => results);

/**
 * Expansions store (cached by URL)
 */
export const $expansions = createStore<Record<string, ValueSetExpansion>>({});
export const $expansionLoading = fetchExpansionFx.pending;

$expansions.on(fetchExpansionFx.doneData, (state, { valueSetUrl, expansion }) => ({
  ...state,
  [valueSetUrl]: expansion,
}));

/**
 * Update binding effect
 */
const updateBindingFx = createEffect(
  async ({
    profileId,
    elementPath,
    binding,
  }: {
    profileId: string;
    elementPath: string;
    binding: BindingConstraint | null;
  }) => {
    return await api.profiles.updateElement(profileId, elementPath, { binding });
  }
);

/**
 * Handle binding changes
 */
sample({
  clock: bindingChanged,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { binding }) => ({
    profileId: 'current-profile', // TODO: Get from profile context
    elementPath: element.path,
    binding,
  }),
  target: updateBindingFx,
});

/**
 * Handle remove binding
 */
sample({
  clock: removeBinding,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element) => ({
    profileId: 'current-profile', // TODO: Get from profile context
    elementPath: element.path,
    binding: null,
  }),
  target: updateBindingFx,
});
