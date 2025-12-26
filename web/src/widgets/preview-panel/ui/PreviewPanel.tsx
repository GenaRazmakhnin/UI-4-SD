import { ActionIcon, Group, Paper, Tabs, Text, Tooltip } from '@mantine/core';
import {
  IconArrowsMaximize,
  IconArrowsMinimize,
  IconCode,
  IconFileImport,
  IconSchema,
} from '@tabler/icons-react';
import { useState } from 'react';
import { DiffView } from './DiffView';
import { FhirSchemaPreview } from './FhirSchemaPreview';
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
          <FhirSchemaPreview
            projectId={projectId}
            profileId={profileId}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>
      </Tabs>
    </Paper>
  );
}
