export type { CardinalityValidation } from './lib/validation';
export {
  getImpactMessage,
  parseMaxToNumber,
  validateCardinality,
} from './lib/validation';
export {
  $cardinalityValidation,
  $isEditingCardinality,
  cardinalityChanged,
  cardinalityEditCancelled,
} from './model';
export { CardinalityEditor } from './ui/CardinalityEditor';
