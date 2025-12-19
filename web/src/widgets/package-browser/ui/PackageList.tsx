import {
  ActionIcon,
  Badge,
  Box,
  Button,
  Card,
  Group,
  Menu,
  ScrollArea,
  Skeleton,
  Stack,
  Text,
  Tooltip,
} from '@mantine/core';
import type { Package } from '@shared/types';
import {
  IconArrowUp,
  IconChevronRight,
  IconDotsVertical,
  IconDownload,
  IconExternalLink,
  IconPackage,
  IconTrash,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState } from 'react';
import {
  $filteredInstalledPackages,
  $installProgress,
  $isLoadingPackages,
  packageSelected,
  uninstallPackageFx,
  uninstallRequested,
  updateRequested,
} from '../model';
import styles from './PackageList.module.css';
import { UninstallConfirmModal } from './UninstallConfirmModal';

interface PackageListProps {
  onPackageClick?: (pkg: Package) => void;
}

export function PackageList({ onPackageClick }: PackageListProps) {
  const [uninstallModalOpen, setUninstallModalOpen] = useState(false);
  const [packageToUninstall, setPackageToUninstall] = useState<Package | null>(null);

  const [
    packages,
    isLoading,
    installProgress,
    isUninstalling,
    onPackageSelected,
    onUninstall,
    onUpdate,
  ] = useUnit([
    $filteredInstalledPackages,
    $isLoadingPackages,
    $installProgress,
    uninstallPackageFx.pending,
    packageSelected,
    uninstallRequested,
    updateRequested,
  ]);

  const handlePackageClick = (pkg: Package) => {
    onPackageSelected(pkg.id);
    onPackageClick?.(pkg);
  };

  const handleUninstallClick = (e: React.MouseEvent, pkg: Package) => {
    e.stopPropagation();
    setPackageToUninstall(pkg);
    setUninstallModalOpen(true);
  };

  const handleUninstallConfirm = () => {
    if (packageToUninstall) {
      onUninstall(packageToUninstall.id);
      setUninstallModalOpen(false);
      setPackageToUninstall(null);
    }
  };

  const handleUninstallCancel = () => {
    setUninstallModalOpen(false);
    setPackageToUninstall(null);
  };

  const handleUpdate = (e: React.MouseEvent, packageId: string) => {
    e.stopPropagation();
    onUpdate(packageId);
  };

  if (isLoading) {
    return (
      <Stack gap="sm">
        {[1, 2, 3].map((i) => (
          <Skeleton key={i} height={100} radius="md" />
        ))}
      </Stack>
    );
  }

  if (packages.length === 0) {
    return (
      <Box className={styles.emptyState}>
        <IconPackage size={48} stroke={1.5} className={styles.emptyIcon} />
        <Text size="lg" fw={500} mt="md">
          No packages installed
        </Text>
        <Text size="sm" c="dimmed" mt="xs">
          Search the registry to find and install FHIR packages
        </Text>
      </Box>
    );
  }

  return (
    <>
      <ScrollArea h="100%" offsetScrollbars>
        <Stack gap="sm" p="sm">
          {packages.map((pkg) => {
            const progress = installProgress[pkg.id];
            const isInstalling = progress?.status === 'installing';

            return (
              <Card
                key={pkg.id}
                className={styles.packageCard}
                padding="md"
                radius="md"
                withBorder
                onClick={() => handlePackageClick(pkg)}
              >
                <Group justify="space-between" wrap="nowrap">
                  <Box style={{ flex: 1, minWidth: 0 }}>
                    <Group gap="xs" mb={4}>
                      <Text fw={600} truncate>
                        {pkg.name}
                      </Text>
                      <Badge size="sm" variant="light">
                        {pkg.version}
                      </Badge>
                      {pkg.hasUpdate && (
                        <Tooltip label={`Update available: ${pkg.latestVersion}`}>
                          <Badge size="sm" color="orange" variant="filled">
                            Update
                          </Badge>
                        </Tooltip>
                      )}
                    </Group>

                    <Text size="sm" c="dimmed" lineClamp={2}>
                      {pkg.description}
                    </Text>

                    <Group gap="md" mt="sm">
                      <Group gap={4}>
                        <Text size="xs" c="dimmed">
                          FHIR
                        </Text>
                        <Badge size="xs" variant="outline" color="gray">
                          {pkg.fhirVersion}
                        </Badge>
                      </Group>

                      {pkg.publisher && (
                        <Text size="xs" c="dimmed">
                          {pkg.publisher}
                        </Text>
                      )}

                      {pkg.resourceCounts && (
                        <Text size="xs" c="dimmed">
                          {pkg.resourceCounts.total} resources
                        </Text>
                      )}

                      <Text size="xs" c="dimmed">
                        {pkg.size}
                      </Text>
                    </Group>
                  </Box>

                  <Group gap="xs" wrap="nowrap">
                    {pkg.hasUpdate && (
                      <Tooltip label="Update to latest version">
                        <Button
                          size="xs"
                          variant="light"
                          color="orange"
                          leftSection={<IconArrowUp size={14} />}
                          onClick={(e) => handleUpdate(e, pkg.id)}
                          loading={isInstalling}
                        >
                          Update
                        </Button>
                      </Tooltip>
                    )}

                    <Menu shadow="md" width={180} position="bottom-end">
                      <Menu.Target>
                        <ActionIcon
                          variant="subtle"
                          color="gray"
                          onClick={(e) => e.stopPropagation()}
                        >
                          <IconDotsVertical size={16} />
                        </ActionIcon>
                      </Menu.Target>

                      <Menu.Dropdown>
                        <Menu.Item
                          leftSection={<IconChevronRight size={14} />}
                          onClick={(e) => {
                            e.stopPropagation();
                            handlePackageClick(pkg);
                          }}
                        >
                          View details
                        </Menu.Item>

                        {pkg.homepage && (
                          <Menu.Item
                            leftSection={<IconExternalLink size={14} />}
                            component="a"
                            href={pkg.homepage}
                            target="_blank"
                            onClick={(e) => e.stopPropagation()}
                          >
                            Open homepage
                          </Menu.Item>
                        )}

                        <Menu.Divider />

                        {pkg.hasUpdate && (
                          <Menu.Item
                            leftSection={<IconArrowUp size={14} />}
                            onClick={(e) => handleUpdate(e, pkg.id)}
                          >
                            Update to {pkg.latestVersion}
                          </Menu.Item>
                        )}

                        <Menu.Item
                          leftSection={<IconDownload size={14} />}
                          onClick={(e) => e.stopPropagation()}
                        >
                          Install different version
                        </Menu.Item>

                        <Menu.Divider />

                        <Menu.Item
                          color="red"
                          leftSection={<IconTrash size={14} />}
                          onClick={(e) => handleUninstallClick(e, pkg)}
                        >
                          Uninstall
                        </Menu.Item>
                      </Menu.Dropdown>
                    </Menu>

                    <ActionIcon
                      variant="subtle"
                      color="gray"
                      onClick={(e) => {
                        e.stopPropagation();
                        handlePackageClick(pkg);
                      }}
                    >
                      <IconChevronRight size={16} />
                    </ActionIcon>
                  </Group>
                </Group>
              </Card>
            );
          })}
        </Stack>
      </ScrollArea>

      {/* Uninstall Confirmation Modal */}
      <UninstallConfirmModal
        opened={uninstallModalOpen}
        pkg={packageToUninstall}
        isLoading={isUninstalling}
        onClose={handleUninstallCancel}
        onConfirm={handleUninstallConfirm}
      />
    </>
  );
}
