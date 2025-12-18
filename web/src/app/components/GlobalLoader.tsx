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
