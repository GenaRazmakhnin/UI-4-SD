// UI Components

// Types
export type {
  PackageDependency,
  ProjectConfig,
  ProjectConfigErrors,
  ProjectTemplate,
  RecentProject,
  TemplateCategory,
  TemplateStructure,
  WizardStep,
} from './lib';
// Lib exports
export {
  DEFAULT_PROJECT_CONFIG,
  getTemplate,
  getTemplatesByCategory,
  hasErrors,
  PROJECT_TEMPLATES,
  searchTemplates,
  suggestPackageId,
  validateCanonicalBase,
  validatePackageId,
  validateProjectConfig,
  validateProjectName,
  validateVersion,
} from './lib';
// Model (stores, events, effects)
export {
  $canProceed,
  $configErrors,
  // Stores
  $dialogOpen,
  $isConfigValid,
  $isCreating,
  $projectConfig,
  $recentProjects,
  $selectedTemplate,
  $wizardStep,
  configUpdated,
  // Effects
  createProjectFx,
  dependencyAdded,
  dependencyRemoved,
  dialogClosed,
  // Events
  dialogOpened,
  nextStep,
  openProjectFx,
  prevStep,
  projectSubmitted,
  recentProjectAdded,
  recentProjectOpened,
  recentProjectRemoved,
  recentProjectsCleared,
  stepChanged,
  templateSelected,
} from './model';
export { NewProjectDialog } from './ui/NewProjectDialog';
export { ProjectConfigForm } from './ui/ProjectConfigForm';
export { RecentProjects } from './ui/RecentProjects';
export { TemplateGallery } from './ui/TemplateGallery';
