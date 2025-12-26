import { $currentProject, useRestoreLastProject } from '@entities/project';
import { AppShell, Tooltip, UnstyledButton } from '@mantine/core';
import { useDisclosure, useHotkeys } from '@mantine/hooks';
import { ENV } from '@shared/config';
import { Outlet, Link, useRouterState } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import { useUnit } from 'effector-react';
import {
  IconFolders,
  IconPackage,
  IconHierarchy,
  IconSettings,
  IconChevronLeft,
  IconChevronRight,
} from '@tabler/icons-react';
import styles from './RootLayout.module.css';
import { TopNavigation } from './TopNavigation';
import { CommandPalette } from '../components/CommandPalette';

interface NavItemProps {
  icon: React.ReactNode;
  label: string;
  to: string;
  active?: boolean;
  collapsed?: boolean;
}

function NavItem({ icon, label, to, active, collapsed }: NavItemProps) {
  const button = (
    <UnstyledButton
      component={Link}
      to={to}
      className={styles.navItem}
      data-active={active || undefined}
    >
      <span className={styles.navIcon}>{icon}</span>
      {!collapsed && <span className={styles.navLabel}>{label}</span>}
    </UnstyledButton>
  );

  if (collapsed) {
    return (
      <Tooltip label={label} position="right" withArrow>
        {button}
      </Tooltip>
    );
  }

  return button;
}

export function RootLayout() {
  const currentProject = useUnit($currentProject);
  const { location } = useRouterState();
  const pathname = location.pathname;
  useRestoreLastProject();

  const [collapsed, { toggle: toggleCollapsed }] = useDisclosure(false);
  const [commandPaletteOpened, { open: openCommandPalette, close: closeCommandPalette }] =
    useDisclosure(false);

  // Keyboard shortcuts
  useHotkeys([
    ['mod+b', toggleCollapsed],
    ['mod+k', openCommandPalette],
  ]);

  const navItems = [
    { label: 'Projects', to: '/projects', icon: <IconFolders size={20} /> },
    { label: 'Packages', to: '/packages', icon: <IconPackage size={20} /> },
    ...(currentProject
      ? [
        {
          label: 'Files',
          to: `/projects/${currentProject.id}/tree`,
          icon: <IconHierarchy size={20} />,
        },
      ]
      : []),
  ];

  return (
    <>
      <AppShell
        header={{ height: 48 }}
        navbar={{
          width: collapsed ? 64 : 220,
          breakpoint: 'sm',
          collapsed: { mobile: true },
        }}
        padding={0}
        className={styles.appShell}
      >
        <AppShell.Header className={styles.header}>
          <TopNavigation project={currentProject} collapsed={collapsed} />
        </AppShell.Header>

        <AppShell.Navbar className={styles.navbar}>
          <div className={styles.navContent}>
            <div className={styles.navSection}>
              {navItems.map((item) => (
                <NavItem
                  key={item.to}
                  icon={item.icon}
                  label={item.label}
                  to={item.to}
                  active={pathname === item.to || pathname.startsWith(item.to + '/')}
                  collapsed={collapsed}
                />
              ))}
            </div>

            <div className={styles.navFooter}>
              <NavItem
                icon={<IconSettings size={20} />}
                label="Settings"
                to="/settings"
                active={pathname.startsWith('/settings')}
                collapsed={collapsed}
              />
              <UnstyledButton
                className={styles.collapseButton}
                onClick={toggleCollapsed}
                aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
              >
                {collapsed ? <IconChevronRight size={16} /> : <IconChevronLeft size={16} />}
              </UnstyledButton>
            </div>
          </div>
        </AppShell.Navbar>

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

      <CommandPalette opened={commandPaletteOpened} onClose={closeCommandPalette} />
    </>
  );
}
