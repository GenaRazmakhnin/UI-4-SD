import { NewProjectDialog, RecentProjects } from '@features/project-templates';
import { Container, Stack, Title } from '@mantine/core';

export function ProjectBrowserPage() {
  return (
    <Container size="lg" py="xl">
      <Stack gap="xl">
        <Title order={1}>Projects</Title>

        {/* Recent Projects with New Project button */}
        <RecentProjects maxItems={10} showClearAll showNewButton />

        {/* New Project Dialog (rendered as modal, triggered by button in RecentProjects) */}
        <NewProjectDialog />
      </Stack>
    </Container>
  );
}
