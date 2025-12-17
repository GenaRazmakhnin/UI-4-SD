# Task: App Initialization and Routing

## 📋 Description

Set up application initialization with TanStack Router for type-safe routing, configure all application providers (Effector, TanStack Query, Mantine), implement error boundaries, loading states, and establish the navigation structure for the FHIR Profile Builder.

**Reference**: IMPLEMENTATION_PLAN.md Section 13 "FSD Architecture", Section 15 "Technology Stack"

## 🎯 Context from Implementation Plan

This implements the application layer described in:
- **FSD App Layer** (Section 13): Application initialization, providers, and routing
- **Technology Stack** (Section 15): TanStack Router for type-safe routing
- **UI State Model** (Section 17): Effector integration at app level
- **Parallel Development** (Section 20): Route structure enabling parallel page development

## 📐 Requirements

### R1: TanStack Router Configuration

**Complete Router Setup**:
```typescript
// web/src/app/routes/index.tsx
import { createRootRoute, createRoute, createRouter, Outlet } from '@tanstack/react-router';
import { RootLayout } from '../layouts/RootLayout';
import { ProjectBrowserPage } from '@pages/project-browser';
import { ProfileEditorPage } from '@pages/editor';
import { SettingsPage } from '@pages/settings';
import { NotFoundPage } from '@pages/not-found';

// Root route with layout
const rootRoute = createRootRoute({
  component: RootLayout,
  notFoundComponent: NotFoundPage,
});

// Index route - Project Browser
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: ProjectBrowserPage,
});

// Profile Editor route with dynamic profileId
const editorRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/editor/$profileId',
  component: ProfileEditorPage,
  validateSearch: (search: Record<string, unknown>) => {
    return {
      tab: (search.tab as string) || 'constraints',
    };
  },
});

// Settings route
const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/settings',
  component: SettingsPage,
});

// About route
const aboutRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/about',
  component: () => <div>About FHIR Profile Builder</div>,
});

// Create route tree
export const routeTree = rootRoute.addChildren([
  indexRoute,
  editorRoute,
  settingsRoute,
  aboutRoute,
]);

// Create router instance
export const router = createRouter({
  routeTree,
  defaultPreload: 'intent',
  defaultPreloadStaleTime: 0,
});

// Register router for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}
```

**Type-Safe Navigation Helpers**:
```typescript
// web/src/shared/lib/navigation.ts
import { router } from '@app/routes';

export const navigation = {
  toProjectBrowser: () => {
    router.navigate({ to: '/' });
  },

  toEditor: (profileId: string, tab?: string) => {
    router.navigate({
      to: '/editor/$profileId',
      params: { profileId },
      search: tab ? { tab } : undefined,
    });
  },

  toSettings: () => {
    router.navigate({ to: '/settings' });
  },

  // Get current route params/search type-safely
  useEditorParams: () => {
    const params = router.state.location.params as { profileId: string };
    const search = router.state.location.search as { tab?: string };
    return { profileId: params.profileId, tab: search.tab || 'constraints' };
  },
};
```

### R2: Root Layout Component

**Complete Layout Implementation**:
```typescript
// web/src/app/layouts/RootLayout.tsx
import { Outlet } from '@tanstack/react-router';
import { AppShell } from '@mantine/core';
import { TopNavigation } from './TopNavigation';
import { useUnit } from 'effector-react';
import { $currentProject } from '@entities/project';
import styles from './RootLayout.module.css';

export function RootLayout() {
  const currentProject = useUnit($currentProject);

  return (
    <AppShell
      header={{ height: 56 }}
      padding={0}
      className={styles.appShell}
    >
      <AppShell.Header>
        <TopNavigation project={currentProject} />
      </AppShell.Header>

      <AppShell.Main className={styles.main}>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}
```

**Layout Styles**:
```css
/* web/src/app/layouts/RootLayout.module.css */
.appShell {
  height: 100vh;
  overflow: hidden;
}

.main {
  height: calc(100vh - 56px);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
```

### R3: Top Navigation Component

**Complete Navigation Bar**:
```typescript
// web/src/app/layouts/TopNavigation.tsx
import { Group, Text, ActionIcon, Menu, Breadcrumbs, Anchor } from '@mantine/core';
import { IconSettings, IconHelp, IconFileCode, IconBrandGithub } from '@tabler/icons-react';
import { Link } from '@tanstack/react-router';
import { navigation } from '@shared/lib/navigation';
import type { Project } from '@entities/project';
import styles from './TopNavigation.module.css';

interface TopNavigationProps {
  project: Project | null;
}

export function TopNavigation({ project }: TopNavigationProps) {
  return (
    <div className={styles.container}>
      {/* Left section - Logo & Breadcrumbs */}
      <Group gap="md">
        <Link to="/" className={styles.logo}>
          <IconFileCode size={24} />
          <Text size="lg" fw={600}>
            FHIR Profile Builder
          </Text>
        </Link>

        {project && (
          <Breadcrumbs separator="›" className={styles.breadcrumbs}>
            <Anchor component={Link} to="/" size="sm">
              Projects
            </Anchor>
            <Text size="sm" fw={500}>
              {project.name}
            </Text>
          </Breadcrumbs>
        )}
      </Group>

      {/* Right section - Actions */}
      <Group gap="xs">
        {/* Help Menu */}
        <Menu position="bottom-end" width={220}>
          <Menu.Target>
            <ActionIcon variant="subtle" size="lg" aria-label="Help">
              <IconHelp size={20} />
            </ActionIcon>
          </Menu.Target>

          <Menu.Dropdown>
            <Menu.Label>Documentation</Menu.Label>
            <Menu.Item
              leftSection={<IconFileCode size={16} />}
              component="a"
              href="https://fhir-profile-builder.dev/docs"
              target="_blank"
            >
              User Guide
            </Menu.Item>
            <Menu.Item
              leftSection={<IconBrandGithub size={16} />}
              component="a"
              href="https://github.com/your-org/fhir-profile-builder"
              target="_blank"
            >
              GitHub
            </Menu.Item>

            <Menu.Divider />

            <Menu.Item component={Link} to="/about">
              About
            </Menu.Item>
          </Menu.Dropdown>
        </Menu>

        {/* Settings */}
        <ActionIcon
          variant="subtle"
          size="lg"
          aria-label="Settings"
          onClick={() => navigation.toSettings()}
        >
          <IconSettings size={20} />
        </ActionIcon>
      </Group>
    </div>
  );
}
```

**Navigation Styles**:
```css
/* web/src/app/layouts/TopNavigation.module.css */
.container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 100%;
  padding: 0 var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-primary);
}

.logo {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  color: var(--text-primary);
  text-decoration: none;
  transition: opacity 150ms;
}

.logo:hover {
  opacity: 0.8;
}

.breadcrumbs {
  color: var(--text-secondary);
}
```

### R4: Router Provider Component

**RouterProvider Setup**:
```typescript
// web/src/app/providers/RouterProvider.tsx
import { RouterProvider as TanStackRouterProvider } from '@tanstack/react-router';
import { router } from '@app/routes';

export function RouterProvider() {
  return <TanStackRouterProvider router={router} />;
}
```

### R5: Complete Provider Composition

**Combined Providers**:
```typescript
// web/src/app/providers/index.tsx
import { ReactNode } from 'react';
import { EffectorProvider } from './EffectorProvider';
import { QueryProvider } from './QueryProvider';
import { MantineProvider } from './MantineProvider';

interface ProvidersProps {
  children: ReactNode;
}

export function Providers({ children }: ProvidersProps) {
  return (
    <EffectorProvider>
      <QueryProvider>
        <MantineProvider>
          {children}
        </MantineProvider>
      </QueryProvider>
    </EffectorProvider>
  );
}
```

### R6: Error Boundary Implementation

**Global Error Boundary**:
```typescript
// web/src/app/components/ErrorBoundary.tsx
import { Component, ReactNode, ErrorInfo } from 'react';
import { Container, Title, Text, Button, Stack, Code } from '@mantine/core';
import { IconAlertTriangle } from '@tabler/icons-react';
import styles from './ErrorBoundary.module.css';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
    };
  }

  static getDerivedStateFromError(error: Error): Partial<State> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Error caught by boundary:', error, errorInfo);
    this.setState({ errorInfo });

    // Log to error reporting service
    // reportError(error, errorInfo);
  }

  handleReset = () => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
    window.location.href = '/';
  };

  render() {
    if (this.state.hasError) {
      return (
        <Container size="sm" className={styles.container}>
          <Stack align="center" gap="lg">
            <IconAlertTriangle size={64} color="var(--error-color)" />

            <Title order={2}>Something went wrong</Title>

            <Text size="sm" c="dimmed" ta="center">
              An unexpected error occurred. Please try refreshing the page or returning to the
              home page.
            </Text>

            {this.state.error && (
              <Code block className={styles.errorCode}>
                {this.state.error.toString()}
                {this.state.errorInfo && (
                  <>
                    {'\n\n'}
                    {this.state.errorInfo.componentStack}
                  </>
                )}
              </Code>
            )}

            <Stack gap="sm" w="100%">
              <Button onClick={this.handleReset} fullWidth>
                Return to Home
              </Button>
              <Button variant="subtle" onClick={() => window.location.reload()} fullWidth>
                Refresh Page
              </Button>
            </Stack>
          </Stack>
        </Container>
      );
    }

    return this.props.children;
  }
}
```

**Error Boundary Styles**:
```css
/* web/src/app/components/ErrorBoundary.module.css */
.container {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.errorCode {
  max-height: 300px;
  overflow: auto;
  width: 100%;
  font-size: var(--font-xs);
}
```

### R7: Loading States

**Global Loading Indicator**:
```typescript
// web/src/app/components/GlobalLoader.tsx
import { LoadingOverlay } from '@mantine/core';
import { useIsFetching, useIsMutating } from '@tanstack/react-query';

export function GlobalLoader() {
  const isFetching = useIsFetching();
  const isMutating = useIsMutating();

  const isLoading = isFetching > 0 || isMutating > 0;

  return (
    <LoadingOverlay
      visible={isLoading}
      overlayProps={{ blur: 1 }}
      loaderProps={{ size: 'lg', type: 'dots' }}
      zIndex={1000}
    />
  );
}
```

**Route Loading Fallback**:
```typescript
// web/src/app/components/RouteLoader.tsx
import { Center, Loader, Stack, Text } from '@mantine/core';

interface RouteLoaderProps {
  message?: string;
}

export function RouteLoader({ message = 'Loading...' }: RouteLoaderProps) {
  return (
    <Center h="100vh">
      <Stack align="center" gap="md">
        <Loader size="lg" />
        <Text size="sm" c="dimmed">
          {message}
        </Text>
      </Stack>
    </Center>
  );
}
```

### R8: Not Found Page

**404 Page Component**:
```typescript
// web/src/pages/not-found/ui/NotFoundPage.tsx
import { Container, Title, Text, Button, Stack } from '@mantine/core';
import { IconError404 } from '@tabler/icons-react';
import { Link } from '@tanstack/react-router';
import styles from './NotFoundPage.module.css';

export function NotFoundPage() {
  return (
    <Container size="sm" className={styles.container}>
      <Stack align="center" gap="lg">
        <IconError404 size={120} stroke={1} color="var(--text-tertiary)" />

        <Title order={1}>Page Not Found</Title>

        <Text size="lg" c="dimmed" ta="center">
          The page you're looking for doesn't exist or has been moved.
        </Text>

        <Button component={Link} to="/" size="lg">
          Go to Home
        </Button>
      </Stack>
    </Container>
  );
}
```

### R9: Application Entry Point

**Updated main.tsx**:
```typescript
// web/src/main.tsx
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from '@app/App';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import './app/styles/globals.css';

const rootElement = document.getElementById('root');

if (!rootElement) {
  throw new Error('Root element not found');
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>
);
```

**Updated App.tsx**:
```typescript
// web/src/app/App.tsx
import { Providers } from './providers';
import { RouterProvider } from './providers/RouterProvider';
import { ErrorBoundary } from './components/ErrorBoundary';
import { GlobalLoader } from './components/GlobalLoader';
import { TanStackRouterDevtools } from '@tanstack/router-devtools';
import { ENV } from '@shared/config';

export function App() {
  return (
    <ErrorBoundary>
      <Providers>
        <RouterProvider />
        <GlobalLoader />
        {ENV.isDev && <TanStackRouterDevtools position="bottom-right" />}
      </Providers>
    </ErrorBoundary>
  );
}
```

### R10: Development Tools Integration

**DevTools Configuration**:
```typescript
// web/src/app/components/DevTools.tsx
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { TanStackRouterDevtools } from '@tanstack/router-devtools';
import { ENV } from '@shared/config';

export function DevTools() {
  if (!ENV.isDev) return null;

  return (
    <>
      <ReactQueryDevtools
        initialIsOpen={false}
        position="bottom-left"
        buttonPosition="bottom-left"
      />
      <TanStackRouterDevtools
        position="bottom-right"
        toggleButtonProps={{
          style: {
            marginLeft: 'auto',
          },
        }}
      />
    </>
  );
}
```

### R11: Route Preloading Configuration

**Preload Strategy**:
```typescript
// web/src/app/routes/preload.ts
import { router } from './index';

// Preload common routes on idle
export function setupRoutePreloading() {
  if ('requestIdleCallback' in window) {
    requestIdleCallback(() => {
      // Preload settings page
      router.preloadRoute({ to: '/settings' });
    });
  }
}

// Call after app initialization
setupRoutePreloading();
```

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] App renders without errors in development mode
- [ ] Routing works correctly:
  - [ ] Navigate to project browser (/)
  - [ ] Navigate to profile editor (/editor/:profileId)
  - [ ] Navigate to settings (/settings)
  - [ ] 404 page shows for invalid routes
- [ ] All providers are configured and working:
  - [ ] Effector state management functional
  - [ ] TanStack Query can fetch data
  - [ ] Mantine components render with theme
- [ ] Navigation component works:
  - [ ] Logo links to home
  - [ ] Settings button navigates to settings
  - [ ] Help menu opens with links
  - [ ] Breadcrumbs show current location
- [ ] Error boundary catches and displays errors gracefully
- [ ] Loading states display correctly:
  - [ ] Global loader shows during data fetching
  - [ ] Route transitions have loading indicators
- [ ] Type-safe routing works (TypeScript autocomplete for routes)
- [ ] DevTools are available in development:
  - [ ] React Query DevTools visible
  - [ ] TanStack Router DevTools visible

### Performance Requirements
- [ ] Initial page load <2 seconds
- [ ] Route transitions <500ms
- [ ] No layout shift during navigation
- [ ] Preloading works on idle
- [ ] Bundle size optimized (<300KB for initial route)

### Accessibility Requirements (WCAG 2.1 AA)
- [ ] Keyboard navigation works for all routes
- [ ] Focus management works on route changes
- [ ] ARIA labels on navigation buttons
- [ ] Skip to main content link available
- [ ] Error messages are announced to screen readers
- [ ] Color contrast meets WCAG AA standards

### Testing Requirements
- [ ] Can navigate between all routes
- [ ] URL parameters are type-safe
- [ ] Search parameters work correctly
- [ ] Error boundary catches component errors
- [ ] 404 page shows for invalid routes
- [ ] Providers render in correct order
- [ ] Theme is applied globally
- [ ] DevTools work in development mode

## 🔗 Dependencies

### Required Before
- **UI 01**: React App Scaffold (FSD structure, dependencies)

### Required For
- **UI 03**: Mock Data Layer (needs routing for pages)
- **UI 19**: Profile Editor Page (uses routing)
- **All page-level tasks**: Routing infrastructure

### Integration Points
- **Entities**: Project, Profile (state accessed in layout)
- **Pages**: All pages use routing

## 📚 API Contract

Not applicable - this task is frontend-only routing setup.

## 🧪 Testing Examples

**Test Type-Safe Navigation**:
```typescript
// Type errors should show if route doesn't exist
navigation.toEditor('profile-123'); // ✅
navigation.toEditor('profile-123', 'constraints'); // ✅
navigation.toNonExistent(); // ❌ TypeScript error

// Params should be type-safe
const { profileId, tab } = navigation.useEditorParams();
// profileId: string
// tab: string
```

**Test Route Navigation**:
```typescript
// Manual testing in browser
// 1. Navigate to /
// 2. Navigate to /editor/test-profile
// 3. Navigate to /settings
// 4. Navigate to /invalid-route (should show 404)
```

**Test Error Boundary**:
```typescript
// Create a component that throws
function BrokenComponent() {
  throw new Error('Test error');
}

// Add to a route temporarily
// Should show error boundary UI
```

**Test Loading States**:
```typescript
// In browser console
import { router } from '@app/routes';

// Navigate and observe loading indicator
router.navigate({ to: '/editor/$profileId', params: { profileId: 'test' } });
```

**Storybook Stories**:
```typescript
// web/src/app/layouts/TopNavigation.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import { TopNavigation } from './TopNavigation';

const meta: Meta<typeof TopNavigation> = {
  title: 'App/TopNavigation',
  component: TopNavigation,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;
type Story = StoryObj<typeof TopNavigation>;

export const WithProject: Story = {
  args: {
    project: {
      id: 'proj-1',
      name: 'My FHIR Project',
    },
  },
};

export const WithoutProject: Story = {
  args: {
    project: null,
  },
};
```

## 📖 Related Documentation

- **IMPLEMENTATION_PLAN.md Section 13**: FSD Architecture - App Layer
- **IMPLEMENTATION_PLAN.md Section 15**: Technology Stack - TanStack Router
- **IMPLEMENTATION_PLAN.md Section 17**: UI State Model - Effector integration
- **IMPLEMENTATION_PLAN.md Section 20**: Parallel Development workflow

## 🎨 Priority

🔴 **Critical** - Core app infrastructure, blocks all page development

## ⏱️ Estimated Complexity

**Medium** - 1 week
- Days 1-2: Router setup, route configuration, type safety
- Day 3: Layout components, navigation, breadcrumbs
- Day 4: Error boundaries, loading states, 404 page
- Day 5: DevTools integration, testing, polish
