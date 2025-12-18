import { StrictMode } from 'react';
import { EffectorProvider } from './providers/EffectorProvider';
import { MantineProvider } from './providers/MantineProvider';
import { QueryProvider } from './providers/QueryProvider';
import { RouterProvider } from './providers/RouterProvider';
import './styles/globals.css';

export function App() {
  return (
    <StrictMode>
      <EffectorProvider>
        <QueryProvider>
          <MantineProvider>
            <RouterProvider />
          </MantineProvider>
        </QueryProvider>
      </EffectorProvider>
    </StrictMode>
  );
}
