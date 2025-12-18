import { ActionIcon, Badge, Group, Stack, Text, Tooltip } from '@mantine/core';
import type { ElementNode, Extension, ExtensionContextValidation } from '@shared/types';
import {
  IconAlertCircle,
  IconCheck,
  IconExternalLink,
  IconStar,
  IconStarFilled,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { getContextDescription, validateExtensionContext } from '../lib/validation';
import { $extensionUsage, extensionSelected, toggleFavorite } from '../model';
import styles from './ExtensionCard.module.css';

interface ExtensionCardProps {
  extension: Extension;
  element: ElementNode;
  onSelect: () => void;
}

export function ExtensionCard({ extension, element, onSelect }: ExtensionCardProps) {
  const extensionUsage = useUnit($extensionUsage);
  const usage = extensionUsage[extension.url];
  const isFavorite = usage?.isFavorite ?? false;

  // Validate context
  const validation = validateExtensionContext(extension, element);

  // Handle favorite toggle
  const handleToggleFavorite = (e: React.MouseEvent) => {
    e.stopPropagation();
    toggleFavorite(extension.url);
  };

  // Handle select
  const handleSelect = () => {
    if (validation.isValid) {
      onSelect();
      extensionSelected({ extension, elementId: element.id });
    }
  };

  return (
    <div
      className={`${styles.card} ${!validation.isValid ? styles.invalid : ''}`}
      onClick={handleSelect}
      role="button"
      tabIndex={0}
      onKeyPress={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          handleSelect();
        }
      }}
    >
      {/* Header */}
      <Group justify="space-between" mb="xs">
        <Group gap="xs">
          <Text size="sm" fw={600}>
            {extension.title}
          </Text>

          {/* Validation indicator */}
          {validation.isValid && !validation.isWarning && (
            <Tooltip label="Compatible with this element">
              <IconCheck size={16} className={styles.checkIcon} />
            </Tooltip>
          )}
        </Group>

        <Group gap="xs">
          {/* Favorite button */}
          <ActionIcon
            variant="subtle"
            size="sm"
            onClick={handleToggleFavorite}
            aria-label={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
          >
            {isFavorite ? (
              <IconStarFilled size={16} className={styles.favoriteIcon} />
            ) : (
              <IconStar size={16} />
            )}
          </ActionIcon>

          {/* Status badge */}
          <Badge size="sm" variant="light" color={getStatusColor(extension.status)}>
            {extension.status}
          </Badge>

          {extension.experimental && (
            <Badge size="sm" variant="light" color="orange">
              Experimental
            </Badge>
          )}

          {extension.isComplex && (
            <Badge size="sm" variant="light" color="purple">
              Complex
            </Badge>
          )}
        </Group>
      </Group>

      {/* URL */}
      <Group gap="xs" mb="xs">
        <Text size="xs" c="dimmed" className={styles.url}>
          {extension.url}
        </Text>
        <a
          href={extension.url}
          target="_blank"
          rel="noopener noreferrer"
          onClick={(e) => e.stopPropagation()}
          aria-label="Open extension definition"
        >
          <IconExternalLink size={12} />
        </a>
      </Group>

      {/* Description */}
      {extension.description && (
        <Text size="xs" c="dimmed" mb="xs" lineClamp={2}>
          {extension.description}
        </Text>
      )}

      {/* Context */}
      <Text size="xs" c="dimmed" mb="xs">
        {getContextDescription(extension)}
      </Text>

      {/* Value types */}
      {extension.valueTypes && extension.valueTypes.length > 0 && (
        <Group gap="xs" mb="xs">
          <Text size="xs" c="dimmed">
            Value type{extension.valueTypes.length > 1 ? 's' : ''}:
          </Text>
          {extension.valueTypes.map((type) => (
            <Badge key={type} size="xs" variant="light">
              {type}
            </Badge>
          ))}
        </Group>
      )}

      {/* Package & Publisher */}
      <Group gap="md" mt="xs">
        {extension.package && (
          <Text size="xs" c="dimmed">
            Package: {extension.package}
          </Text>
        )}
        {extension.publisher && (
          <Text size="xs" c="dimmed">
            Publisher: {extension.publisher}
          </Text>
        )}
      </Group>

      {/* Validation warnings/errors */}
      {!validation.isValid && validation.message && (
        <Group gap="xs" mt="xs" className={styles.validationError}>
          <IconAlertCircle size={14} />
          <Text size="xs">{validation.message}</Text>
        </Group>
      )}

      {validation.isValid && validation.isWarning && validation.message && (
        <Group gap="xs" mt="xs" className={styles.validationWarning}>
          <IconAlertCircle size={14} />
          <Text size="xs">{validation.message}</Text>
        </Group>
      )}

      {/* Usage count */}
      {usage && usage.useCount > 0 && (
        <Text size="xs" c="dimmed" mt="xs">
          Used {usage.useCount} time{usage.useCount !== 1 ? 's' : ''}
        </Text>
      )}
    </div>
  );
}

function getStatusColor(status: string): string {
  switch (status) {
    case 'active':
      return 'green';
    case 'draft':
      return 'blue';
    case 'retired':
      return 'gray';
    default:
      return 'gray';
  }
}
