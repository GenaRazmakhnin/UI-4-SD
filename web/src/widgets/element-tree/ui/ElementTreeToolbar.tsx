import { useUnit } from 'effector-react';
import {
  TextInput,
  Group,
  Button,
  Switch,
  ActionIcon,
  Tooltip,
} from '@mantine/core';
import {
  IconSearch,
  IconChevronDown,
  IconChevronRight,
  IconFilter,
} from '@tabler/icons-react';
import {
  $filterOptions,
  filterChanged,
  searchQueryChanged,
  expandAll,
  collapseAll,
} from '../model';

export function ElementTreeToolbar() {
  const filters = useUnit($filterOptions);

  return (
    <div style={{ padding: '12px', borderBottom: '1px solid var(--mantine-color-gray-3)' }}>
      <Group gap="md">
        <TextInput
          placeholder="Search elements..."
          leftSection={<IconSearch size={16} />}
          value={filters.searchQuery}
          onChange={(e) => searchQueryChanged(e.currentTarget.value)}
          style={{ flex: 1 }}
          size="sm"
        />

        <Group gap="xs">
          <Tooltip label="Expand all">
            <ActionIcon
              variant="subtle"
              onClick={() => expandAll()}
              aria-label="Expand all"
            >
              <IconChevronDown size={18} />
            </ActionIcon>
          </Tooltip>

          <Tooltip label="Collapse all">
            <ActionIcon
              variant="subtle"
              onClick={() => collapseAll()}
              aria-label="Collapse all"
            >
              <IconChevronRight size={18} />
            </ActionIcon>
          </Tooltip>
        </Group>
      </Group>

      <Group gap="md" mt="sm">
        <Switch
          label="Modified only"
          size="sm"
          checked={filters.modifiedOnly}
          onChange={(e) =>
            filterChanged({ modifiedOnly: e.currentTarget.checked })
          }
        />
        <Switch
          label="Must Support only"
          size="sm"
          checked={filters.mustSupportOnly}
          onChange={(e) =>
            filterChanged({ mustSupportOnly: e.currentTarget.checked })
          }
        />
      </Group>
    </div>
  );
}
