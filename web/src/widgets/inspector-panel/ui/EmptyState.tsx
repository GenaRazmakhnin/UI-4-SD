import { List, Paper, Stack, Text, ThemeIcon } from '@mantine/core';
import { IconInfoCircle, IconKeyboard } from '@tabler/icons-react';
import styles from './EmptyState.module.css';

export function EmptyState() {
  return (
    <Paper className={styles.container} p="xl">
      <Stack align="center" gap="md">
        <ThemeIcon size={64} radius="xl" variant="light" color="gray">
          <IconInfoCircle size={32} />
        </ThemeIcon>

        <Stack align="center" gap="xs">
          <Text size="lg" fw={500}>
            No Element Selected
          </Text>
          <Text size="sm" c="dimmed" ta="center">
            Select an element from the tree to view and edit its properties
          </Text>
        </Stack>

        {/* Quick Tips */}
        <div className={styles.tips}>
          <Text size="sm" fw={500} mb="xs">
            <IconKeyboard size={16} style={{ verticalAlign: 'middle', marginRight: 4 }} />
            Keyboard Shortcuts
          </Text>
          <List size="xs" spacing="xs" c="dimmed">
            <List.Item>↑↓ Navigate elements</List.Item>
            <List.Item>→ Expand element</List.Item>
            <List.Item>← Collapse element</List.Item>
            <List.Item>Enter Edit selected element</List.Item>
            <List.Item>Esc Clear selection</List.Item>
          </List>
        </div>
      </Stack>
    </Paper>
  );
}
