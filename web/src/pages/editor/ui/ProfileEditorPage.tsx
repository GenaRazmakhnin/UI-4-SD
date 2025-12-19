import {
  ActionConfirmationDialog,
  ElementContextMenu,
  elementSelectionChanged,
  QuickActionsToolbar,
  ShortcutHelpOverlay,
  useQuickActionShortcuts,
} from '@features/quick-constraints';
import { HistoryViewer, UndoRedoProvider, useUndoRedoShortcuts } from '@features/undo-redo';
import { Tabs } from '@mantine/core';
import { useHotkeys } from '@mantine/hooks';
import { useProfileResourceGuard } from '@shared/routing/guards';
import { useParams } from '@tanstack/react-router';
import {
  $errorCount,
  $isValidating,
  $warningCount,
  DiagnosticsPanel,
  validateProfileFx,
} from '@widgets/diagnostics-panel';
import {
  $profileContext,
  $selectedElement,
  $loadError as $treeError,
  $isLoading as $treeLoading,
  ElementTree,
  loadProfileFx,
} from '@widgets/element-tree';
import { InspectorPanel } from '@widgets/inspector-panel';
import { PreviewPanel } from '@widgets/preview-panel';
import { useUnit } from 'effector-react';
import { useCallback, useEffect, useState } from 'react';
import { Panel, Group as PanelGroup, Separator as PanelResizeHandle } from 'react-resizable-panels';
import { useUnsavedChangesWarning } from '../lib';
import {
  $hasUnsavedChanges,
  $saveStatus,
  type ExportFormat,
  exportProfileFx,
  saveProfileFx,
} from '../model';
import { EditorToolbar } from './EditorToolbar';
import styles from './ProfileEditorPage.module.css';

type BottomTab = 'preview' | 'diagnostics';

export function ProfileEditorPage() {
  const { projectId, profileId } = useParams({
    from: '/projects/$projectId/profiles/$profileId',
  });
  const { resource } = useProfileResourceGuard({ projectId, profileId });
  const selectedElement = useUnit($selectedElement);
  const profilesContext = useUnit($profileContext);
  const treeLoading = useUnit($treeLoading);
  const treeError = useUnit($treeError);
  const [bottomTab, setBottomTab] = useState<BottomTab>('preview');

  // Editor state
  const saveStatus = useUnit($saveStatus);
  const hasUnsavedChanges = useUnit($hasUnsavedChanges);
  const errorCount = useUnit($errorCount);
  const warningCount = useUnit($warningCount);
  const isValidating = useUnit($isValidating);

  // Load element tree when profile changes
  useEffect(() => {
    if (projectId && profileId) {
      loadProfileFx({ projectId, profileId });
    }
  }, [projectId, profileId]);

  // Sync selected element to quick-constraints model
  useEffect(() => {
    elementSelectionChanged(selectedElement);
  }, [selectedElement]);

  // Register shortcuts and warnings
  useQuickActionShortcuts();
  useUndoRedoShortcuts();
  useUnsavedChangesWarning();

  // Handlers
  const handleSave = useCallback(() => {
    saveProfileFx({ projectId, profileId });
  }, [projectId, profileId]);

  const handleValidate = useCallback(() => {
    validateProfileFx({ projectId, profileId });
  }, [projectId, profileId]);

  const handleExport = useCallback(
    (format: ExportFormat) => {
      exportProfileFx({ projectId, profileId, format });
    },
    [projectId, profileId]
  );

  const handleSettingsClick = useCallback(() => {
    // TODO: Open settings modal
  }, []);

  // Keyboard shortcuts
  useHotkeys([
    [
      'mod+s',
      (e) => {
        e.preventDefault();
        handleSave();
      },
    ],
    [
      'F5',
      (e) => {
        e.preventDefault();
        handleValidate();
      },
    ],
  ]);

  const profileName = resource?.name ?? `Profile: ${profileId}`;
  const profileType = resource?.resourceType ?? 'StructureDefinition';
  const resourceKind = resource?.resourceKind ?? 'profile';

  return (
    <UndoRedoProvider>
      <div className={styles.container}>
        {/* Editor Toolbar */}
        <EditorToolbar
          profileName={profileName}
          profileType={profileType}
          resourceKind={resourceKind}
          saveStatus={saveStatus}
          hasUnsavedChanges={hasUnsavedChanges}
          isValidating={isValidating}
          errorCount={errorCount}
          warningCount={warningCount}
          onSave={handleSave}
          onValidate={handleValidate}
          onExport={handleExport}
          onSettingsClick={handleSettingsClick}
        />

        <div className={styles.mainContent}>
          <PanelGroup className={styles.panelGroup} orientation="horizontal">
            <Panel id="tree" defaultSize="20" minSize="15" maxSize="35">
              <div className={styles.leftPanel}>
                <ElementTree />
              </div>
            </Panel>

            <PanelResizeHandle className={styles.resizeHandleHorizontal} />

            <Panel id="center" defaultSize="45" minSize="20" maxSize="65">
              <div className={styles.centerPanel}>
                <QuickActionsToolbar element={selectedElement} />

                <PanelGroup className={styles.centerSplitGroup} orientation="vertical">
                  <Panel id="inspector" defaultSize="60" minSize="25">
                    <div className={styles.inspectorContainer}>
                      <InspectorPanel />
                    </div>
                  </Panel>
                </PanelGroup>
              </div>
            </Panel>

            <PanelResizeHandle className={styles.resizeHandleHorizontal} />

            <Panel id="preview" defaultSize="35" minSize="20" maxSize="50">
              <div className={styles.rightPanel}>
                <Tabs
                  className={styles.tabs}
                  value={bottomTab}
                  onChange={(v) => setBottomTab(v as BottomTab)}
                >
                  <Tabs.List>
                    <Tabs.Tab value="preview">Preview</Tabs.Tab>
                    <Tabs.Tab value="diagnostics">
                      Diagnostics
                      {(errorCount > 0 || warningCount > 0) && (
                        <span className={styles.tabBadge}>{errorCount + warningCount}</span>
                      )}
                    </Tabs.Tab>
                  </Tabs.List>
                  <Tabs.Panel value="preview" className={styles.tabPanel}>
                    <PreviewPanel projectId={projectId} profileId={profileId} />
                  </Tabs.Panel>
                  <Tabs.Panel value="diagnostics" className={styles.tabPanel}>
                    <DiagnosticsPanel profileId={profileId} collapsible={false} />
                  </Tabs.Panel>
                </Tabs>
              </div>
            </Panel>
          </PanelGroup>
        </div>

        <ElementContextMenu />
        <ActionConfirmationDialog />
        <ShortcutHelpOverlay />
        <HistoryViewer />
      </div>
    </UndoRedoProvider>
  );
}
