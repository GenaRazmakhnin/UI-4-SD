export {
  getTemplate,
  getTemplatesByCategory,
  PROJECT_TEMPLATES,
  searchTemplates,
} from './templates';
export type {
  PackageDependency,
  ProjectConfig,
  ProjectConfigErrors,
  ProjectTemplate,
  RecentProject,
  TemplateCategory,
  TemplateStructure,
  WizardStep,
} from './types';
export { DEFAULT_PROJECT_CONFIG } from './types';
export {
  hasErrors,
  suggestPackageId,
  validateCanonicalBase,
  validatePackageId,
  validateProjectConfig,
  validateProjectName,
  validateVersion,
} from './validation';
