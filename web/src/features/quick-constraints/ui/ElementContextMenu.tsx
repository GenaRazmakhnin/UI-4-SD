import { Menu, Portal, Text } from '@mantine/core';
import {
  IconAsterisk,
  IconBan,
  IconCopy,
  IconCut,
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
import { getAvailableActions } from '../lib/actions';
import type { QuickActionId } from '../lib/types';
import {
  $contextMenuState,
  $preferences,
  $recentExtensions,
  $recentValueSets,
  actionTriggered,
  contextMenuClosed,
  favoriteToggled,
} from '../model';

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

export function ElementContextMenu() {
  const [state, preferences, recentValueSets, recentExtensions] = useUnit([
    $contextMenuState,
    $preferences,
    $recentValueSets,
    $recentExtensions,
  ]);

  if (!state.isOpen || !state.element) {
    return null;
  }

  const element = state.element;
  const availableActions = getAvailableActions(element);
  const cardinalityActions = availableActions.filter((a) => a.category === 'cardinality');
  const flagActions = availableActions.filter((a) => a.category === 'flags');
  const constraintActions = availableActions.filter((a) => a.category === 'constraints');

  const handleAction = (actionId: QuickActionId) => {
    actionTriggered(actionId);
    contextMenuClosed();
  };

  const handleToggleFavorite = (actionId: QuickActionId) => {
    favoriteToggled(actionId);
  };

  const handleClose = () => {
    contextMenuClosed();
  };

  return (
    <Portal>
      <Menu
        opened={state.isOpen}
        onClose={handleClose}
        position="right-start"
        offset={0}
        shadow="md"
        width={260}
      >
        <div
          style={{
            position: 'fixed',
            left: state.position.x,
            top: state.position.y,
            zIndex: 1000,
          }}
        >
          <Menu.Dropdown>
            {/* Element Info */}
            <Menu.Label>
              <Text size="xs" fw={500} truncate>
                {element.path}
              </Text>
              <Text size="xs" c="dimmed">
                {element.min}..{element.max}
                {element.type?.[0]?.code ? ` (${element.type[0].code})` : ''}
              </Text>
            </Menu.Label>

            <Menu.Divider />

            {/* Favorites Section */}
            {preferences.favoriteActions.length > 0 && (
              <>
                <Menu.Label>Favorites</Menu.Label>
                {preferences.favoriteActions
                  .map((id) => availableActions.find((a) => a.id === id))
                  .filter(Boolean)
                  .slice(0, 3)
                  .map((action) => {
                    if (!action) return null;
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
                        onClick={() => handleAction(action.id)}
                      >
                        {action.shortLabel}
                      </Menu.Item>
                    );
                  })}
                <Menu.Divider />
              </>
            )}

            {/* Cardinality */}
            {cardinalityActions.length > 0 && (
              <>
                <Menu.Label>Cardinality</Menu.Label>
                {cardinalityActions.map((action) => {
                  const Icon = ACTION_ICONS[action.icon] || IconStar;
                  const isFavorite = preferences.favoriteActions.includes(action.id);
                  return (
                    <Menu.Item
                      key={action.id}
                      leftSection={<Icon size={14} />}
                      rightSection={
                        <IconStar
                          size={12}
                          style={{
                            opacity: isFavorite ? 1 : 0.3,
                            cursor: 'pointer',
                          }}
                          onClick={(e) => {
                            e.stopPropagation();
                            handleToggleFavorite(action.id);
                          }}
                        />
                      }
                      onClick={() => handleAction(action.id)}
                    >
                      {action.shortLabel}
                    </Menu.Item>
                  );
                })}
                <Menu.Divider />
              </>
            )}

            {/* Flags */}
            {flagActions.length > 0 && (
              <>
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
                      onClick={() => handleAction(action.id)}
                    >
                      {action.shortLabel}
                    </Menu.Item>
                  );
                })}
                <Menu.Divider />
              </>
            )}

            {/* Constraints */}
            {constraintActions.length > 0 && (
              <>
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
                        ) : null
                      }
                      onClick={() => handleAction(action.id)}
                    >
                      {action.label}
                    </Menu.Item>
                  );
                })}
              </>
            )}

            {/* Recent ValueSets */}
            {recentValueSets.length > 0 && (
              <>
                <Menu.Divider />
                <Menu.Label>Recent ValueSets</Menu.Label>
                {recentValueSets.slice(0, 3).map((item) => (
                  <Menu.Item
                    key={item.url}
                    leftSection={<IconLink size={14} />}
                    onClick={() => {
                      // TODO: Apply recent ValueSet binding
                      contextMenuClosed();
                    }}
                  >
                    <Text size="xs" truncate>
                      {item.name}
                    </Text>
                  </Menu.Item>
                ))}
              </>
            )}

            {/* Recent Extensions */}
            {recentExtensions.length > 0 && (
              <>
                <Menu.Divider />
                <Menu.Label>Recent Extensions</Menu.Label>
                {recentExtensions.slice(0, 3).map((item) => (
                  <Menu.Item
                    key={item.url}
                    leftSection={<IconPlug size={14} />}
                    onClick={() => {
                      // TODO: Add recent extension
                      contextMenuClosed();
                    }}
                  >
                    <Text size="xs" truncate>
                      {item.name}
                    </Text>
                  </Menu.Item>
                ))}
              </>
            )}
          </Menu.Dropdown>
        </div>
      </Menu>
    </Portal>
  );
}
