import { router } from '@app/routes';

export const navigation = {
  toProjects: () => {
    router.navigate({ to: '/projects' });
  },

  toProject: (projectId: string) => {
    router.navigate({ to: '/projects/$projectId', params: { projectId } });
  },

  toProjectFiles: (projectId: string) => {
    router.navigate({ to: '/projects/$projectId/tree', params: { projectId } });
  },

  toProfiles: (projectId: string, profileId: string = 'patient') => {
    router.navigate({
      to: '/projects/$projectId/profiles/$profileId',
      params: { projectId, profileId },
    });
  },

  // Legacy alias
  toProjectBrowser: () => {
    router.navigate({ to: '/projects' });
  },

  toEditor: (projectId: string, profileId: string, tab?: string) => {
    router.navigate({
      to: '/projects/$projectId/profiles/$profileId',
      params: { projectId, profileId },
      search: tab ? { tab } : undefined,
    });
  },

  toSettings: () => {
    router.navigate({ to: '/settings' });
  },

  toPackages: () => {
    router.navigate({ to: '/packages' });
  },

  toAbout: () => {
    router.navigate({ to: '/about' });
  },
};
