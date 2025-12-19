import {
  Alert,
  Button,
  Code,
  Group,
  List,
  Modal,
  SegmentedControl,
  Stack,
  Text,
  Textarea,
  TextInput,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { notifications } from '@mantine/notifications';
import { api } from '@shared/api';
import type { ImportDiagnostic, ImportFormat } from '@shared/types';
import {
  IconAlertCircle,
  IconCheck,
  IconCloudUpload,
  IconFileCode,
  IconFileText,
} from '@tabler/icons-react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { createElement, useCallback, useMemo, useState } from 'react';

interface ImportModalProps {
  opened: boolean;
  onClose: () => void;
  projectId: string;
  onImported?: (profileId: string) => void;
}

interface FormValues {
  format: ImportFormat;
  profileId: string;
  content: string;
  replace: boolean;
}

const slugify = (value: string) =>
  value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/(^-|-$)+/g, '');

export function ImportModal({ opened, onClose, projectId, onImported }: ImportModalProps) {
  const queryClient = useQueryClient();
  const [diagnostics, setDiagnostics] = useState<ImportDiagnostic[]>([]);

  const form = useForm<FormValues>({
    initialValues: {
      format: 'json',
      profileId: '',
      content: '',
      replace: false,
    },
    validate: {
      content: (value) => (!value.trim() ? 'Content is required' : null),
      profileId: (value, values) => {
        // Profile ID is required if we can't extract it from content
        if (!value.trim()) {
          if (values.format === 'json') {
            try {
              const parsed = JSON.parse(values.content);
              if (parsed.id || parsed.name) return null;
            } catch {
              // Can't parse, need explicit ID
            }
          }
          return 'Profile ID is required';
        }
        return null;
      },
    },
  });

  const importMutation = useMutation({
    mutationFn: async (values: FormValues) => {
      // Extract or use provided profile ID
      let profileId = values.profileId.trim();

      if (!profileId && values.format === 'json') {
        try {
          const parsed = JSON.parse(values.content);
          profileId = slugify(parsed.id || parsed.name || 'imported-profile');
        } catch {
          profileId = 'imported-profile';
        }
      } else if (!profileId && values.format === 'fsh') {
        // Try to extract from FSH Profile declaration
        const match = values.content.match(/Profile:\s*(\S+)/i);
        profileId = match ? slugify(match[1]) : 'imported-profile';
      }

      return api.profiles.import(projectId, profileId, {
        format: values.format,
        content: values.content,
        replace: values.replace,
      });
    },
    onSuccess: (data) => {
      const importDiagnostics = data.diagnostics ?? [];
      setDiagnostics(importDiagnostics);

      const hasErrors = importDiagnostics.some((d) => d.severity === 'error');

      // Handle the actual response structure from backend
      // The profile is nested with metadata containing id and name
      const profileData = data.profile as
        | {
            metadata?: { id?: string; name?: string };
            documentId?: string;
          }
        | { id?: string; name?: string }
        | undefined;

      const profileId = profileData?.metadata?.id ?? (profileData as { id?: string })?.id;
      const profileName =
        profileData?.metadata?.name ?? (profileData as { name?: string })?.name ?? 'Profile';

      if (!hasErrors && profileId) {
        notifications.show({
          title: 'Import successful',
          message: `Profile "${profileName}" imported successfully.`,
          icon: createElement(IconCheck, { size: 16 }),
          color: 'green',
        });

        // Invalidate project tree to show new profile
        queryClient.invalidateQueries({ queryKey: ['projectTree', projectId] });

        form.reset();
        setDiagnostics([]);
        onClose();
        onImported?.(profileId);
      } else if (hasErrors) {
        notifications.show({
          title: 'Import completed with errors',
          message: 'Check the diagnostics for details.',
          icon: createElement(IconAlertCircle, { size: 16 }),
          color: 'yellow',
        });
      } else {
        // No errors but no profile - unexpected response
        notifications.show({
          title: 'Import issue',
          message: 'Import completed but no profile was returned.',
          icon: createElement(IconAlertCircle, { size: 16 }),
          color: 'yellow',
        });
      }
    },
    onError: (error) => {
      notifications.show({
        title: 'Import failed',
        message: error instanceof Error ? error.message : 'Unknown error occurred',
        icon: createElement(IconAlertCircle, { size: 16 }),
        color: 'red',
      });
    },
  });

  const handleSubmit = form.onSubmit((values) => {
    setDiagnostics([]);
    importMutation.mutate(values);
  });

  const handleFileUpload = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      if (!file) return;

      const reader = new FileReader();
      reader.onload = (e) => {
        let content = e.target?.result as string;

        // Remove BOM if present (common in files from Windows)
        if (content.charCodeAt(0) === 0xfeff) {
          content = content.slice(1);
        }

        form.setFieldValue('content', content);

        // Auto-detect format from file extension
        if (file.name.endsWith('.fsh')) {
          form.setFieldValue('format', 'fsh');
        } else if (file.name.endsWith('.json')) {
          form.setFieldValue('format', 'json');
        }

        // Try to extract profile ID from filename
        const baseName = file.name.replace(/\.(json|fsh)$/i, '');
        if (!form.values.profileId) {
          form.setFieldValue('profileId', slugify(baseName));
        }
      };
      reader.onerror = () => {
        notifications.show({
          title: 'File read error',
          message: 'Could not read the file. Make sure it is a valid text file.',
          color: 'red',
        });
      };
      // Read as UTF-8 explicitly
      reader.readAsText(file, 'UTF-8');
    },
    [form]
  );

  const inferredId = useMemo(() => {
    if (form.values.profileId.trim()) {
      return form.values.profileId.trim();
    }

    if (form.values.format === 'json' && form.values.content.trim()) {
      try {
        const parsed = JSON.parse(form.values.content);
        return slugify(parsed.id || parsed.name || '');
      } catch {
        return '';
      }
    }

    if (form.values.format === 'fsh' && form.values.content.trim()) {
      const match = form.values.content.match(/Profile:\s*(\S+)/i);
      return match ? slugify(match[1]) : '';
    }

    return '';
  }, [form.values]);

  const handleClose = useCallback(() => {
    form.reset();
    setDiagnostics([]);
    onClose();
  }, [form, onClose]);

  const disabled = importMutation.isPending;

  return (
    <Modal opened={opened} onClose={handleClose} title="Import profile" radius="lg" size="lg">
      <form onSubmit={handleSubmit}>
        <Stack gap="md">
          <SegmentedControl
            value={form.values.format}
            onChange={(value) => form.setFieldValue('format', value as ImportFormat)}
            data={[
              {
                label: (
                  <Group gap="xs">
                    <IconFileCode size={16} />
                    <span>SD JSON</span>
                  </Group>
                ),
                value: 'json',
              },
              {
                label: (
                  <Group gap="xs">
                    <IconFileText size={16} />
                    <span>FSH</span>
                  </Group>
                ),
                value: 'fsh',
              },
            ]}
          />

          <Group>
            <Button
              component="label"
              variant="light"
              leftSection={<IconCloudUpload size={16} />}
              size="sm"
            >
              Upload file
              <input
                type="file"
                accept=".json,.fsh"
                onChange={handleFileUpload}
                style={{ display: 'none' }}
              />
            </Button>
            <Text size="sm" c="dimmed">
              or paste content below
            </Text>
          </Group>

          <Textarea
            label={form.values.format === 'json' ? 'StructureDefinition JSON' : 'FSH content'}
            placeholder={
              form.values.format === 'json'
                ? '{\n  "resourceType": "StructureDefinition",\n  "id": "my-profile",\n  ...\n}'
                : 'Profile: MyProfile\nParent: Patient\n* identifier 1..*'
            }
            autosize
            minRows={8}
            maxRows={16}
            styles={{ input: { fontFamily: 'monospace', fontSize: 12 } }}
            {...form.getInputProps('content')}
          />

          <TextInput
            label="Profile ID"
            description="Leave empty to auto-detect from content"
            placeholder={inferredId || 'e.g., my-custom-profile'}
            {...form.getInputProps('profileId')}
          />

          {inferredId && !form.values.profileId && (
            <Text size="sm" c="dimmed">
              Will use inferred ID: <Code>{inferredId}</Code>
            </Text>
          )}

          {diagnostics.length > 0 && (
            <Alert
              color={diagnostics.some((d) => d.severity === 'error') ? 'red' : 'yellow'}
              title="Import diagnostics"
              icon={<IconAlertCircle size={16} />}
            >
              <List size="sm" spacing="xs">
                {diagnostics.map((d, i) => (
                  <List.Item key={i}>
                    <Text size="sm" c={d.severity === 'error' ? 'red' : 'yellow'}>
                      [{d.code}] {d.message}
                      {d.path && (
                        <Text span c="dimmed">
                          {' '}
                          at {d.path}
                        </Text>
                      )}
                    </Text>
                  </List.Item>
                ))}
              </List>
            </Alert>
          )}

          <Group justify="flex-end">
            <Button variant="light" onClick={handleClose}>
              Cancel
            </Button>
            <Button
              type="submit"
              leftSection={<IconCloudUpload size={16} />}
              loading={disabled}
              disabled={disabled}
            >
              Import
            </Button>
          </Group>
        </Stack>
      </form>
    </Modal>
  );
}
