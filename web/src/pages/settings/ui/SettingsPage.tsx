import { Container, Stack, Text, Title } from '@mantine/core';

export function SettingsPage() {
  return (
    <Container fluid py="xl" px="md">
      <Stack gap="lg">
        <Title order={1}>Settings</Title>
        <Text c="dimmed">Settings page will be implemented in a future task.</Text>
      </Stack>
    </Container>
  );
}
