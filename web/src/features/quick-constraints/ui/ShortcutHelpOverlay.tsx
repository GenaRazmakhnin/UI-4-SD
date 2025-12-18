import { Badge, Group, Kbd, Modal, Stack, Table, Text } from '@mantine/core';
import { useUnit } from 'effector-react';
import { QUICK_ACTIONS } from '../lib/actions';
import { $shortcutHelpOpen, shortcutHelpToggled } from '../model';

export function ShortcutHelpOverlay() {
  const isOpen = useUnit($shortcutHelpOpen);

  const handleClose = () => {
    shortcutHelpToggled();
  };

  const actionsWithShortcuts = QUICK_ACTIONS.filter((a) => a.shortcut);

  return (
    <Modal opened={isOpen} onClose={handleClose} title="Quick Action Shortcuts" size="md">
      <Stack gap="md">
        <Text size="sm" c="dimmed">
          Press <Kbd>Ctrl</Kbd> + <Kbd>K</Kbd> followed by a letter to trigger quick actions.
        </Text>

        <Table striped>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Action</Table.Th>
              <Table.Th>Shortcut</Table.Th>
              <Table.Th>Category</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {actionsWithShortcuts.map((action) => (
              <Table.Tr key={action.id}>
                <Table.Td>
                  <Text size="sm">{action.label}</Text>
                </Table.Td>
                <Table.Td>
                  <Group gap={4}>
                    <Kbd size="xs">Ctrl+K</Kbd>
                    <Text size="sm">,</Text>
                    <Kbd size="xs">{action.shortcut?.toUpperCase()}</Kbd>
                  </Group>
                </Table.Td>
                <Table.Td>
                  <Badge
                    size="xs"
                    variant="light"
                    color={
                      action.category === 'cardinality'
                        ? 'blue'
                        : action.category === 'flags'
                          ? 'green'
                          : 'grape'
                    }
                  >
                    {action.category}
                  </Badge>
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>

        <Group gap="xs">
          <Kbd size="xs">Ctrl+K</Kbd>
          <Text size="sm">,</Text>
          <Kbd size="xs">?</Kbd>
          <Text size="sm" c="dimmed">
            Show this help
          </Text>
        </Group>
      </Stack>
    </Modal>
  );
}
