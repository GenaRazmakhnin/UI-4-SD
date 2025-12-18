import { Stack, Checkbox, Alert, Group, Anchor } from '@mantine/core';
import { IconAlertCircle, IconExternalLink } from '@tabler/icons-react';
import type { ElementNode } from '@shared/types';
import { flagChanged } from '../model';
import styles from './FlagsEditor.module.css';

interface FlagsEditorProps {
  element: ElementNode;
}

export function FlagsEditor({ element }: FlagsEditorProps) {
  const handleMustSupportChange = (checked: boolean) => {
    flagChanged({
      elementId: element.id,
      flag: 'mustSupport',
      value: checked,
    });
  };

  const handleIsModifierChange = (checked: boolean) => {
    flagChanged({
      elementId: element.id,
      flag: 'isModifier',
      value: checked,
    });
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
            MustSupport with min=0 means: systems must be capable of populating
            this element, but it's not required in every instance.
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

        {element.isModifier && (
          <Alert
            icon={<IconAlertCircle size={16} />}
            color="red"
            variant="light"
            mt="xs"
          >
            <strong>Warning:</strong> Setting isModifier changes the semantics
            of the resource. Only use for elements that truly modify the meaning
            (e.g., negation, status).
          </Alert>
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
        <strong>Tip:</strong> Use Must Support for elements critical to your use
        case. Is Modifier should only be set on elements that change the
        resource's meaning.
      </Alert>
    </Stack>
  );
}
