import { Button, Container, Stack, Text, Title } from '@mantine/core';
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
