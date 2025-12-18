import { config } from '@shared/config/env';
import { mockApi } from './mock';
import { realApi } from './real';

// Export API client for direct use
export { ApiClient, apiClient } from './client';

// Switch between mock and real API based on environment
export const api = config.USE_MOCK_API ? mockApi : realApi;

// Export types for use in components
export type Api = typeof api;

// Log which API is being used
if (config.IS_DEV) {
  console.log(`[API] Using ${config.USE_MOCK_API ? 'MOCK' : 'REAL'} API`);
}
