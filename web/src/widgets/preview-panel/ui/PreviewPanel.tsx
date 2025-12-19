import { ActionIcon, Badge, Group, Paper, Stack, Tabs, Text, Tooltip } from '@mantine/core';
import {
  IconArrowsMaximize,
  IconArrowsMinimize,
  IconCode,
  IconFileImport,
  IconFlask2,
  IconSchema,
  IconSparkles,
} from '@tabler/icons-react';
import { useState } from 'react';
import { DiffView } from './DiffView';
import { FSHPreview } from './FSHPreview';
import { InputItPreview } from './InputItPreview';
import styles from './PreviewPanel.module.css';
import { SDJsonPreview } from './SDJsonPreview';

export type PreviewTab = 'json' | 'fsh' | 'diff' | 'inputIt' | 'schema';

export interface PreviewPanelProps {
  projectId: string;
  profileId: string;
}

export function PreviewPanel({ projectId, profileId }: PreviewPanelProps) {
  const [activeTab, setActiveTab] = useState<PreviewTab>('json');
  const [isFullscreen, setIsFullscreen] = useState(false);

  const handleToggleFullscreen = () => {
    setIsFullscreen((prev) => !prev);
  };

  if (!profileId) {
    return (
      <Paper className={styles.panel} shadow="sm" withBorder>
        <div className={styles.emptyState}>
          <IconCode size={48} stroke={1.5} />
          <Text size="lg" fw={500} mt="md">
            No profile selected
          </Text>
          <Text size="sm" c="dimmed">
            Select a profile to see the preview
          </Text>
        </div>
      </Paper>
    );
  }

  return (
    <Paper
      className={`${styles.panel} ${isFullscreen ? styles.panelFullscreen : ''}`}
      shadow="sm"
      withBorder
    >
      <div className={styles.header}>
        <span className={styles.headerTitle}>Preview</span>
        <div className={styles.headerActions}>
          <Tooltip label={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>
            <ActionIcon variant="subtle" color="gray" onClick={handleToggleFullscreen}>
              {isFullscreen ? <IconArrowsMinimize size={16} /> : <IconArrowsMaximize size={16} />}
            </ActionIcon>
          </Tooltip>
        </div>
      </div>

      <Tabs
        value={activeTab}
        onChange={(value) => setActiveTab(value as PreviewTab)}
        className={styles.tabs}
        classNames={{
          root: styles.tabsRoot,
          list: styles.tabsList,
          panel: styles.tabsPanel,
        }}
      >
        <Tabs.List>
          <Tabs.Tab value="json">SD JSON</Tabs.Tab>
          <Tabs.Tab value="fsh">FSH</Tabs.Tab>
          <Tabs.Tab value="diff">Diff</Tabs.Tab>
          <Tabs.Tab value="inputIt">
            <Group gap={6}>
              <IconFileImport size={14} />
              <span>Input IT</span>
            </Group>
          </Tabs.Tab>
          <Tabs.Tab value="schema">
            <Group gap={6}>
              <IconSchema size={14} />
              <span>FHIR Schema</span>
              <Badge
                size="xs"
                variant="gradient"
                gradient={{ from: 'grape', to: 'pink', deg: 135 }}
                leftSection={<IconSparkles size={8} />}
              >
                Soon
              </Badge>
            </Group>
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="json">
          <SDJsonPreview
            projectId={projectId}
            profileId={profileId}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>

        <Tabs.Panel value="fsh">
          <FSHPreview
            projectId={projectId}
            profileId={profileId}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>

        <Tabs.Panel value="diff">
          <DiffView
            projectId={projectId}
            profileId={profileId}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>

        <Tabs.Panel value="inputIt">
          <InputItPreview
            projectId={projectId}
            profileId={profileId}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>

        <Tabs.Panel value="schema">
          <FhirSchemaPreview />
        </Tabs.Panel>
      </Tabs>
    </Paper>
  );
}

/** FHIR Schema preview placeholder */
function FhirSchemaPreview() {
  return (
    <div className={styles.schemaPreview}>
      <Stack align="center" justify="center" gap="lg" h="100%">
        <div className={styles.schemaIconWrapper}>
          <IconSchema size={48} stroke={1.5} />
        </div>
        <Stack align="center" gap="xs">
          <Group gap="sm">
            <Text size="xl" fw={600}>
              FHIR Schema
            </Text>
            <Badge
              size="lg"
              variant="gradient"
              gradient={{ from: 'grape', to: 'pink', deg: 135 }}
              leftSection={<IconSparkles size={12} />}
            >
              Coming Soon
            </Badge>
          </Group>
          <Text size="sm" c="dimmed" ta="center" maw={400}>
            Next-generation schema validation using the official FHIR Schema format. Faster, more
            precise, and spec-compliant validation.
          </Text>
        </Stack>
        <Group gap="xs" mt="md">
          <Badge variant="light" color="grape" leftSection={<IconFlask2 size={12} />}>
            Experimental
          </Badge>
          <Badge variant="light" color="blue">
            JSON Schema Compatible
          </Badge>
        </Group>
      </Stack>
    </div>
  );
}
