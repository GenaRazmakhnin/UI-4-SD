import { router } from '@app/routes';
import { RouterProvider as TanStackRouterProvider } from '@tanstack/react-router';

export function RouterProvider() {
  return <TanStackRouterProvider router={router} />;
}
