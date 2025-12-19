import { Alert, Center, Loader, SegmentedControl, Stack, Text } from '@mantine/core';
import { DiffEditor } from '@monaco-editor/react';
import { IconAlertCircle, IconGitCompare } from '@tabler/icons-react';
import { useCallback, useMemo, useState } from 'react';
import { computeDiff, type DiffLine, useSDJsonPreview } from '../lib/usePreview';
import styles from './PreviewPanel.module.css';
import { DiffToolbar } from './PreviewToolbar';

export interface DiffViewProps {
  projectId: string;
  profileId: string;
  baseContent: string;
  isFullscreen: boolean;
  onToggleFullscreen: () => void;
}

export function DiffView({
  projectId,
  profileId,
  baseContent,
  isFullscreen,
  onToggleFullscreen,
}: DiffViewProps) {
  const { data, isLoading, error } = useSDJsonPreview(projectId, profileId);
  const [diffMode, setDiffMode] = useState<'side-by-side' | 'unified'>('side-by-side');
  const [exportMode, setExportMode] = useState<'differential' | 'snapshot' | 'both'>(
    'differential'
  );

  const modifiedContent = data?.content || '';

  // Compute diff for unified view
  const diffLines = useMemo(() => {
    if (diffMode === 'unified') {
      return computeDiff(baseContent, modifiedContent);
    }
    return [];
  }, [baseContent, modifiedContent, diffMode]);

  const handleDownload = useCallback(() => {
    // Implement download logic based on exportMode
    const filename = `${profileId}-${exportMode}.json`;
    const blob = new Blob([modifiedContent], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  }, [profileId, exportMode, modifiedContent]);

  if (isLoading) {
    return (
      <Center h="100%">
        <Stack align="center" gap="sm">
          <Loader size="lg" />
          <Text size="sm" c="dimmed">
            Computing diff...
          </Text>
        </Stack>
      </Center>
    );
  }

  if (error) {
    return (
      <div className={styles.errorState}>
        <Alert icon={<IconAlertCircle size={16} />} title="Failed to generate diff" color="red">
          {error.message}
        </Alert>
      </div>
    );
  }

  if (!modifiedContent || !baseContent) {
    return (
      <div className={styles.emptyState}>
        <IconGitCompare size={48} stroke={1.5} />
        <Text size="lg" fw={500} mt="md">
          No diff available
        </Text>
        <Text size="sm" c="dimmed">
          Select a profile to compare changes with the base definition
        </Text>
      </div>
    );
  }

  return (
    <>
      <DiffToolbar
        diffMode={diffMode}
        onDiffModeChange={setDiffMode}
        exportMode={exportMode}
        onExportModeChange={setExportMode}
        onDownload={handleDownload}
        isFullscreen={isFullscreen}
        onToggleFullscreen={onToggleFullscreen}
      />
      <div className={styles.editorContainer}>
        {diffMode === 'side-by-side' ? (
          <DiffEditor
            height="100%"
            language="json"
            original={baseContent}
            modified={modifiedContent}
            options={{
              readOnly: true,
              renderSideBySide: true,
              minimap: { enabled: false },
              scrollBeyondLastLine: false,
              automaticLayout: true,
              fontSize: 13,
              fontFamily: 'JetBrains Mono, Menlo, Monaco, Consolas, monospace',
              originalEditable: false,
              scrollbar: {
                vertical: 'auto',
                horizontal: 'auto',
                verticalScrollbarSize: 10,
                horizontalScrollbarSize: 10,
              },
            }}
            theme="vs"
          />
        ) : (
          <UnifiedDiffView lines={diffLines} />
        )}
      </div>
    </>
  );
}

interface UnifiedDiffViewProps {
  lines: DiffLine[];
}

function UnifiedDiffView({ lines }: UnifiedDiffViewProps) {
  return (
    <div className={styles.diffContainer}>
      {lines.map((line, index) => (
        <div
          key={index}
          className={`${styles.diffLine} ${
            line.type === 'added'
              ? styles.diffLineAdded
              : line.type === 'removed'
                ? styles.diffLineRemoved
                : line.type === 'modified'
                  ? styles.diffLineModified
                  : ''
          }`}
        >
          <div className={styles.diffLineNumber}>
            {line.type === 'removed' ? line.originalLineNumber : line.lineNumber}
          </div>
          <div className={styles.diffLineContent}>
            {line.type === 'added' && '+ '}
            {line.type === 'removed' && '- '}
            {line.content}
          </div>
        </div>
      ))}
    </div>
  );
}
