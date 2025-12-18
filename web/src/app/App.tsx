import { DevTools } from './components/DevTools';
import { ErrorBoundary } from './components/ErrorBoundary';
import { GlobalLoader } from './components/GlobalLoader';
import { Providers, RouterProvider } from './providers';
import './styles/globals.css';

export function App() {
  return (
    <Providers>
      <ErrorBoundary>
        <RouterProvider />
        <GlobalLoader />
        <DevTools />
      </ErrorBoundary>
    </Providers>
  );
}
