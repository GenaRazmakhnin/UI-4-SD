import { mantineCssVariablesResolver, mantineTheme } from '@app/theme/mantineTheme';
import { MantineProvider as MantineUIProvider } from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import { Notifications } from '@mantine/notifications';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';

interface Props {
  children: React.ReactNode;
}

export function MantineProvider({ children }: Props) {
  return (
    <MantineUIProvider
      theme={mantineTheme}
      cssVariablesResolver={mantineCssVariablesResolver}
      defaultColorScheme="light"
    >
      <Notifications position="top-right" />
      <ModalsProvider>{children}</ModalsProvider>
    </MantineUIProvider>
  );
}
