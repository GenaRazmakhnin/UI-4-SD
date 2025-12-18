import {
  Group,
  Stack,
  Text,
  Badge,
  ActionIcon,
  Tooltip,
  CopyButton,
} from '@mantine/core';
import {
  IconCopy,
  IconExternalLink,
  IconAlertCircle,
} from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';
import styles from './ElementHeader.module.css';

interface ElementHeaderProps {
  element: ElementNode;
}

export function ElementHeader({ element }: ElementHeaderProps) {
  const baseUrl = getBaseDefinitionUrl(element);
  const types = element.type?.map((t) => t.code).join(' | ') || 'Element';

  return (
    <Stack gap="xs">
      {/* Path and Actions */}
      <Group justify="space-between">
        <Group gap="xs">
          <Text size="sm" fw={600} className={styles.path}>
            {element.path}
          </Text>
          {element.sliceName && (
            <Badge size="sm" variant="light" color="blue">
              :{element.sliceName}
            </Badge>
          )}
        </Group>

        <Group gap="xs">
          {/* Copy Path Button */}
          <CopyButton value={element.path}>
            {({ copied, copy }) => (
              <Tooltip label={copied ? 'Copied!' : 'Copy path'}>
                <ActionIcon
                  size="sm"
                  variant="subtle"
                  color={copied ? 'teal' : 'gray'}
                  onClick={copy}
                >
                  <IconCopy size={16} />
                </ActionIcon>
              </Tooltip>
            )}
          </CopyButton>

          {/* Link to Base Definition */}
          {baseUrl && (
            <Tooltip label="View base definition">
              <ActionIcon
                size="sm"
                variant="subtle"
                component="a"
                href={baseUrl}
                target="_blank"
                rel="noopener noreferrer"
              >
                <IconExternalLink size={16} />
              </ActionIcon>
            </Tooltip>
          )}
        </Group>
      </Group>

      {/* Element Type and Status */}
      <Group gap="xs">
        <Text size="xs" c="dimmed">
          {types}
        </Text>

        {element.isModified && (
          <Badge size="xs" variant="light" color="orange">
            Modified
          </Badge>
        )}

        {element.mustSupport && (
          <Badge size="xs" variant="light" color="blue">
            Must Support
          </Badge>
        )}

        {element.isModifier && (
          <Badge size="xs" variant="light" color="red">
            <Group gap={4}>
              <IconAlertCircle size={12} />
              <span>Modifier</span>
            </Group>
          </Badge>
        )}

        {element.isSummary && (
          <Badge size="xs" variant="light" color="gray">
            Summary
          </Badge>
        )}
      </Group>

      {/* Short Description */}
      {element.short && (
        <Text size="xs" c="dimmed" lineClamp={2}>
          {element.short}
        </Text>
      )}
    </Stack>
  );
}

/**
 * Get base definition URL for element
 */
function getBaseDefinitionUrl(element: ElementNode): string | null {
  // Extract resource type from path (e.g., "Patient.name" -> "Patient")
  const resourceType = element.path.split('.')[0];
  return `https://hl7.org/fhir/R4/${resourceType}.html#${element.path}`;
}
