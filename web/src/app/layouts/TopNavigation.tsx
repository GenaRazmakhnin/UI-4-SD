import type { Project } from '@entities/project';
import { ActionIcon, Anchor, Breadcrumbs, Group, Menu, Text } from '@mantine/core';
import { navigation } from '@shared/lib/navigation';
import { IconBrandGithub, IconFileCode, IconHelp, IconSettings } from '@tabler/icons-react';
import { Link } from '@tanstack/react-router';
import styles from './TopNavigation.module.css';

interface TopNavigationProps {
  project: Project | null;
}

export function TopNavigation({ project }: TopNavigationProps) {
  return (
    <div className={styles.container}>
      {/* Left section - Logo & Breadcrumbs */}
      <Group gap="md">
        <Link to="/" className={styles.logo}>
          <IconFileCode size={24} />
          <Text size="lg" fw={600}>
            FHIR Profile Builder
          </Text>
        </Link>

        {project && (
          <Breadcrumbs separator="›" className={styles.breadcrumbs}>
            <Anchor component={Link} to="/" size="sm">
              Projects
            </Anchor>
            <Text size="sm" fw={500}>
              {project.name}
            </Text>
          </Breadcrumbs>
        )}
      </Group>

      {/* Right section - Actions */}
      <Group gap="xs">
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

        {/* Settings */}
        <ActionIcon
          variant="subtle"
          size="lg"
          aria-label="Settings"
          onClick={() => navigation.toSettings()}
        >
          <IconSettings size={20} />
        </ActionIcon>
      </Group>
    </div>
  );
}
