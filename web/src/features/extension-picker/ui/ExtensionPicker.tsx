import {
  Badge,
  Button,
  Group,
  Loader,
  Modal,
  MultiSelect,
  ScrollArea,
  Stack,
  Tabs,
  Text,
  TextInput,
} from '@mantine/core';
import type { ElementNode, Extension } from '@shared/types';
import { IconHistory, IconPackage, IconSearch, IconStar } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect, useState } from 'react';
import { calculateExtensionRelevance } from '../lib/validation';
import {
  $favoriteExtensions,
  $packageFilter,
  $recentExtensions,
  $searchLoading,
  $searchQuery,
  $searchResults,
  packageFilterChanged,
  searchExtensionsFx,
  searchQueryChanged,
} from '../model';
import { ExtensionCard } from './ExtensionCard';
import styles from './ExtensionPicker.module.css';

interface ExtensionPickerProps {
  opened: boolean;
  onClose: () => void;
  element: ElementNode;
}

export function ExtensionPicker({ opened, onClose, element }: ExtensionPickerProps) {
  const [activeTab, setActiveTab] = useState<string | null>('search');

  const query = useUnit($searchQuery);
  const packageFilter = useUnit($packageFilter);
  const searchResults = useUnit($searchResults);
  const isLoading = useUnit($searchLoading);
  const recentExtensionUrls = useUnit($recentExtensions);
  const favoriteExtensionUrls = useUnit($favoriteExtensions);

  // Get extension objects for recent/favorites
  const recentExtensions = searchResults.filter((ext) => recentExtensionUrls.includes(ext.url));
  const favoriteExtensions = searchResults.filter((ext) => favoriteExtensionUrls.includes(ext.url));

  // Search on modal open
  useEffect(() => {
    if (opened) {
      searchExtensionsFx({ query, packageFilter });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [opened]);

  // Handle search
  const handleSearch = () => {
    searchExtensionsFx({ query, packageFilter });
  };

  // Handle key press in search input
  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  // Sort results by relevance
  const sortedResults = [...searchResults].sort((a, b) => {
    const scoreA = calculateExtensionRelevance(a, element);
    const scoreB = calculateExtensionRelevance(b, element);
    return scoreB - scoreA;
  });

  // Available packages for filtering
  const availablePackages = Array.from(
    new Set(searchResults.map((ext) => ext.package).filter(Boolean))
  ).map((pkg) => ({
    value: pkg!,
    label: pkg!,
  }));

  return (
    <Modal
      opened={opened}
      onClose={onClose}
      title="Add Extension"
      size="xl"
      className={styles.modal}
    >
      <Stack gap="md">
        {/* Context info */}
        <Group gap="xs" className={styles.contextInfo}>
          <Text size="sm" fw={500}>
            Adding extension to:
          </Text>
          <Badge size="lg" variant="light">
            {element.path}
          </Badge>
        </Group>

        {/* Tabs */}
        <Tabs value={activeTab} onChange={setActiveTab}>
          <Tabs.List>
            <Tabs.Tab value="search" leftSection={<IconSearch size={14} />}>
              Search
            </Tabs.Tab>
            <Tabs.Tab
              value="favorites"
              leftSection={<IconStar size={14} />}
              rightSection={
                favoriteExtensions.length > 0 ? (
                  <Badge size="xs" variant="filled" circle>
                    {favoriteExtensions.length}
                  </Badge>
                ) : null
              }
            >
              Favorites
            </Tabs.Tab>
            <Tabs.Tab
              value="recent"
              leftSection={<IconHistory size={14} />}
              rightSection={
                recentExtensions.length > 0 ? (
                  <Badge size="xs" variant="filled" circle>
                    {recentExtensions.length}
                  </Badge>
                ) : null
              }
            >
              Recent
            </Tabs.Tab>
          </Tabs.List>

          {/* Search tab */}
          <Tabs.Panel value="search" pt="md">
            <Stack gap="md">
              {/* Search controls */}
              <Group grow align="flex-start">
                <TextInput
                  placeholder="Search extensions by name or description..."
                  leftSection={<IconSearch size={16} />}
                  value={query}
                  onChange={(e) => searchQueryChanged(e.currentTarget.value)}
                  onKeyPress={handleKeyPress}
                />

                <MultiSelect
                  placeholder="Filter by package"
                  leftSection={<IconPackage size={16} />}
                  data={availablePackages}
                  value={packageFilter}
                  onChange={packageFilterChanged}
                  clearable
                  searchable
                />

                <Button onClick={handleSearch} loading={isLoading}>
                  Search
                </Button>
              </Group>

              {/* Search results */}
              <ScrollArea h={500} className={styles.results}>
                {isLoading ? (
                  <Group justify="center" p="xl">
                    <Loader size="sm" />
                    <Text size="sm" c="dimmed">
                      Searching extensions...
                    </Text>
                  </Group>
                ) : sortedResults.length === 0 ? (
                  <Text size="sm" c="dimmed" ta="center" p="xl">
                    {query
                      ? 'No extensions found. Try a different search term.'
                      : 'Enter a search term to find extensions.'}
                  </Text>
                ) : (
                  <Stack gap="sm">
                    {sortedResults.map((extension) => (
                      <ExtensionCard
                        key={extension.url}
                        extension={extension}
                        element={element}
                        onSelect={onClose}
                      />
                    ))}
                  </Stack>
                )}
              </ScrollArea>
            </Stack>
          </Tabs.Panel>

          {/* Favorites tab */}
          <Tabs.Panel value="favorites" pt="md">
            <ScrollArea h={500} className={styles.results}>
              {favoriteExtensions.length === 0 ? (
                <Stack align="center" justify="center" p="xl" gap="md">
                  <IconStar size={48} style={{ opacity: 0.3 }} />
                  <Text size="sm" c="dimmed" ta="center">
                    No favorite extensions yet. Star extensions in the search tab to add them here.
                  </Text>
                </Stack>
              ) : (
                <Stack gap="sm">
                  {favoriteExtensions.map((extension) => (
                    <ExtensionCard
                      key={extension.url}
                      extension={extension}
                      element={element}
                      onSelect={onClose}
                    />
                  ))}
                </Stack>
              )}
            </ScrollArea>
          </Tabs.Panel>

          {/* Recent tab */}
          <Tabs.Panel value="recent" pt="md">
            <ScrollArea h={500} className={styles.results}>
              {recentExtensions.length === 0 ? (
                <Stack align="center" justify="center" p="xl" gap="md">
                  <IconHistory size={48} style={{ opacity: 0.3 }} />
                  <Text size="sm" c="dimmed" ta="center">
                    No recent extensions. Extensions you use will appear here.
                  </Text>
                </Stack>
              ) : (
                <Stack gap="sm">
                  {recentExtensions.map((extension) => (
                    <ExtensionCard
                      key={extension.url}
                      extension={extension}
                      element={element}
                      onSelect={onClose}
                    />
                  ))}
                </Stack>
              )}
            </ScrollArea>
          </Tabs.Panel>
        </Tabs>

        {/* Footer actions */}
        <Group justify="flex-end">
          <Button variant="subtle" onClick={onClose}>
            Cancel
          </Button>
        </Group>
      </Stack>
    </Modal>
  );
}
