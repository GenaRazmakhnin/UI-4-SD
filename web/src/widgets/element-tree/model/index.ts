import { api } from '@shared/api';
import type { ElementNode, ElementSource } from '@shared/types';
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
type SliceView = 'base' | string; // 'base' = core fields only, otherwise specific sliceName

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
export const $sliceViews = createStore<Record<string, SliceView>>({});

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
export const sliceViewChanged = createEvent<{ path: string; view: SliceView }>();

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
  const source = normalizeSource(node.source);
  const slicing = node.slicing ?? node.constraints?.slicing;
  const baseChildren = (node.children || []).map(transformElementNode);
  const sliceChildren = node.slices
    ? Object.values(node.slices).map((slice: any) => {
        const element = slice?.element ?? {};
        const transformed = transformElementNode({
          ...element,
          source: element.source ?? slice.source,
        });
        return {
          ...transformed,
          sliceName: slice.name ?? transformed.sliceName,
        };
      })
    : [];

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
    slicing: slicing
      ? {
          discriminator: slicing.discriminator ?? [],
          rules: slicing.rules ?? 'open',
          ordered: slicing.ordered ?? false,
          description: slicing.description,
        }
      : undefined,
    mustSupport: node.constraints?.flags?.mustSupport ?? node.mustSupport,
    isModifier: node.constraints?.flags?.isModifier ?? node.isModifier,
    isSummary: node.constraints?.flags?.isSummary ?? node.isSummary,
    short: node.constraints?.short ?? node.short,
    definition: node.constraints?.definition ?? node.definition,
    comment: node.constraints?.comment ?? node.comment,
    source,
    isModified: source !== 'inherited' || node.isModified === true,
    children: [...baseChildren, ...sliceChildren],
  };
}

function normalizeSource(source?: string): ElementSource {
  if (!source) return 'inherited';
  const lowered = source.toLowerCase();
  if (lowered === 'modified') return 'modified';
  if (lowered === 'added') return 'added';
  if (lowered === 'base' || lowered === 'inherited') return 'inherited';
  return 'inherited';
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

// Derive slice controls for non-extension slicing nodes
export const $sliceControls = $elementTree.map((tree) => {
  const controls: { path: string; slices: string[] }[] = [];

  const visit = (nodes: ElementNode[]) => {
    for (const node of nodes) {
      if (node.slicing && !node.path.endsWith('extension')) {
        const sliceNames = node.children.map((c) => c.sliceName).filter(Boolean) as string[];
        if (sliceNames.length > 0) {
          controls.push({ path: node.path, slices: sliceNames });
        }
      }
      if (node.children.length > 0) visit(node.children);
    }
  };

  visit(tree);
  return controls;
});

export const $flattenedElements = combine(
  $filteredTree,
  $expandedPaths,
  $sliceViews,
  (tree, expanded, sliceViews) => {
    const normalizeSliceChildPath = (path: string): string => {
      const [base, rest] = path.split(':', 2);
      if (!rest) return path;
      const dotIndex = rest.indexOf('.');
      if (dotIndex === -1) return base;
      const suffix = rest.slice(dotIndex + 1);
      return suffix ? `${base}.${suffix}` : base;
    };

    const flatten = (nodes: ElementNode[], depth: number): ElementNode[] => {
      return nodes.flatMap((node) => {
        const isHiddenExtensionContainer =
          node.path.endsWith('.extension') && node.children.length > 0 && !node.sliceName;

        // Skip only non-sliced extension containers; render children at same depth
        if (isHiddenExtensionContainer) {
          return flatten(node.children, depth);
        }

        const isNonExtensionSlicing = !!node.slicing && !node.path.endsWith('extension');
        const currentView = isNonExtensionSlicing ? (sliceViews[node.path] ?? 'base') : 'base';
        const sliceChildren = isNonExtensionSlicing ? node.children.filter((c) => c.sliceName) : [];
        const baseChildren = isNonExtensionSlicing
          ? node.children.filter((c) => !c.sliceName)
          : node.children;

        const matchedSlice =
          currentView !== 'base' ? sliceChildren.find((c) => c.sliceName === currentView) : null;

        const childrenFromSlice =
          isNonExtensionSlicing && currentView !== 'base' ? (matchedSlice?.children ?? []) : [];

        let childrenToRender: ElementNode[] = baseChildren;

        if (isNonExtensionSlicing && currentView !== 'base') {
          const basePaths = new Set(baseChildren.map((child) => child.path));
          const sliceByBasePath = new Map<string, ElementNode>();

          for (const child of childrenFromSlice) {
            const normalized = normalizeSliceChildPath(child.path);
            if (basePaths.has(normalized)) {
              sliceByBasePath.set(normalized, child);
            }
          }

          childrenToRender = baseChildren.map((child) => sliceByBasePath.get(child.path) ?? child);

          const appended = new Set<string>();
          for (const child of childrenFromSlice) {
            const normalized = normalizeSliceChildPath(child.path);
            if (!basePaths.has(normalized) && !appended.has(normalized)) {
              childrenToRender.push(child);
              appended.add(normalized);
            }
          }
        }

        const isSimpleExtension =
          node.path.endsWith('extension') &&
          node.children.length > 0 &&
          node.children.every(
            (c) =>
              (c.path.endsWith('.url') ||
                c.path.endsWith('.value[x]') ||
                c.path.endsWith('.value')) &&
              c.children.length === 0
          );
        if (isSimpleExtension) {
          childrenToRender = [];
        }

        // Keep non-extension children first, push extension nodes to the bottom
        if (childrenToRender.length > 0) {
          childrenToRender = [...childrenToRender].sort((a, b) => {
            const aExt = a.path.endsWith('extension') ? 1 : 0;
            const bExt = b.path.endsWith('extension') ? 1 : 0;
            return aExt - bExt;
          });
        }

        const sliceNames = sliceChildren.map((c) => c.sliceName).filter(Boolean) as string[];

        const extendedNode = {
          ...node,
          children: childrenToRender,
          __depth: depth,
          __sliceNames: sliceNames,
          __displayName:
            node.sliceName && node.path.endsWith('extension')
              ? node.sliceName
              : isNonExtensionSlicing && currentView !== 'base'
                ? `${node.path.split('.').slice(-1)[0]}:${currentView}`
                : undefined,
        } as ElementNode & { __depth?: number; __displayName?: string };

        const isExpanded = expanded.has(node.path);

        return [
          extendedNode,
          ...(isExpanded && childrenToRender.length > 0
            ? flatten(childrenToRender, depth + 1)
            : []),
        ];
      });
    };
    return flatten(tree, 0);
  }
);

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
$expandedPaths.on(sliceViewChanged, (paths, { path }) => {
  const newPaths = new Set(paths);
  newPaths.add(path);
  return newPaths;
});

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
$sliceViews.on(sliceViewChanged, (current, { path, view }) => ({
  ...current,
  [path]: view,
}));

// $elementTree.on(loadElementTreeFx.doneData, (_, tree) => tree);
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
