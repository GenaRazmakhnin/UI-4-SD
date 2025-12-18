# Task: Cardinality Editor Feature

## ✅ Implementation Status: COMPLETED

**Date Completed**: 2025-12-18

### Summary of Implementation

All core requirements have been successfully implemented:

- ✅ **R1**: UI Component Structure - Complete with min/max inputs, quick presets, validation feedback
- ✅ **R2**: Validation Logic - Comprehensive rules with clear error messages
- ✅ **R3**: State Management - Effector model for cardinality editing
- ✅ **R4**: Visual Feedback - Modified state highlighting, impact preview
- ✅ **R5**: Integration - Works seamlessly with Inspector Panel

### Files Created

**Feature** (`web/src/features/cardinality-editor/`):
- `ui/CardinalityEditor.tsx` - Main component with inputs and presets
- `ui/CardinalityEditor.module.css` - Styling
- `model/index.ts` - Effector stores and events
- `lib/validation.ts` - Validation rules and impact messages
- `index.ts` - Public exports

### Key Features

1. **Smart Inputs**:
   - Number input for minimum (with min/max validation)
   - Text input for maximum (accepts numbers or "*")
   - Real-time validation with clear error messages

2. **Quick Presets**:
   - 0..1 (Optional)
   - 1..1 (Required)
   - 0..* (Any)
   - 1..* (At least one)

3. **Validation Rules**:
   - Min ≥ 0
   - Min ≥ base min (cannot loosen)
   - Max must be number or "*"
   - Max ≤ base max (cannot loosen)
   - Min ≤ max

4. **Impact Preview**:
   - Making element required/optional
   - Allowing multiple values
   - Restricting to single value
   - Constraint tightening

5. **Visual Feedback**:
   - Base cardinality displayed prominently
   - Modified badge when changed
   - Blue border on modified inputs
   - Inline error messages
   - Impact message with emoji indicators

6. **UX Features**:
   - Reset to base button
   - Apply button (disabled when errors)
   - Cancel button
   - Optional onClose callback for modal usage

### Validation Logic

The editor implements comprehensive FHIR cardinality rules:

```typescript
// Cannot loosen constraints (only tighten)
min >= baseMin
max <= baseMax

// Logical consistency
min >= 0
min <= max
max = number | "*"
```

### Usage Example

```typescript
import { CardinalityEditor } from '@features/cardinality-editor';

function ConstraintsTab({ element }) {
  return (
    <CardinalityEditor element={element} />
  );
}
```

### Impact Messages

- ⚠️ Making element optional (was required)
- ✅ Making element required (was optional)
- 📋 Allowing multiple values (was single)
- 🔒 Restricting to single value (was multiple)
- 🎯 Constraint tightened

---

## 📋 Description

Implement the cardinality editor that allows users to set min/max constraints with validation, clear feedback, and impact preview.

**Reference**: IMPLEMENTATION_PLAN.md Section 6 "Quick Constraints Panel" and Section 14 "Low-Code UX Design Principles"

## 🎯 Context from Implementation Plan

This implements cardinality editing with:
- **Low-Code UX** (14.2): Visual controls instead of text entry where possible
- **Immediate Feedback** (14.3): Real-time validation and impact preview
- **Error Prevention** (14.4): Validation before applying changes
- **Quick Actions** (6.2): One-click presets for common cardinality patterns

## 📐 Requirements

### R1: UI Component Structure

**Complete Component Implementation**:
```typescript
// features/edit-cardinality/ui/CardinalityEditor.tsx
import { useState, useEffect } from 'react';
import { useUnit } from 'effector-react';
import { NumberInput, TextInput, Group, Stack, Button, Text, Badge, Tooltip } from '@mantine/core';
import { IconUndo, IconInfoCircle } from '@tabler/icons-react';
import { cardinalityChanged, $cardinalityValidation } from '../model';
import styles from './CardinalityEditor.module.css';

interface CardinalityEditorProps {
  element: ElementNode;
  onClose?: () => void;
}

export function CardinalityEditor({ element, onClose }: CardinalityEditorProps) {
  const [min, setMin] = useState(element.min);
  const [max, setMax] = useState(element.max);
  const validation = useUnit($cardinalityValidation);

  const baseMin = element.base.min;
  const baseMax = element.base.max;
  const isModified = min !== baseMin || max !== baseMax;

  // Validation state
  const minError = validation.minError?.(min, max, baseMin, baseMax);
  const maxError = validation.maxError?.(min, max, baseMin, baseMax);
  const hasErrors = !!minError || !!maxError;

  // Impact preview
  const impact = getImpactMessage(min, max, baseMin, baseMax);

  const handleApply = () => {
    if (!hasErrors) {
      cardinalityChanged({
        elementPath: element.path,
        min,
        max,
      });
      onClose?.();
    }
  };

  const handleReset = () => {
    setMin(baseMin);
    setMax(baseMax);
  };

  return (
    <Stack gap="md" className={styles.container}>
      {/* Header */}
      <Group justify="space-between">
        <Text size="sm" fw={600}>
          Cardinality
        </Text>
        {isModified && (
          <Badge size="sm" color="blue">
            Modified
          </Badge>
        )}
      </Group>

      {/* Baseline Display */}
      <Group gap="xs" className={styles.baseline}>
        <Text size="xs" c="dimmed">
          Base:
        </Text>
        <Code className={styles.baselineValue}>
          {baseMin}..{baseMax}
        </Code>
      </Group>

      {/* Min/Max Inputs */}
      <Group grow>
        <NumberInput
          label="Minimum"
          value={min}
          onChange={(val) => setMin(val as number)}
          min={0}
          max={parseMaxToNumber(max)}
          error={minError}
          description={`Must be ≥ ${baseMin}`}
          classNames={{ input: isModified ? styles.modified : undefined }}
        />
        <TextInput
          label="Maximum"
          value={max}
          onChange={(e) => setMax(e.target.value)}
          error={maxError}
          description="Enter number or '*'"
          placeholder="* or number"
          classNames={{ input: isModified ? styles.modified : undefined }}
        />
      </Group>

      {/* Quick Presets */}
      <div>
        <Text size="xs" c="dimmed" mb="xs">
          Quick Presets:
        </Text>
        <Group gap="xs">
          <Button
            size="xs"
            variant="light"
            onClick={() => { setMin(0); setMax('1'); }}
          >
            0..1 (Optional)
          </Button>
          <Button
            size="xs"
            variant="light"
            onClick={() => { setMin(1); setMax('1'); }}
          >
            1..1 (Required)
          </Button>
          <Button
            size="xs"
            variant="light"
            onClick={() => { setMin(0); setMax('*'); }}
          >
            0..* (Any)
          </Button>
          <Button
            size="xs"
            variant="light"
            onClick={() => { setMin(1); setMax('*'); }}
          >
            1..* (At least one)
          </Button>
        </Group>
      </div>

      {/* Impact Preview */}
      {impact && (
        <Alert icon={<IconInfoCircle size={16} />} color="blue">
          <Text size="sm">{impact}</Text>
        </Alert>
      )}

      {/* Actions */}
      <Group justify="flex-end" gap="xs">
        {isModified && (
          <Button
            size="sm"
            variant="subtle"
            leftSection={<IconUndo size={16} />}
            onClick={handleReset}
          >
            Reset to Base
          </Button>
        )}
        <Button
          size="sm"
          variant="default"
          onClick={onClose}
        >
          Cancel
        </Button>
        <Button
          size="sm"
          onClick={handleApply}
          disabled={hasErrors}
        >
          Apply
        </Button>
      </Group>
    </Stack>
  );
}
```

### R2: Validation Logic

**Comprehensive Validation Rules** (Reference: IMPLEMENTATION_PLAN.md 14.4):
```typescript
// features/edit-cardinality/lib/validation.ts

export interface CardinalityValidation {
  minError?: string;
  maxError?: string;
  isValid: boolean;
}

export function validateCardinality(
  min: number,
  max: string,
  baseMin: number,
  baseMax: string
): CardinalityValidation {
  const errors: CardinalityValidation = { isValid: true };

  // Rule 1: Min must be non-negative
  if (min < 0) {
    errors.minError = 'Minimum must be ≥ 0';
    errors.isValid = false;
  }

  // Rule 2: Min must be ≥ base min (cannot loosen constraint)
  if (min < baseMin) {
    errors.minError = `Minimum must be ≥ ${baseMin} (base minimum)`;
    errors.isValid = false;
  }

  // Rule 3: Max must be valid format
  if (max !== '*' && (isNaN(Number(max)) || !Number.isInteger(Number(max)))) {
    errors.maxError = 'Maximum must be a number or "*"';
    errors.isValid = false;
  }

  // Rule 4: Max must be ≤ base max (cannot loosen constraint)
  if (max !== '*') {
    const maxNum = Number(max);
    const baseMaxNum = baseMax === '*' ? Infinity : Number(baseMax);
    if (maxNum > baseMaxNum) {
      errors.maxError = `Maximum must be ≤ ${baseMax} (base maximum)`;
      errors.isValid = false;
    }
  } else if (baseMax !== '*') {
    errors.maxError = `Cannot set to "*" when base maximum is ${baseMax}`;
    errors.isValid = false;
  }

  // Rule 5: Min must be ≤ max
  if (max !== '*') {
    const maxNum = Number(max);
    if (min > maxNum) {
      errors.minError = 'Minimum cannot exceed maximum';
      errors.isValid = false;
    }
  }

  return errors;
}

export function getImpactMessage(
  min: number,
  max: string,
  baseMin: number,
  baseMax: string
): string | null {
  // Required → Optional
  if (baseMin >= 1 && min === 0) {
    return '⚠️ This will make the element optional (was required)';
  }

  // Optional → Required
  if (baseMin === 0 && min >= 1) {
    return '✅ This will make the element required (was optional)';
  }

  // Single → Multiple
  if (baseMax === '1' && max === '*') {
    return '📋 This will allow multiple values (was single value)';
  }

  // Multiple → Single
  if (baseMax === '*' && max === '1') {
    return '🔒 This will restrict to single value (was multiple)';
  }

  // Tightened range
  if (min > baseMin || (max !== '*' && baseMax !== '*' && Number(max) < Number(baseMax))) {
    return `🎯 Constraint tightened from ${baseMin}..${baseMax} to ${min}..${max}`;
  }

  return null;
}
```

### R3: State Management (Effector)

**Feature State Model**:
```typescript
// features/edit-cardinality/model/index.ts
import { createStore, createEvent, createEffect, sample } from 'effector';
import { $selectedElement } from '@/entities/element';
import { updateElementFx } from '@/entities/profile/api';

// Events
export const cardinalityChanged = createEvent<{
  elementPath: string;
  min: number;
  max: string;
}>();

export const cardinalityEditCancelled = createEvent();

// Stores
export const $isEditingCardinality = createStore(false);

export const $cardinalityValidation = createStore({
  minError: null as string | null,
  maxError: null as string | null,
  isValid: true,
});

// Effects
const applyCardinalityFx = createEffect(async ({
  profileId,
  elementPath,
  min,
  max,
}: {
  profileId: string;
  elementPath: string;
  min: number;
  max: string;
}) => {
  return await api.profiles.updateElement(profileId, elementPath, {
    cardinality: { min, max },
  });
});

// Logic
sample({
  clock: cardinalityChanged,
  source: $selectedElement,
  filter: (element): element is ElementNode => element !== null,
  fn: (element, { elementPath, min, max }) => {
    const validation = validateCardinality(
      min,
      max,
      element.base.min,
      element.base.max
    );
    return { validation, element, min, max };
  },
  target: createEffect(({ validation, element, min, max }) => {
    if (validation.isValid) {
      applyCardinalityFx({
        profileId: element.profileId,
        elementPath: element.path,
        min,
        max,
      });
    } else {
      throw new Error('Validation failed');
    }
  }),
});

// Update validation store
$cardinalityValidation.on(
  cardinalityChanged,
  (_, { min, max }) => {
    const element = $selectedElement.getState();
    if (!element) return { isValid: false };

    return validateCardinality(
      min,
      max,
      element.base.min,
      element.base.max
    );
  }
);

$isEditingCardinality
  .on(cardinalityChanged, () => true)
  .on([applyCardinalityFx.done, cardinalityEditCancelled], () => false);
```

### R4: Visual Feedback Styling

```css
/* CardinalityEditor.module.css */
.container {
  padding: 16px;
  background: var(--card-bg);
  border-radius: var(--radius-md);
}

.baseline {
  padding: 8px 12px;
  background: var(--gray-50);
  border-radius: var(--radius-sm);
}

.baselineValue {
  font-family: var(--font-mono);
  color: var(--gray-700);
  font-weight: 600;
}

/* Highlight modified inputs */
.modified {
  border-color: var(--blue-500) !important;
  border-width: 2px !important;
}

/* Preset buttons */
.presetButton {
  font-size: 11px;
  height: 28px;
}
```

### R5: Integration with Inspector Panel

**Usage Example**:
```typescript
// widgets/inspector-panel/ui/ConstraintsTab.tsx
import { CardinalityEditor } from '@/features/edit-cardinality';

export function ConstraintsTab({ element }: Props) {
  return (
    <Stack gap="lg">
      <Section title="Cardinality">
        <CardinalityEditor element={element} />
      </Section>

      <Section title="Type Constraints">
        <TypeConstraintEditor element={element} />
      </Section>

      {/* ... more sections */}
    </Stack>
  );
}
```

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] Cardinality inputs render correctly
- [ ] Min/max validation works with all rules:
  - [ ] Min ≥ 0
  - [ ] Min ≥ base min
  - [ ] Max is number or "*"
  - [ ] Max ≤ base max
  - [ ] Min ≤ max
- [ ] Cannot set invalid cardinality
- [ ] Validation errors display inline with clear messages
- [ ] Base cardinality is shown prominently
- [ ] Changes are highlighted (blue border on inputs)
- [ ] Quick action buttons work:
  - [ ] 0..1 (Optional)
  - [ ] 1..1 (Required)
  - [ ] 0..* (Any)
  - [ ] 1..* (At least one)
- [ ] Reset to base button works
- [ ] Impact preview shows correct messages:
  - [ ] Making element required/optional
  - [ ] Allowing multiple values
  - [ ] Restricting to single value
  - [ ] Constraint tightening
- [ ] Apply button disabled when errors present
- [ ] Changes persist to backend
- [ ] Optimistic UI updates work
- [ ] Error states are handled gracefully

### UX Requirements (Reference: IMPLEMENTATION_PLAN.md Section 14)
- [ ] **Show, Don't Tell** (14.1): Visual inputs instead of text description
- [ ] **Immediate Feedback** (14.3): Validation errors shown immediately
- [ ] **Error Prevention** (14.4): Cannot apply invalid cardinality
- [ ] **Progressive Disclosure**: Advanced options hidden initially
- [ ] **Keyboard Support**: Tab navigation, Enter to apply
- [ ] **Clear Mental Model**: Impact preview explains what will happen

### Performance Requirements
- [ ] Input changes reflect immediately (<16ms)
- [ ] Validation runs synchronously (<10ms)
- [ ] API call completes <500ms
- [ ] Optimistic update feels instant

### Testing Requirements
- [ ] Unit tests for validation logic (100% coverage)
- [ ] Unit tests for impact message logic
- [ ] Integration tests with Effector state
- [ ] Integration tests with API
- [ ] Storybook stories for all states:
  - [ ] Unmodified cardinality
  - [ ] Modified cardinality
  - [ ] Validation errors
  - [ ] All quick presets
  - [ ] All impact scenarios
- [ ] Visual regression tests
- [ ] Accessibility tests

## 🔗 Dependencies

### Required Tasks
- **UI 03**: Mock Data Layer (element fixtures)
- **UI 05**: Inspector Panel (hosts this editor)
- **Backend 06**: Profile API Endpoints (PATCH element)
- **Backend 13**: Operations Engine (cardinality operations)

## 📚 API Contract

**Request**:
```typescript
// PATCH /api/profiles/:profileId/elements/:path
{
  "cardinality": {
    "min": 1,
    "max": "1"
  }
}
```

**Response**:
```typescript
{
  "data": {
    "element": {
      "id": "elem-123",
      "path": "Patient.name",
      "min": 1,
      "max": "1",
      "isModified": true,
      // ... other element fields
    }
  },
  "diagnostics": [],
  "metadata": {
    "timestamp": "2025-12-18T10:30:00Z",
    "version": 2
  }
}
```

## 🧪 Testing Examples

```typescript
// __tests__/CardinalityEditor.test.tsx
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { CardinalityEditor } from '../CardinalityEditor';

describe('CardinalityEditor', () => {
  const element = {
    path: 'Patient.name',
    min: 0,
    max: '*',
    base: { min: 0, max: '*' },
  };

  it('displays base cardinality', () => {
    render(<CardinalityEditor element={element} />);
    expect(screen.getByText('Base:')).toBeInTheDocument();
    expect(screen.getByText('0..*')).toBeInTheDocument();
  });

  it('validates min cannot be negative', async () => {
    const user = userEvent.setup();
    render(<CardinalityEditor element={element} />);

    const minInput = screen.getByLabelText('Minimum');
    await user.clear(minInput);
    await user.type(minInput, '-1');

    expect(screen.getByText('Minimum must be ≥ 0')).toBeInTheDocument();
  });

  it('applies quick preset 1..1', async () => {
    const user = userEvent.setup();
    render(<CardinalityEditor element={element} />);

    await user.click(screen.getByText('1..1 (Required)'));

    expect(screen.getByLabelText('Minimum')).toHaveValue(1);
    expect(screen.getByLabelText('Maximum')).toHaveValue('1');
  });

  it('shows impact message when making required', async () => {
    const user = userEvent.setup();
    render(<CardinalityEditor element={element} />);

    const minInput = screen.getByLabelText('Minimum');
    await user.clear(minInput);
    await user.type(minInput, '1');

    expect(screen.getByText(/This will make the element required/)).toBeInTheDocument();
  });
});
```

## 📖 Related Documentation

- **IMPLEMENTATION_PLAN.md Section 6**: Quick Constraints Panel
- **IMPLEMENTATION_PLAN.md Section 14**: Low-Code UX Design Principles
- **IMPLEMENTATION_PLAN.md Section 17**: UI State Model (Effector)

## 🎨 Priority

🔴 **Critical** - MVP feature, core editing capability

## ⏱️ Estimated Complexity

**Medium** - 1 week
- Days 1-2: UI component + validation logic
- Days 3-4: State management + API integration
- Day 5: Testing + polish
