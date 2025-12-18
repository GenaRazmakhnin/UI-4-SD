import { Button, Group, MultiSelect, Select, Stack, Text } from '@mantine/core';
import type { SearchFilters as SearchFiltersType } from '@shared/types';
import { IconFilterOff } from '@tabler/icons-react';
import styles from './SearchFilters.module.css';

export interface SearchFiltersProps {
  /**
   * Current filter values
   */
  filters: Partial<SearchFiltersType>;

  /**
   * Callback when filters change
   */
  onChange: (filters: Partial<SearchFiltersType>) => void;

  /**
   * Available resource types to filter by
   */
  resourceTypes?: string[];

  /**
   * Available packages to filter by
   */
  packages?: string[];

  /**
   * Available FHIR versions to filter by
   */
  fhirVersions?: string[];

  /**
   * Whether to show the resource type filter
   * @default true
   */
  showResourceType?: boolean;

  /**
   * Whether to show the package filter
   * @default true
   */
  showPackage?: boolean;

  /**
   * Whether to show the FHIR version filter
   * @default true
   */
  showFhirVersion?: boolean;

  /**
   * Layout orientation
   * @default 'horizontal'
   */
  layout?: 'horizontal' | 'vertical';
}

const DEFAULT_FHIR_VERSIONS = [
  { value: '4.0.1', label: 'R4 (4.0.1)' },
  { value: '4.3.0', label: 'R4B (4.3.0)' },
  { value: '5.0.0', label: 'R5 (5.0.0)' },
];

export function SearchFilters({
  filters,
  onChange,
  resourceTypes = [],
  packages = [],
  fhirVersions = DEFAULT_FHIR_VERSIONS.map((v) => v.value),
  showResourceType = true,
  showPackage = true,
  showFhirVersion = true,
  layout = 'horizontal',
}: SearchFiltersProps) {
  // Convert arrays to select options
  const resourceTypeOptions = resourceTypes.map((type) => ({
    value: type,
    label: type,
  }));

  const packageOptions = packages.map((pkg) => ({
    value: pkg,
    label: pkg,
  }));

  const fhirVersionOptions = DEFAULT_FHIR_VERSIONS.filter((v) => fhirVersions.includes(v.value));

  // Handle filter changes
  const handleResourceTypeChange = (value: string[] | null) => {
    onChange({
      ...filters,
      type: value || undefined,
    });
  };

  const handlePackageChange = (value: string[] | null) => {
    onChange({
      ...filters,
      package: value || undefined,
    });
  };

  const handleFhirVersionChange = (value: string[] | null) => {
    onChange({
      ...filters,
      fhirVersion: value || undefined,
    });
  };

  // Clear all filters
  const handleClearAll = () => {
    onChange({});
  };

  // Check if any filters are active
  const hasActiveFilters =
    (filters.type && filters.type.length > 0) ||
    (filters.package && filters.package.length > 0) ||
    (filters.fhirVersion && filters.fhirVersion.length > 0);

  const Container = layout === 'horizontal' ? Group : Stack;

  return (
    <div className={styles.container}>
      <Container gap="md" align={layout === 'horizontal' ? 'flex-start' : 'stretch'}>
        {/* Resource Type Filter */}
        {showResourceType && resourceTypeOptions.length > 0 && (
          <MultiSelect
            label="Resource Type"
            placeholder="All types"
            data={resourceTypeOptions}
            value={filters.type || []}
            onChange={handleResourceTypeChange}
            clearable
            searchable
            className={styles.filter}
          />
        )}

        {/* Package Filter */}
        {showPackage && packageOptions.length > 0 && (
          <MultiSelect
            label="Package"
            placeholder="All packages"
            data={packageOptions}
            value={filters.package || []}
            onChange={handlePackageChange}
            clearable
            searchable
            className={styles.filter}
          />
        )}

        {/* FHIR Version Filter */}
        {showFhirVersion && fhirVersionOptions.length > 0 && (
          <MultiSelect
            label="FHIR Version"
            placeholder="All versions"
            data={fhirVersionOptions}
            value={filters.fhirVersion || []}
            onChange={handleFhirVersionChange}
            clearable
            className={styles.filter}
          />
        )}

        {/* Clear All Button */}
        {hasActiveFilters && (
          <Button
            variant="subtle"
            color="gray"
            leftSection={<IconFilterOff size={16} />}
            onClick={handleClearAll}
            className={styles.clearButton}
          >
            Clear Filters
          </Button>
        )}
      </Container>

      {/* Active filters summary */}
      {hasActiveFilters && (
        <Text size="xs" c="dimmed" mt="xs">
          {getFilterSummary(filters)}
        </Text>
      )}
    </div>
  );
}

/**
 * Get a human-readable summary of active filters
 */
function getFilterSummary(filters: Partial<SearchFiltersType>): string {
  const parts: string[] = [];

  if (filters.type && filters.type.length > 0) {
    parts.push(`${filters.type.length} type${filters.type.length !== 1 ? 's' : ''}`);
  }

  if (filters.package && filters.package.length > 0) {
    parts.push(`${filters.package.length} package${filters.package.length !== 1 ? 's' : ''}`);
  }

  if (filters.fhirVersion && filters.fhirVersion.length > 0) {
    parts.push(
      `${filters.fhirVersion.length} version${filters.fhirVersion.length !== 1 ? 's' : ''}`
    );
  }

  if (parts.length === 0) return '';

  return `Filtering by: ${parts.join(', ')}`;
}
