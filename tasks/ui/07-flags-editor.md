# Task: Flags Editor Feature

## 📋 Description

Implement the flags editor feature for setting mustSupport, isModifier, and isSummary flags on elements with validation, warnings, and bulk operations. The editor provides clear explanations for each flag and prevents invalid combinations.

**Reference**: IMPLEMENTATION_PLAN.md Section 16.1.1 "Element Constraints Editing"

## 🎯 Context from Implementation Plan

This implements element flags editing with:
- **Flag Constraints** (16.1.1): MustSupport, IsModifier, IsSummary toggles
- **Live Validation** (16.1.3): Prevent invalid flag combinations
- **UX Design** (14.2): Clear explanations and warnings for each flag
- **Effector State** (17): Reactive state management for flag changes
- **FSD Architecture** (13): Feature-level component

## 📐 Requirements

### R1: Main FlagsEditor Component

**Complete Implementation**:
```typescript
// web/src/features/flags-editor/ui/FlagsEditor.tsx
import { Stack, Checkbox, TextInput, Alert, Group, Anchor } from '@mantine/core';
import { IconAlertCircle, IconExternalLink } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useState, useEffect } from 'react';
import type { ElementNode } from '@shared/types';
import { flagChanged } from '../model';
import styles from './FlagsEditor.module.css';

interface FlagsEditorProps {
  element: ElementNode;
}

export function FlagsEditor({ element }: FlagsEditorProps) {
  const [modifierReason, setModifierReason] = useState(element.isModifierReason || '');
  const [showModifierWarning, setShowModifierWarning] = useState(false);

  // Update local state when element changes
  useEffect(() => {
    setModifierReason(element.isModifierReason || '');
  }, [element.isModifierReason]);

  const handleMustSupportChange = (checked: boolean) => {
    flagChanged({
      elementId: element.id,
      flag: 'mustSupport',
      value: checked,
    });
  };

  const handleIsModifierChange = (checked: boolean) => {
    if (checked) {
      setShowModifierWarning(true);
    }

    flagChanged({
      elementId: element.id,
      flag: 'isModifier',
      value: checked,
      reason: checked ? modifierReason : undefined,
    });
  };

  const handleModifierReasonChange = (value: string) => {
    setModifierReason(value);
    if (element.isModifier) {
      flagChanged({
        elementId: element.id,
        flag: 'isModifierReason',
        value: true,
        reason: value,
      });
    }
  };

  const handleIsSummaryChange = (checked: boolean) => {
    flagChanged({
      elementId: element.id,
      flag: 'isSummary',
      value: checked,
    });
  };

  return (
    <Stack gap="md" className={styles.container}>
      {/* Must Support Flag */}
      <div>
        <Checkbox
          label={
            <Group gap="xs">
              <span>Must Support (MS)</span>
              <Anchor
                href="https://www.hl7.org/fhir/conformance-rules.html#mustSupport"
                target="_blank"
                size="xs"
              >
                <IconExternalLink size={14} />
              </Anchor>
            </Group>
          }
          description="This element must be supported by implementations claiming conformance to this profile"
          checked={element.mustSupport || false}
          onChange={(e) => handleMustSupportChange(e.currentTarget.checked)}
        />

        {element.mustSupport && element.min === 0 && (
          <Alert
            icon={<IconAlertCircle size={16} />}
            color="blue"
            variant="light"
            mt="xs"
          >
            MustSupport with min=0 means: systems must be capable of populating this element,
            but it's not required in every instance.
          </Alert>
        )}
      </div>

      {/* Is Modifier Flag */}
      <div>
        <Checkbox
          label={
            <Group gap="xs">
              <span>Is Modifier</span>
              <Anchor
                href="https://www.hl7.org/fhir/conformance-rules.html#isModifier"
                target="_blank"
                size="xs"
              >
                <IconExternalLink size={14} />
              </Anchor>
            </Group>
          }
          description="Changes the meaning or interpretation of the resource"
          checked={element.isModifier || false}
          onChange={(e) => handleIsModifierChange(e.currentTarget.checked)}
        />

        {showModifierWarning && element.isModifier && (
          <Alert
            icon={<IconAlertCircle size={16} />}
            color="red"
            variant="light"
            mt="xs"
            withCloseButton
            onClose={() => setShowModifierWarning(false)}
          >
            <strong>Warning:</strong> Setting isModifier changes the semantics of the resource.
            Only use for elements that truly modify the meaning (e.g., negation, status).
          </Alert>
        )}

        {element.isModifier && (
          <TextInput
            label="Modifier Reason"
            description="Explain why this element is a modifier"
            value={modifierReason}
            onChange={(e) => handleModifierReasonChange(e.currentTarget.value)}
            placeholder="e.g., This element negates the primary observation"
            required
            error={!modifierReason && 'Modifier reason is required'}
            mt="sm"
          />
        )}
      </div>

      {/* Is Summary Flag */}
      <div>
        <Checkbox
          label={
            <Group gap="xs">
              <span>Is Summary</span>
              <Anchor
                href="https://www.hl7.org/fhir/search.html#summary"
                target="_blank"
                size="xs"
              >
                <IconExternalLink size={14} />
              </Anchor>
            </Group>
          }
          description="Included in summary views of the resource (_summary=true)"
          checked={element.isSummary || false}
          onChange={(e) => handleIsSummaryChange(e.currentTarget.checked)}
        />
      </div>

      {/* Help Text */}
      <Alert color="gray" variant="light" icon={<IconAlertCircle size={16} />}>
        <strong>Tip:</strong> Use Must Support for elements critical to your use case.
        Is Modifier should only be set on elements that change the resource's meaning.
      </Alert>
    </Stack>
  );
}
```

**Styling**:
```css
/* web/src/features/flags-editor/ui/FlagsEditor.module.css */
.container {
  width: 100%;
}
```

### R2: Flag Descriptions Component

**Detailed Flag Information**:
```typescript
// web/src/features/flags-editor/ui/FlagDescription.tsx
import { Popover, Text, Stack, List, Button } from '@mantine/core';
import { IconInfoCircle } from '@tabler/icons-react';

interface FlagDescriptionProps {
  flag: 'mustSupport' | 'isModifier' | 'isSummary';
}

export function FlagDescription({ flag }: FlagDescriptionProps) {
  const descriptions = {
    mustSupport: {
      title: 'Must Support',
      description: 'Elements flagged as MustSupport must be supported by systems claiming conformance.',
      when: [
        'The element is critical to the use case',
        'Systems need to be able to populate it',
        'Systems need to be able to process it meaningfully',
      ],
      examples: [
        'Patient.identifier in US Core (required for patient matching)',
        'Observation.value in vital signs (the main data point)',
        'MedicationRequest.medication (what is being prescribed)',
      ],
      notes: [
        'MustSupport with min=0 means "capable of" not "required in every instance"',
        'Commonly used with cardinality constraints',
        'Different from required (min > 0)',
      ],
    },
    isModifier: {
      title: 'Is Modifier',
      description: 'Elements that change the meaning or interpretation of the resource.',
      when: [
        'The element negates the resource (e.g., Condition.verificationStatus)',
        'The element changes the interpretation (e.g., Observation.status)',
        'Missing the element would lead to misunderstanding',
      ],
      examples: [
        'Observation.status = "entered-in-error" (invalidates the observation)',
        'MedicationRequest.doNotPerform = true (reverses the meaning)',
        'AllergyIntolerance.verificationStatus = "refuted" (allergy doesn't exist)',
      ],
      notes: [
        'Requires isModifierReason to explain why',
        'Cannot be removed if base element is a modifier',
        'Use sparingly - most elements are NOT modifiers',
      ],
    },
    isSummary: {
      title: 'Is Summary',
      description: 'Elements included in summary views (_summary=true search parameter).',
      when: [
        'The element is useful for quick overview',
        'The element helps identify the resource',
        'The element is frequently needed without full resource',
      ],
      examples: [
        'Patient.name (needed for patient lists)',
        'Observation.code (what was observed)',
        'Condition.clinicalStatus (active vs inactive)',
      ],
      notes: [
        'Informational only - doesn't affect conformance',
        'Cannot be changed if base element defines it',
        'Typically set on key identifying/status elements',
      ],
    },
  };

  const info = descriptions[flag];

  return (
    <Popover width={400} position="right" withArrow shadow="md">
      <Popover.Target>
        <Button size="xs" variant="subtle" leftSection={<IconInfoCircle size={14} />}>
          Learn more
        </Button>
      </Popover.Target>
      <Popover.Dropdown>
        <Stack gap="sm">
          <div>
            <Text size="sm" fw={600}>{info.title}</Text>
            <Text size="xs" c="dimmed">{info.description}</Text>
          </div>

          <div>
            <Text size="xs" fw={500} mb={4}>When to use:</Text>
            <List size="xs" spacing={2}>
              {info.when.map((item, i) => (
                <List.Item key={i}>{item}</List.Item>
              ))}
            </List>
          </div>

          <div>
            <Text size="xs" fw={500} mb={4}>Examples:</Text>
            <List size="xs" spacing={2}>
              {info.examples.map((item, i) => (
                <List.Item key={i}>{item}</List.Item>
              ))}
            </List>
          </div>

          <div>
            <Text size="xs" fw={500} mb={4}>Important notes:</Text>
            <List size="xs" spacing={2}>
              {info.notes.map((item, i) => (
                <List.Item key={i}>{item}</List.Item>
              ))}
            </List>
          </div>
        </Stack>
      </Popover.Dropdown>
    </Popover>
  );
}
```

### R3: Bulk Operations Component

**Bulk Flag Operations**:
```typescript
// web/src/features/flags-editor/ui/BulkFlagOperations.tsx
import { Stack, Button, Group, Text, Modal, Radio } from '@mantine/core';
import { IconWand, IconTrash } from '@tabler/icons-react';
import { useState } from 'react';
import { useUnit } from 'effector-react';
import { bulkMustSupportSet, bulkMustSupportClear } from '../model';

export function BulkFlagOperations() {
  const [opened, setOpened] = useState(false);
  const [condition, setCondition] = useState<'required' | 'modified' | 'all'>('required');

  const handleApply = () => {
    bulkMustSupportSet({ condition });
    setOpened(false);
  };

  const handleClear = () => {
    if (confirm('Clear Must Support from all elements? This cannot be undone.')) {
      bulkMustSupportClear();
    }
  };

  return (
    <>
      <Stack gap="sm">
        <Text size="sm" fw={500}>Bulk Operations</Text>

        <Group>
          <Button
            size="xs"
            variant="light"
            leftSection={<IconWand size={14} />}
            onClick={() => setOpened(true)}
          >
            Set Must Support
          </Button>

          <Button
            size="xs"
            variant="light"
            color="red"
            leftSection={<IconTrash size={14} />}
            onClick={handleClear}
          >
            Clear All MS
          </Button>
        </Group>
      </Stack>

      {/* Bulk Set Modal */}
      <Modal
        opened={opened}
        onClose={() => setOpened(false)}
        title="Bulk Set Must Support"
      >
        <Stack gap="md">
          <Text size="sm">
            Set Must Support flag on elements matching:
          </Text>

          <Radio.Group value={condition} onChange={(value) => setCondition(value as any)}>
            <Stack gap="xs">
              <Radio
                value="required"
                label="Required elements (min ≥ 1)"
                description="Set MS on all elements with minimum cardinality of 1 or greater"
              />
              <Radio
                value="modified"
                label="Modified elements"
                description="Set MS on all elements that have been constrained from the base"
              />
              <Radio
                value="all"
                label="All elements"
                description="Set MS on every element in the profile"
              />
            </Stack>
          </Radio.Group>

          <Group justify="flex-end">
            <Button variant="subtle" onClick={() => setOpened(false)}>
              Cancel
            </Button>
            <Button onClick={handleApply}>
              Apply
            </Button>
          </Group>
        </Stack>
      </Modal>
    </>
  );
}
```

### R4: Validation Logic

**Flag Validation Rules**:
```typescript
// web/src/features/flags-editor/lib/validation.ts
import type { ElementNode } from '@shared/types';

export interface FlagValidation {
  isValid: boolean;
  errors: string[];
  warnings: string[];
}

/**
 * Validate flag changes
 */
export function validateFlags(element: ElementNode, updates: Partial<ElementNode>): FlagValidation {
  const errors: string[] = [];
  const warnings: string[] = [];

  // IsModifier validation
  if (updates.isModifier !== undefined) {
    if (updates.isModifier && !updates.isModifierReason && !element.isModifierReason) {
      errors.push('Modifier reason is required when setting isModifier flag');
    }

    if (!updates.isModifier && element.isModifier && isModifierInBase(element)) {
      errors.push('Cannot remove isModifier flag - it is defined in the base element');
    }

    if (updates.isModifier && !isModifierInBase(element)) {
      warnings.push('Adding isModifier flag changes the semantics of the resource. Use with caution.');
    }
  }

  // MustSupport validation
  if (updates.mustSupport !== undefined) {
    if (updates.mustSupport && element.min === 0) {
      warnings.push(
        'Setting MustSupport on optional element (min=0). ' +
        'This means systems must be capable of handling it, but it is not required in every instance.'
      );
    }

    if (updates.mustSupport && !element.isModified) {
      warnings.push(
        'Setting MustSupport without constraining the element is unusual. ' +
        'Consider adding cardinality or type constraints.'
      );
    }
  }

  // IsSummary validation
  if (updates.isSummary !== undefined) {
    if (updates.isSummary !== element.isSummary && isSummaryInBase(element)) {
      errors.push('Cannot change isSummary flag - it is defined in the base element');
    }
  }

  return {
    isValid: errors.length === 0,
    errors,
    warnings,
  };
}

/**
 * Check if isModifier is defined in base element
 */
function isModifierInBase(element: ElementNode): boolean {
  // TODO: Check against base definition
  // For now, assume false (will be implemented when base comparison is available)
  return false;
}

/**
 * Check if isSummary is defined in base element
 */
function isSummaryInBase(element: ElementNode): boolean {
  // TODO: Check against base definition
  // For now, assume false (will be implemented when base comparison is available)
  return false;
}

/**
 * Check if setting MS without constraints is suspicious
 */
export function isSuspiciousMustSupport(element: ElementNode): boolean {
  return (
    element.mustSupport === true &&
    element.min === 0 &&
    !element.isModified
  );
}

/**
 * Get recommended flags based on element state
 */
export function getRecommendedFlags(element: ElementNode): Partial<ElementNode> {
  const recommendations: Partial<ElementNode> = {};

  // Recommend MS for required elements
  if (element.min >= 1 && !element.mustSupport) {
    recommendations.mustSupport = true;
  }

  return recommendations;
}
```

### R5: Effector State Management

**Complete State Model**:
```typescript
// web/src/features/flags-editor/model/index.ts
import { createEvent, createEffect, sample } from 'effector';
import { api } from '@shared/api';
import { $currentProfile, profileUpdated } from '@entities/profile/model';
import { validateFlags } from '../lib/validation';
import type { ElementNode } from '@shared/types';

/**
 * Flag changed event
 */
export const flagChanged = createEvent<{
  elementId: string;
  flag: 'mustSupport' | 'isModifier' | 'isSummary' | 'isModifierReason';
  value: boolean | string;
  reason?: string;
}>();

/**
 * Bulk must support operations
 */
export const bulkMustSupportSet = createEvent<{
  condition: 'required' | 'modified' | 'all';
}>();

export const bulkMustSupportClear = createEvent<void>();

/**
 * Update element flag effect
 */
const updateFlagFx = createEffect(async ({
  profileId,
  elementId,
  updates,
}: {
  profileId: string;
  elementId: string;
  updates: Partial<ElementNode>;
}) => {
  return await api.profiles.updateElement(profileId, elementId, updates);
});

/**
 * Bulk update must support effect
 */
const bulkUpdateMustSupportFx = createEffect(async ({
  profileId,
  elements,
}: {
  profileId: string;
  elements: { id: string; mustSupport: boolean }[];
}) => {
  // Update all elements in parallel
  await Promise.all(
    elements.map(({ id, mustSupport }) =>
      api.profiles.updateElement(profileId, id, { mustSupport })
    )
  );

  // Fetch updated profile
  return await api.profiles.get(profileId);
});

/**
 * Handle flag changes
 */
sample({
  clock: flagChanged,
  source: $currentProfile,
  filter: (profile) => profile !== null,
  fn: (profile, { elementId, flag, value, reason }) => {
    const updates: Partial<ElementNode> = {};

    if (flag === 'isModifierReason') {
      updates.isModifierReason = reason;
    } else {
      updates[flag] = value as boolean;
      if (flag === 'isModifier' && value) {
        updates.isModifierReason = reason || '';
      }
    }

    return {
      profileId: profile!.id,
      elementId,
      updates,
    };
  },
  target: updateFlagFx,
});

/**
 * Handle bulk must support set
 */
sample({
  clock: bulkMustSupportSet,
  source: $currentProfile,
  filter: (profile) => profile !== null,
  fn: (profile, { condition }) => {
    const elements: { id: string; mustSupport: boolean }[] = [];

    // Recursively process all elements
    const processElement = (element: ElementNode) => {
      let shouldSet = false;

      switch (condition) {
        case 'required':
          shouldSet = element.min >= 1;
          break;
        case 'modified':
          shouldSet = element.isModified;
          break;
        case 'all':
          shouldSet = true;
          break;
      }

      if (shouldSet) {
        elements.push({ id: element.id, mustSupport: true });
      }

      element.children.forEach(processElement);
    };

    profile!.elements.forEach(processElement);

    return {
      profileId: profile!.id,
      elements,
    };
  },
  target: bulkUpdateMustSupportFx,
});

/**
 * Handle bulk must support clear
 */
sample({
  clock: bulkMustSupportClear,
  source: $currentProfile,
  filter: (profile) => profile !== null,
  fn: (profile) => {
    const elements: { id: string; mustSupport: boolean }[] = [];

    // Recursively process all elements
    const processElement = (element: ElementNode) => {
      if (element.mustSupport) {
        elements.push({ id: element.id, mustSupport: false });
      }
      element.children.forEach(processElement);
    };

    profile!.elements.forEach(processElement);

    return {
      profileId: profile!.id,
      elements,
    };
  },
  target: bulkUpdateMustSupportFx,
});

/**
 * Update profile after flag changes
 */
sample({
  clock: updateFlagFx.doneData,
  target: profileUpdated,
});

sample({
  clock: bulkUpdateMustSupportFx.doneData,
  target: profileUpdated,
});
```

### R6: Visual Indicators

**Flag Badges for Element Tree**:
```typescript
// web/src/widgets/element-tree/ui/FlagBadges.tsx
import { Group, Badge, Tooltip } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';

interface FlagBadgesProps {
  element: ElementNode;
  compact?: boolean;
}

export function FlagBadges({ element, compact = false }: FlagBadgesProps) {
  if (!element.mustSupport && !element.isModifier && !element.isSummary) {
    return null;
  }

  return (
    <Group gap={4}>
      {element.mustSupport && (
        <Tooltip label="Must Support">
          <Badge size={compact ? 'xs' : 'sm'} variant="light" color="blue">
            MS
          </Badge>
        </Tooltip>
      )}

      {element.isModifier && (
        <Tooltip label={`Modifier: ${element.isModifierReason || 'No reason provided'}`}>
          <Badge size={compact ? 'xs' : 'sm'} variant="light" color="red">
            <Group gap={2}>
              <IconAlertCircle size={12} />
              <span>Modifier</span>
            </Group>
          </Badge>
        </Tooltip>
      )}

      {element.isSummary && (
        <Tooltip label="Included in summary view">
          <Badge size={compact ? 'xs' : 'sm'} variant="light" color="gray">
            Σ
          </Badge>
        </Tooltip>
      )}
    </Group>
  );
}
```

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] All flag checkboxes render and toggle correctly
- [ ] IsModifierReason field appears/hides based on isModifier state
- [ ] IsModifierReason field is required when isModifier is true
- [ ] Flag changes persist immediately via API
- [ ] Optimistic UI updates work (checkboxes respond immediately)
- [ ] Validation prevents invalid flag combinations
- [ ] Warnings display for potentially problematic changes
- [ ] Flag badges appear in element tree
- [ ] Bulk "Set MS on required elements" works correctly
- [ ] Bulk "Set MS on modified elements" works correctly
- [ ] Bulk "Clear all MS" works with confirmation
- [ ] Links to FHIR spec open in new tabs

### User Experience Requirements
- [ ] Clear descriptions for each flag
- [ ] Tooltips explain flag meaning
- [ ] Warning messages are helpful and actionable
- [ ] Bulk operations modal is clear and safe
- [ ] Visual distinction between flag types (colors)
- [ ] Inherited vs profile-defined flags are distinguishable
- [ ] Keyboard navigation works for all controls
- [ ] Focus management is logical

### Performance Requirements
- [ ] Flag updates complete in <100ms
- [ ] Bulk operations complete in <2s for 100+ elements
- [ ] No UI blocking during bulk operations
- [ ] Optimistic updates prevent perceived lag

### Validation Requirements
- [ ] Cannot remove isModifier if defined in base
- [ ] Cannot change isSummary if defined in base
- [ ] IsModifierReason required when isModifier = true
- [ ] Warnings shown for suspicious MS patterns
- [ ] Validation errors prevent invalid states

## 🔗 Dependencies

### Required Tasks
- **UI 03**: Mock Data Layer - Provides mock API for development
- **UI 05**: Inspector Panel - Hosts the FlagsEditor component

### Optional Integration
- **UI 04**: Element Tree Viewer - Displays flag badges
- **Backend 06**: Profile API Endpoints - Updates element flags

### Integration Points
- **Profile Model**: Reads current element state
- **Validation Model**: Checks flag validity
- **Element Tree**: Shows flag visual indicators

## 📚 API Contract

**Update Element Flags**:
```typescript
PATCH /api/profiles/:profileId/elements/:elementPath
Body: {
  mustSupport?: boolean;
  isModifier?: boolean;
  isModifierReason?: string;
  isSummary?: boolean;
}
Response: Profile (updated)
```

**Bulk Update Flags** (future enhancement):
```typescript
POST /api/profiles/:profileId/bulk-update-flags
Body: {
  updates: Array<{
    elementPath: string;
    mustSupport?: boolean;
  }>;
}
Response: Profile (updated)
```

## 📖 Related Documentation

- **IMPLEMENTATION_PLAN.md Section 16.1.1**: Element Constraints Editing
- **IMPLEMENTATION_PLAN.md Section 14.2**: Low-Code UX Design
- **IMPLEMENTATION_PLAN.md Section 17**: UI State Model (Effector)
- **FHIR MustSupport**: https://www.hl7.org/fhir/conformance-rules.html#mustSupport
- **FHIR IsModifier**: https://www.hl7.org/fhir/conformance-rules.html#isModifier
- **FHIR Summary**: https://www.hl7.org/fhir/search.html#summary

## 🎨 Priority

🔴 **Critical** - MVP feature for profile editing

## ⏱️ Estimated Complexity

**Low-Medium** - 3-5 days (24-40 hours)

### Breakdown:
- FlagsEditor component: 6 hours
- FlagDescription component: 4 hours
- BulkFlagOperations component: 6 hours
- Validation logic: 6 hours
- Effector model integration: 8 hours
- Flag badges integration: 4 hours
- Documentation: 2 hours
