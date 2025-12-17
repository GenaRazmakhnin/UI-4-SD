# Task: Inspector Panel

## 📋 Description

Implement the inspector panel widget that displays detailed information about the currently selected element and provides access to all editing features through a tabbed interface. The panel aggregates multiple editor components (cardinality, flags, bindings, etc.) and manages the editing workflow.

**Reference**: IMPLEMENTATION_PLAN.md Section 16.1 "Element Inspector Panel"

## 🎯 Context from Implementation Plan

This implements the element inspector with:
- **Inspector Panel** (16.1): Central panel for viewing and editing element details
- **Tabbed Interface** (16.1.2): Organize editing features by category
- **Live Validation** (16.1.3): Real-time validation feedback as users edit
- **FSD Architecture** (13): Widget-level component composition
- **Effector State** (17): Reactive state management for selected element

## 📐 Requirements

### R1: Main Inspector Panel Component

**Complete InspectorPanel Implementation**:
```typescript
// web/src/widgets/inspector-panel/ui/InspectorPanel.tsx
import { Stack, Tabs, Paper } from '@mantine/core';
import { useUnit } from 'effector-react';
import { $selectedElement } from '@entities/profile/model';
import { ElementHeader } from './ElementHeader';
import { ConstraintsTab } from './ConstraintsTab';
import { BindingTab } from './BindingTab';
import { SlicingTab } from './SlicingTab';
import { MetadataTab } from './MetadataTab';
import { EmptyState } from './EmptyState';
import styles from './InspectorPanel.module.css';

export function InspectorPanel() {
  const selectedElement = useUnit($selectedElement);

  if (!selectedElement) {
    return <EmptyState />;
  }

  return (
    <Paper className={styles.panel} shadow="sm" withBorder>
      <div className={styles.header}>
        <ElementHeader element={selectedElement} />
      </div>

      <Tabs
        defaultValue="constraints"
        className={styles.tabs}
        classNames={{
          root: styles.tabsRoot,
          list: styles.tabsList,
          panel: styles.tabsPanel,
        }}
      >
        <Tabs.List>
          <Tabs.Tab value="constraints">Constraints</Tabs.Tab>
          <Tabs.Tab value="binding">Binding</Tabs.Tab>
          <Tabs.Tab
            value="slicing"
            disabled={!canSlice(selectedElement)}
          >
            Slicing
          </Tabs.Tab>
          <Tabs.Tab value="metadata">Metadata</Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="constraints" className={styles.scrollablePanel}>
          <ConstraintsTab element={selectedElement} />
        </Tabs.Panel>

        <Tabs.Panel value="binding" className={styles.scrollablePanel}>
          <BindingTab element={selectedElement} />
        </Tabs.Panel>

        <Tabs.Panel value="slicing" className={styles.scrollablePanel}>
          <SlicingTab element={selectedElement} />
        </Tabs.Panel>

        <Tabs.Panel value="metadata" className={styles.scrollablePanel}>
          <MetadataTab element={selectedElement} />
        </Tabs.Panel>
      </Tabs>
    </Paper>
  );
}

/**
 * Check if an element can be sliced
 */
function canSlice(element: ElementNode): boolean {
  // Elements with max > 1 can be sliced
  return element.max === '*' || Number(element.max) > 1;
}
```

**Styling**:
```css
/* web/src/widgets/inspector-panel/ui/InspectorPanel.module.css */
.panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--mantine-color-white);
}

.header {
  position: sticky;
  top: 0;
  z-index: 10;
  background: var(--mantine-color-white);
  border-bottom: 1px solid var(--mantine-color-gray-3);
  padding: var(--mantine-spacing-md);
}

.tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tabsRoot {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.tabsList {
  position: sticky;
  top: 0;
  z-index: 9;
  background: var(--mantine-color-white);
  border-bottom: 1px solid var(--mantine-color-gray-3);
  padding: 0 var(--mantine-spacing-md);
}

.tabsPanel {
  flex: 1;
  overflow: hidden;
}

.scrollablePanel {
  height: 100%;
  overflow-y: auto;
  padding: var(--mantine-spacing-md);
}

/* Smooth scrolling */
.scrollablePanel::-webkit-scrollbar {
  width: 8px;
}

.scrollablePanel::-webkit-scrollbar-track {
  background: var(--mantine-color-gray-0);
}

.scrollablePanel::-webkit-scrollbar-thumb {
  background: var(--mantine-color-gray-4);
  border-radius: 4px;
}

.scrollablePanel::-webkit-scrollbar-thumb:hover {
  background: var(--mantine-color-gray-5);
}
```

### R2: Element Header Component

**Complete ElementHeader Implementation**:
```typescript
// web/src/widgets/inspector-panel/ui/ElementHeader.tsx
import { Group, Stack, Text, Badge, ActionIcon, Tooltip, CopyButton } from '@mantine/core';
import { IconCopy, IconExternalLink, IconAlertCircle } from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';
import styles from './ElementHeader.module.css';

interface ElementHeaderProps {
  element: ElementNode;
}

export function ElementHeader({ element }: ElementHeaderProps) {
  const baseUrl = getBaseDefinitionUrl(element);
  const types = element.type?.map(t => t.code).join(' | ') || 'Element';

  return (
    <Stack gap="xs">
      {/* Path and Actions */}
      <Group justify="space-between">
        <Group gap="xs">
          <Text size="sm" fw={600} className={styles.path}>
            {element.path}
          </Text>
          {element.sliceName && (
            <Badge size="sm" variant="light" color="blue">
              :{element.sliceName}
            </Badge>
          )}
        </Group>

        <Group gap="xs">
          {/* Copy Path Button */}
          <CopyButton value={element.path}>
            {({ copied, copy }) => (
              <Tooltip label={copied ? 'Copied!' : 'Copy path'}>
                <ActionIcon
                  size="sm"
                  variant="subtle"
                  color={copied ? 'teal' : 'gray'}
                  onClick={copy}
                >
                  <IconCopy size={16} />
                </ActionIcon>
              </Tooltip>
            )}
          </CopyButton>

          {/* Link to Base Definition */}
          {baseUrl && (
            <Tooltip label="View base definition">
              <ActionIcon
                size="sm"
                variant="subtle"
                component="a"
                href={baseUrl}
                target="_blank"
                rel="noopener noreferrer"
              >
                <IconExternalLink size={16} />
              </ActionIcon>
            </Tooltip>
          )}
        </Group>
      </Group>

      {/* Element Type and Status */}
      <Group gap="xs">
        <Text size="xs" c="dimmed">
          {types}
        </Text>

        {element.isModified && (
          <Badge size="xs" variant="light" color="orange">
            Modified
          </Badge>
        )}

        {element.mustSupport && (
          <Badge size="xs" variant="light" color="blue">
            Must Support
          </Badge>
        )}

        {element.isModifier && (
          <Badge size="xs" variant="light" color="red">
            <Group gap={4}>
              <IconAlertCircle size={12} />
              <span>Modifier</span>
            </Group>
          </Badge>
        )}

        {element.isSummary && (
          <Badge size="xs" variant="light" color="gray">
            Summary
          </Badge>
        )}
      </Group>

      {/* Short Description */}
      {element.short && (
        <Text size="xs" c="dimmed" lineClamp={2}>
          {element.short}
        </Text>
      )}
    </Stack>
  );
}

/**
 * Get base definition URL for element
 */
function getBaseDefinitionUrl(element: ElementNode): string | null {
  // Extract resource type from path (e.g., "Patient.name" -> "Patient")
  const resourceType = element.path.split('.')[0];
  return `https://hl7.org/fhir/R4/${resourceType}.html#${element.path}`;
}
```

**Styling**:
```css
/* web/src/widgets/inspector-panel/ui/ElementHeader.module.css */
.path {
  font-family: var(--mantine-font-family-monospace);
  word-break: break-all;
}
```

### R3: Constraints Tab

**Complete ConstraintsTab Implementation**:
```typescript
// web/src/widgets/inspector-panel/ui/ConstraintsTab.tsx
import { Stack, Title, Divider } from '@mantine/core';
import { CardinalityEditor } from '@features/cardinality-editor';
import { FlagsEditor } from '@features/flags-editor';
import { TypeConstraintEditor } from '@features/type-constraint-editor';
import { TextFieldEditor } from '@features/text-field-editor';
import type { ElementNode } from '@shared/types';

interface ConstraintsTabProps {
  element: ElementNode;
}

export function ConstraintsTab({ element }: ConstraintsTabProps) {
  return (
    <Stack gap="lg">
      {/* Cardinality Section */}
      <section>
        <Title order={6} mb="sm">Cardinality</Title>
        <CardinalityEditor element={element} />
      </section>

      <Divider />

      {/* Type Constraints Section */}
      <section>
        <Title order={6} mb="sm">Type Constraints</Title>
        <TypeConstraintEditor element={element} />
      </section>

      <Divider />

      {/* Flags Section */}
      <section>
        <Title order={6} mb="sm">Flags</Title>
        <FlagsEditor element={element} />
      </section>

      <Divider />

      {/* Documentation Section */}
      <section>
        <Title order={6} mb="sm">Documentation</Title>
        <Stack gap="sm">
          <TextFieldEditor
            label="Short Description"
            value={element.short || ''}
            onChange={(value) => updateElement(element.id, { short: value })}
            maxLength={254}
            placeholder="Brief description..."
          />

          <TextFieldEditor
            label="Definition"
            value={element.definition || ''}
            onChange={(value) => updateElement(element.id, { definition: value })}
            multiline
            rows={4}
            placeholder="Detailed definition..."
          />

          <TextFieldEditor
            label="Comment"
            value={element.comment || ''}
            onChange={(value) => updateElement(element.id, { comment: value })}
            multiline
            rows={3}
            placeholder="Additional comments..."
          />
        </Stack>
      </section>
    </Stack>
  );
}

// Placeholder for update function (will be implemented in element model)
function updateElement(id: string, updates: Partial<ElementNode>): void {
  // Will use Effector event
  console.log('Update element', id, updates);
}
```

### R4: Binding Tab

**Complete BindingTab Implementation**:
```typescript
// web/src/widgets/inspector-panel/ui/BindingTab.tsx
import { Stack, Title, Select, TextInput, Button, Group, Alert, Text } from '@mantine/core';
import { IconAlertCircle, IconSearch } from '@tabler/icons-react';
import { useState } from 'react';
import type { ElementNode, BindingConstraint } from '@shared/types';
import { BindingEditor } from '@features/binding-editor';

interface BindingTabProps {
  element: ElementNode;
}

export function BindingTab({ element }: BindingTabProps) {
  const canBind = canHaveBinding(element);

  if (!canBind) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="gray">
        This element type cannot have terminology bindings.
        Only code, Coding, CodeableConcept, Quantity, and string elements can be bound to ValueSets.
      </Alert>
    );
  }

  if (!element.binding) {
    return (
      <Stack gap="md">
        <Text size="sm" c="dimmed">
          No binding configured for this element.
        </Text>
        <Button
          leftSection={<IconSearch size={16} />}
          onClick={() => {/* Open binding wizard */}}
        >
          Add Binding
        </Button>
      </Stack>
    );
  }

  return (
    <Stack gap="lg">
      {/* Binding Configuration */}
      <section>
        <Title order={6} mb="sm">Binding Configuration</Title>
        <BindingEditor element={element} />
      </section>

      {/* ValueSet Details */}
      {element.binding.valueSet && (
        <section>
          <Title order={6} mb="sm">ValueSet Details</Title>
          <Stack gap="xs">
            <Text size="sm" fw={500}>
              {element.binding.valueSet}
            </Text>
            {element.binding.description && (
              <Text size="xs" c="dimmed">
                {element.binding.description}
              </Text>
            )}
          </Stack>
        </section>
      )}

      {/* Binding Strength Info */}
      <Alert color="blue" variant="light">
        <Text size="xs">
          <strong>{element.binding.strength}:</strong>{' '}
          {getBindingStrengthDescription(element.binding.strength)}
        </Text>
      </Alert>
    </Stack>
  );
}

/**
 * Check if element can have a terminology binding
 */
function canHaveBinding(element: ElementNode): boolean {
  if (!element.type || element.type.length === 0) {
    return false;
  }

  const bindableTypes = ['code', 'Coding', 'CodeableConcept', 'Quantity', 'string', 'uri'];
  return element.type.some(t => bindableTypes.includes(t.code));
}

/**
 * Get description for binding strength
 */
function getBindingStrengthDescription(strength: string): string {
  switch (strength) {
    case 'required':
      return 'To be conformant, the concept in this element SHALL be from the specified value set.';
    case 'extensible':
      return 'To be conformant, the concept in this element SHALL be from the specified value set if any of the codes within the value set can apply to the concept being communicated.';
    case 'preferred':
      return 'Instances are encouraged to draw from the specified codes for interoperability purposes but are not required to do so.';
    case 'example':
      return 'Instances are not expected or even encouraged to draw from the specified value set.';
    default:
      return '';
  }
}
```

### R5: Slicing Tab

**Complete SlicingTab Implementation**:
```typescript
// web/src/widgets/inspector-panel/ui/SlicingTab.tsx
import { Stack, Title, Button, Alert, Group, Badge, Text } from '@mantine/core';
import { IconPlus, IconAlertCircle } from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';
import { SlicingWizard } from '@features/slicing-wizard';
import { SliceList } from '@features/slicing-wizard/ui/SliceList';

interface SlicingTabProps {
  element: ElementNode;
}

export function SlicingTab({ element }: SlicingTabProps) {
  const canSlice = element.max === '*' || Number(element.max) > 1;

  if (!canSlice) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="gray">
        This element cannot be sliced because its maximum cardinality is 1.
        Only elements with max &gt; 1 or max = * can be sliced.
      </Alert>
    );
  }

  const hasSlicing = !!element.slicing;
  const slices = element.children.filter(c => c.sliceName);

  return (
    <Stack gap="lg">
      {/* Slicing Status */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Slicing Status</Title>
          {hasSlicing ? (
            <Badge color="green">Sliced</Badge>
          ) : (
            <Badge color="gray">Not Sliced</Badge>
          )}
        </Group>

        {!hasSlicing ? (
          <Button
            leftSection={<IconPlus size={16} />}
            onClick={() => {/* Open slicing wizard */}}
          >
            Create Slicing
          </Button>
        ) : (
          <Stack gap="sm">
            {/* Slicing Rules */}
            <div>
              <Text size="sm" fw={500} mb={4}>Discriminator</Text>
              <Stack gap={4}>
                {element.slicing!.discriminator.map((d, i) => (
                  <Text key={i} size="xs" c="dimmed">
                    {d.type} @ {d.path}
                  </Text>
                ))}
              </Stack>
            </div>

            <Group gap="lg">
              <div>
                <Text size="sm" fw={500}>Rules</Text>
                <Badge size="sm" variant="light">
                  {element.slicing!.rules}
                </Badge>
              </div>

              <div>
                <Text size="sm" fw={500}>Ordered</Text>
                <Badge size="sm" variant="light" color={element.slicing!.ordered ? 'blue' : 'gray'}>
                  {element.slicing!.ordered ? 'Yes' : 'No'}
                </Badge>
              </div>
            </Group>

            {element.slicing!.description && (
              <div>
                <Text size="sm" fw={500} mb={4}>Description</Text>
                <Text size="xs" c="dimmed">
                  {element.slicing!.description}
                </Text>
              </div>
            )}
          </Stack>
        )}
      </section>

      {/* Slices List */}
      {hasSlicing && slices.length > 0 && (
        <section>
          <Group justify="space-between" mb="sm">
            <Title order={6}>Slices ({slices.length})</Title>
            <Button
              size="xs"
              variant="light"
              leftSection={<IconPlus size={14} />}
              onClick={() => {/* Add new slice */}}
            >
              Add Slice
            </Button>
          </Group>

          <SliceList slices={slices} onSelect={(slice) => {/* Select slice */}} />
        </section>
      )}
    </Stack>
  );
}
```

### R6: Metadata Tab

**Complete MetadataTab Implementation**:
```typescript
// web/src/widgets/inspector-panel/ui/MetadataTab.tsx
import { Stack, Title, Divider, TextInput, Button, Group } from '@mantine/core';
import { IconPlus } from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';

interface MetadataTabProps {
  element: ElementNode;
}

export function MetadataTab({ element }: MetadataTabProps) {
  return (
    <Stack gap="lg">
      {/* Aliases */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Aliases</Title>
          <Button
            size="xs"
            variant="light"
            leftSection={<IconPlus size={14} />}
          >
            Add Alias
          </Button>
        </Group>

        {/* Placeholder for aliases list */}
        <TextInput
          placeholder="No aliases defined"
          disabled
        />
      </section>

      <Divider />

      {/* Mappings */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Mappings</Title>
          <Button
            size="xs"
            variant="light"
            leftSection={<IconPlus size={14} />}
          >
            Add Mapping
          </Button>
        </Group>

        {/* Placeholder for mappings list */}
        <TextInput
          placeholder="No mappings defined"
          disabled
        />
      </section>

      <Divider />

      {/* Constraints */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Constraints</Title>
          <Button
            size="xs"
            variant="light"
            leftSection={<IconPlus size={14} />}
          >
            Add Constraint
          </Button>
        </Group>

        {/* Placeholder for constraints list */}
        <TextInput
          placeholder="No constraints defined"
          disabled
        />
      </section>

      <Divider />

      {/* Examples */}
      <section>
        <Group justify="space-between" mb="sm">
          <Title order={6}>Examples</Title>
          <Button
            size="xs"
            variant="light"
            leftSection={<IconPlus size={14} />}
          >
            Add Example
          </Button>
        </Group>

        {/* Placeholder for examples list */}
        <TextInput
          placeholder="No examples defined"
          disabled
        />
      </section>
    </Stack>
  );
}
```

### R7: Empty State Component

**Complete EmptyState Implementation**:
```typescript
// web/src/widgets/inspector-panel/ui/EmptyState.tsx
import { Stack, Text, ThemeIcon, List, Paper } from '@mantine/core';
import { IconInfoCircle, IconKeyboard } from '@tabler/icons-react';
import styles from './EmptyState.module.css';

export function EmptyState() {
  return (
    <Paper className={styles.container} p="xl">
      <Stack align="center" gap="md">
        <ThemeIcon size={64} radius="xl" variant="light" color="gray">
          <IconInfoCircle size={32} />
        </ThemeIcon>

        <Stack align="center" gap="xs">
          <Text size="lg" fw={500}>
            No Element Selected
          </Text>
          <Text size="sm" c="dimmed" ta="center">
            Select an element from the tree to view and edit its properties
          </Text>
        </Stack>

        {/* Quick Tips */}
        <div className={styles.tips}>
          <Text size="sm" fw={500} mb="xs">
            <IconKeyboard size={16} style={{ verticalAlign: 'middle' }} /> Keyboard Shortcuts
          </Text>
          <List size="xs" spacing="xs" c="dimmed">
            <List.Item>↑↓ Navigate elements</List.Item>
            <List.Item>→ Expand element</List.Item>
            <List.Item>← Collapse element</List.Item>
            <List.Item>Enter Edit selected element</List.Item>
            <List.Item>Esc Clear selection</List.Item>
          </List>
        </div>
      </Stack>
    </Paper>
  );
}
```

**Styling**:
```css
/* web/src/widgets/inspector-panel/ui/EmptyState.module.css */
.container {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.tips {
  margin-top: var(--mantine-spacing-lg);
  padding: var(--mantine-spacing-md);
  background: var(--mantine-color-gray-0);
  border-radius: var(--mantine-radius-md);
  max-width: 300px;
}
```

### R8: Effector Model Integration

**State Management**:
```typescript
// web/src/widgets/inspector-panel/model/index.ts
import { createStore, createEvent, sample } from 'effector';
import { $selectedElement } from '@entities/profile/model';

/**
 * Active tab in inspector panel
 */
export const $activeTab = createStore<string>('constraints');

/**
 * Change active tab
 */
export const tabChanged = createEvent<string>();

$activeTab.on(tabChanged, (_, tab) => tab);

/**
 * Reset to constraints tab when element changes
 */
sample({
  clock: $selectedElement,
  filter: (element) => element !== null,
  fn: () => 'constraints',
  target: $activeTab,
});

/**
 * Panel width (for resizing)
 */
export const $panelWidth = createStore<number>(400);

export const panelWidthChanged = createEvent<number>();

$panelWidth.on(panelWidthChanged, (_, width) => {
  // Clamp width between 300 and 800
  return Math.max(300, Math.min(800, width));
});
```

### R9: Validation Integration

**Validation Display**:
```typescript
// web/src/widgets/inspector-panel/lib/validation.ts
import type { ElementNode, ValidationMessage } from '@shared/types';

/**
 * Get validation messages for an element
 */
export function getElementValidation(
  element: ElementNode,
  allMessages: ValidationMessage[]
): ValidationMessage[] {
  return allMessages.filter(msg =>
    msg.path === element.path || msg.path.startsWith(`${element.path}.`)
  );
}

/**
 * Group validation messages by severity
 */
export function groupMessagesBySeverity(messages: ValidationMessage[]) {
  return {
    errors: messages.filter(m => m.severity === 'error'),
    warnings: messages.filter(m => m.severity === 'warning'),
    info: messages.filter(m => m.severity === 'info'),
  };
}

/**
 * Get highest severity for an element
 */
export function getHighestSeverity(messages: ValidationMessage[]): 'error' | 'warning' | 'info' | null {
  if (messages.some(m => m.severity === 'error')) return 'error';
  if (messages.some(m => m.severity === 'warning')) return 'warning';
  if (messages.some(m => m.severity === 'info')) return 'info';
  return null;
}
```

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] Inspector panel displays when element is selected
- [ ] Empty state shows with helpful information when no element selected
- [ ] All four tabs render correctly (Constraints, Binding, Slicing, Metadata)
- [ ] Element header displays path, type, and status badges
- [ ] Copy path button works correctly
- [ ] Link to base definition opens in new tab
- [ ] Constraints tab shows all constraint editors
- [ ] Binding tab shows binding configuration or "Add Binding" button
- [ ] Slicing tab shows slicing status and slice list
- [ ] Slicing tab disabled for non-sliceable elements (max = 1)
- [ ] Metadata tab shows additional fields
- [ ] Tab state resets to "constraints" when element changes
- [ ] Changes to element trigger immediate update

### Performance Requirements
- [ ] Panel renders in <50ms when element changes
- [ ] Scrolling is smooth (60fps)
- [ ] Tab switching completes in <16ms
- [ ] No unnecessary re-renders when element unchanged
- [ ] Virtualized lists for large slice collections (>20 slices)

### Accessibility Requirements (WCAG 2.1 AA)
- [ ] All interactive elements keyboard accessible
- [ ] Tab navigation follows ARIA tabs pattern
- [ ] Empty state has proper heading hierarchy
- [ ] Badge semantics correctly conveyed to screen readers
- [ ] Copy button announces success to screen readers
- [ ] External link button has descriptive label
- [ ] Focus visible on all interactive elements
- [ ] Color not sole means of conveying information

### User Experience Requirements
- [ ] Panel header remains visible when scrolling (sticky)
- [ ] Tab list remains visible when scrolling (sticky)
- [ ] Tab content scrolls independently
- [ ] Visual feedback for modified elements
- [ ] Clear indication of disabled tabs
- [ ] Tooltips on all action buttons
- [ ] Smooth transitions between states

### Testing Requirements
- [ ] Unit tests for InspectorPanel component (>80% coverage)
- [ ] Unit tests for ElementHeader component
- [ ] Unit tests for all tab components
- [ ] Unit tests for EmptyState component
- [ ] Unit tests for validation helpers
- [ ] Integration tests with Effector stores
- [ ] Storybook stories for all components
- [ ] Storybook stories for all states (empty, selected, modified)
- [ ] Visual regression tests for panel layouts

## 🔗 Dependencies

### Required Tasks
- **UI 04**: Element Tree Viewer - Provides element selection
- **UI 06**: Cardinality Editor - Used in Constraints tab
- **UI 07**: Flags Editor - Used in Constraints tab

### Optional Enhancement Tasks
- **UI 08**: Type Constraint Editor - Used in Constraints tab
- **UI 09**: Binding Editor - Used in Binding tab
- **UI 10**: Slicing Wizard - Used in Slicing tab
- **UI 13**: Diagnostics Panel - Displays validation messages

### Integration Points
- **Profile Model**: Reads selected element from $selectedElement store
- **Validation Model**: Displays validation messages for selected element
- **Editor Features**: Aggregates multiple editor components

## 📚 API Contract

No direct API calls (uses existing profile API through Effector stores)

**State Dependencies**:
```typescript
// Read selected element
$selectedElement: Store<ElementNode | null>

// Read validation messages
$validationMessages: Store<ValidationMessage[]>

// Write element updates
updateElement: Event<{ id: string; updates: Partial<ElementNode> }>
```

## 🧪 Testing Examples

**InspectorPanel Test**:
```typescript
// web/src/widgets/inspector-panel/ui/__tests__/InspectorPanel.test.tsx
import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { allSettled, fork } from 'effector';
import { Provider } from 'effector-react';
import { InspectorPanel } from '../InspectorPanel';
import { $selectedElement, elementSelected } from '@entities/profile/model';
import { createMockElement } from '@shared/api/mock/factories';

describe('InspectorPanel', () => {
  it('shows empty state when no element selected', () => {
    const scope = fork();

    render(
      <Provider value={scope}>
        <InspectorPanel />
      </Provider>
    );

    expect(screen.getByText('No Element Selected')).toBeInTheDocument();
  });

  it('shows element header when element selected', async () => {
    const scope = fork();
    const element = createMockElement('Patient.name');

    await allSettled(elementSelected, { scope, params: element });

    render(
      <Provider value={scope}>
        <InspectorPanel />
      </Provider>
    );

    expect(screen.getByText('Patient.name')).toBeInTheDocument();
  });

  it('switches tabs correctly', async () => {
    const scope = fork();
    const element = createMockElement('Patient.name');
    await allSettled(elementSelected, { scope, params: element });

    const user = userEvent.setup();

    render(
      <Provider value={scope}>
        <InspectorPanel />
      </Provider>
    );

    // Initially on Constraints tab
    expect(screen.getByRole('tabpanel')).toHaveTextContent('Cardinality');

    // Click Binding tab
    await user.click(screen.getByRole('tab', { name: 'Binding' }));

    expect(screen.getByRole('tabpanel')).toHaveTextContent('Binding Configuration');
  });

  it('copies element path to clipboard', async () => {
    const scope = fork();
    const element = createMockElement('Patient.name');
    await allSettled(elementSelected, { scope, params: element });

    const user = userEvent.setup();

    render(
      <Provider value={scope}>
        <InspectorPanel />
      </Provider>
    );

    const copyButton = screen.getByLabelText('Copy path');
    await user.click(copyButton);

    expect(await navigator.clipboard.readText()).toBe('Patient.name');
  });
});
```

**EmptyState Test**:
```typescript
// web/src/widgets/inspector-panel/ui/__tests__/EmptyState.test.tsx
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { EmptyState } from '../EmptyState';

describe('EmptyState', () => {
  it('renders empty state message', () => {
    render(<EmptyState />);

    expect(screen.getByText('No Element Selected')).toBeInTheDocument();
    expect(screen.getByText(/Select an element from the tree/)).toBeInTheDocument();
  });

  it('shows keyboard shortcuts', () => {
    render(<EmptyState />);

    expect(screen.getByText('Keyboard Shortcuts')).toBeInTheDocument();
    expect(screen.getByText(/↑↓ Navigate elements/)).toBeInTheDocument();
  });
});
```

**Storybook Stories**:
```typescript
// web/src/widgets/inspector-panel/ui/InspectorPanel.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import { Provider } from 'effector-react';
import { fork, allSettled } from 'effector';
import { InspectorPanel } from './InspectorPanel';
import { elementSelected } from '@entities/profile/model';
import { createMockElement } from '@shared/api/mock/factories';

const meta: Meta<typeof InspectorPanel> = {
  title: 'Widgets/InspectorPanel',
  component: InspectorPanel,
  decorators: [
    (Story) => {
      const scope = fork();
      return (
        <Provider value={scope}>
          <div style={{ height: '600px' }}>
            <Story />
          </div>
        </Provider>
      );
    },
  ],
};

export default meta;
type Story = StoryObj<typeof InspectorPanel>;

export const Empty: Story = {
  name: 'Empty State',
};

export const SimpleElement: Story = {
  name: 'Simple Element Selected',
  decorators: [
    (Story) => {
      const scope = fork();
      const element = createMockElement('Patient.name', {
        type: [{ code: 'HumanName' }],
        min: 1,
        max: '*',
      });

      allSettled(elementSelected, { scope, params: element });

      return (
        <Provider value={scope}>
          <div style={{ height: '600px' }}>
            <Story />
          </div>
        </Provider>
      );
    },
  ],
};

export const ModifiedElement: Story = {
  name: 'Modified Element',
  decorators: [
    (Story) => {
      const scope = fork();
      const element = createMockElement('Patient.identifier', {
        isModified: true,
        mustSupport: true,
        min: 1,
        max: '*',
      });

      allSettled(elementSelected, { scope, params: element });

      return (
        <Provider value={scope}>
          <div style={{ height: '600px' }}>
            <Story />
          </div>
        </Provider>
      );
    },
  ],
};

export const WithBinding: Story = {
  name: 'Element with Binding',
  decorators: [
    (Story) => {
      const scope = fork();
      const element = createMockElement('Patient.gender', {
        type: [{ code: 'code' }],
        binding: {
          strength: 'required',
          valueSet: 'http://hl7.org/fhir/ValueSet/administrative-gender',
        },
      });

      allSettled(elementSelected, { scope, params: element });

      return (
        <Provider value={scope}>
          <div style={{ height: '600px' }}>
            <Story />
          </div>
        </Provider>
      );
    },
  ],
};

export const WithSlicing: Story = {
  name: 'Element with Slicing',
  decorators: [
    (Story) => {
      const scope = fork();
      const element = createMockElement('Observation.component', {
        min: 2,
        max: '*',
        slicing: {
          discriminator: [{ type: 'pattern', path: 'code' }],
          rules: 'open',
          ordered: false,
        },
        children: [
          createMockElement('Observation.component:systolic', {
            sliceName: 'systolic',
            min: 1,
            max: '1',
          }),
          createMockElement('Observation.component:diastolic', {
            sliceName: 'diastolic',
            min: 1,
            max: '1',
          }),
        ],
      });

      allSettled(elementSelected, { scope, params: element });

      return (
        <Provider value={scope}>
          <div style={{ height: '600px' }}>
            <Story />
          </div>
        </Provider>
      );
    },
  ],
};
```

## 📖 Related Documentation

- **IMPLEMENTATION_PLAN.md Section 16.1**: Element Inspector Panel specification
- **IMPLEMENTATION_PLAN.md Section 13**: FSD Architecture (widget layer)
- **IMPLEMENTATION_PLAN.md Section 17**: UI State Model (Effector stores)
- **IMPLEMENTATION_PLAN.md Section 14.2**: Low-Code UX Design principles
- **Mantine Tabs**: https://mantine.dev/core/tabs/
- **Effector Stores**: https://effector.dev/docs/api/effector/store/

## 🎨 Priority

🔴 **Critical** - Core UI widget for element editing

## ⏱️ Estimated Complexity

**Medium-High** - 1.5-2 weeks (60-80 hours)

### Breakdown:
- InspectorPanel main component: 8 hours
- ElementHeader component: 4 hours
- Tab components (4 tabs): 16 hours
- EmptyState component: 2 hours
- Effector model integration: 8 hours
- Validation integration: 8 hours
- Styling and responsive design: 8 hours
- Testing (unit + integration + Storybook): 16 hours
- Documentation: 4 hours
