import {
  ActionIcon,
  Badge,
  Box,
  Card,
  Group,
  ScrollArea,
  SegmentedControl,
  Skeleton,
  Stack,
  Text,
  TextInput,
  Tooltip,
} from '@mantine/core';
import { useDebouncedValue } from '@mantine/hooks';
import type { PackageResource } from '@shared/types';
import {
  IconChevronRight,
  IconCode,
  IconDatabase,
  IconFileCode,
  IconSearch,
  IconX,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect, useState } from 'react';
import {
  $isLoadingResources,
  $packageResources,
  $resourceFilters,
  type ResourceTypeFilter,
  resourceFiltersChanged,
  resourceSelected,
} from '../model';
import styles from './ResourceBrowser.module.css';

interface ResourceBrowserProps {
  packageId: string;
  onSelectResource?: (resource: PackageResource) => void;
}

const resourceTypeIcons: Record<string, React.ReactNode> = {
  StructureDefinition: <IconFileCode size={14} />,
  ValueSet: <IconDatabase size={14} />,
  CodeSystem: <IconCode size={14} />,
  SearchParameter: <IconSearch size={14} />,
};

const resourceTypeColors: Record<string, string> = {
  StructureDefinition: 'blue',
  ValueSet: 'violet',
  CodeSystem: 'green',
  SearchParameter: 'orange',
};

export function ResourceBrowser({ packageId, onSelectResource }: ResourceBrowserProps) {
  const [resources, isLoading, filters, onResourceFiltersChanged, onResourceSelected] = useUnit([
    $packageResources,
    $isLoadingResources,
    $resourceFilters,
    resourceFiltersChanged,
    resourceSelected,
  ]);
  const [searchInput, setSearchInput] = useState('');
  const [debouncedSearch] = useDebouncedValue(searchInput, 300);

  // Update filters when search changes
  useEffect(() => {
    onResourceFiltersChanged({ searchQuery: debouncedSearch });
  }, [debouncedSearch, onResourceFiltersChanged]);

  const handleTypeChange = (value: string) => {
    onResourceFiltersChanged({ type: value as ResourceTypeFilter });
  };

  const handleResourceClick = (resource: PackageResource) => {
    onResourceSelected(resource);
    onSelectResource?.(resource);
  };

  // Group resources by type
  const groupedResources = resources.reduce(
    (acc, resource) => {
      const type = resource.resourceType;
      if (!acc[type]) {
        acc[type] = [];
      }
      acc[type].push(resource);
      return acc;
    },
    {} as Record<string, PackageResource[]>
  );

  const resourceTypes = Object.keys(groupedResources);

  if (isLoading) {
    return (
      <Stack gap="sm" p="sm">
        {[1, 2, 3, 4].map((i) => (
          <Skeleton key={i} height={60} radius="md" />
        ))}
      </Stack>
    );
  }

  return (
    <Stack gap="md" h="100%">
      {/* Filters */}
      <Group gap="md" px="sm">
        <TextInput
          placeholder="Search resources..."
          leftSection={<IconSearch size={16} />}
          rightSection={
            searchInput ? (
              <ActionIcon size="sm" variant="subtle" onClick={() => setSearchInput('')}>
                <IconX size={14} />
              </ActionIcon>
            ) : null
          }
          value={searchInput}
          onChange={(e) => setSearchInput(e.currentTarget.value)}
          style={{ flex: 1 }}
        />

        <SegmentedControl
          size="xs"
          value={filters.type}
          onChange={handleTypeChange}
          data={[
            { value: 'all', label: 'All' },
            { value: 'StructureDefinition', label: 'Profiles' },
            { value: 'ValueSet', label: 'ValueSets' },
            { value: 'CodeSystem', label: 'CodeSystems' },
          ]}
        />
      </Group>

      {/* Resource list */}
      <ScrollArea style={{ flex: 1 }} offsetScrollbars>
        {resources.length === 0 ? (
          <Box className={styles.emptyState}>
            <IconFileCode size={48} stroke={1.5} className={styles.emptyIcon} />
            <Text size="lg" fw={500} mt="md">
              No resources found
            </Text>
            <Text size="sm" c="dimmed" mt="xs">
              {searchInput
                ? 'Try a different search term'
                : 'This package has no resources of the selected type'}
            </Text>
          </Box>
        ) : (
          <Stack gap="sm" p="sm">
            {filters.type === 'all'
              ? // Grouped view when showing all
                resourceTypes.map((type) => (
                  <Box key={type}>
                    <Group gap="xs" mb="xs">
                      {resourceTypeIcons[type]}
                      <Text size="sm" fw={600}>
                        {type}
                      </Text>
                      <Badge size="xs" variant="light">
                        {groupedResources[type].length}
                      </Badge>
                    </Group>
                    <Stack gap="xs">
                      {groupedResources[type].map((resource) => (
                        <ResourceCard
                          key={resource.id}
                          resource={resource}
                          onClick={handleResourceClick}
                        />
                      ))}
                    </Stack>
                  </Box>
                ))
              : // Flat list when filtered by type
                resources.map((resource) => (
                  <ResourceCard
                    key={resource.id}
                    resource={resource}
                    onClick={handleResourceClick}
                  />
                ))}
          </Stack>
        )}
      </ScrollArea>
    </Stack>
  );
}

interface ResourceCardProps {
  resource: PackageResource;
  onClick: (resource: PackageResource) => void;
}

function ResourceCard({ resource, onClick }: ResourceCardProps) {
  const color = resourceTypeColors[resource.resourceType] || 'gray';

  return (
    <Card
      className={styles.resourceCard}
      padding="sm"
      radius="md"
      withBorder
      onClick={() => onClick(resource)}
    >
      <Group justify="space-between" wrap="nowrap">
        <Box style={{ flex: 1, minWidth: 0 }}>
          <Group gap="xs" mb={4}>
            <Text size="sm" fw={500} truncate>
              {resource.title || resource.name}
            </Text>
            <Badge size="xs" variant="light" color={color}>
              {resource.resourceType === 'StructureDefinition' ? 'Profile' : resource.resourceType}
            </Badge>
            {resource.status && (
              <Badge
                size="xs"
                variant="outline"
                color={resource.status === 'active' ? 'green' : 'gray'}
              >
                {resource.status}
              </Badge>
            )}
          </Group>

          {resource.description && (
            <Text size="xs" c="dimmed" lineClamp={1}>
              {resource.description}
            </Text>
          )}

          <Text size="xs" c="dimmed" mt={4} truncate>
            {resource.url}
          </Text>
        </Box>

        <Tooltip label="Use as base profile">
          <ActionIcon variant="subtle" color="gray">
            <IconChevronRight size={16} />
          </ActionIcon>
        </Tooltip>
      </Group>
    </Card>
  );
}
