import type { SearchFilters } from '@shared/types';
import { createEffect, createEvent, createStore, sample } from 'effector';

export interface CreateSearchStateOptions<T> {
  /**
   * Function to perform the search
   */
  searchFn: (query: string, filters: Partial<SearchFilters>) => Promise<T[]>;

  /**
   * Initial query value
   */
  initialQuery?: string;

  /**
   * Initial filters
   */
  initialFilters?: Partial<SearchFilters>;

  /**
   * Whether to automatically search when query or filters change
   * @default true
   */
  autoSearch?: boolean;
}

export interface SearchState<T> {
  /**
   * Current search query
   */
  $query: ReturnType<typeof createStore<string>>;

  /**
   * Current search filters
   */
  $filters: ReturnType<typeof createStore<Partial<SearchFilters>>>;

  /**
   * Search results
   */
  $results: ReturnType<typeof createStore<T[]>>;

  /**
   * Loading state
   */
  $isLoading: ReturnType<typeof createStore<boolean>>;

  /**
   * Selected result index
   */
  $selectedIndex: ReturnType<typeof createStore<number>>;

  /**
   * Event: Query changed
   */
  queryChanged: ReturnType<typeof createEvent<string>>;

  /**
   * Event: Filters changed
   */
  filtersChanged: ReturnType<typeof createEvent<Partial<SearchFilters>>>;

  /**
   * Event: Search submitted
   */
  searchSubmitted: ReturnType<typeof createEvent<void>>;

  /**
   * Event: Result selected
   */
  resultSelected: ReturnType<typeof createEvent<number>>;

  /**
   * Event: Selected index changed
   */
  selectedIndexChanged: ReturnType<typeof createEvent<number>>;

  /**
   * Event: Clear search
   */
  searchCleared: ReturnType<typeof createEvent<void>>;

  /**
   * Effect: Perform search
   */
  searchFx: ReturnType<
    typeof createEffect<{ query: string; filters: Partial<SearchFilters> }, T[]>
  >;
}

/**
 * Creates a search state management system using effector
 */
export function createSearchState<T>({
  searchFn,
  initialQuery = '',
  initialFilters = {},
  autoSearch = true,
}: CreateSearchStateOptions<T>): SearchState<T> {
  // Events
  const queryChanged = createEvent<string>();
  const filtersChanged = createEvent<Partial<SearchFilters>>();
  const searchSubmitted = createEvent<void>();
  const resultSelected = createEvent<number>();
  const selectedIndexChanged = createEvent<number>();
  const searchCleared = createEvent<void>();

  // Effect
  const searchFx = createEffect(
    async ({ query, filters }: { query: string; filters: Partial<SearchFilters> }) => {
      return await searchFn(query, filters);
    }
  );

  // Stores
  const $query = createStore(initialQuery);
  const $filters = createStore(initialFilters);
  const $results = createStore<T[]>([]);
  const $isLoading = searchFx.pending;
  const $selectedIndex = createStore(-1);

  // Update query
  $query.on(queryChanged, (_, query) => query);

  // Update filters
  $filters.on(filtersChanged, (current, filters) => ({
    ...current,
    ...filters,
  }));

  // Update results
  $results.on(searchFx.doneData, (_, results) => results);

  // Update selected index
  $selectedIndex.on(selectedIndexChanged, (_, index) => index);
  $selectedIndex.on(resultSelected, (_, index) => index);

  // Reset selected index when results change
  $selectedIndex.reset(searchFx.doneData);

  // Clear search
  $query.reset(searchCleared);
  $filters.reset(searchCleared);
  $results.reset(searchCleared);
  $selectedIndex.reset(searchCleared);

  // Auto-search when query or filters change
  if (autoSearch) {
    sample({
      clock: [queryChanged, filtersChanged],
      source: { query: $query, filters: $filters },
      fn: ({ query, filters }) => ({ query, filters }),
      target: searchFx,
    });
  }

  // Manual search
  sample({
    clock: searchSubmitted,
    source: { query: $query, filters: $filters },
    fn: ({ query, filters }) => ({ query, filters }),
    target: searchFx,
  });

  return {
    $query,
    $filters,
    $results,
    $isLoading,
    $selectedIndex,
    queryChanged,
    filtersChanged,
    searchSubmitted,
    resultSelected,
    selectedIndexChanged,
    searchCleared,
    searchFx,
  };
}
