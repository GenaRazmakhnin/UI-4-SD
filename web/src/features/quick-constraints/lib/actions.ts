import type { ElementNode } from '@shared/types';
import type { QuickAction, QuickActionId } from './types';

/**
 * Check if element can be made required (min=0 → min=1)
 */
function canMakeRequired(element: ElementNode): boolean {
  return element.min === 0;
}

/**
 * Check if element can be made optional (min=1 → min=0)
 */
function canMakeOptional(element: ElementNode): boolean {
  return element.min > 0;
}

/**
 * Check if element can allow multiple (max=1 → max=*)
 */
function canAllowMultiple(element: ElementNode): boolean {
  return element.max === '1';
}

/**
 * Check if element can be prohibited (max → 0)
 */
function canProhibit(element: ElementNode): boolean {
  return element.max !== '0';
}

/**
 * Check if element has multiple types that can be constrained
 */
function hasMultipleTypes(element: ElementNode): boolean {
  return (element.type?.length ?? 0) > 1;
}

/**
 * Check if element is a primitive type
 */
function isPrimitiveType(element: ElementNode): boolean {
  const primitives = [
    'boolean',
    'integer',
    'string',
    'decimal',
    'uri',
    'url',
    'canonical',
    'base64Binary',
    'instant',
    'date',
    'dateTime',
    'time',
    'code',
    'oid',
    'id',
    'markdown',
    'unsignedInt',
    'positiveInt',
    'uuid',
  ];
  const typeCode = element.type?.[0]?.code;
  return typeCode ? primitives.includes(typeCode) : false;
}

/**
 * Check if element can have a binding
 */
function canHaveBinding(element: ElementNode): boolean {
  const bindableTypes = ['code', 'Coding', 'CodeableConcept', 'Quantity', 'string', 'uri'];
  const typeCode = element.type?.[0]?.code;
  return typeCode ? bindableTypes.includes(typeCode) : false;
}

/**
 * All available quick actions
 */
export const QUICK_ACTIONS: QuickAction[] = [
  // Cardinality actions
  {
    id: 'make-required',
    label: 'Make Required',
    shortLabel: 'Required',
    description: 'Change cardinality from 0..n to 1..n',
    icon: 'IconAsterisk',
    shortcut: 'r',
    category: 'cardinality',
    isAvailable: canMakeRequired,
  },
  {
    id: 'make-optional',
    label: 'Make Optional',
    shortLabel: 'Optional',
    description: 'Change cardinality from 1..n to 0..n',
    icon: 'IconQuestionMark',
    shortcut: 'o',
    category: 'cardinality',
    isAvailable: canMakeOptional,
  },
  {
    id: 'allow-multiple',
    label: 'Allow Multiple',
    shortLabel: 'Multiple',
    description: 'Change maximum from 1 to *',
    icon: 'IconCopy',
    shortcut: 'm',
    category: 'cardinality',
    isAvailable: canAllowMultiple,
  },
  {
    id: 'make-prohibited',
    label: 'Prohibit Element',
    shortLabel: 'Prohibit',
    description: 'Set maximum to 0 (remove from profile)',
    icon: 'IconBan',
    shortcut: 'x',
    category: 'cardinality',
    requiresConfirmation: true,
    isAvailable: canProhibit,
  },

  // Flag actions
  {
    id: 'toggle-must-support',
    label: 'Toggle Must Support',
    shortLabel: 'MustSupport',
    description: 'Toggle the mustSupport flag',
    icon: 'IconStar',
    shortcut: 's',
    category: 'flags',
    isAvailable: () => true,
    isActive: (element) => element.mustSupport === true,
  },
  {
    id: 'toggle-is-modifier',
    label: 'Toggle isModifier',
    shortLabel: 'Modifier',
    description: 'Toggle the isModifier flag',
    icon: 'IconAlertTriangle',
    category: 'flags',
    isAvailable: () => true,
    isActive: (element) => element.isModifier === true,
  },
  {
    id: 'toggle-is-summary',
    label: 'Toggle isSummary',
    shortLabel: 'Summary',
    description: 'Toggle the isSummary flag',
    icon: 'IconList',
    category: 'flags',
    isAvailable: () => true,
    isActive: (element) => element.isSummary === true,
  },

  // Constraint actions
  {
    id: 'set-binding',
    label: 'Set Binding',
    shortLabel: 'Binding',
    description: 'Set or change terminology binding',
    icon: 'IconLink',
    shortcut: 'b',
    category: 'constraints',
    isAvailable: canHaveBinding,
    isActive: (element) => element.binding !== undefined,
  },
  {
    id: 'add-extension',
    label: 'Add Extension',
    shortLabel: 'Extension',
    description: 'Add an extension to this element',
    icon: 'IconPlug',
    shortcut: 'e',
    category: 'constraints',
    isAvailable: () => true,
  },
  {
    id: 'create-slice',
    label: 'Create Slice',
    shortLabel: 'Slice',
    description: 'Create a new slice for this element',
    icon: 'IconCut',
    shortcut: 'l',
    category: 'constraints',
    isAvailable: (element) => element.max !== '1' && element.max !== '0',
    isActive: (element) => element.slicing !== undefined,
  },
  {
    id: 'constrain-type',
    label: 'Constrain Type',
    shortLabel: 'Type',
    description: 'Constrain to a specific type',
    icon: 'IconFilter',
    shortcut: 't',
    category: 'constraints',
    isAvailable: hasMultipleTypes,
  },
  {
    id: 'add-pattern',
    label: 'Add Pattern',
    shortLabel: 'Pattern',
    description: 'Add a pattern constraint',
    icon: 'IconTemplate',
    shortcut: 'p',
    category: 'constraints',
    isAvailable: isPrimitiveType,
  },
  {
    id: 'add-fixed-value',
    label: 'Add Fixed Value',
    shortLabel: 'Fixed',
    description: 'Set a fixed value for this element',
    icon: 'IconLock',
    shortcut: 'f',
    category: 'constraints',
    requiresConfirmation: true,
    isAvailable: isPrimitiveType,
  },
  {
    id: 'add-invariant',
    label: 'Add Invariant',
    shortLabel: 'Invariant',
    description: 'Add a constraint/invariant rule',
    icon: 'IconShield',
    shortcut: 'i',
    category: 'constraints',
    isAvailable: () => true,
  },
];

/**
 * Get action by ID
 */
export function getAction(id: QuickActionId): QuickAction | undefined {
  return QUICK_ACTIONS.find((action) => action.id === id);
}

/**
 * Get available actions for an element
 */
export function getAvailableActions(element: ElementNode): QuickAction[] {
  return QUICK_ACTIONS.filter((action) => action.isAvailable(element));
}

/**
 * Get actions by category
 */
export function getActionsByCategory(
  category: QuickAction['category'],
  element?: ElementNode
): QuickAction[] {
  const actions = QUICK_ACTIONS.filter((action) => action.category === category);
  if (element) {
    return actions.filter((action) => action.isAvailable(element));
  }
  return actions;
}
