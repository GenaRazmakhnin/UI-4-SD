import { Modal, TextInput, Stack, UnstyledButton, Text, Group, Kbd } from '@mantine/core';
import { useDebouncedValue } from '@mantine/hooks';
import {
    IconSearch,
    IconFolders,
    IconPackage,
    IconSettings,
} from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useState, useMemo, useCallback, useEffect } from 'react';
import styles from './CommandPalette.module.css';

interface CommandItem {
    id: string;
    label: string;
    description?: string;
    icon: React.ReactNode;
    category: 'navigation' | 'recent' | 'action';
    action: () => void;
    shortcut?: string;
}

interface CommandPaletteProps {
    opened: boolean;
    onClose: () => void;
}

export function CommandPalette({ opened, onClose }: CommandPaletteProps) {
    const navigate = useNavigate();
    const [query, setQuery] = useState('');
    const [debouncedQuery] = useDebouncedValue(query, 150);
    const [selectedIndex, setSelectedIndex] = useState(0);

    const commands: CommandItem[] = useMemo(
        () => [
            {
                id: 'nav-projects',
                label: 'Projects',
                description: 'Browse all projects',
                icon: <IconFolders size={18} />,
                category: 'navigation',
                action: () => {
                    navigate({ to: '/projects' });
                    onClose();
                },
                shortcut: 'G P',
            },
            {
                id: 'nav-packages',
                label: 'Packages',
                description: 'Browse FHIR packages',
                icon: <IconPackage size={18} />,
                category: 'navigation',
                action: () => {
                    navigate({ to: '/packages' });
                    onClose();
                },
                shortcut: 'G K',
            },
            {
                id: 'nav-settings',
                label: 'Settings',
                description: 'Application settings',
                icon: <IconSettings size={18} />,
                category: 'navigation',
                action: () => {
                    navigate({ to: '/settings' });
                    onClose();
                },
                shortcut: 'G S',
            },
        ],
        [navigate, onClose]
    );

    const filteredCommands = useMemo(() => {
        if (!debouncedQuery) return commands;
        const lower = debouncedQuery.toLowerCase();
        return commands.filter(
            (cmd) =>
                cmd.label.toLowerCase().includes(lower) ||
                cmd.description?.toLowerCase().includes(lower)
        );
    }, [commands, debouncedQuery]);

    const handleKeyDown = useCallback(
        (e: React.KeyboardEvent) => {
            if (e.key === 'ArrowDown') {
                e.preventDefault();
                setSelectedIndex((prev) => Math.min(prev + 1, filteredCommands.length - 1));
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                setSelectedIndex((prev) => Math.max(prev - 1, 0));
            } else if (e.key === 'Enter' && filteredCommands[selectedIndex]) {
                e.preventDefault();
                filteredCommands[selectedIndex].action();
            }
        },
        [filteredCommands, selectedIndex]
    );

    // Reset state when opened
    useEffect(() => {
        if (opened) {
            setQuery('');
            setSelectedIndex(0);
        }
    }, [opened]);

    // Reset selection when results change
    useEffect(() => {
        setSelectedIndex(0);
    }, [filteredCommands.length]);

    return (
        <Modal
            opened={opened}
            onClose={onClose}
            withCloseButton={false}
            padding={0}
            radius="lg"
            size="md"
            overlayProps={{ blur: 2, opacity: 0.3 }}
            classNames={{ body: styles.modalBody }}
        >
            <div className={styles.container}>
                <div className={styles.searchWrapper}>
                    <IconSearch size={18} className={styles.searchIcon} />
                    <TextInput
                        placeholder="Search commands..."
                        value={query}
                        onChange={(e) => setQuery(e.currentTarget.value)}
                        onKeyDown={handleKeyDown}
                        variant="unstyled"
                        classNames={{ input: styles.searchInput }}
                        autoFocus
                    />
                    <Kbd className={styles.escHint}>esc</Kbd>
                </div>

                <div className={styles.results}>
                    {filteredCommands.length === 0 ? (
                        <div className={styles.emptyState}>
                            <Text size="sm" c="dimmed">
                                No results found
                            </Text>
                        </div>
                    ) : (
                        <Stack gap={0}>
                            {filteredCommands.map((cmd, index) => (
                                <UnstyledButton
                                    key={cmd.id}
                                    className={styles.commandItem}
                                    data-selected={index === selectedIndex || undefined}
                                    onClick={cmd.action}
                                    onMouseEnter={() => setSelectedIndex(index)}
                                >
                                    <Group gap="sm" style={{ flex: 1 }}>
                                        <span className={styles.commandIcon}>{cmd.icon}</span>
                                        <div>
                                            <Text size="sm" fw={500}>
                                                {cmd.label}
                                            </Text>
                                            {cmd.description && (
                                                <Text size="xs" c="dimmed">
                                                    {cmd.description}
                                                </Text>
                                            )}
                                        </div>
                                    </Group>
                                    {cmd.shortcut && (
                                        <Text size="xs" c="dimmed" ff="monospace">
                                            {cmd.shortcut}
                                        </Text>
                                    )}
                                </UnstyledButton>
                            ))}
                        </Stack>
                    )}
                </div>
            </div>
        </Modal>
    );
}
