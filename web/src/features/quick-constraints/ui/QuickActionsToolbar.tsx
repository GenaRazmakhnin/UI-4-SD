import { ActionIcon, Divider, Group, Menu, Text, Tooltip } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import {
  IconAsterisk,
  IconBan,
  IconChevronDown,
  IconCopy,
  IconCut,
  IconDotsVertical,
  IconFilter,
  IconLink,
  IconLock,
  IconPlug,
  IconQuestionMark,
  IconShield,
  IconStar,
  IconStarFilled,
  IconTemplate,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import type { QuickAction, QuickActionId } from '../lib/types';
import {
  $availableActions,
  $favoriteActions,
  $preferences,
  actionTriggered,
  favoriteToggled,
} from '../model';
import styles from './QuickActionsToolbar.module.css';

// Icon mapping
const ACTION_ICONS: Record<string, React.ComponentType<{ size?: number }>> = {
  IconAsterisk,
  IconQuestionMark,
  IconCopy,
  IconBan,
  IconStar,
  IconLink,
  IconPlug,
  IconCut,
  IconFilter,
  IconTemplate,
  IconLock,
  IconShield,
};

interface QuickActionsToolbarProps {
  element: ElementNode | null;
  compact?: boolean;
}

function ActionButton({
  action,
  element,
  isFavorite,
  showShortcut,
  onTrigger,
  onToggleFavorite,
}: {
  action: QuickAction;
  element: ElementNode;
  isFavorite: boolean;
  showShortcut: boolean;
  onTrigger: (id: QuickActionId) => void;
  onToggleFavorite: (id: QuickActionId) => void;
}) {
  const Icon = ACTION_ICONS[action.icon] || IconStar;
  const isActive = action.isActive?.(element) ?? false;

  const tooltipLabel = (
    <div>
      <Text size="xs" fw={500}>
        {action.label}
      </Text>
      <Text size="xs" c="dimmed">
        {action.description}
      </Text>
      {showShortcut && action.shortcut && (
        <Text size="xs" c="blue" mt={4}>
          Ctrl+K, {action.shortcut.toUpperCase()}
        </Text>
      )}
    </div>
  );

  return (
    <Menu position="bottom" withArrow shadow="md" width={200}>
      <Menu.Target>
        <Tooltip label={tooltipLabel} multiline w={220} position="bottom" withArrow>
          <ActionIcon
            variant={isActive ? 'filled' : 'subtle'}
            color={isActive ? 'blue' : 'gray'}
            onClick={() => onTrigger(action.id)}
            aria-label={action.label}
          >
            <Icon size={16} />
          </ActionIcon>
        </Tooltip>
      </Menu.Target>
      <Menu.Dropdown>
        <Menu.Item onClick={() => onTrigger(action.id)}>{action.label}</Menu.Item>
        <Menu.Divider />
        <Menu.Item
          leftSection={isFavorite ? <IconStarFilled size={14} /> : <IconStar size={14} />}
          onClick={() => onToggleFavorite(action.id)}
        >
          {isFavorite ? 'Remove from Favorites' : 'Add to Favorites'}
        </Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
}

export function QuickActionsToolbar({ element, compact = false }: QuickActionsToolbarProps) {
  const [availableActions, favoriteActions, preferences] = useUnit([
    $availableActions,
    $favoriteActions,
    $preferences,
  ]);

  const handleTrigger = (actionId: QuickActionId) => {
    actionTriggered(actionId);
  };

  const handleToggleFavorite = (actionId: QuickActionId) => {
    favoriteToggled(actionId);
  };

  if (!element) {
    return (
      <div className={styles.container}>
        <Text size="xs" c="dimmed">
          Select an element to see quick actions
        </Text>
      </div>
    );
  }

  // Group actions by category
  const cardinalityActions = availableActions.filter((a) => a.category === 'cardinality');
  const flagActions = availableActions.filter((a) => a.category === 'flags');
  const constraintActions = availableActions.filter((a) => a.category === 'constraints');

  if (compact) {
    // Compact mode: show only favorites and a "more" menu
    return (
      <Group gap="xs" className={styles.container}>
        {favoriteActions
          .slice(0, 4)
          .map((action) =>
            action ? (
              <ActionButton
                key={action.id}
                action={action}
                element={element}
                isFavorite={true}
                showShortcut={preferences.showKeyboardHints}
                onTrigger={handleTrigger}
                onToggleFavorite={handleToggleFavorite}
              />
            ) : null
          )}

        {availableActions.length > 4 && (
          <Menu position="bottom-end" withArrow shadow="md" width={220}>
            <Menu.Target>
              <ActionIcon variant="subtle" color="gray" aria-label="More actions">
                <IconDotsVertical size={16} />
              </ActionIcon>
            </Menu.Target>
            <Menu.Dropdown>
              {cardinalityActions.length > 0 && (
                <>
                  <Menu.Label>Cardinality</Menu.Label>
                  {cardinalityActions.map((action) => {
                    const Icon = ACTION_ICONS[action.icon] || IconStar;
                    return (
                      <Menu.Item
                        key={action.id}
                        leftSection={<Icon size={14} />}
                        onClick={() => handleTrigger(action.id)}
                      >
                        {action.shortLabel}
                      </Menu.Item>
                    );
                  })}
                </>
              )}
              {flagActions.length > 0 && (
                <>
                  <Menu.Divider />
                  <Menu.Label>Flags</Menu.Label>
                  {flagActions.map((action) => {
                    const Icon = ACTION_ICONS[action.icon] || IconStar;
                    const isActive = action.isActive?.(element) ?? false;
                    return (
                      <Menu.Item
                        key={action.id}
                        leftSection={<Icon size={14} />}
                        rightSection={
                          isActive ? (
                            <Text size="xs" c="blue">
                              ON
                            </Text>
                          ) : null
                        }
                        onClick={() => handleTrigger(action.id)}
                      >
                        {action.shortLabel}
                      </Menu.Item>
                    );
                  })}
                </>
              )}
              {constraintActions.length > 0 && (
                <>
                  <Menu.Divider />
                  <Menu.Label>Constraints</Menu.Label>
                  {constraintActions.map((action) => {
                    const Icon = ACTION_ICONS[action.icon] || IconStar;
                    return (
                      <Menu.Item
                        key={action.id}
                        leftSection={<Icon size={14} />}
                        onClick={() => handleTrigger(action.id)}
                      >
                        {action.shortLabel}
                      </Menu.Item>
                    );
                  })}
                </>
              )}
            </Menu.Dropdown>
          </Menu>
        )}
      </Group>
    );
  }

  // Full mode: show all actions grouped
  return (
    <div className={styles.container}>
      <Group gap="xs">
        {/* Cardinality Actions */}
        {cardinalityActions.length > 0 && (
          <>
            <Menu position="bottom" withArrow shadow="md" width={200}>
              <Menu.Target>
                <Tooltip label="Cardinality" position="bottom">
                  <ActionIcon variant="subtle" color="gray">
                    <IconAsterisk size={16} />
                    <IconChevronDown size={10} style={{ marginLeft: 2 }} />
                  </ActionIcon>
                </Tooltip>
              </Menu.Target>
              <Menu.Dropdown>
                <Menu.Label>Cardinality</Menu.Label>
                {cardinalityActions.map((action) => {
                  const Icon = ACTION_ICONS[action.icon] || IconStar;
                  return (
                    <Menu.Item
                      key={action.id}
                      leftSection={<Icon size={14} />}
                      rightSection={
                        action.shortcut ? (
                          <Text size="xs" c="dimmed">
                            Ctrl+K, {action.shortcut.toUpperCase()}
                          </Text>
                        ) : null
                      }
                      onClick={() => handleTrigger(action.id)}
                    >
                      {action.label}
                    </Menu.Item>
                  );
                })}
              </Menu.Dropdown>
            </Menu>
            <Divider orientation="vertical" />
          </>
        )}

        {/* Flag Actions - shown as individual toggles */}
        {flagActions.map((action) => (
          <ActionButton
            key={action.id}
            action={action}
            element={element}
            isFavorite={preferences.favoriteActions.includes(action.id)}
            showShortcut={preferences.showKeyboardHints}
            onTrigger={handleTrigger}
            onToggleFavorite={handleToggleFavorite}
          />
        ))}

        {flagActions.length > 0 && constraintActions.length > 0 && (
          <Divider orientation="vertical" />
        )}

        {/* Constraint Actions */}
        {constraintActions.length > 0 && (
          <Menu position="bottom" withArrow shadow="md" width={220}>
            <Menu.Target>
              <Tooltip label="Add Constraints" position="bottom">
                <ActionIcon variant="subtle" color="gray">
                  <IconPlug size={16} />
                  <IconChevronDown size={10} style={{ marginLeft: 2 }} />
                </ActionIcon>
              </Tooltip>
            </Menu.Target>
            <Menu.Dropdown>
              <Menu.Label>Constraints</Menu.Label>
              {constraintActions.map((action) => {
                const Icon = ACTION_ICONS[action.icon] || IconStar;
                const isActive = action.isActive?.(element) ?? false;
                return (
                  <Menu.Item
                    key={action.id}
                    leftSection={<Icon size={14} />}
                    rightSection={
                      isActive ? (
                        <Text size="xs" c="blue">
                          SET
                        </Text>
                      ) : action.shortcut ? (
                        <Text size="xs" c="dimmed">
                          {action.shortcut.toUpperCase()}
                        </Text>
                      ) : null
                    }
                    onClick={() => handleTrigger(action.id)}
                  >
                    {action.label}
                  </Menu.Item>
                );
              })}
            </Menu.Dropdown>
          </Menu>
        )}
      </Group>
    </div>
  );
}
