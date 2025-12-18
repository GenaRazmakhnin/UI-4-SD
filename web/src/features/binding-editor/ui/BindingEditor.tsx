import { Accordion, Alert, Button, Group, Select, Stack, Text, TextInput } from '@mantine/core';
import type { BindingConstraint, ElementNode } from '@shared/types';
import { IconAlertCircle, IconExternalLink, IconSearch } from '@tabler/icons-react';
import { useState } from 'react';
import { canChangeBindingStrength, getBindingStrengthDescription } from '../lib/validation';
import { bindingChanged, removeBinding } from '../model';
import styles from './BindingEditor.module.css';
import { ExpansionPreview } from './ExpansionPreview';
import { ValueSetBrowser } from './ValueSetBrowser';

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
          <Button size="xs" variant="subtle" onClick={() => setBrowserOpen(true)}>
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
          <Text size="xs">Learn more about FHIR terminology bindings</Text>
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
