export const config = {
  // API configuration
  USE_MOCK_API: import.meta.env.VITE_USE_MOCK_API === 'true',
  API_BASE_URL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',

  // Development flags
  IS_DEV: import.meta.env.DEV,
  IS_PROD: import.meta.env.PROD,

  // Feature flags
  ENABLE_UNDO_REDO: import.meta.env.VITE_ENABLE_UNDO_REDO !== 'false',
  ENABLE_FSH_EXPORT: import.meta.env.VITE_ENABLE_FSH_EXPORT !== 'false',

  // Performance
  VIRTUALIZATION_THRESHOLD: Number(import.meta.env.VITE_VIRTUALIZATION_THRESHOLD) || 100,
  DEBOUNCE_MS: Number(import.meta.env.VITE_DEBOUNCE_MS) || 300,
} as const;

// Legacy export for backwards compatibility
export const ENV = {
  API_BASE_URL: config.API_BASE_URL,
  NODE_ENV: import.meta.env.MODE,
  isDev: config.IS_DEV,
  isProd: config.IS_PROD,
} as const;
