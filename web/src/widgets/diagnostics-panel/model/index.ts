import { api } from '@shared/api';
import type { Diagnostic, QuickFix, ValidationSeverity } from '@shared/types';
import { selectElement } from '@widgets/element-tree';
import { createEffect, createEvent, createStore, sample } from 'effector';

/**
 * Validation params
 */
export interface ValidateParams {
  projectId: string;
  profileId: string;
}

/**
 * Diagnostic filters
 */
export interface DiagnosticFilters {
  severity?: ValidationSeverity[];
  elementPath?: string;
  searchQuery?: string;
  showFixed?: boolean;
}

/**
 * Events
 */
export const diagnosticsReceived = createEvent<Diagnostic[]>();
export const diagnosticClicked = createEvent<Diagnostic>();
export const diagnosticDismissed = createEvent<string>(); // diagnostic id
export const quickFixApplied = createEvent<{
  diagnosticId: string;
  fix: QuickFix;
  projectId: string;
  profileId: string;
}>();
export const filtersChanged = createEvent<Partial<DiagnosticFilters>>();
export const filterCleared = createEvent<void>();
export const diagnosticsCleared = createEvent<void>();
export const applyAllFixes = createEvent<void>();

/**
 * Validate profile effect
 */
export const validateProfileFx = createEffect(async ({ projectId, profileId }: ValidateParams) => {
  const result = await api.validation.validate(projectId, profileId);

  // Convert ValidationResult to Diagnostic[]
  const diagnostics: Diagnostic[] = [];

  result.errors.forEach((msg, index) => {
    diagnostics.push({
      id: `error-${index}-${Date.now()}`,
      severity: 'error',
      code: 'FHIR-ERR',
      message: msg.message,
      elementPath: msg.path,
      timestamp: new Date().toISOString(),
      isNew: true,
    });
  });

  result.warnings.forEach((msg, index) => {
    diagnostics.push({
      id: `warning-${index}-${Date.now()}`,
      severity: 'warning',
      code: 'FHIR-WARN',
      message: msg.message,
      elementPath: msg.path,
      timestamp: new Date().toISOString(),
      isNew: true,
    });
  });

  result.info.forEach((msg, index) => {
    diagnostics.push({
      id: `info-${index}-${Date.now()}`,
      severity: 'info',
      code: 'FHIR-INFO',
      message: msg.message,
      elementPath: msg.path,
      timestamp: new Date().toISOString(),
      isNew: true,
    });
  });

  return diagnostics;
});

/**
 * Apply quick fix effect
 */
export const applyQuickFixFx = createEffect(
  async ({
    diagnosticId,
    fix,
    projectId,
    profileId,
  }: {
    diagnosticId: string;
    fix: QuickFix;
    projectId: string;
    profileId: string;
  }) => {
    // Apply fix via API
    if (fix.id) {
      await api.validation.applyFix(projectId, profileId, fix.id);
    }
    return { diagnosticId, fix };
  }
);

/**
 * Stores
 */
export const $diagnostics = createStore<Diagnostic[]>([]);
export const $filters = createStore<DiagnosticFilters>({
  showFixed: false,
});
export const $isValidating = validateProfileFx.pending;
export const $selectedDiagnosticId = createStore<string | null>(null);

/**
 * Update diagnostics on validation complete
 */
$diagnostics.on(validateProfileFx.doneData, (_, diagnostics) => diagnostics);

/**
 * Update diagnostics when received directly
 */
$diagnostics.on(diagnosticsReceived, (_, diagnostics) => diagnostics);

/**
 * Clear diagnostics
 */
$diagnostics.on(diagnosticsCleared, () => []);

/**
 * Mark diagnostic as fixed
 */
$diagnostics.on(applyQuickFixFx.doneData, (state, { diagnosticId }) =>
  state.map((d) => (d.id === diagnosticId ? { ...d, isFixed: true, isNew: false } : d))
);

/**
 * Dismiss diagnostic
 */
$diagnostics.on(diagnosticDismissed, (state, id) =>
  state.map((d) => (d.id === id ? { ...d, isFixed: true } : d))
);

/**
 * Mark all diagnostics as not new (after initial load)
 */
export const markAllAsRead = createEvent<void>();
$diagnostics.on(markAllAsRead, (state) => state.map((d) => ({ ...d, isNew: false })));

/**
 * Filters store
 */
$filters.on(filtersChanged, (state, filters) => ({
  ...state,
  ...filters,
}));

$filters.on(filterCleared, () => ({
  showFixed: false,
}));

/**
 * Selected diagnostic store
 */
$selectedDiagnosticId.on(diagnosticClicked, (_, d) => d.id);

/**
 * Derived stores
 */
export const $filteredDiagnostics = createStore<Diagnostic[]>([]);

sample({
  source: { diagnostics: $diagnostics, filters: $filters },
  fn: ({ diagnostics, filters }) => {
    return diagnostics.filter((d) => {
      // Filter by severity
      if (filters.severity && filters.severity.length > 0) {
        if (!filters.severity.includes(d.severity)) return false;
      }

      // Filter by element path
      if (filters.elementPath) {
        if (!d.elementPath?.includes(filters.elementPath)) return false;
      }

      // Filter by search query
      if (filters.searchQuery) {
        const query = filters.searchQuery.toLowerCase();
        if (
          !d.message.toLowerCase().includes(query) &&
          !d.code.toLowerCase().includes(query) &&
          !d.elementPath?.toLowerCase().includes(query)
        ) {
          return false;
        }
      }

      // Filter fixed issues
      if (!filters.showFixed && d.isFixed) {
        return false;
      }

      return true;
    });
  },
  target: $filteredDiagnostics,
});

/**
 * Count stores
 */
export const $errorCount = $diagnostics.map(
  (diagnostics) => diagnostics.filter((d) => d.severity === 'error' && !d.isFixed).length
);

export const $warningCount = $diagnostics.map(
  (diagnostics) => diagnostics.filter((d) => d.severity === 'warning' && !d.isFixed).length
);

export const $infoCount = $diagnostics.map(
  (diagnostics) => diagnostics.filter((d) => d.severity === 'info' && !d.isFixed).length
);

export const $totalCount = $diagnostics.map(
  (diagnostics) => diagnostics.filter((d) => !d.isFixed).length
);

export const $hasNewDiagnostics = $diagnostics.map((diagnostics) =>
  diagnostics.some((d) => d.isNew && !d.isFixed)
);

/**
 * Grouped diagnostics by element path
 */
export const $groupedDiagnostics = $filteredDiagnostics.map((diagnostics) => {
  const groups: Record<string, Diagnostic[]> = {};

  for (const d of diagnostics) {
    const path = d.elementPath || 'General';
    if (!groups[path]) {
      groups[path] = [];
    }
    groups[path].push(d);
  }

  // Sort groups by severity (errors first)
  const sortedPaths = Object.keys(groups).sort((a, b) => {
    const aHasError = groups[a].some((d) => d.severity === 'error');
    const bHasError = groups[b].some((d) => d.severity === 'error');
    if (aHasError && !bHasError) return -1;
    if (!aHasError && bHasError) return 1;
    return a.localeCompare(b);
  });

  return sortedPaths.map((path) => ({
    path,
    diagnostics: groups[path],
  }));
});

/**
 * Navigate to element when diagnostic is clicked
 */
sample({
  clock: diagnosticClicked,
  filter: (d) => !!d.elementPath,
  fn: (d) => d.elementPath!,
  target: selectElement,
});

/**
 * Apply quick fix
 */
sample({
  clock: quickFixApplied,
  target: applyQuickFixFx,
});
