import type { Profile } from '@shared/types';

const STORAGE_KEY_PREFIX = 'mock-api';

export const persistence = {
  /**
   * Save profiles to localStorage
   */
  saveProfiles(profiles: Profile[]): void {
    try {
      localStorage.setItem(
        `${STORAGE_KEY_PREFIX}:profiles`,
        JSON.stringify(profiles),
      );
    } catch (error) {
      console.error('[Mock Persistence] Failed to save profiles:', error);
    }
  },

  /**
   * Load profiles from localStorage
   */
  loadProfiles(): Profile[] | null {
    try {
      const data = localStorage.getItem(`${STORAGE_KEY_PREFIX}:profiles`);
      return data ? JSON.parse(data) : null;
    } catch (error) {
      console.error('[Mock Persistence] Failed to load profiles:', error);
      return null;
    }
  },

  /**
   * Clear all mock data
   */
  clear(): void {
    const keys = Object.keys(localStorage).filter((key) =>
      key.startsWith(STORAGE_KEY_PREFIX),
    );
    for (const key of keys) {
      localStorage.removeItem(key);
    }
  },

  /**
   * Save undo/redo stacks
   */
  saveUndoStack(profileId: string, stack: unknown[]): void {
    try {
      localStorage.setItem(
        `${STORAGE_KEY_PREFIX}:undo:${profileId}`,
        JSON.stringify(stack),
      );
    } catch (error) {
      console.error('[Mock Persistence] Failed to save undo stack:', error);
    }
  },

  loadUndoStack(profileId: string): unknown[] {
    try {
      const data = localStorage.getItem(
        `${STORAGE_KEY_PREFIX}:undo:${profileId}`,
      );
      return data ? JSON.parse(data) : [];
    } catch (error) {
      console.error('[Mock Persistence] Failed to load undo stack:', error);
      return [];
    }
  },
};

// Auto-save on window unload
if (typeof window !== 'undefined') {
  window.addEventListener('beforeunload', () => {
    // Auto-save will be triggered by individual stores
  });
}
