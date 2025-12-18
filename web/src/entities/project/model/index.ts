import { createStore } from 'effector';

export interface Project {
  id: string;
  name: string;
}

// Placeholder store for current project
export const $currentProject = createStore<Project | null>(null);
