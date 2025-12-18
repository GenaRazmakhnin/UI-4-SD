import type { ElementNode } from '@shared/types';

/**
 * Quick action identifiers
 */
export type QuickActionId =
  // Cardinality actions
  | 'make-required'
  | 'make-optional'
  | 'allow-multiple'
  | 'make-prohibited'
  // Flag actions
  | 'toggle-must-support'
  | 'toggle-is-modifier'
  | 'toggle-is-summary'
  // Constraint actions
  | 'add-extension'
  | 'create-slice'
  | 'set-binding'
  | 'constrain-type'
  | 'add-pattern'
  | 'add-fixed-value'
  | 'add-invariant';

/**
 * Quick action definition
 */
export interface QuickAction {
  id: QuickActionId;
  label: string;
  shortLabel: string;
  description: string;
  icon: string;
  shortcut?: string;
  category: 'cardinality' | 'flags' | 'constraints';
  requiresConfirmation?: boolean;
  isAvailable: (element: ElementNode) => boolean;
  isActive?: (element: ElementNode) => boolean;
}

/**
 * Recently used item
 */
export interface RecentItem {
  type: 'valueset' | 'extension';
  url: string;
  name: string;
  usedAt: number;
}

/**
 * Quick constraints preferences
 */
export interface QuickConstraintsPreferences {
  favoriteActions: QuickActionId[];
  recentValueSets: RecentItem[];
  recentExtensions: RecentItem[];
  maxRecentItems: number;
  skipConfirmation: boolean;
  showKeyboardHints: boolean;
}

export const DEFAULT_PREFERENCES: QuickConstraintsPreferences = {
  favoriteActions: ['make-required', 'toggle-must-support', 'set-binding'],
  recentValueSets: [],
  recentExtensions: [],
  maxRecentItems: 5,
  skipConfirmation: false,
  showKeyboardHints: true,
};
