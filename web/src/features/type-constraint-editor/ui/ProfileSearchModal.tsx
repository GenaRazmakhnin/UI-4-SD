import {
  Badge,
  Button,
  Group,
  Loader,
  Modal,
  ScrollArea,
  Stack,
  Text,
  TextInput,
} from '@mantine/core';
import type { Profile } from '@shared/types';
import { IconSearch } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect, useState } from 'react';
import { $searchLoading, $searchResults, searchProfilesFx } from '../model';
import styles from './ProfileSearchModal.module.css';

interface ProfileSearchModalProps {
  opened: boolean;
  onClose: () => void;
  typeFilter: string | null;
  onProfileSelected: (profileUrl: string) => void;
}

export function ProfileSearchModal({
  opened,
  onClose,
  typeFilter,
  onProfileSelected,
}: ProfileSearchModalProps) {
  const [query, setQuery] = useState('');
  const searchResults = useUnit($searchResults);
  const isLoading = useUnit($searchLoading);

  // Search when modal opens or type filter changes
  useEffect(() => {
    if (opened && typeFilter) {
      searchProfilesFx({ query: '', typeFilter });
    }
  }, [opened, typeFilter]);

  const handleSearch = () => {
    if (typeFilter) {
      searchProfilesFx({ query, typeFilter });
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  return (
    <Modal
      opened={opened}
      onClose={onClose}
      title={`Search Profiles for ${typeFilter || 'Type'}`}
      size="lg"
    >
      <Stack gap="md">
        {/* Search Input */}
        <Group gap="xs">
          <TextInput
            style={{ flex: 1 }}
            placeholder="Search profiles by name, URL, or description..."
            leftSection={<IconSearch size={16} />}
            value={query}
            onChange={(e) => setQuery(e.currentTarget.value)}
            onKeyPress={handleKeyPress}
          />
          <Button size="sm" onClick={handleSearch} loading={isLoading}>
            Search
          </Button>
        </Group>

        {/* Search Results */}
        <ScrollArea h={400}>
          {isLoading ? (
            <Group justify="center" p="xl">
              <Loader size="sm" />
              <Text size="sm" c="dimmed">
                Searching profiles...
              </Text>
            </Group>
          ) : searchResults.length === 0 ? (
            <Text size="sm" c="dimmed" ta="center" p="xl">
              No profiles found. Try a different search term.
            </Text>
          ) : (
            <Stack gap="sm">
              {searchResults.map((profile) => (
                <div
                  key={profile.url}
                  className={styles.profileCard}
                  onClick={() => onProfileSelected(profile.url)}
                >
                  <Group justify="space-between" mb="xs">
                    <Text size="sm" fw={500}>
                      {profile.title || profile.name}
                    </Text>
                    <Badge size="sm" variant="light">
                      {profile.status}
                    </Badge>
                  </Group>

                  <Text size="xs" c="dimmed" className={styles.profileUrl}>
                    {profile.url}
                  </Text>

                  {profile.description && (
                    <Text size="xs" c="dimmed" mt="xs" lineClamp={2}>
                      {profile.description}
                    </Text>
                  )}

                  {profile.publisher && (
                    <Text size="xs" c="dimmed" mt="xs">
                      Publisher: {profile.publisher}
                    </Text>
                  )}
                </div>
              ))}
            </Stack>
          )}
        </ScrollArea>

        {/* Actions */}
        <Group justify="flex-end">
          <Button variant="subtle" onClick={onClose}>
            Cancel
          </Button>
        </Group>
      </Stack>
    </Modal>
  );
}
