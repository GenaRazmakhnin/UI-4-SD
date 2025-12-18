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

  toAbout: () => {
    router.navigate({ to: '/about' });
  },
};
