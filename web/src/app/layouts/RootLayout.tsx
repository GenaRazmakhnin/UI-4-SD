import { $currentProject } from '@entities/project';
import { AppShell } from '@mantine/core';
import { ENV } from '@shared/config';
import { Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import { useUnit } from 'effector-react';
import styles from './RootLayout.module.css';
import { TopNavigation } from './TopNavigation';

export function RootLayout() {
  const currentProject = useUnit($currentProject);

  return (
    <>
      <AppShell header={{ height: 56 }} padding={0} className={styles.appShell}>
        <AppShell.Header>
          <TopNavigation project={currentProject} />
        </AppShell.Header>

        <AppShell.Main className={styles.main}>
          <Outlet />
        </AppShell.Main>
      </AppShell>

      {ENV.isDev && (
        <TanStackRouterDevtools
          position="bottom-right"
          toggleButtonProps={{
            style: {
              marginLeft: 'auto',
            },
          }}
        />
      )}
    </>
  );
}
