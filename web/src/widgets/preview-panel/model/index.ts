import { api } from '@shared/api';
import type { ExportResult } from '@shared/types';
import { createEffect, createEvent, createStore, sample } from 'effector';
import { debounce } from 'patronum';

/**
 * Preview tab types
 */
export type PreviewTab = 'json' | 'fsh' | 'diff';

/**
 * Diff view mode
 */
export type DiffMode = 'side-by-side' | 'unified';

/**
 * Export mode
 */
export type ExportMode = 'differential' | 'snapshot' | 'both';

/**
 * Preview panel state
 */
export interface PreviewState {
  isFullscreen: boolean;
  searchQuery: string;
  showLineNumbers: boolean;
  showMinimap: boolean;
  wordWrap: boolean;
}

// Stores
export const $activePreviewTab = createStore<PreviewTab>('json');
export const $diffMode = createStore<DiffMode>('side-by-side');
export const $exportMode = createStore<ExportMode>('differential');
export const $previewState = createStore<PreviewState>({
  isFullscreen: false,
  searchQuery: '',
  showLineNumbers: true,
  showMinimap: true,
  wordWrap: false,
});

// Preview content stores
export const $sdJsonContent = createStore<string>('');
export const $fshContent = createStore<string>('');
export const $baseDefinitionContent = createStore<string>('');
export const $isPreviewLoading = createStore<boolean>(false);
export const $previewError = createStore<string | null>(null);

// Events
export const previewTabChanged = createEvent<PreviewTab>();
export const diffModeChanged = createEvent<DiffMode>();
export const exportModeChanged = createEvent<ExportMode>();
export const fullscreenToggled = createEvent();
export const searchQueryChanged = createEvent<string>();
export const settingChanged = createEvent<Partial<PreviewState>>();
export const previewRequested = createEvent<string>(); // profileId
export const previewReset = createEvent();

// Debounced preview request (500ms as per requirements)
const debouncedPreviewRequest = debounce({
  source: previewRequested,
  timeout: 500,
});

// Effects
export const fetchSDJsonFx = createEffect<string, ExportResult>(async (profileId) => {
  return api.export.toSD(profileId);
});

export const fetchFSHFx = createEffect<string, ExportResult>(async (profileId) => {
  return api.export.toFSH(profileId);
});

export const fetchBaseDefinitionFx = createEffect<string, string>(async (profileId) => {
  // Fetch base definition for diff comparison
  const profile = await api.profiles.get(profileId);
  // In real implementation, we'd fetch the base SD from the package
  // For now, return a simplified base structure
  return JSON.stringify(
    {
      resourceType: 'StructureDefinition',
      id: profile.baseDefinition.split('/').pop(),
      url: profile.baseDefinition,
      name: profile.baseDefinition.split('/').pop(),
      status: 'active',
      kind: 'resource',
      abstract: false,
      type: profile.baseDefinition.split('/').pop(),
    },
    null,
    2
  );
});

// Store updates
$activePreviewTab.on(previewTabChanged, (_, tab) => tab);
$diffMode.on(diffModeChanged, (_, mode) => mode);
$exportMode.on(exportModeChanged, (_, mode) => mode);

$previewState
  .on(fullscreenToggled, (state) => ({
    ...state,
    isFullscreen: !state.isFullscreen,
  }))
  .on(searchQueryChanged, (state, query) => ({
    ...state,
    searchQuery: query,
  }))
  .on(settingChanged, (state, updates) => ({
    ...state,
    ...updates,
  }));

// Loading state
$isPreviewLoading
  .on(fetchSDJsonFx.pending, (_, pending) => pending)
  .on(fetchFSHFx.pending, (_, pending) => pending);

// Error handling
$previewError
  .on(fetchSDJsonFx.failData, (_, error) => error.message)
  .on(fetchFSHFx.failData, (_, error) => error.message)
  .reset(previewRequested);

// Content updates
$sdJsonContent.on(fetchSDJsonFx.doneData, (_, result) => result.content).reset(previewReset);

$fshContent.on(fetchFSHFx.doneData, (_, result) => result.content).reset(previewReset);

$baseDefinitionContent
  .on(fetchBaseDefinitionFx.doneData, (_, content) => content)
  .reset(previewReset);

// Trigger fetches on debounced preview request
sample({
  clock: debouncedPreviewRequest,
  target: [fetchSDJsonFx, fetchFSHFx, fetchBaseDefinitionFx],
});
