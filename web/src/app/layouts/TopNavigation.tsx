import type { Project } from '@entities/project';
import {
  ActionIcon,
  Breadcrumbs,
  Group,
  Menu,
  Text,
  Anchor,
  TextInput,
} from '@mantine/core';
import {
  IconBrandGithub,
  IconFileCode,
  IconHelp,
  IconSearch,
  IconCommand,
} from '@tabler/icons-react';
import { Link, useRouterState } from '@tanstack/react-router';
import styles from './TopNavigation.module.css';

interface TopNavigationProps {
  project: Project | null;
  collapsed?: boolean;
}

export function TopNavigation({ project, collapsed }: TopNavigationProps) {
  const { location } = useRouterState();
  const pathname = location.pathname;

  // Build breadcrumbs based on current path
  const buildBreadcrumbs = () => {
    const parts = pathname.split('/').filter(Boolean);
    const crumbs: { label: string; to: string }[] = [];

    if (parts[0] === 'projects') {
      crumbs.push({ label: 'Projects', to: '/projects' });
      if (project && parts.length > 1) {
        crumbs.push({ label: project.name, to: `/projects/${project.id}` });
        if (parts[2] === 'tree') {
          crumbs.push({ label: 'Files', to: `/projects/${project.id}/tree` });
        } else if (parts[2] === 'profiles' && parts[3]) {
          crumbs.push({ label: 'Profile', to: pathname });
        }
      }
    } else if (parts[0] === 'packages') {
      crumbs.push({ label: 'Packages', to: '/packages' });
    } else if (parts[0] === 'settings') {
      crumbs.push({ label: 'Settings', to: '/settings' });
    }

    return crumbs;
  };

  const breadcrumbs = buildBreadcrumbs();

  return (
    <div className={styles.container}>
      <Group gap="md" style={{ flex: 1 }}>
        {/* Logo */}
        <Link to="/projects" className={styles.logo}>
          <img src="/logo.png" alt="" className={styles.logoImage} />
          {!collapsed && (
            <Text size="sm" fw={600} className={styles.logoText}>
              NITEN
            </Text>
          )}
        </Link>

        {/* Breadcrumbs */}
        {breadcrumbs.length > 0 && (
          <>
            <span className={styles.divider} />
            <Breadcrumbs
              separator="/"
              separatorMargin={6}
              classNames={{ separator: styles.breadcrumbSeparator }}
            >
              {breadcrumbs.map((crumb, index) => (
                <Anchor
                  key={crumb.to}
                  component={Link}
                  to={crumb.to}
                  size="sm"
                  className={styles.breadcrumbLink}
                  data-active={index === breadcrumbs.length - 1 || undefined}
                >
                  {crumb.label}
                </Anchor>
              ))}
            </Breadcrumbs>
          </>
        )}
      </Group>

      {/* Right section - Actions */}
      <Group gap="xs">
        {/* Search / Command Palette Trigger */}
        <TextInput
          placeholder="Search or jump to..."
          size="xs"
          leftSection={<IconSearch size={14} />}
          rightSection={
            <Group gap={4} className={styles.shortcutHint}>
              <IconCommand size={12} />
              <Text size="xs">K</Text>
            </Group>
          }
          classNames={{ input: styles.searchInput }}
          readOnly
          onClick={() => {
            // TODO: Open command palette
          }}
          style={{ width: 200 }}
        />

        {/* Help Menu */}
        <Menu position="bottom-end" width={200}>
          <Menu.Target>
            <ActionIcon variant="subtle" size="md" radius="md" aria-label="Help">
              <IconHelp size={18} />
            </ActionIcon>
          </Menu.Target>

          <Menu.Dropdown>
            <Menu.Label>Documentation</Menu.Label>
            <Menu.Item
              leftSection={<IconFileCode size={14} />}
              component="a"
              href="https://fhir-profile-builder.dev/docs"
              target="_blank"
            >
              User Guide
            </Menu.Item>
            <Menu.Item
              leftSection={<IconBrandGithub size={14} />}
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
