import type { Project } from '@entities/project';
import { ProjectSwitcher } from '@features/project/switch-project';
import {
  ActionIcon,
  Anchor,
  Button,
  Divider,
  Flex,
  Group,
  Menu,
  NavLink,
  Text,
  Tooltip,
} from '@mantine/core';
import { navigation } from '@shared/lib/navigation';
import {
  IconBrandGithub,
  IconFileCode,
  IconFolders,
  IconHelp,
  IconHierarchy,
  IconPackage,
} from '@tabler/icons-react';
import { Link, useRouterState } from '@tanstack/react-router';
import styles from './TopNavigation.module.css';

interface TopNavigationProps {
  project: Project | null;
}

export function TopNavigation({ project }: TopNavigationProps) {
  const { location } = useRouterState();
  const pathname = location.pathname;

  const navLinks = [
    { label: 'Projects', to: '/projects', icon: <IconFolders size={16} /> },
    { label: 'Packages', to: '/packages', icon: <IconPackage size={16} /> },
    project
      ? {
          label: 'Files',
          to: `/projects/${project.id}/tree`,
          icon: <IconHierarchy size={16} />,
        }
      : null,
  ].filter(Boolean) as { label: string; to: string; icon: React.ReactNode }[];

  return (
    <div className={styles.container}>
      <Group gap="md">
        <Link to="/projects" className={styles.logo}>
          <span className={styles.logoMark} aria-hidden="true">
            <img src="/logo.png" alt="" className={styles.logoImage} />
          </span>
          <Text size="lg" fw={600}>
            NITEN
          </Text>
        </Link>

        <Flex gap="md">
          {navLinks.map((link) => (
            <NavLink
              key={link.to}
              component={Link}
              to={link.to}
              leftSection={link.icon}
              variant="subtle"
              aria-current={
                pathname === link.to || pathname.startsWith(link.to) ? 'page' : undefined
              }
              className={styles.navButton}
              label={link.label}
            />
          ))}
        </Flex>
      </Group>

      {/* Right section - Actions */}
      <Group gap="xs">
        <ProjectSwitcher />
        <Divider orientation="vertical" />

        {/* Help Menu */}
        <Menu position="bottom-end" width={220}>
          <Menu.Target>
            <ActionIcon variant="subtle" size="lg" aria-label="Help">
              <IconHelp size={20} />
            </ActionIcon>
          </Menu.Target>

          <Menu.Dropdown>
            <Menu.Label>Documentation</Menu.Label>
            <Menu.Item
              leftSection={<IconFileCode size={16} />}
              component="a"
              href="https://fhir-profile-builder.dev/docs"
              target="_blank"
            >
              User Guide
            </Menu.Item>
            <Menu.Item
              leftSection={<IconBrandGithub size={16} />}
              component="a"
              href="https://github.com/your-org/fhir-profile-builder"
              target="_blank"
            >
              GitHub
            </Menu.Item>

            <Menu.Divider />

            <Menu.Item component={Link} to="/about">
              About
            </Menu.Item>
          </Menu.Dropdown>
        </Menu>
      </Group>
    </div>
  );
}
