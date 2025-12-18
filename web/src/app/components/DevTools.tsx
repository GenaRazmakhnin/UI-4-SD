import { ENV } from '@shared/config';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

// Query DevTools only - Router DevTools are in RootLayout
export function DevTools() {
  if (!ENV.isDev) return null;

  return (
    <ReactQueryDevtools initialIsOpen={false}  buttonPosition="bottom-left" />
  );
}
