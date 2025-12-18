import { ActionIcon, Badge, type BadgeProps, Group, Stack, Text } from '@mantine/core';
import type { ReactNode } from 'react';
import styles from './SearchResultItem.module.css';

export interface SearchResultItemProps {
  /**
   * Icon to display (optional)
   */
  icon?: ReactNode;

  /**
   * Primary title text
   */
  title: string;

  /**
   * Secondary subtitle text (optional)
   */
  subtitle?: string;

  /**
   * Description text (optional)
   */
  description?: string;

  /**
   * URL or identifier to display (optional)
   */
  url?: string;

  /**
   * Badges to display (optional)
   */
  badges?: Array<{
    label: string;
    color?: BadgeProps['color'];
    variant?: BadgeProps['variant'];
  }>;

  /**
   * Quick action buttons (optional)
   */
  actions?: ReactNode;

  /**
   * Query string for highlighting matches (optional)
   */
  highlightQuery?: string;

  /**
   * Click handler
   */
  onClick?: () => void;

  /**
   * Whether the item is selected
   */
  selected?: boolean;

  /**
   * Additional metadata to display
   */
  metadata?: Array<{
    label: string;
    value: string;
  }>;
}

export function SearchResultItem({
  icon,
  title,
  subtitle,
  description,
  url,
  badges = [],
  actions,
  highlightQuery,
  onClick,
  selected = false,
  metadata = [],
}: SearchResultItemProps) {
  return (
    <div
      className={`${styles.item} ${selected ? styles.selected : ''}`}
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyPress={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          onClick?.();
        }
      }}
    >
      <Group gap="md" wrap="nowrap" align="flex-start">
        {/* Icon */}
        {icon && <div className={styles.icon}>{icon}</div>}

        {/* Content */}
        <Stack gap="xs" className={styles.content}>
          {/* Title & Badges */}
          <Group gap="xs" wrap="nowrap">
            <Text size="sm" fw={600} className={styles.title}>
              {highlightQuery ? highlightText(title, highlightQuery) : title}
            </Text>

            {badges.map((badge, index) => (
              <Badge key={index} size="sm" variant={badge.variant || 'light'} color={badge.color}>
                {badge.label}
              </Badge>
            ))}
          </Group>

          {/* Subtitle */}
          {subtitle && (
            <Text size="xs" c="dimmed">
              {highlightQuery ? highlightText(subtitle, highlightQuery) : subtitle}
            </Text>
          )}

          {/* URL */}
          {url && (
            <Text size="xs" c="dimmed" className={styles.url}>
              {url}
            </Text>
          )}

          {/* Description */}
          {description && (
            <Text size="xs" c="dimmed" lineClamp={2}>
              {highlightQuery ? highlightText(description, highlightQuery) : description}
            </Text>
          )}

          {/* Metadata */}
          {metadata.length > 0 && (
            <Group gap="md">
              {metadata.map((meta, index) => (
                <Text key={index} size="xs" c="dimmed">
                  <strong>{meta.label}:</strong> {meta.value}
                </Text>
              ))}
            </Group>
          )}
        </Stack>

        {/* Actions */}
        {actions && <div className={styles.actions}>{actions}</div>}
      </Group>
    </div>
  );
}

/**
 * Highlight matching text in search results
 */
function highlightText(text: string, query: string): ReactNode {
  if (!query || !text) return text;

  const lowerText = text.toLowerCase();
  const lowerQuery = query.toLowerCase();
  const index = lowerText.indexOf(lowerQuery);

  if (index === -1) return text;

  const before = text.slice(0, index);
  const match = text.slice(index, index + query.length);
  const after = text.slice(index + query.length);

  return (
    <>
      {before}
      <mark className={styles.highlight}>{match}</mark>
      {highlightText(after, query)}
    </>
  );
}
