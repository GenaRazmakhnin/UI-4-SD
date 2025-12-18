export type ProjectTreeRoot = 'IR' | 'SD' | 'FSH';

export type ProjectResourceKind =
  | 'profile'
  | 'instance'
  | 'valueset'
  | 'codesystem'
  | 'extension'
  | 'example'
  | 'operation'
  | 'mapping'
  | 'other';

export interface ProjectTreeNode {
  /** Project-relative path including root folder */
  path: string;
  /** Display label for the node (usually the last segment of the path) */
  name: string;
  kind: 'folder' | 'file';
  root: ProjectTreeRoot;
  resourceId?: string;
  canonicalUrl?: string;
  resourceType?: string;
  resourceKind?: ProjectResourceKind;
  children: ProjectTreeNode[];
}

export type ArtifactKind = 'profile' | 'extension' | 'valueset';

export interface CreateArtifactInput {
  kind: ArtifactKind;
  name: string;
  id?: string;
  baseResource?: string; // profiles only
  description?: string;
  context?: string; // extensions optional
  purpose?: string; // valueset optional
}

export interface CreatedArtifact {
  path: string;
  resourceId: string;
  resourceType: string;
  resourceKind: ProjectResourceKind;
  canonicalUrl?: string;
}
