import type { ProjectResourceKind, ProjectTreeRoot } from './file-tree';

export interface ProjectResourceMetadata {
  projectId: string;
  resourceId: string;
  resourceType?: string;
  resourceKind: ProjectResourceKind;
  path?: string;
  name?: string;
  canonicalUrl?: string;
  root?: ProjectTreeRoot;
}
