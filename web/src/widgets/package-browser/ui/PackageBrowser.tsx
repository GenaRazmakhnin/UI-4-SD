import {
  ActionIcon,
  Badge,
  Box,
  Group,
  Paper,
  SegmentedControl,
  Text,
  TextInput,
  Tooltip,
} from '@mantine/core';
import type { Package, PackageResource } from '@shared/types';
import { IconPackage, IconRefresh, IconSearch, IconX } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect } from 'react';
import {
  $currentView,
  $filters,
  $isLoadingPackages,
  $selectedPackageId,
  $updateCount,
  fetchPackagesFx,
  type PackageView,
  searchQueryChanged,
  viewChanged,
} from '../model';
import styles from './PackageBrowser.module.css';
import { PackageDetails } from './PackageDetails';
import { PackageList } from './PackageList';
import { PackageSearch } from './PackageSearch';

interface PackageBrowserProps {
  height?: number | string;
  onSelectResource?: (resource: PackageResource) => void;
  onPackageInstalled?: (pkg: Package) => void;
}

export function PackageBrowser({ height = '100%', onSelectResource }: PackageBrowserProps) {
  const [
    currentView,
    selectedPackageId,
    updateCount,
    isLoading,
    filters,
    onViewChanged,
    onSearchQueryChanged,
    onFetchPackages,
  ] = useUnit([
    $currentView,
    $selectedPackageId,
    $updateCount,
    $isLoadingPackages,
    $filters,
    viewChanged,
    searchQueryChanged,
    fetchPackagesFx,
  ]);

  // Fetch packages on mount
  useEffect(() => {
    onFetchPackages();
  }, [onFetchPackages]);

  const handleViewChange = (value: string) => {
    onViewChanged(value as PackageView);
  };

  const handleRefresh = () => {
    onFetchPackages();
  };

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onSearchQueryChanged(e.currentTarget.value);
  };

  const handleClearSearch = () => {
    onSearchQueryChanged('');
  };

  // Show details view when a package is selected
  if (selectedPackageId) {
    return (
      <Paper className={styles.container} style={{ height }}>
        <PackageDetails
          onBack={() => onViewChanged(currentView)}
          onSelectResource={onSelectResource}
        />
      </Paper>
    );
  }

  return (
    <Paper className={styles.container} style={{ height }}>
      {/* Header */}
      <Box className={styles.header}>
        <Group justify="space-between" wrap="nowrap">
          <Group gap="sm">
            <IconPackage size={20} />
            <Text size="lg" fw={600}>
              Package Browser
            </Text>
          </Group>

          <Group gap="sm">
            <Tooltip label="Refresh packages">
              <ActionIcon variant="subtle" onClick={handleRefresh} loading={isLoading}>
                <IconRefresh size={18} />
              </ActionIcon>
            </Tooltip>
          </Group>
        </Group>

        {/* View selector */}
        <Group justify="space-between" mt="md">
          <SegmentedControl
            value={currentView}
            onChange={handleViewChange}
            data={[
              {
                value: 'installed',
                label: (
                  <Group gap={6}>
                    <span>Installed</span>
                    {updateCount > 0 && (
                      <Badge size="xs" color="orange" variant="filled">
                        {updateCount}
                      </Badge>
                    )}
                  </Group>
                ),
              },
              { value: 'browse', label: 'Browse Registry' },
            ]}
          />

          {currentView === 'installed' && (
            <TextInput
              placeholder="Filter packages..."
              size="xs"
              leftSection={<IconSearch size={14} />}
              rightSection={
                filters.searchQuery ? (
                  <ActionIcon size="xs" variant="subtle" onClick={handleClearSearch}>
                    <IconX size={12} />
                  </ActionIcon>
                ) : null
              }
              value={filters.searchQuery}
              onChange={handleSearchChange}
              style={{ width: 200 }}
            />
          )}
        </Group>
      </Box>

      {/* Content */}
      <Box className={styles.content}>
        {currentView === 'installed' && <PackageList />}
        {currentView === 'browse' && <PackageSearch />}
      </Box>
    </Paper>
  );
}
