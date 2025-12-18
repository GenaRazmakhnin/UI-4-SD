import { api } from '@shared/api';
import type { CreatedArtifact, ProjectTreeNode, ProjectTreeRoot } from '@shared/types';
import { combine, createEffect, createEvent, createStore, sample } from 'effector';

const stubTree: ProjectTreeNode[] = [
  {
    path: 'IR',
    name: 'IR',
    root: 'IR',
    kind: 'folder',
    children: [
      {
        path: 'IR/input',
        name: 'input',
        root: 'IR',
        kind: 'folder',
        children: [
          {
            path: 'IR/input/profiles',
            name: 'profiles',
            root: 'IR',
            kind: 'folder',
            children: [
              {
                path: 'IR/input/profiles/example-profile.json',
                name: 'example-profile.json',
                root: 'IR',
                kind: 'file',
                resourceId: 'example-profile',
                resourceType: 'StructureDefinition',
                resourceKind: 'profile',
                canonicalUrl: 'http://example.org/StructureDefinition/example-profile',
                children: [],
              },
            ],
          },
        ],
      },
    ],
  },
  {
    path: 'SD',
    name: 'SD',
    root: 'SD',
    kind: 'folder',
    children: [
      {
        path: 'SD/extensions',
        name: 'extensions',
        root: 'SD',
        kind: 'folder',
        children: [
          {
            path: 'SD/extensions/example-extension.json',
            name: 'example-extension.json',
            root: 'SD',
            kind: 'file',
            resourceId: 'example-extension',
            resourceType: 'StructureDefinition',
            resourceKind: 'extension',
            canonicalUrl: 'http://example.org/StructureDefinition/example-extension',
            children: [],
          },
        ],
      },
      {
        path: 'SD/value-sets',
        name: 'value-sets',
        root: 'SD',
        kind: 'folder',
        children: [],
      },
    ],
  },
  {
    path: 'FSH',
    name: 'FSH',
    root: 'FSH',
    kind: 'folder',
    children: [],
  },
];

export interface TreeLoadedPayload {
  projectId: string;
  nodes: ProjectTreeNode[];
}

export interface FlattenedTreeNode {
  node: ProjectTreeNode;
  depth: number;
  isMatch: boolean;
}

// Events
export const treeLoaded = createEvent<TreeLoadedPayload>();
export const pathToggled = createEvent<string>();
export const nodeSelected = createEvent<ProjectTreeNode>();
export const selectPath = createEvent<string>();
export const searchChanged = createEvent<string>();
export const expandAll = createEvent();
export const collapseAll = createEvent();
export const artifactAdded = createEvent<CreatedArtifact>();

// Effects
export const loadProjectTreeFx = createEffect(async (projectId: string) => ({
  projectId,
  nodes: await api.projects.tree(projectId),
}));

// Stores
export const $projectTree = createStore<ProjectTreeNode[]>(stubTree);
export const $currentProjectId = createStore<string | null>(null);
export const $expandedPaths = createStore<Set<string>>(collectRootPaths(stubTree));
export const $searchQuery = createStore<string>('');
export const $selectedPath = createStore<string | null>(null);

// Derived stores
const $filteredWithMatches = combine($projectTree, $searchQuery, (tree, query) => {
  const matches = new Set<string>();
  const trimmed = query.trim().toLowerCase();

  if (!trimmed) {
    return { nodes: tree, matches };
  }

  return {
    nodes: filterTree(tree, trimmed, matches),
    matches,
  };
});

export const $filteredTree = $filteredWithMatches.map(({ nodes }) => nodes);
export const $searchMatches = $filteredWithMatches.map(({ matches }) => matches);

export const $flattenedTree = combine(
  $filteredTree,
  $expandedPaths,
  $searchMatches,
  (tree, expanded, matches) => flattenTree(tree, expanded, matches)
);

export const $selectedNode = combine($projectTree, $selectedPath, (tree, selectedPath) => {
  if (!selectedPath) return null;
  return findNodeByPath(tree, selectedPath);
});

// Store updates
$projectTree.on(treeLoaded, (current, payload) =>
  payload.nodes.length > 0 ? payload.nodes : current
);
$currentProjectId.on(treeLoaded, (_, payload) => payload.projectId);
$expandedPaths.on(treeLoaded, (current, payload) =>
  payload.nodes.length > 0 ? collectRootPaths(payload.nodes) : current
);
$selectedPath.on(nodeSelected, (_, node) => node.path);
$selectedPath.reset(treeLoaded);
$searchQuery.on(searchChanged, (_, query) => query);
$searchQuery.reset(treeLoaded);

$projectTree.on(artifactAdded, (tree, artifact) => insertArtifact(tree, artifact));

$expandedPaths.on(artifactAdded, (paths, artifact) => {
  const next = new Set(paths);
  const parents = getParentPaths(artifact.path);
  parents.forEach((p) => next.add(p));
  return next;
});

$expandedPaths.on(pathToggled, (paths, path) => {
  const next = new Set(paths);
  if (next.has(path)) {
    next.delete(path);
  } else {
    next.add(path);
  }
  return next;
});

// When loading finishes, route the payload through treeLoaded
sample({
  clock: loadProjectTreeFx.doneData,
  target: treeLoaded,
});

// Expand/collapse helpers
sample({
  clock: expandAll,
  source: $projectTree,
  fn: (tree) => collectFolderPaths(tree),
  target: $expandedPaths,
});

sample({
  clock: collapseAll,
  source: $projectTree,
  fn: (tree) => collectRootPaths(tree),
  target: $expandedPaths,
});

sample({
  clock: searchChanged,
  source: $projectTree,
  fn: (tree, query) => {
    if (!query.trim()) {
      return collectRootPaths(tree);
    }
    return collectFolderPaths(tree);
  },
  target: $expandedPaths,
});

// Selecting by path expands parent folders and updates selection
sample({
  clock: selectPath,
  source: $projectTree,
  fn: (tree, path) => findNodeByPath(tree, path),
  filter: (node): node is ProjectTreeNode => Boolean(node),
  target: nodeSelected,
});

sample({
  clock: selectPath,
  source: $expandedPaths,
  fn: (expanded, path) => expandToPath(expanded, path),
  target: $expandedPaths,
});

// Helpers
function collectRootPaths(nodes: ProjectTreeNode[]): Set<string> {
  return new Set(nodes.filter((node) => node.kind === 'folder').map((node) => node.path));
}

function collectFolderPaths(nodes: ProjectTreeNode[]): Set<string> {
  const paths = new Set<string>();

  const walk = (items: ProjectTreeNode[]) => {
    for (const item of items) {
      if (item.kind === 'folder') {
        paths.add(item.path);
        walk(item.children);
      }
    }
  };

  walk(nodes);
  return paths;
}

function expandToPath(expanded: Set<string>, path: string): Set<string> {
  const next = new Set(expanded);
  const parents = getParentPaths(path);
  parents.forEach((parent) => next.add(parent));
  return next;
}

function getParentPaths(path: string): string[] {
  const parts = path.split('/').filter(Boolean);
  const parents: string[] = [];

  for (let i = 1; i < parts.length; i += 1) {
    parents.push(parts.slice(0, i).join('/'));
  }

  return parents;
}

function findNodeByPath(nodes: ProjectTreeNode[], path: string): ProjectTreeNode | null {
  for (const node of nodes) {
    if (node.path === path) {
      return node;
    }
    const child = findNodeByPath(node.children, path);
    if (child) {
      return child;
    }
  }
  return null;
}

function filterTree(
  nodes: ProjectTreeNode[],
  query: string,
  matches: Set<string>
): ProjectTreeNode[] {
  return nodes
    .map((node) => {
      const filteredChildren = filterTree(node.children, query, matches);
      const name = node.name.toLowerCase();
      const path = node.path.toLowerCase();
      const resourceId = node.resourceId?.toLowerCase() ?? '';
      const resourceType = node.resourceType?.toLowerCase() ?? '';
      const canonical = node.canonicalUrl?.toLowerCase() ?? '';

      const isMatch =
        name.includes(query) ||
        path.includes(query) ||
        resourceId.includes(query) ||
        resourceType.includes(query) ||
        canonical.includes(query);

      if (isMatch) {
        matches.add(node.path);
      }

      if (isMatch || filteredChildren.length > 0) {
        return {
          ...node,
          children: filteredChildren,
        };
      }

      return null;
    })
    .filter(Boolean) as ProjectTreeNode[];
}

function flattenTree(
  nodes: ProjectTreeNode[],
  expanded: Set<string>,
  matches: Set<string>,
  depth = 0
): FlattenedTreeNode[] {
  return nodes.flatMap((node) => {
    const item: FlattenedTreeNode = {
      node,
      depth,
      isMatch: matches.has(node.path),
    };

    if (node.children.length > 0 && expanded.has(node.path)) {
      return [item, ...flattenTree(node.children, expanded, matches, depth + 1)];
    }

    return [item];
  });
}

function cloneTree(nodes: ProjectTreeNode[]): ProjectTreeNode[] {
  return nodes.map((node) => ({
    ...node,
    children: cloneTree(node.children),
  }));
}

function ensureFolder(
  nodes: ProjectTreeNode[],
  root: ProjectTreeRoot,
  path: string,
  displayName: string
): ProjectTreeNode {
  const parts = path.split('/').filter(Boolean);
  if (parts.length === 0) {
    throw new Error('Invalid folder path');
  }

  let current = nodes.find((n) => n.path === parts[0]);
  if (!current) {
    current = {
      path: parts[0],
      name: parts[0],
      root,
      kind: 'folder',
      children: [],
    };
    nodes.push(current);
  }

  for (let i = 1; i < parts.length; i += 1) {
    const segmentPath = parts.slice(0, i + 1).join('/');
    let child = current.children.find((n) => n.path === segmentPath);
    if (!child) {
      child = {
        path: segmentPath,
        name: i === parts.length - 1 ? displayName : parts[i],
        root,
        kind: 'folder',
        children: [],
      };
      current.children.unshift(child);
    }
    current = child;
  }

  return current;
}

function insertArtifact(tree: ProjectTreeNode[], artifact: CreatedArtifact): ProjectTreeNode[] {
  if (!artifact.path) return tree;

  const rootSeg = artifact.path.split('/')[0] as ProjectTreeRoot;
  const root: ProjectTreeRoot = rootSeg === 'IR' || rootSeg === 'SD' || rootSeg === 'FSH' ? rootSeg : 'SD';
  const folderPath = artifact.path.split('/').slice(0, -1).join('/');
  const fileName = artifact.path.split('/').pop() || artifact.resourceId;

  const cloned = cloneTree(tree);
  const folder = ensureFolder(cloned, root, folderPath, folderPath.split('/').pop() || 'files');

  const node: ProjectTreeNode = {
    path: artifact.path,
    name: fileName,
    root,
    kind: 'file',
    resourceId: artifact.resourceId,
    resourceType: artifact.resourceType,
    resourceKind: artifact.resourceKind,
    canonicalUrl: artifact.canonicalUrl,
    children: [],
  };

  folder.children.unshift(node);
  return cloned;
}
