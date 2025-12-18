import { Alert, Badge, Code, Group, Loader, ScrollArea, Stack, Text } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect } from 'react';
import { $expansionLoading, $expansions, fetchExpansionFx } from '../model';
import styles from './ExpansionPreview.module.css';

interface ExpansionPreviewProps {
  valueSetUrl: string;
}

export function ExpansionPreview({ valueSetUrl }: ExpansionPreviewProps) {
  const expansions = useUnit($expansions);
  const isLoading = useUnit($expansionLoading);

  const expansion = expansions[valueSetUrl];

  // Fetch expansion when URL changes
  useEffect(() => {
    if (valueSetUrl && !expansion) {
      fetchExpansionFx({ valueSetUrl });
    }
  }, [valueSetUrl, expansion]);

  if (isLoading) {
    return (
      <Group justify="center" p="md">
        <Loader size="sm" />
        <Text size="sm" c="dimmed">
          Loading expansion...
        </Text>
      </Group>
    );
  }

  if (!expansion) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="yellow">
        <Text size="xs">
          Expansion not available. This ValueSet may need to be expanded by a terminology service.
        </Text>
      </Alert>
    );
  }

  if (expansion.error) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="red">
        <Text size="xs">
          <strong>Error:</strong> {expansion.error}
        </Text>
      </Alert>
    );
  }

  const contains = expansion.contains || [];
  const total = expansion.total || contains.length;
  const displayLimit = 20;
  const hasMore = contains.length > displayLimit;

  return (
    <Stack gap="md">
      {/* Expansion Summary */}
      <Group>
        <Badge size="lg" variant="light" color="blue">
          {total} total codes
        </Badge>
        {hasMore && (
          <Text size="xs" c="dimmed">
            Showing first {displayLimit} codes
          </Text>
        )}
      </Group>

      {/* Code List */}
      <ScrollArea h={300}>
        <Stack gap="xs">
          {contains.slice(0, displayLimit).map((concept, idx) => (
            <div key={idx} className={styles.conceptRow}>
              <Group justify="space-between">
                <Group gap="xs">
                  <Code>{concept.code}</Code>
                  <Text size="sm">{concept.display}</Text>
                </Group>
                {concept.system && (
                  <Text size="xs" c="dimmed" className={styles.system}>
                    {concept.system}
                  </Text>
                )}
              </Group>
            </div>
          ))}
        </Stack>
      </ScrollArea>

      {/* More Info */}
      {hasMore && (
        <Alert color="blue" variant="light">
          <Text size="xs">
            {total - displayLimit} more codes not shown. Use a terminology browser for full
            expansion.
          </Text>
        </Alert>
      )}
    </Stack>
  );
}
