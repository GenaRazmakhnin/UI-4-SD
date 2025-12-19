import { Alert, Button, Group, Modal, Stack, Text, ThemeIcon } from '@mantine/core';
import type { Package } from '@shared/types';
import { IconAlertTriangle, IconPackage, IconTrash } from '@tabler/icons-react';

interface UninstallConfirmModalProps {
  opened: boolean;
  pkg: Package | null;
  isLoading?: boolean;
  onClose: () => void;
  onConfirm: () => void;
}

export function UninstallConfirmModal({
  opened,
  pkg,
  isLoading = false,
  onClose,
  onConfirm,
}: UninstallConfirmModalProps) {
  if (!pkg) return null;

  return (
    <Modal
      opened={opened}
      onClose={onClose}
      title={
        <Group gap="sm">
          <ThemeIcon color="red" variant="light">
            <IconTrash size={18} />
          </ThemeIcon>
          <Text fw={600}>Uninstall Package</Text>
        </Group>
      }
      centered
      size="md"
    >
      <Stack gap="lg">
        <Text size="sm">
          Are you sure you want to uninstall{' '}
          <Text span fw={600}>
            {pkg.name}
          </Text>{' '}
          (v{pkg.version})?
        </Text>

        <Alert color="yellow" variant="light" icon={<IconAlertTriangle size={16} />}>
          <Text size="sm">
            This will remove all resources from this package. Any profiles using extensions or
            bindings from this package may become invalid.
          </Text>
        </Alert>

        {pkg.resourceCounts && pkg.resourceCounts.total > 0 && (
          <Stack gap="xs">
            <Text size="sm" fw={500}>
              Resources that will be removed:
            </Text>
            <Group gap="xl">
              {pkg.resourceCounts.profiles > 0 && (
                <Text size="sm" c="dimmed">
                  {pkg.resourceCounts.profiles} profiles
                </Text>
              )}
              {pkg.resourceCounts.extensions > 0 && (
                <Text size="sm" c="dimmed">
                  {pkg.resourceCounts.extensions} extensions
                </Text>
              )}
              {pkg.resourceCounts.valueSets > 0 && (
                <Text size="sm" c="dimmed">
                  {pkg.resourceCounts.valueSets} value sets
                </Text>
              )}
              {pkg.resourceCounts.codeSystems > 0 && (
                <Text size="sm" c="dimmed">
                  {pkg.resourceCounts.codeSystems} code systems
                </Text>
              )}
            </Group>
          </Stack>
        )}

        <Group justify="flex-end" gap="md">
          <Button variant="subtle" onClick={onClose} disabled={isLoading}>
            Cancel
          </Button>
          <Button
            color="red"
            onClick={onConfirm}
            loading={isLoading}
            leftSection={<IconTrash size={16} />}
          >
            Uninstall
          </Button>
        </Group>
      </Stack>
    </Modal>
  );
}
