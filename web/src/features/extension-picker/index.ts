// Export main component

// Export validation utilities
export {
  calculateExtensionRelevance,
  getContextDescription,
  validateExtensionContext,
} from './lib/validation';

// Export model (events, stores)
export {
  $extensionUsage,
  $favoriteExtensions,
  $packageFilter,
  $recentExtensions,
  $searchLoading,
  $searchQuery,
  $searchResults,
  extensionSelected,
  packageFilterChanged,
  searchExtensionsFx,
  searchQueryChanged,
  toggleFavorite,
  pickerOpened,
  pickerClosed,
  $pickerOpen,
} from './model';
export { ExtensionPicker } from './ui/ExtensionPicker';
