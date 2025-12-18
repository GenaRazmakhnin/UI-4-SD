import { Loader, Pagination, ScrollArea, Stack, Text } from '@mantine/core';
import { type ReactNode, useEffect, useRef, useState } from 'react';
import styles from './SearchResultsList.module.css';

export interface SearchResultsListProps<T> {
  /**
   * Array of search results
   */
  results: T[];

  /**
   * Render function for each result item
   */
  renderItem: (item: T, index: number, isSelected: boolean) => ReactNode;

  /**
   * Loading state
   */
  loading?: boolean;

  /**
   * Empty state component
   */
  emptyState?: ReactNode;

  /**
   * Height of the scrollable area
   */
  height?: number | string;

  /**
   * Enable pagination
   * @default false
   */
  enablePagination?: boolean;

  /**
   * Items per page (when pagination is enabled)
   * @default 20
   */
  itemsPerPage?: number;

  /**
   * Callback when an item is selected
   */
  onItemSelect?: (item: T, index: number) => void;

  /**
   * Currently selected index
   */
  selectedIndex?: number;

  /**
   * Callback when selected index changes
   */
  onSelectedIndexChange?: (index: number) => void;

  /**
   * Gap between items
   * @default 'sm'
   */
  gap?: 'xs' | 'sm' | 'md' | 'lg';

  /**
   * Whether to enable keyboard navigation
   * @default true
   */
  enableKeyboardNav?: boolean;
}

export function SearchResultsList<T>({
  results,
  renderItem,
  loading = false,
  emptyState,
  height = 500,
  enablePagination = false,
  itemsPerPage = 20,
  onItemSelect,
  selectedIndex = -1,
  onSelectedIndexChange,
  gap = 'sm',
  enableKeyboardNav = true,
}: SearchResultsListProps<T>) {
  const [currentPage, setCurrentPage] = useState(1);
  const containerRef = useRef<HTMLDivElement>(null);

  // Calculate pagination
  const totalPages = enablePagination ? Math.ceil(results.length / itemsPerPage) : 1;
  const startIndex = enablePagination ? (currentPage - 1) * itemsPerPage : 0;
  const endIndex = enablePagination
    ? Math.min(startIndex + itemsPerPage, results.length)
    : results.length;
  const visibleResults = results.slice(startIndex, endIndex);

  // Handle keyboard navigation
  useEffect(() => {
    if (!enableKeyboardNav || results.length === 0) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      // Only handle if the container or its children have focus
      if (!containerRef.current?.contains(document.activeElement)) return;

      switch (event.key) {
        case 'ArrowDown':
          event.preventDefault();
          if (selectedIndex < visibleResults.length - 1) {
            onSelectedIndexChange?.(selectedIndex + 1);
          }
          break;

        case 'ArrowUp':
          event.preventDefault();
          if (selectedIndex > 0) {
            onSelectedIndexChange?.(selectedIndex - 1);
          }
          break;

        case 'Enter':
          event.preventDefault();
          if (selectedIndex >= 0 && selectedIndex < visibleResults.length) {
            onItemSelect?.(visibleResults[selectedIndex], selectedIndex);
          }
          break;
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [
    enableKeyboardNav,
    selectedIndex,
    visibleResults,
    onSelectedIndexChange,
    onItemSelect,
    results.length,
  ]);

  // Reset to first page when results change
  useEffect(() => {
    setCurrentPage(1);
  }, [results]);

  // Loading state
  if (loading) {
    return (
      <div className={styles.centerContent} style={{ height }}>
        <Loader size="md" />
        <Text size="sm" c="dimmed" mt="md">
          Searching...
        </Text>
      </div>
    );
  }

  // Empty state
  if (results.length === 0) {
    return (
      <div className={styles.centerContent} style={{ height }}>
        {emptyState || (
          <Text size="sm" c="dimmed">
            No results found
          </Text>
        )}
      </div>
    );
  }

  return (
    <div ref={containerRef} className={styles.container}>
      <ScrollArea h={height} className={styles.scrollArea}>
        <Stack gap={gap}>
          {visibleResults.map((item, index) => (
            <div
              key={startIndex + index}
              className={`${styles.item} ${selectedIndex === index ? styles.selected : ''}`}
              onClick={() => onItemSelect?.(item, index)}
              onKeyPress={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  onItemSelect?.(item, index);
                }
              }}
            >
              {renderItem(item, startIndex + index, selectedIndex === index)}
            </div>
          ))}
        </Stack>
      </ScrollArea>

      {/* Pagination */}
      {enablePagination && totalPages > 1 && (
        <div className={styles.pagination}>
          <Pagination value={currentPage} onChange={setCurrentPage} total={totalPages} size="sm" />
          <Text size="xs" c="dimmed" mt="xs">
            Showing {startIndex + 1}-{endIndex} of {results.length} results
          </Text>
        </div>
      )}

      {/* Results count (when no pagination) */}
      {!enablePagination && results.length > 0 && (
        <Text size="xs" c="dimmed" mt="xs" className={styles.resultsCount}>
          {results.length} result{results.length !== 1 ? 's' : ''}
        </Text>
      )}
    </div>
  );
}
