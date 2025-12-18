export class MockApiError extends Error {
  constructor(
    message: string,
    public statusCode: number,
    public details?: unknown
  ) {
    super(message);
    this.name = 'MockApiError';
  }
}

/**
 * Simulates various error scenarios
 */
export const errorSimulator = {
  /**
   * Random network error (5% chance)
   */
  networkError(): void {
    if (Math.random() < 0.05) {
      throw new MockApiError('Network error', 500);
    }
  },

  /**
   * Rate limiting error
   */
  rateLimitError(): void {
    if (Math.random() < 0.02) {
      throw new MockApiError('Rate limit exceeded', 429, {
        retryAfter: 60,
      });
    }
  },

  /**
   * Validation error
   */
  validationError(field: string, message: string): MockApiError {
    return new MockApiError('Validation failed', 422, {
      field,
      message,
    });
  },

  /**
   * Not found error
   */
  notFoundError(resource: string, id: string): MockApiError {
    return new MockApiError(`${resource} not found`, 404, {
      resource,
      id,
    });
  },

  /**
   * Unauthorized error
   */
  unauthorizedError(): MockApiError {
    return new MockApiError('Unauthorized', 401);
  },
};
