import { createEvent, createStore, sample } from 'effector';
import { persist } from 'effector-storage/local';

export interface PanelSizes {
  leftPanel: number;
  rightPanel: number;
  bottomPanel: number;
}

export interface PanelCollapsed {
  leftPanel: boolean;
  rightPanel: boolean;
  bottomPanel: boolean;
}

export interface PanelState {
  sizes: PanelSizes;
  collapsed: PanelCollapsed;
}

const DEFAULT_PANEL_SIZES: PanelSizes = {
  leftPanel: 20,
  rightPanel: 30,
  bottomPanel: 30,
};

const DEFAULT_COLLAPSED: PanelCollapsed = {
  leftPanel: false,
  rightPanel: false,
  bottomPanel: false,
};

// Events
export const leftPanelResized = createEvent<number>();
export const rightPanelResized = createEvent<number>();
export const bottomPanelResized = createEvent<number>();

export const leftPanelToggled = createEvent();
export const rightPanelToggled = createEvent();
export const bottomPanelToggled = createEvent();

export const panelStateReset = createEvent();

// Stores
export const $panelSizes = createStore<PanelSizes>(DEFAULT_PANEL_SIZES);
export const $panelCollapsed = createStore<PanelCollapsed>(DEFAULT_COLLAPSED);

// Derived stores
export const $leftPanelSize = $panelSizes.map((s) => s.leftPanel);
export const $rightPanelSize = $panelSizes.map((s) => s.rightPanel);
export const $bottomPanelSize = $panelSizes.map((s) => s.bottomPanel);

export const $isLeftPanelCollapsed = $panelCollapsed.map((c) => c.leftPanel);
export const $isRightPanelCollapsed = $panelCollapsed.map((c) => c.rightPanel);
export const $isBottomPanelCollapsed = $panelCollapsed.map((c) => c.bottomPanel);

// Reducers
$panelSizes
  .on(leftPanelResized, (state, size) => ({ ...state, leftPanel: size }))
  .on(rightPanelResized, (state, size) => ({ ...state, rightPanel: size }))
  .on(bottomPanelResized, (state, size) => ({ ...state, bottomPanel: size }))
  .reset(panelStateReset);

$panelCollapsed
  .on(leftPanelToggled, (state) => ({ ...state, leftPanel: !state.leftPanel }))
  .on(rightPanelToggled, (state) => ({ ...state, rightPanel: !state.rightPanel }))
  .on(bottomPanelToggled, (state) => ({ ...state, bottomPanel: !state.bottomPanel }))
  .reset(panelStateReset);

// Persist to localStorage
persist({
  store: $panelSizes,
  key: 'profile-editor:panel-sizes',
});

persist({
  store: $panelCollapsed,
  key: 'profile-editor:panel-collapsed',
});
