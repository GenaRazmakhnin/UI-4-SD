import { Container, Stack, Text, Title } from '@mantine/core';
import { useParams, useSearch } from '@tanstack/react-router';

export function ProfileEditorPage() {
  const { profileId } = useParams({ from: '/editor/$profileId' });
  const { tab } = useSearch({ from: '/editor/$profileId' });

  return (
    <Container size="lg" py="xl">
      <Stack gap="lg">
        <Title order={1}>Profile Editor</Title>
        <Text c="dimmed">Editing profile: {profileId}</Text>
        <Text c="dimmed">Active tab: {tab || 'constraints'}</Text>
        <Text c="dimmed">Full editor will be implemented in a future task.</Text>
      </Stack>
    </Container>
  );
}
