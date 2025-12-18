import { Button, Group, Loader, Modal, Stepper, Text } from '@mantine/core';
import type { ElementNode } from '@shared/types';
import { IconCheck } from '@tabler/icons-react';
import { useUnit } from 'effector-react';
import { useEffect, useState } from 'react';
import {
  $canProceed,
  $wizardOpen,
  $wizardState,
  applySlicingFx,
  stepChanged,
  wizardClosed,
  wizardOpened,
} from '../model';
import { Step1Discriminators } from './Step1Discriminators';
import { Step2Rules } from './Step2Rules';
import { Step3Slices } from './Step3Slices';
import { Step4Review } from './Step4Review';
import { TemplateSelector } from './TemplateSelector';

interface SlicingWizardProps {
  element: ElementNode;
  opened: boolean;
  onClose: () => void;
}

export function SlicingWizard({ element, opened, onClose }: SlicingWizardProps) {
  const wizardState = useUnit($wizardState);
  const wizardOpen = useUnit($wizardOpen);
  const canProceed = useUnit($canProceed);
  const applyLoading = useUnit(applySlicingFx.pending);
  const [showTemplates, setShowTemplates] = useState(true);

  // Initialize wizard when opened
  useEffect(() => {
    if (opened && !wizardOpen) {
      wizardOpened({ element });
      setShowTemplates(true);
    }
  }, [opened, wizardOpen, element]);

  const handleClose = () => {
    wizardClosed();
    setShowTemplates(true);
    onClose();
  };

  const handleNext = () => {
    if (wizardState.currentStep < 3) {
      stepChanged(wizardState.currentStep + 1);
    }
  };

  const handleBack = () => {
    if (wizardState.currentStep > 0) {
      stepChanged(wizardState.currentStep - 1);
    }
  };

  const handleStepClick = (step: number) => {
    // Only allow clicking on completed or current step
    if (step <= wizardState.currentStep) {
      stepChanged(step);
    }
  };

  const handleApply = async () => {
    try {
      await applySlicingFx({
        profileId: 'current-profile', // TODO: Get from profile context
        elementPath: wizardState.elementPath,
        slicing: {
          discriminator: wizardState.discriminators,
          rules: wizardState.rules,
          ordered: wizardState.ordered,
          description: wizardState.description || undefined,
        },
        slices: wizardState.slices,
      });
      handleClose();
    } catch (error) {
      console.error('Failed to apply slicing:', error);
      // TODO: Show error toast
    }
  };

  const handleSkipTemplates = () => {
    setShowTemplates(false);
  };

  return (
    <Modal
      opened={opened}
      onClose={handleClose}
      title="Create Slicing"
      size="xl"
      closeOnClickOutside={false}
      closeOnEscape={!applyLoading}
    >
      {showTemplates ? (
        <TemplateSelector onSkip={handleSkipTemplates} />
      ) : (
        <>
          <Stepper
            active={wizardState.currentStep}
            onStepClick={handleStepClick}
            allowNextStepsSelect={false}
            mb="xl"
          >
            <Stepper.Step
              label="Discriminators"
              description="Define discriminators"
              completedIcon={<IconCheck size={16} />}
            >
              <Step1Discriminators />
            </Stepper.Step>

            <Stepper.Step
              label="Rules"
              description="Set slicing rules"
              completedIcon={<IconCheck size={16} />}
            >
              <Step2Rules />
            </Stepper.Step>

            <Stepper.Step
              label="Slices"
              description="Create slices"
              completedIcon={<IconCheck size={16} />}
            >
              <Step3Slices />
            </Stepper.Step>

            <Stepper.Step
              label="Review"
              description="Review & apply"
              completedIcon={<IconCheck size={16} />}
            >
              <Step4Review />
            </Stepper.Step>
          </Stepper>

          {/* Navigation */}
          <Group justify="space-between" mt="xl">
            <Group>
              <Button variant="subtle" onClick={handleClose} disabled={applyLoading}>
                Cancel
              </Button>
              {wizardState.currentStep > 0 && (
                <Button variant="default" onClick={handleBack} disabled={applyLoading}>
                  Back
                </Button>
              )}
            </Group>

            <Group>
              {wizardState.currentStep < 3 ? (
                <Button onClick={handleNext} disabled={!canProceed || applyLoading}>
                  Next Step
                </Button>
              ) : (
                <Button
                  onClick={handleApply}
                  disabled={applyLoading}
                  leftSection={applyLoading ? <Loader size="xs" /> : <IconCheck size={16} />}
                  color="green"
                >
                  {applyLoading ? 'Applying...' : 'Apply Slicing'}
                </Button>
              )}
            </Group>
          </Group>

          {/* Progress Indicator */}
          <Text size="xs" c="dimmed" ta="center" mt="md">
            Step {wizardState.currentStep + 1} of 4
          </Text>
        </>
      )}
    </Modal>
  );
}
