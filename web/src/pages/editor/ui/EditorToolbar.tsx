import { UndoRedoToolbar } from '@features/undo-redo';
import {
  ActionIcon,
  Badge,
  Button,
  Divider,
  Group,
  Loader,
  Menu,
  Text,
  Tooltip,
} from '@mantine/core';
import type { ProjectResourceKind } from '@shared/types';
import {
  IconAlertCircle,
  IconCheck,
  IconChevronDown,
  IconCode,
  IconDownload,
  IconFileCode,
  IconSettings,
} from '@tabler/icons-react';

export type SaveStatus = 'idle' | 'saving' | 'saved' | 'error';
export type ExportFormat = 'json' | 'fsh';

interface EditorToolbarProps {
  profileName: string;
  profileType?: string;
  resourceKind?: ProjectResourceKind | string;
  saveStatus: SaveStatus;
  hasUnsavedChanges: boolean;
  isValidating: boolean;
  errorCount?: number;
  warningCount?: number;
  onSave: () => void;
  onValidate: () => void;
  onExport: (format: ExportFormat) => void;
  onSettingsClick: () => void;
}

export function EditorToolbar({
  profileName,
  profileType,
  saveStatus,
  hasUnsavedChanges,
  resourceKind,
  isValidating,
  errorCount = 0,
  warningCount = 0,
  onSave,
  onValidate,
  onExport,
  onSettingsClick,
}: EditorToolbarProps) {
  const getSaveButtonContent = () => {
    switch (saveStatus) {
      case 'saving':
        return (
          <>
            <Loader size={14} color="white" />
            <span>Saving...</span>
          </>
        );
      case 'saved':
        return (
          <>
            <IconCheck size={14} />
            <span>Saved</span>
          </>
        );
      case 'error':
        return (
          <>
            <IconAlertCircle size={14} />
            <span>Error</span>
          </>
        );
      default:
        return 'Save';
    }
  };

  const getSaveButtonColor = () => {
    switch (saveStatus) {
      case 'saved':
        return 'green';
      case 'error':
        return 'red';
      default:
        return 'blue';
    }
  };

  return (
    <Group
      h={48}
      px="md"
      justify="space-between"
      style={{ borderBottom: '1px solid var(--mantine-color-gray-3)' }}
    >
      {/* Left side - Profile info */}
      <Group gap="sm">
        <Text fw={600} size="sm">
          {profileName}
        </Text>
        {resourceKind && (
          <Badge variant="light" size="sm" color={resourceKindColor(resourceKind)}>
            {formatResourceKind(resourceKind)}
          </Badge>
        )}
        {profileType && (
          <Badge variant="light" size="sm" color="blue">
            {profileType}
          </Badge>
        )}
        {hasUnsavedChanges && (
          <Badge variant="dot" size="sm" color="orange">
            Unsaved
          </Badge>
        )}
      </Group>

      {/* Right side - Actions */}
      <Group gap="xs">
        {/* Undo/Redo */}
        <UndoRedoToolbar showHistoryButton={false} />

        <Divider orientation="vertical" />

        {/* Validate */}
        <Tooltip label="Validate profile (F5)" position="bottom" withArrow>
          <Button
            variant="subtle"
            color="gray"
            size="compact-sm"
            leftSection={isValidating ? <Loader size={14} /> : undefined}
            onClick={onValidate}
            disabled={isValidating}
          >
            {isValidating ? 'Validating...' : 'Validate'}
          </Button>
        </Tooltip>

        {/* Validation status badges */}
        {(errorCount > 0 || warningCount > 0) && (
          <Group gap={4}>
            {errorCount > 0 && (
              <Badge size="sm" color="red" variant="filled">
                {errorCount} {errorCount === 1 ? 'error' : 'errors'}
              </Badge>
            )}
            {warningCount > 0 && (
              <Badge size="sm" color="orange" variant="filled">
                {warningCount} {warningCount === 1 ? 'warning' : 'warnings'}
              </Badge>
            )}
          </Group>
        )}

        <Divider orientation="vertical" />

        {/* Export menu */}
        <Menu shadow="md" width={180} position="bottom-end">
          <Menu.Target>
            <Button
              variant="subtle"
              color="gray"
              size="compact-sm"
              rightSection={<IconChevronDown size={14} />}
            >
              <IconDownload size={14} style={{ marginRight: 4 }} />
              Export
            </Button>
          </Menu.Target>
          <Menu.Dropdown>
            <Menu.Label>Export format</Menu.Label>
            <Menu.Item leftSection={<IconFileCode size={16} />} onClick={() => onExport('json')}>
              StructureDefinition (JSON)
            </Menu.Item>
            <Menu.Item leftSection={<IconCode size={16} />} onClick={() => onExport('fsh')}>
              FHIR Shorthand (FSH)
            </Menu.Item>
          </Menu.Dropdown>
        </Menu>

        {/* Save button */}
        <Tooltip label="Save (Ctrl+S)" position="bottom" withArrow>
          <Button
            size="compact-sm"
            color={getSaveButtonColor()}
            onClick={onSave}
            disabled={saveStatus === 'saving' || (!hasUnsavedChanges && saveStatus !== 'error')}
          >
            {getSaveButtonContent()}
          </Button>
        </Tooltip>

        <Divider orientation="vertical" />

        {/* Settings */}
        <Tooltip label="Settings" position="bottom" withArrow>
          <ActionIcon variant="subtle" color="gray" onClick={onSettingsClick}>
            <IconSettings size={18} />
          </ActionIcon>
        </Tooltip>
      </Group>
    </Group>
  );
}

function resourceKindColor(kind: ProjectResourceKind | string) {
  switch (kind) {
    case 'profile':
      return 'blue';
    case 'extension':
      return 'cyan';
    case 'valueset':
      return 'violet';
    case 'codesystem':
      return 'grape';
    case 'instance':
      return 'teal';
    default:
      return 'gray';
  }
}

function formatResourceKind(kind: ProjectResourceKind | string) {
  switch (kind) {
    case 'valueset':
      return 'ValueSet';
    case 'codesystem':
      return 'CodeSystem';
    case 'extension':
      return 'Extension';
    case 'instance':
      return 'Instance';
    case 'operation':
      return 'Operation';
    case 'mapping':
      return 'Mapping';
    case 'example':
      return 'Example';
    case 'profile':
      return 'Profile';
    default:
      return `${kind.charAt(0).toUpperCase()}${kind.slice(1)}`;
  }
}
