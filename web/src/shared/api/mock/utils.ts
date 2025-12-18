/**
 * Simulates network delay with random jitter
 */
export async function simulateDelay(
  minMs: number,
  maxMs: number,
): Promise<void> {
  const delay = Math.floor(Math.random() * (maxMs - minMs) + minMs);
  await new Promise((resolve) => setTimeout(resolve, delay));
}

/**
 * Simulates random errors for testing error handling
 */
export function simulateError(probability: number): boolean {
  return Math.random() < probability;
}

/**
 * Logs mock API calls in development mode
 */
export function logMockCall(
  method: string,
  endpoint: string,
  data?: unknown,
): void {
  if (import.meta.env.DEV) {
    console.log(`[Mock API] ${method} ${endpoint}`, data || '');
  }
}
