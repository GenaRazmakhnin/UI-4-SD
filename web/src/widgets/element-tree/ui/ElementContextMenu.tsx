import { Menu } from '@mantine/core';
import {
  IconEdit,
  IconPlus,
  IconCut,
  IconCheck,
  IconLock,
  IconCopy,
  IconExternalLink,
} from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';

interface ElementContextMenuProps {
  element: ElementNode;
  opened: boolean;
  x: number;
  y: number;
  onClose: () => void;
}

export function ElementContextMenu({
  element,
  opened,
  x,
  y,
  onClose,
}: ElementContextMenuProps) {
  const handleCopyPath = () => {
    navigator.clipboard.writeText(element.path);
    onClose();
  };

  return (
    <Menu opened={opened} onClose={onClose} position="right-start">
      <Menu.Target>
        <div style={{ position: 'absolute', left: x, top: y, width: 1, height: 1 }} />
      </Menu.Target>

      <Menu.Dropdown>
        <Menu.Label>Quick Actions</Menu.Label>

        <Menu.Item leftSection={<IconEdit size={16} />}>
          Edit Constraints
        </Menu.Item>

        <Menu.Item leftSection={<IconPlus size={16} />}>
          Add Extension
        </Menu.Item>

        {element.children.length > 0 && (
          <Menu.Item leftSection={<IconCut size={16} />}>
            Create Slicing
          </Menu.Item>
        )}

        <Menu.Divider />

        <Menu.Item leftSection={<IconCheck size={16} />}>
          {element.mustSupport ? 'Remove' : 'Set'} Must Support
        </Menu.Item>

        <Menu.Item leftSection={<IconLock size={16} />}>
          Set Fixed Value
        </Menu.Item>

        <Menu.Divider />

        <Menu.Item
          leftSection={<IconCopy size={16} />}
          onClick={handleCopyPath}
        >
          Copy Element Path
        </Menu.Item>

        <Menu.Item leftSection={<IconExternalLink size={16} />}>
          View Base Definition
        </Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
}
