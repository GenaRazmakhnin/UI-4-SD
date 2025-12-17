# Task: Binding Editor (Terminology) Feature

## 📋 Description

Implement the terminology binding editor that allows users to set ValueSet bindings on code/Coding/CodeableConcept elements with binding strength selection, ValueSet search, and expansion preview. The editor ensures binding strength cannot be weakened from the base definition and provides clear explanations of each strength level.

**Reference**: IMPLEMENTATION_PLAN.md Section 16.1.1 "Terminology Bindings"

## 🎯 Context from Implementation Plan

This implements terminology binding editing with:
- **Binding Configuration** (16.1.1): ValueSet URL and binding strength settings
- **ValueSet Search** (16.3): Find ValueSets from loaded packages
- **Live Validation** (16.1.3): Prevent invalid binding strength changes
- **Terminology Integration** (16.7): Expansion preview from terminology service
- **FSD Architecture** (13): Feature-level component

## 📐 Requirements

### R1: Main BindingEditor Component

**Complete Implementation**:
```typescript
// web/src/features/binding-editor/ui/BindingEditor.tsx
import { Stack, Select, TextInput, Button, Alert, Group, Text, Accordion } from '@mantine/core';
import { IconSearch, IconAlertCircle, IconExternalLink } from '@tabler/icons-react';
import { useState } from 'react';
import type { ElementNode, BindingConstraint } from '@shared/types';
import { ValueSetBrowser } from './ValueSetBrowser';
import { ExpansionPreview } from './ExpansionPreview';
import { bindingChanged, removeBinding } from '../model';
import { getBindingStrengthDescription, canChangeBindingStrength } from '../lib/validation';
import styles from './BindingEditor.module.css';

interface BindingEditorProps {
  element: ElementNode;
}

const BINDING_STRENGTHS = [
  { value: 'required', label: 'Required' },
  { value: 'extensible', label: 'Extensible' },
  { value: 'preferred', label: 'Preferred' },
  { value: 'example', label: 'Example' },
];

export function BindingEditor({ element }: BindingEditorProps) {
  const [browserOpen, setBrowserOpen] = useState(false);
  const [valueSetUrl, setValueSetUrl] = useState(element.binding?.valueSet || '');
  const [strength, setStrength] = useState(element.binding?.strength || 'example');
  const [description, setDescription] = useState(element.binding?.description || '');

  // Get base binding (if any)
  const baseBinding = getBaseBinding(element);

  // Handle binding changes
  const handleApplyBinding = () => {
    if (!valueSetUrl) {
      return; // ValueSet URL is required
    }

    bindingChanged({
      elementId: element.id,
      binding: {
        strength: strength as BindingConstraint['strength'],
        valueSet: valueSetUrl,
        description: description || undefined,
      },
    });
  };

  // Handle ValueSet selection from browser
  const handleValueSetSelected = (url: string, name: string) => {
    setValueSetUrl(url);
    if (!description) {
      setDescription(name);
    }
    setBrowserOpen(false);
  };

  // Handle remove binding
  const handleRemoveBinding = () => {
    if (confirm('Remove binding from this element?')) {
      removeBinding({ elementId: element.id });
      setValueSetUrl('');
      setStrength('example');
      setDescription('');
    }
  };

  // Check if strength change is valid
  const strengthValidation = canChangeBindingStrength(
    baseBinding?.strength,
    strength as BindingConstraint['strength']
  );

  return (
    <Stack gap="md" className={styles.container}>
      {/* Base Binding Info */}
      {baseBinding && (
        <Alert color="blue" variant="light" icon={<IconAlertCircle size={16} />}>
          <Text size="xs">
            <strong>Base binding:</strong> {baseBinding.valueSet} ({baseBinding.strength})
          </Text>
        </Alert>
      )}

      {/* ValueSet URL */}
      <TextInput
        label="ValueSet URL"
        description="The ValueSet canonical URL that defines the allowed codes"
        placeholder="http://hl7.org/fhir/ValueSet/..."
        value={valueSetUrl}
        onChange={(e) => setValueSetUrl(e.currentTarget.value)}
        rightSection={
          <Button
            size="xs"
            variant="subtle"
            onClick={() => setBrowserOpen(true)}
          >
            <IconSearch size={14} />
          </Button>
        }
        required
      />

      {/* Binding Strength */}
      <div>
        <Select
          label="Binding Strength"
          description="How strictly implementations must adhere to the ValueSet"
          data={BINDING_STRENGTHS}
          value={strength}
          onChange={(value) => value && setStrength(value)}
        />

        {/* Strength Description */}
        <Alert color="gray" variant="light" mt="xs">
          <Text size="xs">
            {getBindingStrengthDescription(strength as BindingConstraint['strength'])}
          </Text>
        </Alert>

        {/* Strength Validation Warning */}
        {!strengthValidation.isValid && (
          <Alert color="red" variant="light" icon={<IconAlertCircle size={16} />} mt="xs">
            <Text size="xs">{strengthValidation.error}</Text>
          </Alert>
        )}
      </div>

      {/* Description (Optional) */}
      <TextInput
        label="Description (Optional)"
        description="Additional context about this binding"
        placeholder="Describe when/how this ValueSet should be used"
        value={description}
        onChange={(e) => setDescription(e.currentTarget.value)}
      />

      {/* Expansion Preview */}
      {valueSetUrl && (
        <Accordion variant="contained">
          <Accordion.Item value="expansion">
            <Accordion.Control>Preview ValueSet Expansion</Accordion.Control>
            <Accordion.Panel>
              <ExpansionPreview valueSetUrl={valueSetUrl} />
            </Accordion.Panel>
          </Accordion.Item>
        </Accordion>
      )}

      {/* Actions */}
      <Group>
        <Button onClick={handleApplyBinding} disabled={!valueSetUrl || !strengthValidation.isValid}>
          Apply Binding
        </Button>

        {element.binding && (
          <Button variant="subtle" color="red" onClick={handleRemoveBinding}>
            Remove Binding
          </Button>
        )}
      </Group>

      {/* FHIR Spec Link */}
      <Alert color="blue" variant="light">
        <Group gap="xs">
          <Text size="xs">
            Learn more about FHIR terminology bindings
          </Text>
          <a
            href="https://www.hl7.org/fhir/terminologies.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            <IconExternalLink size={14} />
          </a>
        </Group>
      </Alert>

      {/* ValueSet Browser Modal */}
      <ValueSetBrowser
        opened={browserOpen}
        onClose={() => setBrowserOpen(false)}
        onValueSetSelected={handleValueSetSelected}
      />
    </Stack>
  );
}

/**
 * Get base binding (would come from base definition in real implementation)
 */
function getBaseBinding(element: ElementNode): BindingConstraint | null {
  // In real implementation, fetch from base definition
  // For now, return null (no base binding)
  return null;
}
```

**Styling**:
```css
/* web/src/features/binding-editor/ui/BindingEditor.module.css */
.container {
  width: 100%;
}
```

### R2: ValueSet Browser Component

**Complete ValueSetBrowser Implementation**:
```typescript
// web/src/features/binding-editor/ui/ValueSetBrowser.tsx
import {
  Modal,
  TextInput,
  Stack,
  Button,
  Group,
  Text,
  ScrollArea,
  Badge,
  Loader,
  Select,
} from '@mantine/core';
import { IconSearch } from '@tabler/icons-react';
import { useState, useEffect } from 'react';
import { useUnit } from 'effector-react';
import { searchValueSetsFx, $searchResults, $searchLoading } from '../model';
import type { ValueSet } from '@shared/types';
import styles from './ValueSetBrowser.module.css';

interface ValueSetBrowserProps {
  opened: boolean;
  onClose: () => void;
  onValueSetSelected: (url: string, name: string) => void;
}

export function ValueSetBrowser({
  opened,
  onClose,
  onValueSetSelected,
}: ValueSetBrowserProps) {
  const [query, setQuery] = useState('');
  const [codeSystemFilter, setCodeSystemFilter] = useState<string | null>(null);
  const searchResults = useUnit($searchResults);
  const isLoading = useUnit($searchLoading);

  // Search on modal open
  useEffect(() => {
    if (opened) {
      handleSearch();
    }
  }, [opened]);

  const handleSearch = () => {
    searchValueSetsFx({ query, codeSystemFilter });
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  return (
    <Modal
      opened={opened}
      onClose={onClose}
      title="Browse ValueSets"
      size="xl"
    >
      <Stack gap="md">
        {/* Search Controls */}
        <Group grow>
          <TextInput
            placeholder="Search ValueSets by name or description..."
            leftSection={<IconSearch size={16} />}
            value={query}
            onChange={(e) => setQuery(e.currentTarget.value)}
            onKeyPress={handleKeyPress}
          />

          <Select
            placeholder="Filter by code system"
            clearable
            value={codeSystemFilter}
            onChange={setCodeSystemFilter}
            data={[
              { value: 'http://snomed.info/sct', label: 'SNOMED CT' },
              { value: 'http://loinc.org', label: 'LOINC' },
              { value: 'http://hl7.org/fhir/sid/icd-10', label: 'ICD-10' },
              { value: 'http://www.nlm.nih.gov/research/umls/rxnorm', label: 'RxNorm' },
            ]}
          />

          <Button onClick={handleSearch} loading={isLoading}>
            Search
          </Button>
        </Group>

        {/* Search Results */}
        <ScrollArea h={500}>
          {isLoading ? (
            <Group justify="center" p="xl">
              <Loader size="sm" />
              <Text size="sm" c="dimmed">Searching ValueSets...</Text>
            </Group>
          ) : searchResults.length === 0 ? (
            <Text size="sm" c="dimmed" ta="center" p="xl">
              No ValueSets found. Try a different search term.
            </Text>
          ) : (
            <Stack gap="sm">
              {searchResults.map((valueSet) => (
                <div
                  key={valueSet.url}
                  className={styles.valueSetCard}
                  onClick={() => onValueSetSelected(valueSet.url, valueSet.name)}
                >
                  <Group justify="space-between" mb="xs">
                    <Text size="sm" fw={500}>{valueSet.title || valueSet.name}</Text>
                    <Group gap="xs">
                      <Badge size="sm" variant="light">
                        {valueSet.status}
                      </Badge>
                      {valueSet.expansion && (
                        <Badge size="sm" variant="light" color="blue">
                          {valueSet.expansion.total} codes
                        </Badge>
                      )}
                    </Group>
                  </Group>

                  <Text size="xs" c="dimmed" className={styles.url}>
                    {valueSet.url}
                  </Text>

                  {valueSet.description && (
                    <Text size="xs" c="dimmed" mt="xs" lineClamp={2}>
                      {valueSet.description}
                    </Text>
                  )}

                  {valueSet.publisher && (
                    <Text size="xs" c="dimmed" mt="xs">
                      Publisher: {valueSet.publisher}
                    </Text>
                  )}

                  {valueSet.compose?.include && (
                    <Text size="xs" c="dimmed" mt="xs">
                      Code systems: {valueSet.compose.include.map(i => i.system).join(', ')}
                    </Text>
                  )}
                </div>
              ))}
            </Stack>
          )}
        </ScrollArea>

        {/* Actions */}
        <Group justify="flex-end">
          <Button variant="subtle" onClick={onClose}>
            Cancel
          </Button>
        </Group>
      </Stack>
    </Modal>
  );
}
```

**Styling**:
```css
/* web/src/features/binding-editor/ui/ValueSetBrowser.module.css */
.valueSetCard {
  padding: var(--mantine-spacing-sm);
  border: 1px solid var(--mantine-color-gray-3);
  border-radius: var(--mantine-radius-sm);
  cursor: pointer;
  transition: all 0.2s;
}

.valueSetCard:hover {
  background: var(--mantine-color-gray-0);
  border-color: var(--mantine-color-blue-6);
  transform: translateY(-1px);
  box-shadow: var(--mantine-shadow-sm);
}

.url {
  font-family: var(--mantine-font-family-monospace);
  word-break: break-all;
}
```

### R3: Expansion Preview Component

**Complete ExpansionPreview Implementation**:
```typescript
// web/src/features/binding-editor/ui/ExpansionPreview.tsx
import { Stack, Text, Badge, Group, Loader, Alert, ScrollArea, Code } from '@mantine/core';
import { IconAlertCircle } from '@tabler/icons-react';
import { useEffect } from 'react';
import { useUnit } from 'effector-react';
import { fetchExpansionFx, $expansions, $expansionLoading } from '../model';
import styles from './ExpansionPreview.module.css';

interface ExpansionPreviewProps {
  valueSetUrl: string;
}

export function ExpansionPreview({ valueSetUrl }: ExpansionPreviewProps) {
  const expansions = useUnit($expansions);
  const isLoading = useUnit($expansionLoading);

  const expansion = expansions[valueSetUrl];

  // Fetch expansion when URL changes
  useEffect(() => {
    if (valueSetUrl && !expansion) {
      fetchExpansionFx({ valueSetUrl });
    }
  }, [valueSetUrl, expansion]);

  if (isLoading) {
    return (
      <Group justify="center" p="md">
        <Loader size="sm" />
        <Text size="sm" c="dimmed">Loading expansion...</Text>
      </Group>
    );
  }

  if (!expansion) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="yellow">
        <Text size="xs">
          Expansion not available. This ValueSet may need to be expanded by a terminology service.
        </Text>
      </Alert>
    );
  }

  if (expansion.error) {
    return (
      <Alert icon={<IconAlertCircle size={16} />} color="red">
        <Text size="xs">
          <strong>Error:</strong> {expansion.error}
        </Text>
      </Alert>
    );
  }

  const contains = expansion.contains || [];
  const total = expansion.total || contains.length;
  const displayLimit = 20;
  const hasMore = contains.length > displayLimit;

  return (
    <Stack gap="md">
      {/* Expansion Summary */}
      <Group>
        <Badge size="lg" variant="light" color="blue">
          {total} total codes
        </Badge>
        {hasMore && (
          <Text size="xs" c="dimmed">
            Showing first {displayLimit} codes
          </Text>
        )}
      </Group>

      {/* Code List */}
      <ScrollArea h={300}>
        <Stack gap="xs">
          {contains.slice(0, displayLimit).map((concept, idx) => (
            <div key={idx} className={styles.conceptRow}>
              <Group justify="space-between">
                <Group gap="xs">
                  <Code>{concept.code}</Code>
                  <Text size="sm">{concept.display}</Text>
                </Group>
                {concept.system && (
                  <Text size="xs" c="dimmed" className={styles.system}>
                    {concept.system}
                  </Text>
                )}
              </Group>
            </div>
          ))}
        </Stack>
      </ScrollArea>

      {/* More Info */}
      {hasMore && (
        <Alert color="blue" variant="light">
          <Text size="xs">
            {total - displayLimit} more codes not shown. Use a terminology browser for full expansion.
          </Text>
        </Alert>
      )}
    </Stack>
  );
}
```

**Styling**:
```css
/* web/src/features/binding-editor/ui/ExpansionPreview.module.css */
.conceptRow {
  padding: var(--mantine-spacing-xs);
  background: var(--mantine-color-gray-0);
  border-radius: var(--mantine-radius-sm);
}

.system {
  font-family: var(--mantine-font-family-monospace);
  font-size: 10px;
}
```

### R4: Validation Logic

**Binding Validation Functions**:
```typescript
// web/src/features/binding-editor/lib/validation.ts
import type { BindingConstraint } from '@shared/types';

export interface BindingValidation {
  isValid: boolean;
  error?: string;
  warnings: string[];
}

/**
 * Validate binding strength change
 */
export function canChangeBindingStrength(
  baseStrength: BindingConstraint['strength'] | undefined,
  newStrength: BindingConstraint['strength']
): BindingValidation {
  const warnings: string[] = [];

  // If no base binding, any strength is allowed
  if (!baseStrength) {
    return { isValid: true, warnings };
  }

  // Binding strength hierarchy (stronger to weaker)
  const strengthOrder = ['required', 'extensible', 'preferred', 'example'];
  const baseIndex = strengthOrder.indexOf(baseStrength);
  const newIndex = strengthOrder.indexOf(newStrength);

  // Cannot weaken binding strength
  if (newIndex > baseIndex) {
    return {
      isValid: false,
      error: `Cannot weaken binding strength from "${baseStrength}" to "${newStrength}". Profiles can only strengthen bindings.`,
      warnings,
    };
  }

  // Warn if strengthening significantly
  if (newIndex < baseIndex) {
    warnings.push(
      `Strengthening binding from "${baseStrength}" to "${newStrength}". ` +
      `Ensure this aligns with your use case requirements.`
    );
  }

  return { isValid: true, warnings };
}

/**
 * Get description for binding strength
 */
export function getBindingStrengthDescription(strength: BindingConstraint['strength']): string {
  const descriptions = {
    required:
      'REQUIRED: Codes SHALL be from the specified value set. This is the strictest binding.',
    extensible:
      'EXTENSIBLE: Codes SHALL be from the specified value set if applicable. ' +
      'If no suitable code exists, an alternative code may be used.',
    preferred:
      'PREFERRED: Codes SHOULD be from the specified value set for interoperability, ' +
      'but alternative codes are allowed.',
    example:
      'EXAMPLE: Codes MAY be from the specified value set. This is the weakest binding, ' +
      'used for examples only.',
  };

  return descriptions[strength];
}

/**
 * Validate ValueSet URL format
 */
export function isValidValueSetUrl(url: string): boolean {
  if (!url) return false;

  try {
    const parsedUrl = new URL(url);
    // ValueSet URLs should be HTTP(S)
    return parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:';
  } catch {
    return false;
  }
}

/**
 * Get recommended binding strength based on element criticality
 */
export function getRecommendedBindingStrength(
  element: { mustSupport?: boolean; min: number }
): BindingConstraint['strength'] {
  // Must Support required elements should have strong bindings
  if (element.mustSupport && element.min >= 1) {
    return 'required';
  }

  // Must Support optional elements
  if (element.mustSupport) {
    return 'extensible';
  }

  // Optional elements
  return 'preferred';
}
```

### R5: Effector State Management

**Complete State Model**:
```typescript
// web/src/features/binding-editor/model/index.ts
import { createEvent, createEffect, createStore, sample } from 'effector';
import { api } from '@shared/api';
import { $currentProfile, profileUpdated } from '@entities/profile/model';
import type { ElementNode, BindingConstraint, ValueSet, ValueSetExpansion } from '@shared/types';

/**
 * Binding changed
 */
export const bindingChanged = createEvent<{
  elementId: string;
  binding: BindingConstraint;
}>();

/**
 * Remove binding
 */
export const removeBinding = createEvent<{
  elementId: string;
}>();

/**
 * Search ValueSets effect
 */
export const searchValueSetsFx = createEffect(async ({
  query,
  codeSystemFilter,
}: {
  query: string;
  codeSystemFilter: string | null;
}) => {
  const results = await api.search.valueSets(query, {
    codeSystem: codeSystemFilter ? [codeSystemFilter] : undefined,
  });

  return results;
});

/**
 * Fetch ValueSet expansion effect
 */
export const fetchExpansionFx = createEffect(async ({
  valueSetUrl,
}: {
  valueSetUrl: string;
}) => {
  try {
    const expansion = await api.terminology.expand(valueSetUrl);
    return { valueSetUrl, expansion };
  } catch (error) {
    return {
      valueSetUrl,
      expansion: {
        error: error instanceof Error ? error.message : 'Failed to expand ValueSet',
      },
    };
  }
});

/**
 * Search results store
 */
export const $searchResults = createStore<ValueSet[]>([]);
export const $searchLoading = searchValueSetsFx.pending;

$searchResults.on(searchValueSetsFx.doneData, (_, results) => results);

/**
 * Expansions store (cached by URL)
 */
export const $expansions = createStore<Record<string, ValueSetExpansion>>({});
export const $expansionLoading = fetchExpansionFx.pending;

$expansions.on(fetchExpansionFx.doneData, (state, { valueSetUrl, expansion }) => ({
  ...state,
  [valueSetUrl]: expansion,
}));

/**
 * Update binding effect
 */
const updateBindingFx = createEffect(async ({
  profileId,
  elementId,
  binding,
}: {
  profileId: string;
  elementId: string;
  binding: BindingConstraint | null;
}) => {
  return await api.profiles.updateElement(profileId, elementId, { binding });
});

/**
 * Handle binding changes
 */
sample({
  clock: bindingChanged,
  source: $currentProfile,
  filter: (profile) => profile !== null,
  fn: (profile, { elementId, binding }) => ({
    profileId: profile!.id,
    elementId,
    binding,
  }),
  target: updateBindingFx,
});

/**
 * Handle remove binding
 */
sample({
  clock: removeBinding,
  source: $currentProfile,
  filter: (profile) => profile !== null,
  fn: (profile, { elementId }) => ({
    profileId: profile!.id,
    elementId,
    binding: null,
  }),
  target: updateBindingFx,
});

/**
 * Update profile after binding changes
 */
sample({
  clock: updateBindingFx.doneData,
  target: profileUpdated,
});
```

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] ValueSet URL input accepts valid HTTP(S) URLs
- [ ] Binding strength selector displays all four options
- [ ] ValueSet browser modal opens and displays results
- [ ] ValueSet search finds matching ValueSets
- [ ] Profile selection populates URL and description
- [ ] Expansion preview fetches and displays codes
- [ ] Binding changes persist immediately via API
- [ ] Remove binding works correctly
- [ ] Validation prevents weakening binding strength
- [ ] Clear error messages for invalid inputs

### User Experience Requirements
- [ ] Binding strength descriptions are clear
- [ ] Base binding information displayed when present
- [ ] Warning shown when strengthening binding
- [ ] Error shown when attempting to weaken binding
- [ ] Expansion preview shows representative codes
- [ ] Code system filter in browser works
- [ ] Graceful handling when expansion unavailable
- [ ] Link to FHIR spec documentation

### Performance Requirements
- [ ] ValueSet search completes in <1s
- [ ] Expansion preview loads in <2s
- [ ] Expansion results cached (no re-fetch)
- [ ] Modal opens in <100ms
- [ ] Binding update completes in <200ms

### Validation Requirements
- [ ] ValueSet URL must be valid HTTP(S)
- [ ] Binding strength cannot be weakened from base
- [ ] Warning when strengthening significantly
- [ ] All four binding strengths selectable (when allowed)

## 🔗 Dependencies

### Required Tasks
- **UI 03**: Mock Data Layer - Provides mock API
- **UI 05**: Inspector Panel - Hosts BindingEditor

### Optional Integration
- **UI 12**: Search UI - Reuses search patterns
- **Backend 11**: Search API - Provides ValueSet search
- **Backend (future)**: Terminology Service - Provides expansion

### Integration Points
- **Profile Model**: Reads element binding data
- **Search API**: Finds ValueSets
- **Terminology API**: Expands ValueSets (future)

## 📚 API Contract

**Update Element Binding**:
```typescript
PATCH /api/profiles/:profileId/elements/:elementPath
Body: {
  binding?: {
    strength: 'required' | 'extensible' | 'preferred' | 'example';
    valueSet: string;
    description?: string;
  } | null;
}
Response: Profile (updated)
```

**Search ValueSets**:
```typescript
GET /api/search/valuesets?query=:query&codeSystem=:system
Response: ValueSet[]
```

**Expand ValueSet** (future):
```typescript
GET /api/terminology/expand?url=:valueSetUrl
Response: ValueSetExpansion
```

## 📖 Related Documentation

- **IMPLEMENTATION_PLAN.md Section 16.1.1**: Terminology Bindings specification
- **IMPLEMENTATION_PLAN.md Section 16.7**: Terminology Integration
- **FHIR Terminologies**: https://www.hl7.org/fhir/terminologies.html
- **Binding Strength**: https://www.hl7.org/fhir/valueset-binding-strength.html
- **ValueSet Expansion**: https://www.hl7.org/fhir/valueset-operation-expand.html

## 🎨 Priority

🟡 **High** - Beta feature for profile editing

## ⏱️ Estimated Complexity

**High** - 1-2 weeks (60-80 hours)

### Breakdown:
- BindingEditor component: 10 hours
- ValueSetBrowser component: 10 hours
- ExpansionPreview component: 8 hours
- Validation logic: 6 hours
- Effector model integration: 10 hours
- Terminology service integration: 12 hours
- Documentation: 4 hours
