import { ActionIcon, Group, Menu, SegmentedControl, TextInput, Tooltip } from '@mantine/core';
import {
  IconArrowsMaximize,
  IconArrowsMinimize,
  IconCheck,
  IconCopy,
  IconDownload,
  IconSearch,
  IconSettings,
  IconX,
} from '@tabler/icons-react';
import { useClipboard, useFileDownload } from '../lib/usePreview';
import styles from './PreviewPanel.module.css';

export interface PreviewToolbarProps {
  content: string;
  filename: string;
  format: 'json' | 'fsh';
  searchQuery: string;
  onSearchChange: (query: string) => void;
  isFullscreen: boolean;
  onToggleFullscreen: () => void;
  showLineNumbers: boolean;
  showMinimap: boolean;
  wordWrap: boolean;
  onSettingChange: (setting: string, value: boolean) => void;
}

export function PreviewToolbar({
  content,
  filename,
  format,
  searchQuery,
  onSearchChange,
  isFullscreen,
  onToggleFullscreen,
  showLineNumbers,
  showMinimap,
  wordWrap,
  onSettingChange,
}: PreviewToolbarProps) {
  const { copied, copy } = useClipboard();
  const { download } = useFileDownload();

  const mimeType = format === 'json' ? 'application/json' : 'text/plain';

  const handleCopy = () => {
    copy(content);
  };

  const handleDownload = () => {
    download(content, filename, mimeType);
  };

  return (
    <div className={styles.toolbar}>
      <div className={styles.toolbarLeft}>
        <TextInput
          className={styles.searchInput}
          placeholder="Search..."
          size="xs"
          leftSection={<IconSearch size={14} />}
          rightSection={
            searchQuery ? (
              <ActionIcon size="xs" variant="subtle" onClick={() => onSearchChange('')}>
                <IconX size={12} />
              </ActionIcon>
            ) : null
          }
          value={searchQuery}
          onChange={(e) => onSearchChange(e.currentTarget.value)}
        />
      </div>

      <div className={styles.toolbarRight}>
        <Tooltip label={copied ? 'Copied!' : 'Copy to clipboard'}>
          <ActionIcon
            variant="subtle"
            color={copied ? 'green' : 'gray'}
            onClick={handleCopy}
            disabled={!content}
          >
            {copied ? <IconCheck size={16} /> : <IconCopy size={16} />}
          </ActionIcon>
        </Tooltip>

        <Tooltip label="Download">
          <ActionIcon variant="subtle" color="gray" onClick={handleDownload} disabled={!content}>
            <IconDownload size={16} />
          </ActionIcon>
        </Tooltip>

        <Menu shadow="md" width={200}>
          <Menu.Target>
            <Tooltip label="Settings">
              <ActionIcon variant="subtle" color="gray">
                <IconSettings size={16} />
              </ActionIcon>
            </Tooltip>
          </Menu.Target>

          <Menu.Dropdown>
            <Menu.Label>Editor Settings</Menu.Label>
            <Menu.Item
              closeMenuOnClick={false}
              onClick={() => onSettingChange('showLineNumbers', !showLineNumbers)}
            >
              <Group justify="space-between">
                <span>Line Numbers</span>
                {showLineNumbers && <IconCheck size={14} />}
              </Group>
            </Menu.Item>
            <Menu.Item
              closeMenuOnClick={false}
              onClick={() => onSettingChange('showMinimap', !showMinimap)}
            >
              <Group justify="space-between">
                <span>Minimap</span>
                {showMinimap && <IconCheck size={14} />}
              </Group>
            </Menu.Item>
            <Menu.Item
              closeMenuOnClick={false}
              onClick={() => onSettingChange('wordWrap', !wordWrap)}
            >
              <Group justify="space-between">
                <span>Word Wrap</span>
                {wordWrap && <IconCheck size={14} />}
              </Group>
            </Menu.Item>
          </Menu.Dropdown>
        </Menu>

        <Tooltip label={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>
          <ActionIcon variant="subtle" color="gray" onClick={onToggleFullscreen}>
            {isFullscreen ? <IconArrowsMinimize size={16} /> : <IconArrowsMaximize size={16} />}
          </ActionIcon>
        </Tooltip>
      </div>
    </div>
  );
}

export interface DiffToolbarProps {
  diffMode: 'side-by-side' | 'unified';
  onDiffModeChange: (mode: 'side-by-side' | 'unified') => void;
  exportMode: 'differential' | 'snapshot' | 'both';
  onExportModeChange: (mode: 'differential' | 'snapshot' | 'both') => void;
  onDownload: () => void;
  isFullscreen: boolean;
  onToggleFullscreen: () => void;
}

export function DiffToolbar({
  diffMode,
  onDiffModeChange,
  exportMode,
  onExportModeChange,
  onDownload,
  isFullscreen,
  onToggleFullscreen,
}: DiffToolbarProps) {
  return (
    <div className={styles.toolbar}>
      <div className={styles.toolbarLeft}>
        <SegmentedControl
          size="xs"
          value={diffMode}
          onChange={(value) => onDiffModeChange(value as 'side-by-side' | 'unified')}
          data={[
            { label: 'Side by Side', value: 'side-by-side' },
            { label: 'Unified', value: 'unified' },
          ]}
        />
      </div>

      <div className={styles.toolbarRight}>
        <Menu shadow="md" width={200}>
          <Menu.Target>
            <Tooltip label="Export options">
              <ActionIcon variant="subtle" color="gray">
                <IconDownload size={16} />
              </ActionIcon>
            </Tooltip>
          </Menu.Target>

          <Menu.Dropdown>
            <Menu.Label>Export Mode</Menu.Label>
            <Menu.Item
              onClick={() => onExportModeChange('differential')}
              rightSection={exportMode === 'differential' ? <IconCheck size={14} /> : null}
            >
              Differential Only
            </Menu.Item>
            <Menu.Item
              onClick={() => onExportModeChange('snapshot')}
              rightSection={exportMode === 'snapshot' ? <IconCheck size={14} /> : null}
            >
              Snapshot Only
            </Menu.Item>
            <Menu.Item
              onClick={() => onExportModeChange('both')}
              rightSection={exportMode === 'both' ? <IconCheck size={14} /> : null}
            >
              Both
            </Menu.Item>
            <Menu.Divider />
            <Menu.Item onClick={onDownload}>Download</Menu.Item>
          </Menu.Dropdown>
        </Menu>

        <Tooltip label={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>
          <ActionIcon variant="subtle" color="gray" onClick={onToggleFullscreen}>
            {isFullscreen ? <IconArrowsMinimize size={16} /> : <IconArrowsMaximize size={16} />}
          </ActionIcon>
        </Tooltip>
      </div>
    </div>
  );
}
