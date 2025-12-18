import type { FlattenedTreeNode } from '@entities/file-tree';
import { Badge } from '@mantine/core';
import { cn } from '@shared/lib';
import type { ProjectTreeNode } from '@shared/types';
import {
  IconBinaryTree,
  IconBraces,
  IconChevronRight,
  IconCode,
  IconFileText,
  IconFolder,
  IconFolderOpen,
  IconSparkles,
} from '@tabler/icons-react';
import type { CSSProperties } from 'react';
import styles from './ProjectTreeRow.module.css';

export interface ProjectTreeRowData {
  rows: FlattenedTreeNode[];
  expanded: Set<string>;
  selectedPath: string | null;
  onSelect: (node: ProjectTreeNode) => void;
  onToggle: (path: string) => void;
}

interface RenderProps extends ProjectTreeRowData {
  index: number;
  style: CSSProperties;
  ariaAttributes: {
    'aria-posinset': number;
    'aria-setsize': number;
    role: 'listitem';
  };
}

export function ProjectTreeRow({
  index,
  style,
  ariaAttributes,
  rows,
  expanded,
  selectedPath,
  onSelect,
  onToggle,
}: RenderProps) {
  const row = rows[index];

  if (!row) return null;

  const { node, depth, isMatch } = row;
  const isFolder = node.kind === 'folder';
  const isExpanded = expanded.has(node.path);
  const isSelected = selectedPath === node.path;

  const icon = getIcon(node, isExpanded);

  return (
    <div
      style={style}
      role="treeitem"
      aria-selected={isSelected}
      aria-level={depth + 1}
      aria-expanded={isFolder ? isExpanded : undefined}
      className={cn(styles.row, {
        [styles.selected]: isSelected,
      })}
      onClick={() => onSelect(node)}
    >
      <div className={styles.indent} style={{ width: depth * 12 }} />

      {isFolder ? (
        <button
          type="button"
          aria-label={isExpanded ? 'Collapse folder' : 'Expand folder'}
          className={cn(styles.chevron, { [styles.expanded]: isExpanded })}
          onClick={(event) => {
            event.stopPropagation();
            onToggle(node.path);
          }}
        >
          <IconChevronRight size={14} stroke={1.7} />
        </button>
      ) : (
        <div className={styles.chevronSpacer} />
      )}

      <div className={styles.icon}>{icon}</div>

      <div className={styles.content}>
        <div className={styles.titleLine}>
          <span className={cn(styles.name, isMatch && styles.match)}>{node.name}</span>
          <Badge
            size="xs"
            variant="light"
            color={badgeColor(node)}
            className={styles.typeBadge}
          >
            {node.kind === 'folder' ? node.root : resourceKindLabel(node)}
          </Badge>
        </div>
        <div className={styles.meta}>
          <span className={styles.path}>{node.path}</span>
          {node.resourceId && <span className={styles.id}>#{node.resourceId}</span>}
        </div>
      </div>
    </div>
  );
}

function badgeColor(node: ProjectTreeNode) {
  if (node.kind === 'folder') {
    return node.root === 'SD' ? 'blue' : node.root === 'IR' ? 'teal' : 'grape';
  }

  switch (node.resourceKind) {
    case 'profile':
      return 'blue';
    case 'valueset':
      return 'violet';
    case 'codesystem':
      return 'grape';
    case 'extension':
      return 'cyan';
    case 'instance':
      return 'teal';
    default:
      return 'gray';
  }
}

function resourceKindLabel(node: ProjectTreeNode) {
  if (node.kind === 'folder') return node.root;
  if (node.resourceKind) {
    return formatResourceKind(node.resourceKind);
  }
  if (node.resourceType) return node.resourceType;
  return 'File';
}

function formatResourceKind(kind: ProjectTreeNode['resourceKind']) {
  switch (kind) {
    case 'valueset':
      return 'ValueSet';
    case 'codesystem':
      return 'CodeSystem';
    case 'operation':
      return 'Operation';
    case 'mapping':
      return 'Mapping';
    case 'instance':
      return 'Instance';
    case 'example':
      return 'Example';
    case 'extension':
      return 'Extension';
    case 'profile':
      return 'Profile';
    default:
      return `${(kind || 'File').charAt(0).toUpperCase()}${(kind || 'File').slice(1)}`;
  }
}

function getIcon(node: ProjectTreeNode, isExpanded: boolean) {
  if (node.kind === 'folder') {
    return isExpanded ? <IconFolderOpen size={16} /> : <IconFolder size={16} />;
  }

  switch (node.resourceKind) {
    case 'profile':
      return <IconBinaryTree size={16} />;
    case 'valueset':
    case 'codesystem':
      return <IconBraces size={16} />;
    case 'extension':
      return <IconSparkles size={16} />;
    case 'instance':
      return <IconCode size={16} />;
    default:
      return <IconFileText size={16} />;
  }
}
