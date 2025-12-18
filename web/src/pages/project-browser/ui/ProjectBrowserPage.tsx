import { Container, Stack, Text, Title } from '@mantine/core';

export function ProjectBrowserPage() {
  return (
    <Container size="lg" py="xl">
      <Stack gap="lg">
        <Title order={1}>Projects</Title>
        <Text c="dimmed">Project browser will be implemented in a future task.</Text>
      </Stack>
    </Container>
  );
}
