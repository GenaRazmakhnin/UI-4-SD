import {
  Badge,
  Button,
  Divider,
  Group,
  Modal,
  ScrollArea,
  Stack,
  Stepper,
  Text,
} from '@mantine/core';
import { IconArrowLeft, IconArrowRight, IconCheck, IconRocket } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import {
  $canProceed,
  $dialogOpen,
  $isCreating,
  $projectConfig,
  $selectedTemplate,
  $wizardStep,
  dialogClosed,
  nextStep,
  prevStep,
  projectSubmitted,
  stepChanged,
} from '../model';
import { ProjectConfigForm } from './ProjectConfigForm';
import { TemplateGallery } from './TemplateGallery';

const STEP_MAP = {
  template: 0,
  configure: 1,
  review: 2,
};

function ReviewStep() {
  const [config, template] = useUnit([$projectConfig, $selectedTemplate]);

  return (
    <Stack gap="md">
      <Text fw={600}>Review Your Project</Text>

      <Divider />

      <Group justify="space-between">
        <Text c="dimmed" size="sm">
          Template
        </Text>
        <Text size="sm" fw={500}>
          {template?.name || 'None'}
        </Text>
      </Group>

      <Group justify="space-between">
        <Text c="dimmed" size="sm">
          Project Name
        </Text>
        <Text size="sm" fw={500}>
          {config.name}
        </Text>
      </Group>

      <Group justify="space-between">
        <Text c="dimmed" size="sm">
          Canonical Base
        </Text>
        <Text size="sm" fw={500} ff="monospace">
          {config.canonicalBase}
        </Text>
      </Group>

      <Group justify="space-between">
        <Text c="dimmed" size="sm">
          FHIR Version
        </Text>
        <Badge variant="light">{config.fhirVersion}</Badge>
      </Group>

      <Group justify="space-between">
        <Text c="dimmed" size="sm">
          Package ID
        </Text>
        <Text size="sm" fw={500} ff="monospace">
          {config.packageId}
        </Text>
      </Group>

      <Group justify="space-between">
        <Text c="dimmed" size="sm">
          Version
        </Text>
        <Text size="sm" fw={500}>
          {config.version}
        </Text>
      </Group>

      {config.description && (
        <Group justify="space-between" align="flex-start">
          <Text c="dimmed" size="sm">
            Description
          </Text>
          <Text size="sm" maw={300} ta="right">
            {config.description}
          </Text>
        </Group>
      )}

      {config.dependencies.length > 0 && (
        <>
          <Divider />
          <Text c="dimmed" size="sm">
            Dependencies
          </Text>
          <Stack gap="xs">
            {config.dependencies.map((dep) => (
              <Group key={dep.packageId} justify="space-between">
                <Text size="sm">{dep.name}</Text>
                <Badge size="xs" variant="outline">
                  {dep.version}
                </Badge>
              </Group>
            ))}
          </Stack>
        </>
      )}

      <Divider />

      <Group justify="space-between">
        <Text c="dimmed" size="sm">
          Initialize Git
        </Text>
        <Badge color={config.initGit ? 'green' : 'gray'} variant="light">
          {config.initGit ? 'Yes' : 'No'}
        </Badge>
      </Group>
    </Stack>
  );
}

export function NewProjectDialog() {
  const [isOpen, step, canProceed, isCreating] = useUnit([
    $dialogOpen,
    $wizardStep,
    $canProceed,
    $isCreating,
  ]);

  const activeStep = STEP_MAP[step];

  const handleClose = () => {
    if (!isCreating) {
      dialogClosed();
    }
  };

  const handleStepClick = (stepIndex: number) => {
    const steps: Array<'template' | 'configure' | 'review'> = ['template', 'configure', 'review'];
    if (stepIndex <= activeStep) {
      stepChanged(steps[stepIndex]);
    }
  };

  const handleNext = () => {
    if (step === 'review') {
      projectSubmitted();
    } else {
      nextStep();
    }
  };

  const handleBack = () => {
    prevStep();
  };

  return (
    <Modal
      opened={isOpen}
      onClose={handleClose}
      title="Create New Project"
      size="lg"
      closeOnClickOutside={!isCreating}
      closeOnEscape={!isCreating}
    >
      <Stack gap="md">
        <Stepper active={activeStep} onStepClick={handleStepClick} size="sm">
          <Stepper.Step label="Template" description="Choose a template" />
          <Stepper.Step label="Configure" description="Set up your project" />
          <Stepper.Step label="Review" description="Confirm settings" />
        </Stepper>

        <ScrollArea h={400} offsetScrollbars>
          {step === 'template' && <TemplateGallery />}
          {step === 'configure' && <ProjectConfigForm />}
          {step === 'review' && <ReviewStep />}
        </ScrollArea>

        <Divider />

        <Group justify="space-between">
          <Button
            variant="subtle"
            leftSection={<IconArrowLeft size={16} />}
            onClick={handleBack}
            disabled={step === 'template' || isCreating}
          >
            Back
          </Button>

          <Button
            rightSection={
              step === 'review' ? <IconRocket size={16} /> : <IconArrowRight size={16} />
            }
            onClick={handleNext}
            disabled={!canProceed || isCreating}
            loading={isCreating}
          >
            {step === 'review' ? 'Create Project' : 'Next'}
          </Button>
        </Group>
      </Stack>
    </Modal>
  );
}
