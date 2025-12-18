// Export main component

// Export types
export type { DiagnosticFilters } from './model';
// Export model (stores, events, effects)
export {
  // Stores
  $diagnostics,
  $errorCount,
  $filteredDiagnostics,
  $filters,
  $groupedDiagnostics,
  $hasNewDiagnostics,
  $infoCount,
  $isValidating,
  $selectedDiagnosticId,
  $totalCount,
  $warningCount,
  // Events
  applyAllFixes,
  // Effects
  applyQuickFixFx,
  diagnosticClicked,
  diagnosticDismissed,
  diagnosticsCleared,
  diagnosticsReceived,
  filterCleared,
  filtersChanged,
  markAllAsRead,
  quickFixApplied,
  validateProfileFx,
} from './model';
// Export sub-components
export { DiagnosticItem } from './ui/DiagnosticItem';
export { DiagnosticsList } from './ui/DiagnosticsList';
export { DiagnosticsPanel } from './ui/DiagnosticsPanel';
