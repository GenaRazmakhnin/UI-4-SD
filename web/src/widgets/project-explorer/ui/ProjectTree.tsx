import { Badge, Group, Text, Tooltip } from '@mantine/core';
import { IconFlask2, IconRocket, IconSchema, IconSparkles } from '@tabler/icons-react';
import { List } from 'react-window';
import styles from './ProjectExplorer.module.css';
import { ProjectTreeRow, type ProjectTreeRowData } from './ProjectTreeRow';

interface ProjectTreeProps extends ProjectTreeRowData {}

export function ProjectTree(props: ProjectTreeProps) {
  return (
    <div className={styles.treeList} role="tree" aria-label="Project files">
      <List
        defaultHeight={480}
        rowCount={props.rows.length}
        rowHeight={40}
        rowComponent={ProjectTreeRow}
        rowProps={props}
        style={{ height: 'calc(100% - 48px)', width: '100%' }}
      />
      <FhirSchemaTeaser />
    </div>
  );
}

function FhirSchemaTeaser() {
  return (
    <Tooltip
      label="FHIR Schema support is brewing! Validate resources with next-gen schemas."
      position="top"
      withArrow
      multiline
      w={260}
    >
      <div className={styles.schemaTeaser}>
        <Group gap={8} wrap="nowrap">
          <div className={styles.schemaTeaserIcon}>
            <IconSchema size={18} />
          </div>
          <div className={styles.schemaTeaserContent}>
            <Group gap={6} align="center">
              <Text size="sm" fw={500}>
                FHIR Schema
              </Text>
              <Badge
                size="xs"
                variant="gradient"
                gradient={{ from: 'grape', to: 'pink', deg: 135 }}
                leftSection={<IconSparkles size={10} />}
              >
                Soon
              </Badge>
            </Group>
            <Text size="xs" c="dimmed" className={styles.schemaTeaserMeta}>
              <IconFlask2 size={12} style={{ verticalAlign: 'middle', marginRight: 4 }} />
              Experimental validation engine
              <IconRocket size={12} style={{ verticalAlign: 'middle', marginLeft: 6 }} />
            </Text>
          </div>
        </Group>
      </div>
    </Tooltip>
  );
}
