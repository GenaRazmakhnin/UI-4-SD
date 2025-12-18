import { api } from '@shared/api';
import type { Project } from '@shared/types';
import { createEffect, createEvent, createStore, sample } from 'effector';
import { persist } from 'effector-storage/local';

export const $currentProject = createStore<Project | null>(null);
export const $lastProjectId = createStore<string | null>(null);

export const projectSelected = createEvent<Project>();
export const projectCleared = createEvent();

export const fetchProjectFx = createEffect(async (projectId: string) => {
  const project = await api.projects.get(projectId);
  return project;
});

$currentProject.on(projectSelected, (_, project) => project);
$currentProject.on(projectCleared, () => null);
$currentProject.on(fetchProjectFx.fail, () => null);

$lastProjectId.on(projectSelected, (_, project) => project.id);
$lastProjectId.on(projectCleared, () => null);
$lastProjectId.on(fetchProjectFx.fail, () => null);

sample({
  clock: fetchProjectFx.doneData,
  target: projectSelected,
});

persist({
  store: $lastProjectId,
  key: 'last-project-id',
});
