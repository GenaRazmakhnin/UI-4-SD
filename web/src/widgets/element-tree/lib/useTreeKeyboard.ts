import { useEffect } from 'react';
import { useUnit } from 'effector-react';
import {
  $flattenedElements,
  $selectedElementId,
  $expandedPaths,
  elementSelected,
  pathToggled,
} from '../model';

export function useTreeKeyboard() {
  const elements = useUnit($flattenedElements);
  const selectedId = useUnit($selectedElementId);
  const expandedPaths = useUnit($expandedPaths);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!selectedId) return;

      const currentIndex = elements.findIndex((el) => el.id === selectedId);
      if (currentIndex === -1) return;

      const currentElement = elements[currentIndex];

      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          if (currentIndex < elements.length - 1) {
            elementSelected(elements[currentIndex + 1]);
          }
          break;

        case 'ArrowUp':
          e.preventDefault();
          if (currentIndex > 0) {
            elementSelected(elements[currentIndex - 1]);
          }
          break;

        case 'ArrowRight':
          e.preventDefault();
          if (currentElement.children.length > 0) {
            if (!expandedPaths.has(currentElement.path)) {
              pathToggled(currentElement.path);
            } else if (currentIndex < elements.length - 1) {
              // Move to first child
              elementSelected(elements[currentIndex + 1]);
            }
          }
          break;

        case 'ArrowLeft':
          e.preventDefault();
          if (expandedPaths.has(currentElement.path)) {
            pathToggled(currentElement.path);
          } else {
            // Move to parent
            const parentPath = currentElement.path
              .split('.')
              .slice(0, -1)
              .join('.');
            const parent = elements.find((el) => el.path === parentPath);
            if (parent) elementSelected(parent);
          }
          break;

        case ' ':
          e.preventDefault();
          if (currentElement.children.length > 0) {
            pathToggled(currentElement.path);
          }
          break;

        default:
          break;
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [elements, selectedId, expandedPaths]);
}
