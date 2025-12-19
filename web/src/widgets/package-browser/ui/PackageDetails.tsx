import {
  ActionIcon,
  Anchor,
  Badge,
  Box,
  Button,
  Card,
  Divider,
  Group,
  Paper,
  Progress,
  ScrollArea,
  Select,
  SimpleGrid,
  Skeleton,
  Stack,
  Table,
  Tabs,
  Text,
  ThemeIcon,
  Timeline,
  Tooltip,
} from '@mantine/core';
import type { Package, PackageResource } from '@shared/types';
import {
  IconArrowLeft,
  IconArrowUp,
  IconBook,
  IconBox,
  IconCheck,
  IconChevronRight,
  IconCode,
  IconDatabase,
  IconDownload,
  IconExternalLink,
  IconFileCode,
  IconGitBranch,
  IconLicense,
  IconPackage,
  IconSearch,
  IconTrash,
  IconUser,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState } from 'react';
import {
  $installProgress,
  $isLoadingDetails,
  $selectedPackage,
  installRequested,
  installVersionRequested,
  packageSelected,
  uninstallRequested,
  updateRequested,
} from '../model';
import styles from './PackageDetails.module.css';
import { ResourceBrowser } from './ResourceBrowser';

interface PackageDetailsProps {
  onBack?: () => void;
  onSelectResource?: (resource: PackageResource) => void;
}

export function PackageDetails({ onBack, onSelectResource }: PackageDetailsProps) {
  const [
    pkg,
    isLoading,
    installProgress,
    onPackageSelected,
    onInstall,
    onInstallVersion,
    onUninstall,
    onUpdate,
  ] = useUnit([
    $selectedPackage,
    $isLoadingDetails,
    $installProgress,
    packageSelected,
    installRequested,
    installVersionRequested,
    uninstallRequested,
    updateRequested,
  ]);
  const [selectedVersion, setSelectedVersion] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<string | null>('overview');

  if (isLoading || !pkg) {
    return (
      <Stack gap="md" p="md">
        <Skeleton height={40} />
        <Skeleton height={100} />
        <Skeleton height={200} />
      </Stack>
    );
  }

  const progress = installProgress[pkg.id];
  const isInstalling = progress?.status === 'installing';

  const handleBack = () => {
    onPackageSelected(null);
    onBack?.();
  };

  const handleInstall = () => {
    if (selectedVersion) {
      onInstallVersion({ packageId: pkg.id, version: selectedVersion });
    } else {
      onInstall(pkg.id);
    }
  };

  const handleUninstall = () => {
    onUninstall(pkg.id);
  };

  const handleUpdate = () => {
    onUpdate(pkg.id);
  };

  const resourceStats = pkg.resourceCounts;
  const dependencies = pkg.dependencies ?? [];
  const installedDependencies = dependencies.filter((dep) => dep.isInstalled).length;

  return (
    <Stack gap="md" h="100%">
      {/* Header */}
      <Paper p="md" withBorder>
        <Group justify="space-between" wrap="nowrap">
          <Group gap="sm">
            <ActionIcon variant="subtle" onClick={handleBack}>
              <IconArrowLeft size={18} />
            </ActionIcon>
            <Box>
              <Group gap="xs">
                <Text size="xl" fw={600}>
                  {pkg.name}
                </Text>
                <Badge size="lg" variant="light">
                  {pkg.version}
                </Badge>
                {pkg.installed && (
                  <Badge size="lg" color="green" variant="filled">
                    Installed
                  </Badge>
                )}
                {pkg.hasUpdate && (
                  <Tooltip label={`Update available: ${pkg.latestVersion}`}>
                    <Badge size="lg" color="orange" variant="filled">
                      Update Available
                    </Badge>
                  </Tooltip>
                )}
              </Group>
              {pkg.publisher && (
                <Text size="sm" c="dimmed">
                  by {pkg.publisher}
                </Text>
              )}
            </Box>
          </Group>

          <Group gap="sm">
            {pkg.installed ? (
              <>
                {pkg.hasUpdate && (
                  <Button
                    variant="filled"
                    color="orange"
                    leftSection={<IconArrowUp size={16} />}
                    onClick={handleUpdate}
                    loading={isInstalling}
                  >
                    Update to {pkg.latestVersion}
                  </Button>
                )}
                <Button
                  variant="light"
                  color="red"
                  leftSection={<IconTrash size={16} />}
                  onClick={handleUninstall}
                >
                  Uninstall
                </Button>
              </>
            ) : (
              <Group gap="xs">
                {pkg.versions && pkg.versions.length > 1 && (
                  <Select
                    placeholder="Version"
                    size="sm"
                    clearable
                    data={pkg.versions.map((v) => ({
                      value: v.version,
                      label: `${v.version} (${v.fhirVersion})`,
                    }))}
                    value={selectedVersion}
                    onChange={setSelectedVersion}
                    style={{ width: 180 }}
                  />
                )}
                <Button
                  variant="filled"
                  leftSection={<IconDownload size={16} />}
                  onClick={handleInstall}
                  loading={isInstalling}
                >
                  Install{selectedVersion ? ` ${selectedVersion}` : ''}
                </Button>
              </Group>
            )}
          </Group>
        </Group>

        {isInstalling && progress && (
          <Box mt="md">
            <Text size="sm" mb={4}>
              {progress.message}
            </Text>
            <Progress value={progress.progress} animated />
          </Box>
        )}
      </Paper>

      {/* Content */}
      <Tabs
        value={activeTab}
        onChange={setActiveTab}
        style={{ flex: 1, display: 'flex', flexDirection: 'column' }}
      >
        <Tabs.List>
          <Tabs.Tab value="overview" leftSection={<IconBook size={14} />}>
            Overview
          </Tabs.Tab>
          <Tabs.Tab
            value="resources"
            leftSection={<IconFileCode size={14} />}
            rightSection={
              resourceStats && (
                <Badge size="xs" variant="light">
                  {resourceStats.total}
                </Badge>
              )
            }
          >
            Resources
          </Tabs.Tab>
          <Tabs.Tab value="dependencies" leftSection={<IconGitBranch size={14} />}>
            Dependencies
          </Tabs.Tab>
          <Tabs.Tab value="versions" leftSection={<IconPackage size={14} />}>
            Versions
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="overview" pt="md" style={{ flex: 1, overflow: 'auto' }}>
          <ScrollArea h="100%" offsetScrollbars>
            <Stack gap="lg" p="sm">
              {/* Description */}
              <Card withBorder padding="md">
                <Text size="sm" fw={500} mb="xs">
                  Description
                </Text>
                <Text size="sm" c="dimmed">
                  {pkg.description || 'No description available.'}
                </Text>
              </Card>

              {/* Metadata */}
              <SimpleGrid cols={{ base: 1, sm: 2 }} spacing="md">
                <Card withBorder padding="md">
                  <Group gap="sm" mb="md">
                    <ThemeIcon size="md" variant="light" color="blue">
                      <IconCode size={16} />
                    </ThemeIcon>
                    <Text size="sm" fw={500}>
                      Package Info
                    </Text>
                  </Group>
                  <Stack gap="xs">
                    <Group justify="space-between">
                      <Text size="sm" c="dimmed">
                        Package ID
                      </Text>
                      <Text size="sm">{pkg.id}</Text>
                    </Group>
                    <Group justify="space-between">
                      <Text size="sm" c="dimmed">
                        FHIR Version
                      </Text>
                      <Badge variant="outline">{pkg.fhirVersion}</Badge>
                    </Group>
                    <Group justify="space-between">
                      <Text size="sm" c="dimmed">
                        Size
                      </Text>
                      <Text size="sm">{pkg.size}</Text>
                    </Group>
                    {pkg.latestVersion && (
                      <Group justify="space-between">
                        <Text size="sm" c="dimmed">
                          Latest Version
                        </Text>
                        <Badge variant="light" color={pkg.hasUpdate ? 'orange' : 'gray'}>
                          {pkg.latestVersion}
                        </Badge>
                      </Group>
                    )}
                    {pkg.license && (
                      <Group justify="space-between">
                        <Text size="sm" c="dimmed">
                          License
                        </Text>
                        <Group gap={4}>
                          <IconLicense size={14} />
                          <Text size="sm">{pkg.license}</Text>
                        </Group>
                      </Group>
                    )}
                    {pkg.publishedDate && (
                      <Group justify="space-between">
                        <Text size="sm" c="dimmed">
                          Published
                        </Text>
                        <Text size="sm">{new Date(pkg.publishedDate).toLocaleDateString()}</Text>
                      </Group>
                    )}
                  </Stack>
                </Card>

                <Card withBorder padding="md">
                  <Group gap="sm" mb="md">
                    <ThemeIcon size="md" variant="light" color="green">
                      <IconDownload size={16} />
                    </ThemeIcon>
                    <Text size="sm" fw={500}>
                      Usage
                    </Text>
                  </Group>
                  <Stack gap="xs">
                    {pkg.downloadCount && (
                      <Group justify="space-between">
                        <Text size="sm" c="dimmed">
                          Downloads
                        </Text>
                        <Text size="sm">{pkg.downloadCount.toLocaleString()}</Text>
                      </Group>
                    )}
                    {resourceStats && (
                      <Group justify="space-between">
                        <Text size="sm" c="dimmed">
                          Resources
                        </Text>
                        <Text size="sm">{resourceStats.total.toLocaleString()}</Text>
                      </Group>
                    )}
                    {dependencies.length > 0 && (
                      <Group justify="space-between">
                        <Text size="sm" c="dimmed">
                          Dependencies
                        </Text>
                        <Text size="sm">
                          {installedDependencies}/{dependencies.length} installed
                        </Text>
                      </Group>
                    )}
                  </Stack>
                </Card>
              </SimpleGrid>

              {/* Resource counts */}
              {resourceStats && (
                <Card withBorder padding="md">
                  <Group gap="sm" mb="md">
                    <ThemeIcon size="md" variant="light" color="violet">
                      <IconDatabase size={16} />
                    </ThemeIcon>
                    <Text size="sm" fw={500}>
                      Resource Summary
                    </Text>
                  </Group>
                  <SimpleGrid cols={{ base: 2, sm: 4 }} spacing="md">
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.profiles}
                      </Text>
                      <Text size="xs" c="dimmed">
                        Profiles
                      </Text>
                    </Box>
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.extensions}
                      </Text>
                      <Text size="xs" c="dimmed">
                        Extensions
                      </Text>
                    </Box>
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.valueSets}
                      </Text>
                      <Text size="xs" c="dimmed">
                        ValueSets
                      </Text>
                    </Box>
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.codeSystems}
                      </Text>
                      <Text size="xs" c="dimmed">
                        CodeSystems
                      </Text>
                    </Box>
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.searchParameters}
                      </Text>
                      <Text size="xs" c="dimmed">
                        Search Params
                      </Text>
                    </Box>
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.operationDefinitions}
                      </Text>
                      <Text size="xs" c="dimmed">
                        Operations
                      </Text>
                    </Box>
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.capabilityStatements}
                      </Text>
                      <Text size="xs" c="dimmed">
                        Capabilities
                      </Text>
                    </Box>
                    <Box ta="center">
                      <Text size="xl" fw={600}>
                        {resourceStats.total}
                      </Text>
                      <Text size="xs" c="dimmed">
                        Total
                      </Text>
                    </Box>
                  </SimpleGrid>
                </Card>
              )}

              {/* Links */}
              {(pkg.homepage || pkg.repository || pkg.canonical) && (
                <Card withBorder padding="md">
                  <Group gap="sm" mb="md">
                    <ThemeIcon size="md" variant="light" color="cyan">
                      <IconExternalLink size={16} />
                    </ThemeIcon>
                    <Text size="sm" fw={500}>
                      Links
                    </Text>
                  </Group>
                  <Stack gap="xs">
                    {pkg.homepage && (
                      <Group justify="space-between" wrap="nowrap">
                        <Text size="sm" c="dimmed">
                          Homepage
                        </Text>
                        <Anchor
                          href={pkg.homepage}
                          target="_blank"
                          size="sm"
                          title={pkg.homepage}
                          style={{
                            maxWidth: 220,
                            display: 'inline-block',
                            overflow: 'hidden',
                            textOverflow: 'ellipsis',
                            whiteSpace: 'nowrap',
                          }}
                        >
                          {pkg.homepage}
                        </Anchor>
                      </Group>
                    )}
                    {pkg.repository && (
                      <Group justify="space-between" wrap="nowrap">
                        <Text size="sm" c="dimmed">
                          Repository
                        </Text>
                        <Anchor
                          href={pkg.repository}
                          target="_blank"
                          size="sm"
                          title={pkg.repository}
                          style={{
                            maxWidth: 220,
                            display: 'inline-block',
                            overflow: 'hidden',
                            textOverflow: 'ellipsis',
                            whiteSpace: 'nowrap',
                          }}
                        >
                          {pkg.repository}
                        </Anchor>
                      </Group>
                    )}
                    {pkg.canonical && (
                      <Group justify="space-between" wrap="nowrap">
                        <Text size="sm" c="dimmed">
                          Canonical
                        </Text>
                        <Text size="sm" truncate style={{ maxWidth: 220 }} title={pkg.canonical}>
                          {pkg.canonical}
                        </Text>
                      </Group>
                    )}
                  </Stack>
                </Card>
              )}
            </Stack>
          </ScrollArea>
        </Tabs.Panel>

        <Tabs.Panel value="resources" pt="md" style={{ flex: 1 }}>
          <ResourceBrowser packageId={pkg.id} onSelectResource={onSelectResource} />
        </Tabs.Panel>

        <Tabs.Panel value="dependencies" pt="md" style={{ flex: 1, overflow: 'auto' }}>
          <ScrollArea h="100%" offsetScrollbars>
            <Stack gap="md" p="sm">
              {pkg.dependencies && pkg.dependencies.length > 0 ? (
                <Table>
                  <Table.Thead>
                    <Table.Tr>
                      <Table.Th>Package</Table.Th>
                      <Table.Th>Version</Table.Th>
                      <Table.Th>Status</Table.Th>
                      <Table.Th />
                    </Table.Tr>
                  </Table.Thead>
                  <Table.Tbody>
                    {pkg.dependencies.map((dep) => (
                      <Table.Tr key={dep.name}>
                        <Table.Td>
                          <Text size="sm" fw={500}>
                            {dep.name}
                          </Text>
                        </Table.Td>
                        <Table.Td>
                          <Badge variant="light">{dep.version}</Badge>
                        </Table.Td>
                        <Table.Td>
                          {dep.isInstalled ? (
                            <Badge
                              color="green"
                              variant="light"
                              leftSection={<IconCheck size={12} />}
                            >
                              Installed
                            </Badge>
                          ) : (
                            <Badge color="gray" variant="light">
                              Not installed
                            </Badge>
                          )}
                        </Table.Td>
                        <Table.Td>
                          {!dep.isInstalled && (
                            <Button size="xs" variant="light">
                              Install
                            </Button>
                          )}
                        </Table.Td>
                      </Table.Tr>
                    ))}
                  </Table.Tbody>
                </Table>
              ) : (
                <Box className={styles.emptyState}>
                  <IconGitBranch size={48} stroke={1.5} className={styles.emptyIcon} />
                  <Text size="lg" fw={500} mt="md">
                    No dependencies
                  </Text>
                  <Text size="sm" c="dimmed" mt="xs">
                    This package has no dependencies
                  </Text>
                </Box>
              )}
            </Stack>
          </ScrollArea>
        </Tabs.Panel>

        <Tabs.Panel value="versions" pt="md" style={{ flex: 1, overflow: 'auto' }}>
          <ScrollArea h="100%" offsetScrollbars>
            <Stack gap="md" p="sm">
              {pkg.versions && pkg.versions.length > 0 ? (
                <Timeline active={0} bulletSize={24} lineWidth={2}>
                  {pkg.versions.map((version, index) => (
                    <Timeline.Item
                      key={version.version}
                      bullet={index === 0 ? <IconCheck size={12} /> : undefined}
                      title={
                        <Group gap="xs">
                          <Text size="sm" fw={500}>
                            {version.version}
                          </Text>
                          {version.version === pkg.version && pkg.installed && (
                            <Badge size="xs" color="green">
                              Current
                            </Badge>
                          )}
                          {version.version === pkg.latestVersion && (
                            <Badge size="xs" color="blue">
                              Latest
                            </Badge>
                          )}
                        </Group>
                      }
                    >
                      <Group gap="md" mt={4}>
                        <Text size="xs" c="dimmed">
                          FHIR {version.fhirVersion}
                        </Text>
                        <Text size="xs" c="dimmed">
                          {version.size}
                        </Text>
                        <Text size="xs" c="dimmed">
                          {new Date(version.publishedDate).toLocaleDateString()}
                        </Text>
                      </Group>
                      {version.version !== pkg.version && (
                        <Button
                          size="xs"
                          variant="light"
                          mt="xs"
                          onClick={() => {
                            onInstallVersion({
                              packageId: pkg.id,
                              version: version.version,
                            });
                          }}
                        >
                          Install this version
                        </Button>
                      )}
                    </Timeline.Item>
                  ))}
                </Timeline>
              ) : (
                <Box className={styles.emptyState}>
                  <IconPackage size={48} stroke={1.5} className={styles.emptyIcon} />
                  <Text size="lg" fw={500} mt="md">
                    No version history
                  </Text>
                  <Text size="sm" c="dimmed" mt="xs">
                    Version history is not available
                  </Text>
                </Box>
              )}
            </Stack>
          </ScrollArea>
        </Tabs.Panel>
      </Tabs>
    </Stack>
  );
}
