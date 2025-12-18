import { Box } from '@mantine/core';
import { PackageBrowser } from '@widgets/package-browser';

export function PackagesPage() {
  return (
    <Box p="md" h="100%">
      <PackageBrowser height="calc(100vh - 90px)" />
    </Box>
  );
}
