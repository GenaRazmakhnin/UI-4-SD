import { api } from '@shared/api';
import { createEffect, createEvent, createStore, sample } from 'effector';
import { debounce } from 'patronum';

export type SaveStatus = 'idle' | 'saving' | 'saved' | 'error';
export type ExportFormat = 'json' | 'fsh';

export interface ProfileParams {
  projectId: string;
  profileId: string;
}

// Events
export const profileSaveRequested = createEvent();
export const profileValidateRequested = createEvent<string>();
export const profileExportRequested = createEvent<ProfileParams & { format: ExportFormat }>();
export const profileChanged = createEvent();
export const changesCleared = createEvent();
export const saveStatusReset = createEvent();

// Effects
export const saveProfileFx = createEffect(async ({ projectId, profileId }: ProfileParams) => {
  const response = await api.projects.saveProfile(projectId, profileId);
  return { success: true, isDirty: response.isDirty };
});

export const exportProfileFx = createEffect(
  async ({ projectId, profileId, format }: ProfileParams & { format: ExportFormat }) => {
    // Use the real export APIs
    if (format === 'json') {
      const result = await api.export.toSD(projectId, profileId, { pretty: true });
      // Trigger download
      const blob = new Blob([JSON.stringify(result.content, null, 2)], {
        type: 'application/json',
      });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `${profileId}.json`;
      link.click();
      URL.revokeObjectURL(url);
      return { filename: `${profileId}.json`, format };
    } else {
      const result = await api.export.toFSH(projectId, profileId);
      // Trigger download
      const blob = new Blob([result.content], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `${profileId}.fsh`;
      link.click();
      URL.revokeObjectURL(url);
      return { filename: `${profileId}.fsh`, format };
    }
  }
);

// Stores
export const $saveStatus = createStore<SaveStatus>('idle');
export const $hasUnsavedChanges = createStore(false);
export const $lastSavedAt = createStore<Date | null>(null);
export const $isExporting = createStore(false);

// Derived
export const $canSave = $hasUnsavedChanges.map((hasChanges) => hasChanges);

// Save status management
$saveStatus
  .on(saveProfileFx.pending, (_, pending) => (pending ? 'saving' : 'idle'))
  .on(saveProfileFx.done, () => 'saved')
  .on(saveProfileFx.fail, () => 'error')
  .reset(saveStatusReset);

// Auto-reset save status after showing "saved" for a bit
sample({
  clock: saveProfileFx.done,
  target: debounce({
    source: saveProfileFx.done,
    timeout: 2000,
  }),
});

// Unsaved changes tracking
$hasUnsavedChanges
  .on(profileChanged, () => true)
  .on(saveProfileFx.done, () => false)
  .reset(changesCleared);

// Last saved timestamp
$lastSavedAt.on(saveProfileFx.done, () => new Date());

// Export state
$isExporting.on(exportProfileFx.pending, (_, pending) => pending);

// Reset save status after delay
const debouncedStatusReset = debounce({
  source: saveProfileFx.done,
  timeout: 3000,
});

sample({
  clock: debouncedStatusReset,
  target: saveStatusReset,
});
