import { api } from '@shared/api';
import type { ElementNode } from '@shared/types';
import { createEffect, createEvent, createStore, sample } from 'effector';
import { persist } from 'effector-storage/local';

// Filter options type
export interface FilterOptions {
  modifiedOnly: boolean;
  errorsOnly: boolean;
  mustSupportOnly: boolean;
  searchQuery: string;
}

// Stores
export const $elementTree = createStore<ElementNode[]>([]);
export const $selectedElementId = createStore<string | null>(null);
export const $expandedPaths = createStore<Set<string>>(new Set());
export const $filterOptions = createStore<FilterOptions>({
  modifiedOnly: false,
  errorsOnly: false,
  mustSupportOnly: false,
  searchQuery: '',
});

// Events
export const elementSelected = createEvent<ElementNode>();
export const selectElement = createEvent<string>(); // Select by path
export const pathToggled = createEvent<string>();
export const filterChanged = createEvent<Partial<FilterOptions>>();
export const expandAll = createEvent();
export const collapseAll = createEvent();
export const searchQueryChanged = createEvent<string>();
export const treeLoaded = createEvent<ElementNode[]>();

// Effects
export const loadElementTreeFx = createEffect<string, ElementNode[]>(async (profileId) => {
  const response = await api.profiles.get(profileId);
  return response.elements;
});

// Derived stores
export const $selectedElement = sample({
  clock: $selectedElementId,
  source: $elementTree,
  fn: (tree, selectedId) => {
    if (!selectedId) return null;

    const findElement = (nodes: ElementNode[]): ElementNode | null => {
      for (const node of nodes) {
        if (node.id === selectedId) return node;
        const found = findElement(node.children);
        if (found) return found;
      }
      return null;
    };
    return findElement(tree);
  },
});

export const $filteredTree = sample({
  clock: [$elementTree, $filterOptions],
  source: { tree: $elementTree, filters: $filterOptions },
  fn: ({ tree, filters }) => {
    const filterNode = (node: ElementNode): boolean => {
      if (filters.modifiedOnly && !node.isModified) return false;
      if (filters.mustSupportOnly && !node.mustSupport) return false;
      if (
        filters.searchQuery &&
        !node.path.toLowerCase().includes(filters.searchQuery.toLowerCase())
      ) {
        return false;
      }
      return true;
    };

    const filterTree = (nodes: ElementNode[]): ElementNode[] => {
      return nodes.filter(filterNode).map((node) => ({
        ...node,
        children: filterTree(node.children),
      }));
    };

    return filterTree(tree);
  },
});

export const $flattenedElements = sample({
  clock: [$filteredTree, $expandedPaths],
  source: { tree: $filteredTree, expanded: $expandedPaths },
  fn: ({ tree, expanded }) => {
    const flatten = (nodes: ElementNode[]): ElementNode[] => {
      return nodes.flatMap((node) => {
        const isExpanded = expanded.has(node.path);
        return [node, ...(isExpanded && node.children.length > 0 ? flatten(node.children) : [])];
      });
    };
    return flatten(tree);
  },
});

// Store updates
$selectedElementId.on(elementSelected, (_, element) => element.id);

$expandedPaths.on(pathToggled, (paths, path) => {
  const newPaths = new Set(paths);
  if (newPaths.has(path)) {
    newPaths.delete(path);
  } else {
    newPaths.add(path);
  }
  return newPaths;
});

$expandedPaths.on(expandAll, () => {
  const allPaths = new Set<string>();
  const collectPaths = (nodes: ElementNode[]) => {
    for (const node of nodes) {
      if (node.children.length > 0) {
        allPaths.add(node.path);
        collectPaths(node.children);
      }
    }
  };
  collectPaths($elementTree.getState());
  return allPaths;
});

$expandedPaths.on(collapseAll, () => new Set());

$filterOptions.on(filterChanged, (current, updates) => ({
  ...current,
  ...updates,
}));

$filterOptions.on(searchQueryChanged, (current, query) => ({
  ...current,
  searchQuery: query,
}));

$elementTree.on(loadElementTreeFx.doneData, (_, tree) => tree);
$elementTree.on(treeLoaded, (_, tree) => tree);

// Persist expanded paths to localStorage
persist({
  store: $expandedPaths,
  key: 'element-tree-expanded-paths',
  serialize: (set) => JSON.stringify([...set]),
  deserialize: (str) => new Set(JSON.parse(str)),
});

/**
 * Handle selectElement by path - finds the element and triggers elementSelected
 */
sample({
  clock: selectElement,
  source: $elementTree,
  fn: (tree, path) => {
    const findByPath = (nodes: ElementNode[]): ElementNode | null => {
      for (const node of nodes) {
        if (node.path === path) return node;
        const found = findByPath(node.children);
        if (found) return found;
      }
      return null;
    };
    return findByPath(tree);
  },
  filter: (element): element is ElementNode => element !== null,
  target: elementSelected,
});

/**
 * When selecting by path, also expand the parent paths
 */
sample({
  clock: selectElement,
  source: $expandedPaths,
  fn: (currentPaths, path) => {
    const newPaths = new Set(currentPaths);
    // Expand all parent paths
    const parts = path.split('.');
    for (let i = 1; i < parts.length; i++) {
      newPaths.add(parts.slice(0, i).join('.'));
    }
    return newPaths;
  },
  target: $expandedPaths,
});
