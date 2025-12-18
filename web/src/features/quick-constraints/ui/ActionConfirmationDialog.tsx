import { Alert, Button, Checkbox, Group, Modal, Stack, Text } from '@mantine/core';
import { IconAlertTriangle } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState } from 'react';
import { getAction } from '../lib/actions';
import {
  $pendingAction,
  $preferences,
  actionCancelled,
  actionConfirmed,
  actionTriggered,
  preferencesUpdated,
} from '../model';

export function ActionConfirmationDialog() {
  const [pendingAction, preferences] = useUnit([$pendingAction, $preferences]);
  const [skipFuture, setSkipFuture] = useState(false);

  if (!pendingAction) {
    return null;
  }

  const action = getAction(pendingAction.actionId);
  if (!action) {
    return null;
  }

  const handleConfirm = () => {
    if (skipFuture) {
      preferencesUpdated({ skipConfirmation: true });
    }
    // Re-trigger the action (it will execute without confirmation now)
    actionConfirmed();
    actionTriggered(pendingAction.actionId);
  };

  const handleCancel = () => {
    actionCancelled();
  };

  // Get impact description
  const getImpactDescription = () => {
    switch (pendingAction.actionId) {
      case 'make-prohibited':
        return `This will set the maximum cardinality to 0, effectively removing "${pendingAction.element.path}" from the profile. Implementers will not be able to use this element.`;
      case 'add-fixed-value':
        return `This will lock "${pendingAction.element.path}" to a specific value. The element cannot have any other value in instances conforming to this profile.`;
      default:
        return `This action will modify "${pendingAction.element.path}". Please confirm you want to proceed.`;
    }
  };

  return (
    <Modal opened={true} onClose={handleCancel} title={`Confirm: ${action.label}`} size="md">
      <Stack gap="md">
        <Alert color="yellow" icon={<IconAlertTriangle size={16} />}>
          <Text size="sm">{getImpactDescription()}</Text>
        </Alert>

        <Stack gap="xs">
          <Text size="sm" fw={500}>
            Element Details:
          </Text>
          <Text size="sm" c="dimmed" ff="monospace">
            Path: {pendingAction.element.path}
          </Text>
          <Text size="sm" c="dimmed">
            Current: {pendingAction.element.min}..{pendingAction.element.max}
            {pendingAction.element.type?.[0]?.code
              ? ` (${pendingAction.element.type[0].code})`
              : ''}
          </Text>
        </Stack>

        <Checkbox
          label="Don't ask again for this type of action"
          checked={skipFuture}
          onChange={(e) => setSkipFuture(e.currentTarget.checked)}
          size="sm"
        />

        <Group justify="flex-end">
          <Button variant="subtle" onClick={handleCancel}>
            Cancel
          </Button>
          <Button color="red" onClick={handleConfirm}>
            {action.label}
          </Button>
        </Group>
      </Stack>
    </Modal>
  );
}
