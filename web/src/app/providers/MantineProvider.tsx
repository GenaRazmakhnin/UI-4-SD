import { createTheme, MantineProvider as MantineUIProvider } from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import { Notifications } from '@mantine/notifications';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';

const theme = createTheme({
  fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
  fontFamilyMonospace: '"Fira Code", "Consolas", "Monaco", monospace',
  primaryColor: 'blue',
  defaultRadius: 'md',
});

interface Props {
  children: React.ReactNode;
}

export function MantineProvider({ children }: Props) {
  return (
    <MantineUIProvider theme={theme} defaultColorScheme="light">
      <Notifications position="top-right" />
      <ModalsProvider>{children}</ModalsProvider>
    </MantineUIProvider>
  );
}
