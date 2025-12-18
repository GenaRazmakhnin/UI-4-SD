import { ActionIcon, Paper, Tabs, Text, Tooltip } from '@mantine/core';
import { IconArrowsMaximize, IconArrowsMinimize, IconCode } from '@tabler/icons-react';
import { useState } from 'react';
import { DiffView } from './DiffView';
import { FSHPreview } from './FSHPreview';
import styles from './PreviewPanel.module.css';
import { SDJsonPreview } from './SDJsonPreview';

export type PreviewTab = 'json' | 'fsh' | 'diff';

export interface PreviewPanelProps {
  profileId: string;
  baseContent?: string;
}

export function PreviewPanel({ profileId, baseContent = '' }: PreviewPanelProps) {
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
        </Tabs.List>

        <Tabs.Panel value="json">
          <SDJsonPreview
            profileId={profileId}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>

        <Tabs.Panel value="fsh">
          <FSHPreview
            profileId={profileId}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>

        <Tabs.Panel value="diff">
          <DiffView
            profileId={profileId}
            baseContent={baseContent}
            isFullscreen={isFullscreen}
            onToggleFullscreen={handleToggleFullscreen}
          />
        </Tabs.Panel>
      </Tabs>
    </Paper>
  );
}
