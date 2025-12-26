import { api } from '@shared/api';
import type { ElementNode, Extension, ExtensionUsage } from '@shared/types';
import { $selectedElement } from '@widgets/element-tree';
import { createEffect, createEvent, createStore, sample } from 'effector';

/**
 * Picker open state
 */
export const pickerOpened = createEvent();
export const pickerClosed = createEvent();
export const $pickerOpen = createStore(false)
  .on(pickerOpened, () => true)
  .on(pickerClosed, () => false);

/**
 * Search extensions effect
 */
export const searchExtensionsFx = createEffect(
  async ({
    query,
    packageFilter,
    contextPath,
  }: {
    query: string;
    packageFilter?: string[];
    contextPath?: string;
  }) => {
    const response = await api.search.extensions(query, {
      package: packageFilter,
      contextPath,
    });
    // Handle both old format (array) and new format (with facets)
    return 'results' in response ? response.results : response;
  }
);

/**
 * Extension selected event
 */
export const extensionSelected = createEvent<{
  extension: Extension;
  elementId: string;
}>();

/**
 * Toggle favorite event
 */
export const toggleFavorite = createEvent<string>(); // extensionUrl

/**
 * Search query changed
 */
export const searchQueryChanged = createEvent<string>();

/**
 * Package filter changed
 */
export const packageFilterChanged = createEvent<string[]>();

/**
 * Search results store
 */
export const $searchResults = createStore<Extension[]>([]);
export const $searchLoading = searchExtensionsFx.pending;

$searchResults.on(searchExtensionsFx.doneData, (_, results) => results);

/**
 * Search query store
 */
export const $searchQuery = createStore<string>('');
$searchQuery.on(searchQueryChanged, (_, query) => query);

/**
 * Package filter store
 */
export const $packageFilter = createStore<string[]>([]);
$packageFilter.on(packageFilterChanged, (_, filter) => filter);

/**
 * Extension usage tracking (recent & favorites)
 * Stored in localStorage
 */
const STORAGE_KEY = 'extension-picker-usage';

function loadUsageFromStorage(): Record<string, ExtensionUsage> {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored ? JSON.parse(stored) : {};
  } catch {
    return {};
  }
}

function saveUsageToStorage(usage: Record<string, ExtensionUsage>) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(usage));
  } catch {
    // Ignore storage errors
  }
}

/**
 * Extension usage store
 */
export const $extensionUsage = createStore<Record<string, ExtensionUsage>>(loadUsageFromStorage());

// Update usage when extension is selected
$extensionUsage.on(extensionSelected, (state, { extension }) => {
  const now = new Date().toISOString();
  const existing = state[extension.url];

  const updated = {
    ...state,
    [extension.url]: {
      extensionUrl: extension.url,
      lastUsed: now,
      useCount: (existing?.useCount ?? 0) + 1,
      isFavorite: existing?.isFavorite ?? false,
    },
  };

  saveUsageToStorage(updated);
  return updated;
});

// Toggle favorite
$extensionUsage.on(toggleFavorite, (state, extensionUrl) => {
  const existing = state[extensionUrl];

  const updated = {
    ...state,
    [extensionUrl]: {
      extensionUrl,
      lastUsed: existing?.lastUsed ?? new Date().toISOString(),
      useCount: existing?.useCount ?? 0,
      isFavorite: !(existing?.isFavorite ?? false),
    },
  };

  saveUsageToStorage(updated);
  return updated;
});

/**
 * Derived stores for recent and favorite extensions
 */
export const $recentExtensions = $extensionUsage.map((usage) => {
  return Object.values(usage)
    .filter((u) => u.useCount > 0)
    .sort((a, b) => new Date(b.lastUsed).getTime() - new Date(a.lastUsed).getTime())
    .slice(0, 10)
    .map((u) => u.extensionUrl);
});

export const $favoriteExtensions = $extensionUsage.map((usage) => {
  return Object.values(usage)
    .filter((u) => u.isFavorite)
    .sort((a, b) => new Date(b.lastUsed).getTime() - new Date(a.lastUsed).getTime())
    .map((u) => u.extensionUrl);
});

/**
 * Add extension to element effect
 * This will be implemented to actually modify the profile
 */
const addExtensionToElementFx = createEffect(
  async ({
    profileId,
    elementPath,
    extensionUrl,
  }: {
    profileId: string;
    elementPath: string;
    extensionUrl: string;
  }) => {
    // TODO: Implement actual API call to add extension to element
    // For now, this is a placeholder
    console.log('Adding extension', extensionUrl, 'to element', elementPath);
    return { success: true };
  }
);

/**
 * Handle extension selection - add it to the current element
 */
sample({
  clock: extensionSelected,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { extension }) => ({
    profileId: 'current-profile', // TODO: Get from profile context
    elementPath: element!.path,
    extensionUrl: extension.url,
  }),
  target: addExtensionToElementFx,
});

/**
 * Auto-search when query or filters change
 */
sample({
  clock: [searchQueryChanged, packageFilterChanged],
  source: { query: $searchQuery, packageFilter: $packageFilter },
  fn: ({ query, packageFilter }) => ({ query, packageFilter }),
  target: searchExtensionsFx,
});
