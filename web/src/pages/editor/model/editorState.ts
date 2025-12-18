import { createEffect, createEvent, createStore, sample } from 'effector';
import { debounce } from 'patronum';

export type SaveStatus = 'idle' | 'saving' | 'saved' | 'error';
export type ExportFormat = 'json' | 'fsh';

// Events
export const profileSaveRequested = createEvent();
export const profileValidateRequested = createEvent<string>();
export const profileExportRequested = createEvent<{ profileId: string; format: ExportFormat }>();
export const profileChanged = createEvent();
export const changesCleared = createEvent();
export const saveStatusReset = createEvent();

// Effects
export const saveProfileFx = createEffect(async (profileId: string) => {
  // TODO: Implement actual save logic
  await new Promise((resolve) => setTimeout(resolve, 1000));
  return { success: true };
});

export const exportProfileFx = createEffect(
  async ({ profileId, format }: { profileId: string; format: ExportFormat }) => {
    // TODO: Implement actual export logic
    await new Promise((resolve) => setTimeout(resolve, 500));

    // Generate filename
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const extension = format === 'json' ? 'json' : 'fsh';
    const filename = `profile-${profileId}-${timestamp}.${extension}`;

    // For now, just log - actual implementation will download the file
    console.log(`Exporting profile ${profileId} as ${format} to ${filename}`);

    return { filename, format };
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
