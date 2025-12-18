import { Alert, Badge, Button, Checkbox, Group, Stack, Text } from '@mantine/core';
import type { ElementNode, TypeConstraint } from '@shared/types';
import { IconAlertCircle, IconSearch, IconX } from '@tabler/icons-react';
import { useState } from 'react';
import { targetProfileAdded, targetProfileRemoved, typeConstraintChanged } from '../model';
import { ProfileSearchModal } from './ProfileSearchModal';
import styles from './TypeConstraintEditor.module.css';

interface TypeConstraintEditorProps {
  element: ElementNode;
}

export function TypeConstraintEditor({ element }: TypeConstraintEditorProps) {
  const [searchModalOpen, setSearchModalOpen] = useState(false);
  const [searchForType, setSearchForType] = useState<string | null>(null);

  // Get base allowed types (these are the types allowed in the base definition)
  const baseTypes = getBaseTypes(element);

  // Current type constraints
  const currentTypes = element.type || baseTypes;

  // Handle type selection/deselection
  const handleTypeToggle = (typeCode: string, checked: boolean) => {
    if (checked) {
      // Add type
      typeConstraintChanged({
        elementId: element.id,
        add: [{ code: typeCode }],
        remove: [],
      });
    } else {
      // Remove type (but prevent removing all types)
      if (currentTypes.length > 1) {
        typeConstraintChanged({
          elementId: element.id,
          add: [],
          remove: [typeCode],
        });
      }
    }
  };

  // Open profile search for specific type
  const handleSearchProfiles = (typeCode: string) => {
    setSearchForType(typeCode);
    setSearchModalOpen(true);
  };

  // Add target profile to type
  const handleProfileSelected = (profileUrl: string) => {
    if (searchForType) {
      targetProfileAdded({
        elementId: element.id,
        typeCode: searchForType,
        profileUrl,
      });
    }
    setSearchModalOpen(false);
    setSearchForType(null);
  };

  // Remove target profile
  const handleRemoveProfile = (typeCode: string, profileUrl: string) => {
    targetProfileRemoved({
      elementId: element.id,
      typeCode,
      profileUrl,
    });
  };

  return (
    <Stack gap="md" className={styles.container}>
      {/* Base Types Info */}
      <Alert color="blue" variant="light" icon={<IconAlertCircle size={16} />}>
        <Text size="xs">
          <strong>Base allowed types:</strong> {baseTypes.map((t) => t.code).join(', ')}
        </Text>
      </Alert>

      {/* Type Selection */}
      <div>
        <Text size="sm" fw={500} mb="sm">
          Allowed Types
        </Text>

        <Stack gap="sm">
          {baseTypes.map((baseType) => {
            const isSelected = currentTypes.some((t) => t.code === baseType.code);
            const selectedType = currentTypes.find((t) => t.code === baseType.code);
            const hasProfiles = selectedType?.profile && selectedType.profile.length > 0;

            return (
              <div key={baseType.code} className={styles.typeRow}>
                {/* Type Checkbox */}
                <Checkbox
                  label={
                    <Group gap="xs">
                      <span>{baseType.code}</span>
                      {isSelected && hasProfiles && (
                        <Badge size="xs" variant="light">
                          {selectedType.profile!.length} profile(s)
                        </Badge>
                      )}
                    </Group>
                  }
                  checked={isSelected}
                  onChange={(e) => handleTypeToggle(baseType.code, e.currentTarget.checked)}
                  disabled={currentTypes.length === 1 && isSelected}
                />

                {/* Target Profiles */}
                {isSelected && (
                  <Stack gap="xs" ml="xl" mt="xs">
                    {/* Existing Profiles */}
                    {selectedType?.profile?.map((profileUrl, idx) => (
                      <Group key={idx} justify="space-between" className={styles.profileRow}>
                        <Text size="xs" className={styles.profileUrl}>
                          {profileUrl}
                        </Text>
                        <Button
                          size="xs"
                          variant="subtle"
                          color="red"
                          onClick={() => handleRemoveProfile(baseType.code, profileUrl)}
                        >
                          <IconX size={14} />
                        </Button>
                      </Group>
                    ))}

                    {/* Add Profile Button */}
                    <Button
                      size="xs"
                      variant="light"
                      leftSection={<IconSearch size={14} />}
                      onClick={() => handleSearchProfiles(baseType.code)}
                    >
                      {hasProfiles ? 'Add Another Profile' : 'Set Target Profile'}
                    </Button>
                  </Stack>
                )}
              </div>
            );
          })}
        </Stack>
      </div>

      {/* Warnings */}
      {currentTypes.length === 1 && (
        <Alert color="yellow" variant="light" icon={<IconAlertCircle size={16} />}>
          Only one type allowed. You cannot remove the last type from an element.
        </Alert>
      )}

      {currentTypes.length < baseTypes.length && (
        <Alert color="blue" variant="light">
          <Text size="xs">
            <strong>Note:</strong> You've constrained the allowed types from the base definition.
            This is a valid constraint that narrows down the possibilities.
          </Text>
        </Alert>
      )}

      {/* Profile Search Modal */}
      <ProfileSearchModal
        opened={searchModalOpen}
        onClose={() => {
          setSearchModalOpen(false);
          setSearchForType(null);
        }}
        typeFilter={searchForType}
        onProfileSelected={handleProfileSelected}
      />
    </Stack>
  );
}

/**
 * Get base allowed types for element
 * In real implementation, this would come from base definition
 */
function getBaseTypes(element: ElementNode): TypeConstraint[] {
  // If element already has types, use those as base
  // Otherwise, return common base types based on element path
  if (element.type && element.type.length > 0) {
    return element.type;
  }

  // Default fallback (would be replaced with actual base definition lookup)
  return [{ code: 'string' }];
}
