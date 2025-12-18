import {
  $expandedPaths,
  $flattenedTree,
  $selectedPath,
  nodeSelected,
  pathToggled,
} from '@entities/file-tree';
import type { ProjectTreeNode } from '@shared/types';
import { useUnit } from 'effector-react';
import { useEffect } from 'react';

export function useTreeKeyboard(onActivate?: (node: ProjectTreeNode) => void) {
  const rows = useUnit($flattenedTree);
  const selectedPath = useUnit($selectedPath);
  const expanded = useUnit($expandedPaths);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (!selectedPath || rows.length === 0) return;

      const currentIndex = rows.findIndex((row) => row.node.path === selectedPath);
      if (currentIndex === -1) return;

      const current = rows[currentIndex].node;

      switch (event.key) {
        case 'ArrowDown': {
          event.preventDefault();
          const next = rows[currentIndex + 1];
          if (next) {
            nodeSelected(next.node);
          }
          break;
        }
        case 'ArrowUp': {
          event.preventDefault();
          const prev = rows[currentIndex - 1];
          if (prev) {
            nodeSelected(prev.node);
          }
          break;
        }
        case 'ArrowRight': {
          if (current.kind === 'folder') {
            event.preventDefault();
            const isExpanded = expanded.has(current.path);
            if (!isExpanded) {
              pathToggled(current.path);
            } else {
              const next = rows[currentIndex + 1];
              if (next) {
                nodeSelected(next.node);
              }
            }
          }
          break;
        }
        case 'ArrowLeft': {
          if (current.kind === 'folder') {
            event.preventDefault();
            const isExpanded = expanded.has(current.path);
            if (isExpanded) {
              pathToggled(current.path);
              return;
            }
          }

          const parentPath = current.path.split('/').slice(0, -1).join('/');
          if (parentPath) {
            const parent = rows.find((row) => row.node.path === parentPath);
            if (parent) {
              event.preventDefault();
              nodeSelected(parent.node);
            }
          }
          break;
        }
        case 'Enter': {
          if (onActivate) {
            event.preventDefault();
            onActivate(current);
          }
          break;
        }
        case ' ': {
          if (current.kind === 'folder') {
            event.preventDefault();
            pathToggled(current.path);
          }
          break;
        }
        default:
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [rows, selectedPath, expanded, onActivate]);
}
