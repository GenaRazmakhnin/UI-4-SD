import { useUnit } from 'effector-react';
import type { SearchState } from './createSearchState';

/**
 * Hook to use search state in React components
 */
export function useSearch<T>(searchState: SearchState<T>) {
  const query = useUnit(searchState.$query);
  const filters = useUnit(searchState.$filters);
  const results = useUnit(searchState.$results);
  const isLoading = useUnit(searchState.$isLoading);
  const selectedIndex = useUnit(searchState.$selectedIndex);

  return {
    // State
    query,
    filters,
    results,
    isLoading,
    selectedIndex,

    // Actions
    setQuery: searchState.queryChanged,
    setFilters: searchState.filtersChanged,
    submitSearch: searchState.searchSubmitted,
    selectResult: searchState.resultSelected,
    setSelectedIndex: searchState.selectedIndexChanged,
    clearSearch: searchState.searchCleared,
  };
}
