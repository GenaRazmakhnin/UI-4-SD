import { api } from '@shared/api';
import type { ElementNode } from '@shared/types';
import { combine, createEffect, createEvent, createStore, sample } from 'effector';
import { persist } from 'effector-storage/local';

// Filter options type
export interface FilterOptions {
  modifiedOnly: boolean;
  errorsOnly: boolean;
  mustSupportOnly: boolean;
  searchQuery: string;
}

// Profile loading params
export interface LoadProfileParams {
  projectId: string;
  profileId: string;
}

// Current profile context
export interface ProfileContext {
  projectId: string;
  profileId: string;
  profileName: string;
  isDirty: boolean;
  canUndo: boolean;
  canRedo: boolean;
}

// Stores
export const $elementTree = createStore<ElementNode[]>([]);
export const $profileContext = createStore<ProfileContext | null>(null);
export const $selectedElementId = createStore<string | null>(null);
export const $expandedPaths = createStore<Set<string>>(new Set());
export const $filterOptions = createStore<FilterOptions>({
  modifiedOnly: false,
  errorsOnly: false,
  mustSupportOnly: false,
  searchQuery: '',
});
export const $isLoading = createStore(false);
export const $loadError = createStore<string | null>(null);

// Events
export const elementSelected = createEvent<ElementNode>();
export const selectElement = createEvent<string>(); // Select by path
export const pathToggled = createEvent<string>();
export const filterChanged = createEvent<Partial<FilterOptions>>();
export const expandAll = createEvent();
export const collapseAll = createEvent();
export const searchQueryChanged = createEvent<string>();
export const treeLoaded = createEvent<ElementNode[]>();
export const profileContextUpdated = createEvent<Partial<ProfileContext>>();
export const clearProfile = createEvent();

// Effects
export const loadProfileFx = createEffect<
  LoadProfileParams,
  { elements: ElementNode[]; context: ProfileContext }
>(async ({ projectId, profileId }) => {
  // Use the project-scoped profile API
  const response = await api.projects.getProfile(projectId, profileId);

  // Transform the response - backend returns root element with children
  const elements = response.resource?.root ? [transformElementNode(response.resource.root)] : [];

  const context: ProfileContext = {
    projectId,
    profileId,
    profileName: response.metadata?.name || profileId,
    isDirty: response.isDirty ?? false,
    canUndo: response.history?.canUndo ?? false,
    canRedo: response.history?.canRedo ?? false,
  };

  return { elements, context };
});

// Transform backend ElementNode to frontend format
function transformElementNode(node: any): ElementNode {
  return {
    id: node.id || node.path,
    path: node.path,
    sliceName: node.sliceName,
    min: node.constraints?.cardinality?.min ?? node.min ?? 0,
    max: node.constraints?.cardinality?.max?.toString() ?? node.max ?? '*',
    type: node.constraints?.types?.map((t: any) => ({
      code: t.code,
      profile: t.profile,
      targetProfile: t.targetProfile,
    })),
    binding: node.constraints?.binding
      ? {
          strength: node.constraints.binding.strength,
          valueSet: node.constraints.binding.valueSet,
          description: node.constraints.binding.description,
        }
      : undefined,
    mustSupport: node.constraints?.flags?.mustSupport ?? node.mustSupport,
    isModifier: node.constraints?.flags?.isModifier ?? node.isModifier,
    isSummary: node.constraints?.flags?.isSummary ?? node.isSummary,
    short: node.constraints?.short ?? node.short,
    definition: node.constraints?.definition ?? node.definition,
    comment: node.constraints?.comment ?? node.comment,
    isModified: node.source === 'Modified' || node.isModified === true,
    children: (node.children || []).map(transformElementNode),
  };
}

// Derived stores
export const $selectedElement = combine(
  $elementTree,
  $selectedElementId,
  (tree, selectedId): ElementNode | null => {
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
  }
);

export const $filteredTree = combine($elementTree, $filterOptions, (tree, filters) => {
  // Check if any filter is active
  const hasActiveFilters = filters.modifiedOnly || filters.mustSupportOnly || filters.searchQuery;

  if (!hasActiveFilters) {
    return tree;
  }

  // Check if a node matches the filter criteria
  const matchesFilter = (node: ElementNode): boolean => {
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

  // Check if a node or any of its descendants match the filter
  const hasMatchingDescendant = (node: ElementNode): boolean => {
    if (matchesFilter(node)) return true;
    return node.children.some(hasMatchingDescendant);
  };

  // Filter tree, keeping parent nodes if any descendants match
  const filterTree = (nodes: ElementNode[]): ElementNode[] => {
    return nodes.filter(hasMatchingDescendant).map((node) => ({
      ...node,
      children: filterTree(node.children),
    }));
  };

  return filterTree(tree);
});

export const $flattenedElements = combine($filteredTree, $expandedPaths, (tree, expanded) => {
  const flatten = (nodes: ElementNode[]): ElementNode[] => {
    return nodes.flatMap((node) => {
      const isExpanded = expanded.has(node.path);
      return [node, ...(isExpanded && node.children.length > 0 ? flatten(node.children) : [])];
    });
  };
  return flatten(tree);
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

// Handle profile loading
$isLoading.on(loadProfileFx.pending, (_, pending) => pending);
$loadError.on(loadProfileFx.failData, (_, error) => error.message);
$loadError.reset(loadProfileFx);

$elementTree.on(loadProfileFx.doneData, (_, { elements }) => elements);
$elementTree.on(treeLoaded, (_, tree) => tree);
$elementTree.reset(clearProfile);

$profileContext.on(loadProfileFx.doneData, (_, { context }) => context);
$profileContext.on(profileContextUpdated, (current, updates) =>
  current ? { ...current, ...updates } : null
);
$profileContext.reset(clearProfile);

// Expand root path when profile loads
sample({
  clock: loadProfileFx.doneData,
  fn: ({ elements }) => {
    const paths = new Set<string>();
    // Expand root element by default
    if (elements.length > 0) {
      paths.add(elements[0].path);
    }
    return paths;
  },
  target: $expandedPaths,
});

// Clear selection when profile changes
$selectedElementId.reset(clearProfile);
$selectedElementId.reset(loadProfileFx);

// Persist expanded paths to localStorage
persist({
  store: $expandedPaths,
  key: 'element-tree-expanded-paths',
  serialize: (set) => JSON.stringify([...set]),
  deserialize: (str) => {
    try {
      const parsed = JSON.parse(str);
      return new Set(Array.isArray(parsed) ? parsed : []);
    } catch {
      return new Set();
    }
  },
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
