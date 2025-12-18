import {
  Badge,
  Button,
  Group,
  Loader,
  Modal,
  ScrollArea,
  Select,
  Stack,
  Text,
  TextInput,
} from '@mantine/core';
import type { ValueSet } from '@shared/types';
import { IconSearch } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect, useState } from 'react';
import { $searchLoading, $searchResults, searchValueSetsFx } from '../model';
import styles from './ValueSetBrowser.module.css';

interface ValueSetBrowserProps {
  opened: boolean;
  onClose: () => void;
  onValueSetSelected: (url: string, name: string) => void;
}

export function ValueSetBrowser({ opened, onClose, onValueSetSelected }: ValueSetBrowserProps) {
  const [query, setQuery] = useState('');
  const [codeSystemFilter, setCodeSystemFilter] = useState<string | null>(null);
  const searchResults = useUnit($searchResults);
  const isLoading = useUnit($searchLoading);

  // Search on modal open
  useEffect(() => {
    if (opened) {
      handleSearch();
    }
  }, [opened]);

  const handleSearch = () => {
    searchValueSetsFx({ query, codeSystemFilter });
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  return (
    <Modal opened={opened} onClose={onClose} title="Browse ValueSets" size="xl">
      <Stack gap="md">
        {/* Search Controls */}
        <Group grow>
          <TextInput
            placeholder="Search ValueSets by name or description..."
            leftSection={<IconSearch size={16} />}
            value={query}
            onChange={(e) => setQuery(e.currentTarget.value)}
            onKeyPress={handleKeyPress}
          />

          <Select
            placeholder="Filter by code system"
            clearable
            value={codeSystemFilter}
            onChange={setCodeSystemFilter}
            data={[
              { value: 'http://snomed.info/sct', label: 'SNOMED CT' },
              { value: 'http://loinc.org', label: 'LOINC' },
              { value: 'http://hl7.org/fhir/sid/icd-10', label: 'ICD-10' },
              { value: 'http://www.nlm.nih.gov/research/umls/rxnorm', label: 'RxNorm' },
            ]}
          />

          <Button onClick={handleSearch} loading={isLoading}>
            Search
          </Button>
        </Group>

        {/* Search Results */}
        <ScrollArea h={500}>
          {isLoading ? (
            <Group justify="center" p="xl">
              <Loader size="sm" />
              <Text size="sm" c="dimmed">
                Searching ValueSets...
              </Text>
            </Group>
          ) : searchResults.length === 0 ? (
            <Text size="sm" c="dimmed" ta="center" p="xl">
              No ValueSets found. Try a different search term.
            </Text>
          ) : (
            <Stack gap="sm">
              {searchResults.map((valueSet) => (
                <div
                  key={valueSet.url}
                  className={styles.valueSetCard}
                  onClick={() => onValueSetSelected(valueSet.url, valueSet.name)}
                >
                  <Group justify="space-between" mb="xs">
                    <Text size="sm" fw={500}>
                      {valueSet.title || valueSet.name}
                    </Text>
                    <Group gap="xs">
                      <Badge size="sm" variant="light">
                        {valueSet.status}
                      </Badge>
                      {valueSet.expansion && (
                        <Badge size="sm" variant="light" color="blue">
                          {valueSet.expansion.total} codes
                        </Badge>
                      )}
                    </Group>
                  </Group>

                  <Text size="xs" c="dimmed" className={styles.url}>
                    {valueSet.url}
                  </Text>

                  {valueSet.description && (
                    <Text size="xs" c="dimmed" mt="xs" lineClamp={2}>
                      {valueSet.description}
                    </Text>
                  )}

                  {valueSet.publisher && (
                    <Text size="xs" c="dimmed" mt="xs">
                      Publisher: {valueSet.publisher}
                    </Text>
                  )}

                  {valueSet.compose?.include && (
                    <Text size="xs" c="dimmed" mt="xs">
                      Code systems: {valueSet.compose.include.map((i) => i.system).join(', ')}
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
