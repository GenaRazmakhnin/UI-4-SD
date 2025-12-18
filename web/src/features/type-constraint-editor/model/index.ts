import { api } from '@shared/api';
import type { ElementNode, Profile, TypeConstraint } from '@shared/types';
import { $selectedElement } from '@widgets/element-tree';
import { createEffect, createEvent, createStore, sample } from 'effector';
import { validateTypeConstraints } from '../lib/validation';

/**
 * Type constraint changed
 */
export const typeConstraintChanged = createEvent<{
  elementId: string;
  add: TypeConstraint[];
  remove: string[];
}>();

/**
 * Target profile added to type
 */
export const targetProfileAdded = createEvent<{
  elementId: string;
  typeCode: string;
  profileUrl: string;
}>();

/**
 * Target profile removed from type
 */
export const targetProfileRemoved = createEvent<{
  elementId: string;
  typeCode: string;
  profileUrl: string;
}>();

/**
 * Search profiles effect
 */
export const searchProfilesFx = createEffect(
  async ({ query, typeFilter }: { query: string; typeFilter: string }) => {
    // Search for profiles matching type and query
    const results = await api.search.profiles(query, {
      type: [typeFilter],
    });

    return results;
  }
);

/**
 * Search results store
 */
export const $searchResults = createStore<Profile[]>([]);
export const $searchLoading = searchProfilesFx.pending;

$searchResults.on(searchProfilesFx.doneData, (_, results) => results);

/**
 * Update type constraints effect
 */
const updateTypeConstraintsFx = createEffect(
  async ({
    profileId,
    elementPath,
    types,
  }: {
    profileId: string;
    elementPath: string;
    types: TypeConstraint[];
  }) => {
    return await api.profiles.updateElement(profileId, elementPath, {
      type: types,
    });
  }
);

/**
 * Handle type constraint changes
 */
sample({
  clock: typeConstraintChanged,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { add, remove }) => {
    let newTypes = [...(element.type || [])];

    // Remove types
    remove.forEach((typeCode) => {
      newTypes = newTypes.filter((t) => t.code !== typeCode);
    });

    // Add types
    add.forEach((newType) => {
      if (!newTypes.some((t) => t.code === newType.code)) {
        newTypes.push(newType);
      }
    });

    // Validate
    const validation = validateTypeConstraints(element, newTypes);
    if (!validation.isValid) {
      throw new Error(validation.errors.join('; '));
    }

    return {
      profileId: 'current-profile', // TODO: Get from profile context
      elementPath: element.path,
      types: newTypes,
      element,
    };
  },
  target: createEffect(async ({ profileId, elementPath, types, element }) => {
    // Validate first
    const validation = validateTypeConstraints(element, types);

    if (validation.isValid) {
      await updateTypeConstraintsFx({
        profileId,
        elementPath,
        types,
      });
    } else {
      throw new Error(`Validation failed: ${validation.errors.join(', ')}`);
    }
  }),
});

/**
 * Handle target profile added
 */
sample({
  clock: targetProfileAdded,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { typeCode, profileUrl }) => {
    const newTypes =
      element.type?.map((t) => {
        if (t.code === typeCode) {
          return {
            ...t,
            profile: [...(t.profile || []), profileUrl],
          };
        }
        return t;
      }) || [];

    return {
      profileId: 'current-profile', // TODO: Get from profile context
      elementPath: element.path,
      types: newTypes,
    };
  },
  target: updateTypeConstraintsFx,
});

/**
 * Handle target profile removed
 */
sample({
  clock: targetProfileRemoved,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { typeCode, profileUrl }) => {
    const newTypes =
      element.type?.map((t) => {
        if (t.code === typeCode) {
          return {
            ...t,
            profile: (t.profile || []).filter((p) => p !== profileUrl),
          };
        }
        return t;
      }) || [];

    return {
      profileId: 'current-profile', // TODO: Get from profile context
      elementPath: element.path,
      types: newTypes,
    };
  },
  target: updateTypeConstraintsFx,
});
