import { useProjectResource } from '@entities/resource';
import { notifications } from '@mantine/notifications';
import { navigation } from '@shared/lib/navigation';
import { useEffect, useRef } from 'react';

interface ProfileResourceGuardOptions {
  projectId: string;
  profileId: string;
}

export function useProfileResourceGuard({ projectId, profileId }: ProfileResourceGuardOptions) {
  const { data, isLoading, isError } = useProjectResource(projectId, profileId);
  const hasRedirected = useRef(false);

  useEffect(() => {
    if (hasRedirected.current || isLoading) return;

    const isProfile = data?.resourceKind === 'profile';
    const isIR = data?.root === 'IR';
    if (!isProfile || !isIR || isError) {
      hasRedirected.current = true;
      notifications.show({
        title: 'Profile Editor unavailable',
        message: !isProfile
          ? 'Only profile StructureDefinitions can be opened in the editor.'
          : 'Profile Editor is only available for IR resources in this project.',
        color: 'red',
      });
      navigation.toProjectFiles(projectId);
    }
  }, [data?.resourceKind, data?.root, isError, isLoading, projectId]);

  return { resource: data, isLoading, isError };
}
