import { Container } from '@mantine/core';
import { useParams } from '@tanstack/react-router';
import { $selectedPath, selectPath } from '@entities/file-tree';
import { useUnit } from 'effector-react';
import { ProjectExplorer } from '@widgets/project-explorer';
import { CreateArtifactModal } from '@features/project/create-artifact';
import { useDisclosure } from '@mantine/hooks';
import { artifactAdded } from '@entities/file-tree';

export function ProjectTreePage() {
  const { projectId } = useParams({ from: '/projects/$projectId/tree' });
  const selectedPath = useUnit($selectedPath);
  const [isArtifactOpen, { open: openArtifact, close: closeArtifact }] = useDisclosure(false);

  return (
    <Container fluid py="xl">
      <ProjectExplorer
        projectId={projectId}
        onCreateArtifact={() => {
          openArtifact();
        }}
      />
      <CreateArtifactModal
        opened={isArtifactOpen}
        onClose={closeArtifact}
        projectId={projectId}
        onCreated={({ projectId: pid, artifact }) => {
          if (pid === projectId && artifact?.path) {
            selectPath(artifact.path);
          }
          artifactAdded(artifact);
          closeArtifact();
        }}
      />
    </Container>
  );
}
