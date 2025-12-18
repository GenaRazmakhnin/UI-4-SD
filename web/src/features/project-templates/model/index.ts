import { api } from '@shared/api';
import { combine, createEffect, createEvent, createStore, sample } from 'effector';
import { persist } from 'effector-storage/local';
import { getTemplate } from '../lib/templates';
import {
  DEFAULT_PROJECT_CONFIG,
  type PackageDependency,
  type ProjectConfig,
  type ProjectConfigErrors,
  type ProjectTemplate,
  type RecentProject,
  type WizardStep,
} from '../lib/types';
import { hasErrors, suggestPackageId, validateProjectConfig } from '../lib/validation';

// ============================================================================
// Stores
// ============================================================================

/**
 * New project dialog open state
 */
export const $dialogOpen = createStore(false);

/**
 * Current wizard step
 */
export const $wizardStep = createStore<WizardStep>('template');

/**
 * Selected template
 */
export const $selectedTemplate = createStore<ProjectTemplate | null>(null);

/**
 * Project configuration form data
 */
export const $projectConfig = createStore<ProjectConfig>(DEFAULT_PROJECT_CONFIG);

/**
 * Validation errors
 */
export const $configErrors = createStore<ProjectConfigErrors>({});

/**
 * Recent projects
 */
export const $recentProjects = createStore<RecentProject[]>([]);

/**
 * Project creation loading state
 */
export const $isCreating = createStore(false);

// ============================================================================
// Derived Stores
// ============================================================================

/**
 * Check if config is valid for submission
 */
export const $isConfigValid = $configErrors.map((errors) => !hasErrors(errors));

/**
 * Check if can proceed to next step
 */
export const $canProceed = combine(
  $wizardStep,
  $selectedTemplate,
  $isConfigValid,
  (step, template, isValid) => {
    switch (step) {
      case 'template':
        return template !== null;
      case 'configure':
        return isValid;
      case 'review':
        return isValid;
      default:
        return false;
    }
  }
);

// ============================================================================
// Events
// ============================================================================

/**
 * Open new project dialog
 */
export const dialogOpened = createEvent();

/**
 * Close new project dialog
 */
export const dialogClosed = createEvent();

/**
 * Select a template
 */
export const templateSelected = createEvent<string>();

/**
 * Update project configuration
 */
export const configUpdated = createEvent<Partial<ProjectConfig>>();

/**
 * Add dependency
 */
export const dependencyAdded = createEvent<PackageDependency>();

/**
 * Remove dependency
 */
export const dependencyRemoved = createEvent<string>();

/**
 * Go to next wizard step
 */
export const nextStep = createEvent();

/**
 * Go to previous wizard step
 */
export const prevStep = createEvent();

/**
 * Go to specific wizard step
 */
export const stepChanged = createEvent<WizardStep>();

/**
 * Submit project creation
 */
export const projectSubmitted = createEvent();

/**
 * Add to recent projects
 */
export const recentProjectAdded = createEvent<RecentProject>();

/**
 * Remove from recent projects
 */
export const recentProjectRemoved = createEvent<string>();

/**
 * Clear recent projects
 */
export const recentProjectsCleared = createEvent();

/**
 * Open recent project
 */
export const recentProjectOpened = createEvent<string>();

// ============================================================================
// Effects
// ============================================================================

/**
 * Create project effect
 */
export const createProjectFx = createEffect(async (config: ProjectConfig) => {
  const result = await api.projects.create({
    name: config.name,
    canonicalBase: config.canonicalBase,
    fhirVersion: config.fhirVersion,
    packageId: config.packageId,
    version: config.version,
    description: config.description,
    publisher: config.publisher,
    dependencies: config.dependencies.map((d) => ({
      packageId: d.packageId,
      version: d.version,
    })),
  });
  return result;
});

/**
 * Open project effect
 */
export const openProjectFx = createEffect(async (projectId: string) => {
  const project = await api.projects.get(projectId);
  return project;
});

// ============================================================================
// Store Updates
// ============================================================================

// Dialog state
$dialogOpen.on(dialogOpened, () => true);
$dialogOpen.on(dialogClosed, () => false);
$dialogOpen.on(createProjectFx.done, () => false);

// Reset wizard on dialog close
$wizardStep.on(dialogClosed, () => 'template');
$selectedTemplate.on(dialogClosed, () => null);
$projectConfig.on(dialogClosed, () => DEFAULT_PROJECT_CONFIG);
$configErrors.on(dialogClosed, () => ({}));

// Template selection
$selectedTemplate.on(templateSelected, (_, templateId) => getTemplate(templateId) || null);

// When template is selected, update config with template defaults
sample({
  clock: templateSelected,
  fn: (templateId) => {
    const template = getTemplate(templateId);
    if (!template) return DEFAULT_PROJECT_CONFIG;
    return {
      ...DEFAULT_PROJECT_CONFIG,
      fhirVersion: template.fhirVersion,
      dependencies: [...template.dependencies],
    };
  },
  target: $projectConfig,
});

// Config updates
$projectConfig.on(configUpdated, (config, updates) => ({
  ...config,
  ...updates,
}));

// Auto-suggest package ID when name or canonical changes
sample({
  clock: configUpdated,
  source: $projectConfig,
  filter: (_, updates) => 'name' in updates || 'canonicalBase' in updates,
  fn: (config) => {
    // Only suggest if packageId is empty or auto-generated
    if (config.packageId && !config.packageId.startsWith('org.example')) {
      return config;
    }
    return {
      ...config,
      packageId: suggestPackageId(config.name, config.canonicalBase),
    };
  },
  target: $projectConfig,
});

// Validate config on changes
sample({
  clock: $projectConfig,
  fn: (config) => validateProjectConfig(config),
  target: $configErrors,
});

// Dependency management
$projectConfig.on(dependencyAdded, (config, dep) => ({
  ...config,
  dependencies: [...config.dependencies.filter((d) => d.packageId !== dep.packageId), dep],
}));

$projectConfig.on(dependencyRemoved, (config, packageId) => ({
  ...config,
  dependencies: config.dependencies.filter((d) => d.packageId !== packageId),
}));

// Wizard navigation
$wizardStep.on(nextStep, (step) => {
  switch (step) {
    case 'template':
      return 'configure';
    case 'configure':
      return 'review';
    default:
      return step;
  }
});

$wizardStep.on(prevStep, (step) => {
  switch (step) {
    case 'configure':
      return 'template';
    case 'review':
      return 'configure';
    default:
      return step;
  }
});

$wizardStep.on(stepChanged, (_, step) => step);

// Project creation
$isCreating.on(createProjectFx.pending, (_, pending) => pending);

sample({
  clock: projectSubmitted,
  source: combine($projectConfig, $isConfigValid, (config, isValid) => ({ config, isValid })),
  filter: ({ isValid }) => isValid,
  fn: ({ config }) => config,
  target: createProjectFx,
});

// Add to recent on successful creation
sample({
  clock: createProjectFx.doneData,
  fn: (project) => ({
    id: project.id,
    name: project.name,
    path: project.path || '',
    fhirVersion: project.fhirVersion,
    lastOpened: Date.now(),
    packageId: project.packageId,
  }),
  target: recentProjectAdded,
});

// Recent projects management
$recentProjects.on(recentProjectAdded, (projects, newProject) => {
  const filtered = projects.filter((p) => p.id !== newProject.id);
  return [newProject, ...filtered].slice(0, 10); // Keep max 10 recent
});

$recentProjects.on(recentProjectRemoved, (projects, id) => projects.filter((p) => p.id !== id));

$recentProjects.on(recentProjectsCleared, () => []);

// Update lastOpened when opening a recent project
sample({
  clock: recentProjectOpened,
  source: $recentProjects,
  fn: (projects, id) => {
    const project = projects.find((p) => p.id === id);
    if (project) {
      return { ...project, lastOpened: Date.now() };
    }
    return null;
  },
  filter: (project): project is RecentProject => project !== null,
  target: recentProjectAdded,
});

sample({
  clock: recentProjectOpened,
  target: openProjectFx,
});

// ============================================================================
// Persistence
// ============================================================================

persist({
  store: $recentProjects,
  key: 'recent-projects',
});
