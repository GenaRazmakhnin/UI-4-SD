import {
  ActionIcon,
  Badge,
  Box,
  Button,
  Card,
  Group,
  Loader,
  ScrollArea,
  SegmentedControl,
  Select,
  Stack,
  Text,
  TextInput,
  Tooltip,
} from '@mantine/core';
import { useDebouncedValue } from '@mantine/hooks';
import type { Package } from '@shared/types';
import { IconCheck, IconDownload, IconPackage, IconSearch, IconX } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect, useState } from 'react';
import {
  $filters,
  $installProgress,
  $isSearching,
  $searchResults,
  filtersChanged,
  installRequested,
  type PackageSortBy,
  packageSelected,
  registrySearchTriggered,
} from '../model';
import styles from './PackageSearch.module.css';

interface PackageSearchProps {
  onPackageClick?: (pkg: Package) => void;
}

export function PackageSearch({ onPackageClick }: PackageSearchProps) {
  const [searchInput, setSearchInput] = useState('');
  const [debouncedSearch] = useDebouncedValue(searchInput, 300);

  const [
    searchResults,
    isSearching,
    filters,
    installProgress,
    onFiltersChanged,
    onInstall,
    onPackageSelected,
    onRegistrySearch,
  ] = useUnit([
    $searchResults,
    $isSearching,
    $filters,
    $installProgress,
    filtersChanged,
    installRequested,
    packageSelected,
    registrySearchTriggered,
  ]);

  // Trigger search when debounced value changes
  useEffect(() => {
    if (debouncedSearch) {
      onRegistrySearch(debouncedSearch);
    }
  }, [debouncedSearch, onRegistrySearch]);

  const handleSortChange = (value: string) => {
    onFiltersChanged({ sortBy: value as PackageSortBy });
    if (debouncedSearch) {
      onRegistrySearch(debouncedSearch);
    }
  };

  const handleFhirVersionChange = (value: string | null) => {
    onFiltersChanged({ fhirVersion: value || undefined });
    if (debouncedSearch) {
      onRegistrySearch(debouncedSearch);
    }
  };

  const handlePackageClick = (pkg: Package) => {
    onPackageSelected(pkg.id);
    onPackageClick?.(pkg);
  };

  const handleInstall = (e: React.MouseEvent, packageId: string) => {
    e.stopPropagation();
    onInstall(packageId);
  };

  const formatDownloadCount = (count?: number) => {
    if (!count) return '';
    if (count >= 1000000) return `${(count / 1000000).toFixed(1)}M`;
    if (count >= 1000) return `${(count / 1000).toFixed(0)}K`;
    return count.toString();
  };

  return (
    <Stack gap="md" h="100%" p="sm">
      {/* Search input */}
      <TextInput
        placeholder="Search packages.fhir.org..."
        leftSection={<IconSearch size={16} />}
        rightSection={
          searchInput ? (
            <ActionIcon size="sm" variant="subtle" onClick={() => setSearchInput('')}>
              <IconX size={14} />
            </ActionIcon>
          ) : isSearching ? (
            <Loader size="xs" />
          ) : null
        }
        value={searchInput}
        onChange={(e) => setSearchInput(e.currentTarget.value)}
      />

      {/* Filters */}
      <Group gap="md">
        <Select
          placeholder="FHIR Version"
          size="xs"
          clearable
          data={[
            { value: '4.0.1', label: 'R4 (4.0.1)' },
            { value: '5.0.0', label: 'R5 (5.0.0)' },
            { value: '3.0.2', label: 'STU3 (3.0.2)' },
          ]}
          value={filters.fhirVersion || null}
          onChange={handleFhirVersionChange}
          style={{ width: 140 }}
        />

        <SegmentedControl
          size="xs"
          value={filters.sortBy}
          onChange={handleSortChange}
          data={[
            { value: 'relevance', label: 'Relevance' },
            { value: 'downloads', label: 'Downloads' },
            { value: 'date', label: 'Recent' },
          ]}
        />
      </Group>

      {/* Results */}
      <ScrollArea style={{ flex: 1 }} offsetScrollbars>
        {!debouncedSearch && !isSearching && (
          <Box className={styles.emptyState}>
            <IconPackage size={48} stroke={1.5} className={styles.emptyIcon} />
            <Text size="lg" fw={500} mt="md">
              Search FHIR Packages
            </Text>
            <Text size="sm" c="dimmed" mt="xs">
              Enter a package name or keyword to search the registry
            </Text>
          </Box>
        )}

        {debouncedSearch && !isSearching && searchResults.length === 0 && (
          <Box className={styles.emptyState}>
            <IconSearch size={48} stroke={1.5} className={styles.emptyIcon} />
            <Text size="lg" fw={500} mt="md">
              No packages found
            </Text>
            <Text size="sm" c="dimmed" mt="xs">
              Try a different search term or adjust filters
            </Text>
          </Box>
        )}

        <Stack gap="sm" p="sm">
          {searchResults.map((pkg) => {
            const progress = installProgress[pkg.id];
            const isInstalling = progress?.status === 'installing';
            const justInstalled = progress?.status === 'installed';

            return (
              <Card
                key={pkg.id}
                className={styles.resultCard}
                padding="md"
                radius="md"
                withBorder
                onClick={() => handlePackageClick(pkg)}
              >
                <Group justify="space-between" wrap="nowrap">
                  <Box style={{ flex: 1, minWidth: 0 }}>
                    <Group gap="xs" mb={4}>
                      <Text fw={600} truncate>
                        {pkg.name}
                      </Text>
                      <Badge size="sm" variant="light">
                        {pkg.version}
                      </Badge>
                      {pkg.installed && (
                        <Badge size="sm" color="green" variant="filled">
                          Installed
                        </Badge>
                      )}
                    </Group>

                    <Text size="sm" c="dimmed" lineClamp={2}>
                      {pkg.description}
                    </Text>

                    <Group gap="md" mt="sm">
                      <Group gap={4}>
                        <Text size="xs" c="dimmed">
                          FHIR
                        </Text>
                        <Badge size="xs" variant="outline" color="gray">
                          {pkg.fhirVersion}
                        </Badge>
                      </Group>

                      {pkg.publisher && (
                        <Text size="xs" c="dimmed">
                          {pkg.publisher}
                        </Text>
                      )}

                      {pkg.downloadCount && (
                        <Tooltip label={`${pkg.downloadCount.toLocaleString()} downloads`}>
                          <Group gap={4}>
                            <IconDownload size={12} />
                            <Text size="xs" c="dimmed">
                              {formatDownloadCount(pkg.downloadCount)}
                            </Text>
                          </Group>
                        </Tooltip>
                      )}
                    </Group>
                  </Box>

                  <Box>
                    {pkg.installed || justInstalled ? (
                      <Button
                        size="xs"
                        variant="light"
                        color="green"
                        leftSection={<IconCheck size={14} />}
                        disabled
                      >
                        Installed
                      </Button>
                    ) : (
                      <Button
                        size="xs"
                        variant="filled"
                        leftSection={<IconDownload size={14} />}
                        onClick={(e) => handleInstall(e, pkg.id)}
                        loading={isInstalling}
                      >
                        Install
                      </Button>
                    )}
                  </Box>
                </Group>
              </Card>
            );
          })}
        </Stack>
      </ScrollArea>
    </Stack>
  );
}
