import { UndoRedoProvider } from '@features/undo-redo';
import type { ReactNode } from 'react';
import { EffectorProvider } from './EffectorProvider';
import { MantineProvider } from './MantineProvider';
import { QueryProvider } from './QueryProvider';

export { RouterProvider } from './RouterProvider';

interface ProvidersProps {
  children: ReactNode;
}

export function Providers({ children }: ProvidersProps) {
  return (
    <MantineProvider>
      <EffectorProvider>
        <QueryProvider>
          <UndoRedoProvider>{children}</UndoRedoProvider>
        </QueryProvider>
      </EffectorProvider>
    </MantineProvider>
  );
}
