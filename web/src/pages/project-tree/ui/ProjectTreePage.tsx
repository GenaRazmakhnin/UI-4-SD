import { $selectedPath, artifactAdded, selectPath } from '@entities/file-tree';
import { CreateArtifactModal } from '@features/project/create-artifact';
import { Container } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { useParams } from '@tanstack/react-router';
import { ProjectExplorer } from '@widgets/project-explorer';
import { useUnit } from 'effector-react';

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
