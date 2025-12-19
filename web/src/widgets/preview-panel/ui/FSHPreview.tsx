import { Alert, Center, Loader, Stack, Text } from '@mantine/core';
import Editor, { type Monaco, type OnMount } from '@monaco-editor/react';
import { IconAlertCircle, IconCode } from '@tabler/icons-react';
import { useCallback, useRef, useState } from 'react';
import { registerFSHLanguage } from '../lib/fsh-language';
import { useFSHPreview } from '../lib/usePreview';
import styles from './PreviewPanel.module.css';
import { PreviewToolbar } from './PreviewToolbar';

export interface FSHPreviewProps {
  projectId: string;
  profileId: string;
  isFullscreen: boolean;
  onToggleFullscreen: () => void;
}

export function FSHPreview({
  projectId,
  profileId,
  isFullscreen,
  onToggleFullscreen,
}: FSHPreviewProps) {
  const { data, isLoading, error } = useFSHPreview(projectId, profileId);
  const editorRef = useRef<Parameters<OnMount>[0] | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [showLineNumbers, setShowLineNumbers] = useState(true);
  const [showMinimap, setShowMinimap] = useState(true);
  const [wordWrap, setWordWrap] = useState(false);

  const handleEditorWillMount = useCallback((monaco: Monaco) => {
    registerFSHLanguage(monaco);
  }, []);

  const handleEditorMount: OnMount = useCallback((editor) => {
    editorRef.current = editor;
  }, []);

  const handleSearchChange = useCallback((query: string) => {
    setSearchQuery(query);
    if (editorRef.current && query) {
      const model = editorRef.current.getModel();
      if (model) {
        const matches = model.findMatches(query, true, false, false, null, true);
        if (matches.length > 0) {
          editorRef.current.setSelection(matches[0].range);
          editorRef.current.revealLineInCenter(matches[0].range.startLineNumber);
        }
      }
    }
  }, []);

  const handleSettingChange = useCallback((setting: string, value: boolean) => {
    switch (setting) {
      case 'showLineNumbers':
        setShowLineNumbers(value);
        break;
      case 'showMinimap':
        setShowMinimap(value);
        break;
      case 'wordWrap':
        setWordWrap(value);
        break;
    }
  }, []);

  if (isLoading) {
    return (
      <Center h="100%">
        <Stack align="center" gap="sm">
          <Loader size="lg" />
          <Text size="sm" c="dimmed">
            Generating FSH...
          </Text>
        </Stack>
      </Center>
    );
  }

  if (error) {
    return (
      <div className={styles.errorState}>
        <Alert icon={<IconAlertCircle size={16} />} title="Failed to generate preview" color="red">
          {error.message}
        </Alert>
      </div>
    );
  }

  if (!data?.content) {
    return (
      <div className={styles.emptyState}>
        <IconCode size={48} stroke={1.5} />
        <Text size="lg" fw={500} mt="md">
          No preview available
        </Text>
        <Text size="sm" c="dimmed">
          Select a profile to see the FSH preview
        </Text>
      </div>
    );
  }

  return (
    <>
      <PreviewToolbar
        content={data.content}
        filename={data.filename}
        format="fsh"
        searchQuery={searchQuery}
        onSearchChange={handleSearchChange}
        isFullscreen={isFullscreen}
        onToggleFullscreen={onToggleFullscreen}
        showLineNumbers={showLineNumbers}
        showMinimap={showMinimap}
        wordWrap={wordWrap}
        onSettingChange={handleSettingChange}
      />
      <div className={styles.editorContainer}>
        <Editor
          height="100%"
          language="fsh"
          value={data.content}
          beforeMount={handleEditorWillMount}
          onMount={handleEditorMount}
          options={{
            readOnly: true,
            minimap: { enabled: showMinimap },
            lineNumbers: showLineNumbers ? 'on' : 'off',
            wordWrap: wordWrap ? 'on' : 'off',
            scrollBeyondLastLine: false,
            folding: true,
            automaticLayout: true,
            fontSize: 13,
            fontFamily: 'JetBrains Mono, Menlo, Monaco, Consolas, monospace',
            renderLineHighlight: 'none',
            overviewRulerLanes: 0,
            hideCursorInOverviewRuler: true,
            scrollbar: {
              vertical: 'auto',
              horizontal: 'auto',
              verticalScrollbarSize: 10,
              horizontalScrollbarSize: 10,
            },
          }}
          theme="fsh-theme"
        />
      </div>
    </>
  );
}
