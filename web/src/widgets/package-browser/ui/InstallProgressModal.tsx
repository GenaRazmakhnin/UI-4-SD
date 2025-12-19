import { usePackageInstallPolling } from '@features/packages/hooks/usePackageInstallPolling';
import {
  Alert,
  Badge,
  Button,
  Group,
  Modal,
  Progress,
  RingProgress,
  Stack,
  Text,
  ThemeIcon,
} from '@mantine/core';
import type { Package, PackageInstallStatus } from '@shared/types';
import {
  IconAlertCircle,
  IconCheck,
  IconDownload,
  IconLoader,
  IconPackage,
  IconX,
} from '@tabler/icons-react';
import { useEffect } from 'react';
import styles from './InstallProgressModal.module.css';

interface InstallProgressModalProps {
  opened: boolean;
  packageId: string | null;
  packageName?: string;
  onClose: () => void;
  onComplete?: (pkg: Package) => void;
}

export function InstallProgressModal({
  opened,
  packageId,
  packageName,
  onClose,
  onComplete,
}: InstallProgressModalProps) {
  const { status, progress, message, downloadedBytes, totalBytes, install, cancel, reset } =
    usePackageInstallPolling({
      onComplete: (pkg) => {
        onComplete?.(pkg);
      },
    });

  // Start installation when modal opens with a package ID
  useEffect(() => {
    if (opened && packageId && status === 'idle') {
      install(packageId);
    }
  }, [opened, packageId, status, install]);

  // Reset when modal closes
  useEffect(() => {
    if (!opened) {
      // Delay reset to allow closing animation
      const timer = setTimeout(() => {
        reset();
      }, 300);
      return () => clearTimeout(timer);
    }
  }, [opened, reset]);

  const handleClose = () => {
    if (status === 'installing' || status === 'extracting' || status === 'indexing') {
      // Don't allow closing during installation
      return;
    }
    onClose();
  };

  const handleCancel = () => {
    cancel();
    onClose();
  };

  const handleRetry = () => {
    if (packageId) {
      reset();
      install(packageId);
    }
  };

  const displayName = packageName || packageId || 'Package';

  return (
    <Modal
      opened={opened}
      onClose={handleClose}
      title={
        <Group gap="sm">
          <IconPackage size={20} />
          <Text fw={600}>Installing Package</Text>
        </Group>
      }
      centered
      closeOnClickOutside={false}
      closeOnEscape={status === 'installed' || status === 'error' || status === 'idle'}
      withCloseButton={status === 'installed' || status === 'error' || status === 'idle'}
      size="md"
    >
      <Stack gap="lg" className={styles.content}>
        {/* Package info */}
        <Group justify="center">
          <Badge size="lg" variant="light" leftSection={<IconPackage size={14} />}>
            {displayName}
          </Badge>
        </Group>

        {/* Progress indicator */}
        <Stack align="center" gap="md">
          {status === 'idle' && <StatusIdle />}
          {status === 'installing' && (
            <StatusInstalling
              progress={progress}
              downloadedBytes={downloadedBytes}
              totalBytes={totalBytes}
            />
          )}
          {status === 'extracting' && <StatusExtracting />}
          {status === 'indexing' && <StatusIndexing />}
          {status === 'installed' && <StatusComplete />}
          {status === 'error' && <StatusError />}
        </Stack>

        {/* Status message */}
        <Text ta="center" size="sm" c="dimmed">
          {message}
        </Text>

        {/* Error details */}
        {status === 'error' && (
          <Alert color="red" variant="light" icon={<IconAlertCircle size={16} />}>
            <Text size="sm">{message}</Text>
          </Alert>
        )}

        {/* Actions */}
        <Group justify="center" gap="md">
          {(status === 'installing' || status === 'extracting' || status === 'indexing') && (
            <Button
              variant="subtle"
              color="red"
              onClick={handleCancel}
              leftSection={<IconX size={16} />}
            >
              Cancel
            </Button>
          )}

          {status === 'error' && (
            <>
              <Button variant="subtle" onClick={onClose}>
                Close
              </Button>
              <Button onClick={handleRetry} leftSection={<IconDownload size={16} />}>
                Retry
              </Button>
            </>
          )}

          {status === 'installed' && (
            <Button onClick={onClose} leftSection={<IconCheck size={16} />}>
              Done
            </Button>
          )}
        </Group>
      </Stack>
    </Modal>
  );
}

function StatusIdle() {
  return (
    <ThemeIcon size={64} radius="xl" color="gray" variant="light">
      <IconPackage size={32} />
    </ThemeIcon>
  );
}

interface StatusInstallingProps {
  progress: number;
  downloadedBytes: number;
  totalBytes: number | null;
}

function StatusInstalling({ progress, downloadedBytes, totalBytes }: StatusInstallingProps) {
  return (
    <Stack align="center" gap="sm" w="100%">
      <RingProgress
        size={80}
        thickness={6}
        roundCaps
        sections={[{ value: progress, color: 'blue' }]}
        label={
          <Text ta="center" fw={600} size="sm">
            {Math.round(progress)}%
          </Text>
        }
      />
      <Progress value={progress} size="lg" radius="xl" w="100%" animated striped={progress < 100} />
      {totalBytes && (
        <Text size="xs" c="dimmed">
          {formatBytes(downloadedBytes)} / {formatBytes(totalBytes)}
        </Text>
      )}
    </Stack>
  );
}

function StatusExtracting() {
  return (
    <Stack align="center" gap="sm">
      <ThemeIcon size={64} radius="xl" color="blue" variant="light">
        <IconLoader size={32} className={styles.spinning} />
      </ThemeIcon>
      <Text size="sm" c="blue" fw={500}>
        Extracting...
      </Text>
    </Stack>
  );
}

function StatusIndexing() {
  return (
    <Stack align="center" gap="sm">
      <ThemeIcon size={64} radius="xl" color="violet" variant="light">
        <IconLoader size={32} className={styles.spinning} />
      </ThemeIcon>
      <Text size="sm" c="violet" fw={500}>
        Indexing resources...
      </Text>
    </Stack>
  );
}

function StatusComplete() {
  return (
    <Stack align="center" gap="sm">
      <ThemeIcon size={64} radius="xl" color="green" variant="light">
        <IconCheck size={32} />
      </ThemeIcon>
      <Text size="sm" c="green" fw={500}>
        Installation complete!
      </Text>
    </Stack>
  );
}

function StatusError() {
  return (
    <ThemeIcon size={64} radius="xl" color="red" variant="light">
      <IconAlertCircle size={32} />
    </ThemeIcon>
  );
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / k ** i).toFixed(1))} ${sizes[i]}`;
}
