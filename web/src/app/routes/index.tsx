import { ProfileEditorPage } from '@pages/editor';
import { NotFoundPage } from '@pages/not-found';
import { PackagesPage } from '@pages/packages';
import { ProjectTreePage } from '@pages/project-tree';
import { ProjectDetailsPage, ProjectsPage } from '@pages/projects';
import { SettingsPage } from '@pages/settings';
import { createRootRoute, createRoute, createRouter, Navigate } from '@tanstack/react-router';
import { RootLayout } from '../layouts/RootLayout';

// Root route with layout
const rootRoute = createRootRoute({
  component: RootLayout,
  notFoundComponent: NotFoundPage,
});

// Index route - Project Browser
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: () => <Navigate to="/projects" />,
});

const projectsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/projects',
  component: ProjectsPage,
});

const projectDetailRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/projects/$projectId',
  component: ProjectDetailsPage,
});

const projectTreeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/projects/$projectId/tree',
  component: ProjectTreePage,
  validateSearch: (search: Record<string, unknown>) => ({
    q: typeof search.q === 'string' ? search.q : '',
  }),
});

// Profile Editor route with dynamic project and profile ids
const profileEditorRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/projects/$projectId/profiles/$profileId',
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

// Packages route
const packagesRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/packages',
  component: PackagesPage,
});

// About route
const aboutRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/about',
  component: () => (
    <div style={{ padding: '2rem' }}>
      <h1>About FHIR Profile Builder</h1>
      <p>A modern tool for building and editing FHIR profiles.</p>
    </div>
  ),
});

// Create route tree
export const routeTree = rootRoute.addChildren([
  indexRoute,
  projectsRoute,
  projectDetailRoute,
  projectTreeRoute,
  profileEditorRoute,
  packagesRoute,
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
