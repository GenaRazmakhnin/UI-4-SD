import { Badge, Card, Group, SimpleGrid, Stack, Text, TextInput, ThemeIcon } from '@mantine/core';
import {
  IconFile,
  IconFlag,
  IconHeartbeat,
  IconLock,
  IconSearch,
  IconSparkles,
  IconWorld,
} from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState } from 'react';
import { PROJECT_TEMPLATES, searchTemplates } from '../lib/templates';
import type { ProjectTemplate } from '../lib/types';
import { $selectedTemplate, templateSelected } from '../model';
import styles from './TemplateGallery.module.css';

// Icon mapping
const TEMPLATE_ICONS: Record<string, React.ComponentType<{ size?: number }>> = {
  IconFile,
  IconFlag,
  IconWorld,
  IconHeartbeat,
  IconLock,
  IconSparkles,
};

interface TemplateCardProps {
  template: ProjectTemplate;
  isSelected: boolean;
  onSelect: (id: string) => void;
}

function TemplateCard({ template, isSelected, onSelect }: TemplateCardProps) {
  const Icon = TEMPLATE_ICONS[template.icon] || IconFile;

  return (
    <Card
      className={`${styles.card} ${isSelected ? styles.selected : ''}`}
      padding="md"
      radius="md"
      withBorder
      onClick={() => onSelect(template.id)}
    >
      <Stack gap="sm">
        <Group justify="space-between">
          <ThemeIcon size="lg" radius="md" variant="light" color="blue">
            <Icon size={20} />
          </ThemeIcon>
          <Badge size="sm" variant="light">
            FHIR {template.fhirVersion}
          </Badge>
        </Group>

        <div>
          <Text fw={600} size="md">
            {template.name}
          </Text>
          <Text size="sm" c="dimmed" lineClamp={2}>
            {template.description}
          </Text>
        </div>

        {template.dependencies.length > 0 && (
          <Group gap="xs">
            {template.dependencies.slice(0, 2).map((dep) => (
              <Badge key={dep.packageId} size="xs" variant="outline" color="gray">
                {dep.name}
              </Badge>
            ))}
            {template.dependencies.length > 2 && (
              <Badge size="xs" variant="outline" color="gray">
                +{template.dependencies.length - 2}
              </Badge>
            )}
          </Group>
        )}

        {template.structure.profiles.length > 0 && (
          <Text size="xs" c="dimmed">
            Includes: {template.structure.profiles.slice(0, 2).join(', ')}
            {template.structure.profiles.length > 2 && '...'}
          </Text>
        )}
      </Stack>
    </Card>
  );
}

export function TemplateGallery() {
  const selectedTemplate = useUnit($selectedTemplate);
  const [searchQuery, setSearchQuery] = useState('');

  const templates = searchQuery ? searchTemplates(searchQuery) : PROJECT_TEMPLATES;

  const handleSelect = (templateId: string) => {
    templateSelected(templateId);
  };

  // Group templates by category
  const blankTemplates = templates.filter((t) => t.category === 'blank');
  const igTemplates = templates.filter((t) => t.category === 'implementation-guide');
  const regionalTemplates = templates.filter((t) => t.category === 'regional');

  return (
    <Stack gap="md">
      <TextInput
        placeholder="Search templates..."
        leftSection={<IconSearch size={16} />}
        value={searchQuery}
        onChange={(e) => setSearchQuery(e.currentTarget.value)}
      />

      {blankTemplates.length > 0 && (
        <>
          <Text size="sm" fw={600} c="dimmed">
            Start Fresh
          </Text>
          <SimpleGrid cols={{ base: 1, sm: 2 }}>
            {blankTemplates.map((template) => (
              <TemplateCard
                key={template.id}
                template={template}
                isSelected={selectedTemplate?.id === template.id}
                onSelect={handleSelect}
              />
            ))}
          </SimpleGrid>
        </>
      )}

      {regionalTemplates.length > 0 && (
        <>
          <Text size="sm" fw={600} c="dimmed" mt="md">
            Regional
          </Text>
          <SimpleGrid cols={{ base: 1, sm: 2 }}>
            {regionalTemplates.map((template) => (
              <TemplateCard
                key={template.id}
                template={template}
                isSelected={selectedTemplate?.id === template.id}
                onSelect={handleSelect}
              />
            ))}
          </SimpleGrid>
        </>
      )}

      {igTemplates.length > 0 && (
        <>
          <Text size="sm" fw={600} c="dimmed" mt="md">
            Implementation Guides
          </Text>
          <SimpleGrid cols={{ base: 1, sm: 2 }}>
            {igTemplates.map((template) => (
              <TemplateCard
                key={template.id}
                template={template}
                isSelected={selectedTemplate?.id === template.id}
                onSelect={handleSelect}
              />
            ))}
          </SimpleGrid>
        </>
      )}

      {templates.length === 0 && (
        <Text c="dimmed" ta="center" py="xl">
          No templates found matching "{searchQuery}"
        </Text>
      )}
    </Stack>
  );
}
