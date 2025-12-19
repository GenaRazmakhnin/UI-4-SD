import {
  ActionIcon,
  Badge,
  Box,
  Card,
  Chip,
  Group,
  Loader,
  MultiSelect,
  ScrollArea,
  Stack,
  Text,
  TextInput,
  Tooltip,
} from '@mantine/core';
import { useDebouncedValue } from '@mantine/hooks';
import { api } from '@shared/api';
import type { SearchResponseWithFacets, SearchResult } from '@shared/types';
import {
  IconBox,
  IconCode,
  IconFileCode,
  IconList,
  IconPuzzle,
  IconSearch,
  IconX,
} from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import styles from './ResourceSearchPanel.module.css';

export type ResourceType = 'Extension' | 'ValueSet' | 'Profile' | 'CodeSystem' | 'all';

interface ResourceSearchPanelProps {
  onSelect?: (resource: SearchResult) => void;
  resourceType?: ResourceType;
  packages?: string[];
  height?: number | string;
}

const RESOURCE_TYPES: { value: ResourceType; label: string; icon: React.ReactNode }[] = [
  { value: 'all', label: 'All', icon: <IconBox size={14} /> },
  { value: 'Profile', label: 'Profiles', icon: <IconFileCode size={14} /> },
  { value: 'Extension', label: 'Extensions', icon: <IconPuzzle size={14} /> },
  { value: 'ValueSet', label: 'ValueSets', icon: <IconList size={14} /> },
  { value: 'CodeSystem', label: 'CodeSystems', icon: <IconCode size={14} /> },
];

export function ResourceSearchPanel({
  onSelect,
  resourceType: initialResourceType = 'all',
  packages: initialPackages,
  height = 500,
}: ResourceSearchPanelProps) {
  const [searchInput, setSearchInput] = useState('');
  const [debouncedSearch] = useDebouncedValue(searchInput, 300);
  const [selectedType, setSelectedType] = useState<ResourceType>(initialResourceType);
  const [selectedPackages, setSelectedPackages] = useState<string[]>(initialPackages || []);

  // Query for resources
  const { data, isLoading, error } = useQuery({
    queryKey: ['search', 'resources', debouncedSearch, selectedType, selectedPackages],
    queryFn: async (): Promise<SearchResponseWithFacets<SearchResult>> => {
      const types = selectedType === 'all' ? undefined : [getResourceType(selectedType)];
      return api.search.resources(debouncedSearch, {
        type: types,
        package: selectedPackages.length > 0 ? selectedPackages : undefined,
      });
    },
    enabled: debouncedSearch.length > 0,
    staleTime: 5 * 60 * 1000,
  });

  const results = data?.results || [];
  const facets = data?.facets;

  // Build package options from facets
  const packageOptions = facets?.packages
    ? Object.entries(facets.packages).map(([name, count]) => ({
        value: name,
        label: `${name} (${count})`,
      }))
    : [];

  const handleResourceClick = (resource: SearchResult) => {
    onSelect?.(resource);
  };

  const handleClearSearch = () => {
    setSearchInput('');
  };

  return (
    <Stack gap="md" h={height} className={styles.container}>
      {/* Search input */}
      <TextInput
        placeholder="Search resources by name, URL, or description..."
        leftSection={<IconSearch size={16} />}
        rightSection={
          searchInput ? (
            <ActionIcon size="sm" variant="subtle" onClick={handleClearSearch}>
              <IconX size={14} />
            </ActionIcon>
          ) : isLoading ? (
            <Loader size="xs" />
          ) : null
        }
        value={searchInput}
        onChange={(e) => setSearchInput(e.currentTarget.value)}
      />

      {/* Resource type filter */}
      <Chip.Group
        multiple={false}
        value={selectedType}
        onChange={(v) => setSelectedType(v as ResourceType)}
      >
        <Group gap="xs">
          {RESOURCE_TYPES.map((type) => (
            <Chip key={type.value} value={type.value} size="sm" variant="light" icon={type.icon}>
              {type.label}
            </Chip>
          ))}
        </Group>
      </Chip.Group>

      {/* Package filter */}
      {packageOptions.length > 0 && (
        <MultiSelect
          placeholder="Filter by package..."
          data={packageOptions}
          value={selectedPackages}
          onChange={setSelectedPackages}
          size="xs"
          clearable
          searchable
          maxDropdownHeight={200}
        />
      )}

      {/* Results */}
      <ScrollArea style={{ flex: 1 }} offsetScrollbars>
        {!debouncedSearch && (
          <Box className={styles.emptyState}>
            <IconSearch size={48} stroke={1.5} className={styles.emptyIcon} />
            <Text size="lg" fw={500} mt="md">
              Search Resources
            </Text>
            <Text size="sm" c="dimmed" mt="xs">
              Enter a search term to find profiles, extensions, value sets, and more
            </Text>
          </Box>
        )}

        {error && (
          <Box className={styles.emptyState}>
            <Text size="sm" c="red">
              Error loading results: {error instanceof Error ? error.message : 'Unknown error'}
            </Text>
          </Box>
        )}

        {debouncedSearch && !isLoading && results.length === 0 && !error && (
          <Box className={styles.emptyState}>
            <IconSearch size={48} stroke={1.5} className={styles.emptyIcon} />
            <Text size="lg" fw={500} mt="md">
              No resources found
            </Text>
            <Text size="sm" c="dimmed" mt="xs">
              Try a different search term or adjust filters
            </Text>
          </Box>
        )}

        <Stack gap="sm">
          {results.map((resource) => (
            <ResourceCard
              key={`${resource.id}-${resource.url}`}
              resource={resource}
              onClick={() => handleResourceClick(resource)}
            />
          ))}
        </Stack>
      </ScrollArea>

      {/* Results count */}
      {data && debouncedSearch && (
        <Text size="xs" c="dimmed" ta="center">
          {data.total_count} result{data.total_count !== 1 ? 's' : ''} found
        </Text>
      )}
    </Stack>
  );
}

interface ResourceCardProps {
  resource: SearchResult;
  onClick: () => void;
}

function ResourceCard({ resource, onClick }: ResourceCardProps) {
  const typeIcon = getTypeIcon(resource.type);
  const typeColor = getTypeColor(resource.type);

  return (
    <Card className={styles.resourceCard} padding="sm" radius="sm" withBorder onClick={onClick}>
      <Group justify="space-between" wrap="nowrap" gap="sm">
        <Box style={{ flex: 1, minWidth: 0 }}>
          <Group gap="xs" mb={4}>
            <Text fw={600} size="sm" truncate>
              {resource.title || resource.name}
            </Text>
            <Badge size="xs" variant="light" color={typeColor} leftSection={typeIcon}>
              {resource.type}
            </Badge>
          </Group>

          <Tooltip label={resource.url} multiline w={400}>
            <Text size="xs" c="dimmed" truncate className={styles.url}>
              {resource.url}
            </Text>
          </Tooltip>

          {resource.description && (
            <Text size="xs" c="dimmed" mt="xs" lineClamp={2}>
              {resource.description}
            </Text>
          )}

          {resource.package && (
            <Group gap="xs" mt="xs">
              <Badge size="xs" variant="outline" color="gray">
                {resource.package}
              </Badge>
            </Group>
          )}
        </Box>
      </Group>
    </Card>
  );
}

function getResourceType(type: ResourceType): string {
  switch (type) {
    case 'Profile':
      return 'StructureDefinition';
    case 'Extension':
      return 'Extension';
    case 'ValueSet':
      return 'ValueSet';
    case 'CodeSystem':
      return 'CodeSystem';
    default:
      return '';
  }
}

function getTypeIcon(type: string): React.ReactNode {
  switch (type) {
    case 'profile':
    case 'Profile':
    case 'StructureDefinition':
      return <IconFileCode size={10} />;
    case 'extension':
    case 'Extension':
      return <IconPuzzle size={10} />;
    case 'valueset':
    case 'ValueSet':
      return <IconList size={10} />;
    case 'CodeSystem':
      return <IconCode size={10} />;
    default:
      return <IconBox size={10} />;
  }
}

function getTypeColor(type: string): string {
  switch (type) {
    case 'profile':
    case 'Profile':
    case 'StructureDefinition':
      return 'blue';
    case 'extension':
    case 'Extension':
      return 'violet';
    case 'valueset':
    case 'ValueSet':
      return 'green';
    case 'CodeSystem':
      return 'orange';
    default:
      return 'gray';
  }
}
