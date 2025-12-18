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
