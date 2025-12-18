import { ActionIcon, Loader, TextInput, type TextInputProps } from '@mantine/core';
import { useDebouncedCallback } from '@mantine/hooks';
import { IconSearch, IconX } from '@tabler/icons-react';
import { useEffect, useRef, useState } from 'react';
import styles from './SearchInput.module.css';

export interface SearchInputProps extends Omit<TextInputProps, 'onChange'> {
  /**
   * Callback when search value changes (debounced)
   */
  onSearch?: (value: string) => void;

  /**
   * Debounce delay in milliseconds
   * @default 300
   */
  debounceMs?: number;

  /**
   * Whether to show loading indicator
   */
  loading?: boolean;

  /**
   * Initial value
   */
  initialValue?: string;

  /**
   * Whether to enable Ctrl+K keyboard shortcut to focus
   * @default true
   */
  enableShortcut?: boolean;

  /**
   * Callback when Enter key is pressed
   */
  onEnter?: (value: string) => void;
}

export function SearchInput({
  onSearch,
  debounceMs = 300,
  loading = false,
  initialValue = '',
  enableShortcut = true,
  onEnter,
  placeholder = 'Search...',
  ...props
}: SearchInputProps) {
  const [value, setValue] = useState(initialValue);
  const inputRef = useRef<HTMLInputElement>(null);

  // Debounced search callback
  const debouncedSearch = useDebouncedCallback((searchValue: string) => {
    onSearch?.(searchValue);
  }, debounceMs);

  // Handle value change
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = event.currentTarget.value;
    setValue(newValue);
    debouncedSearch(newValue);
  };

  // Handle clear
  const handleClear = () => {
    setValue('');
    onSearch?.('');
    inputRef.current?.focus();
  };

  // Handle Enter key
  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      onEnter?.(value);
      // Also trigger immediate search on Enter (bypass debounce)
      onSearch?.(value);
    }
  };

  // Keyboard shortcut (Ctrl+K or Cmd+K)
  useEffect(() => {
    if (!enableShortcut) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key === 'k') {
        event.preventDefault();
        inputRef.current?.focus();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [enableShortcut]);

  // Right section: loading indicator or clear button
  const rightSection = loading ? (
    <Loader size="xs" />
  ) : value ? (
    <ActionIcon
      variant="subtle"
      color="gray"
      size="sm"
      onClick={handleClear}
      aria-label="Clear search"
    >
      <IconX size={16} />
    </ActionIcon>
  ) : null;

  return (
    <TextInput
      ref={inputRef}
      value={value}
      onChange={handleChange}
      onKeyDown={handleKeyDown}
      placeholder={placeholder}
      leftSection={<IconSearch size={16} />}
      rightSection={rightSection}
      className={styles.input}
      {...props}
    />
  );
}
