export { CardinalityEditor } from './ui/CardinalityEditor';
export {
  cardinalityChanged,
  cardinalityEditCancelled,
  $isEditingCardinality,
  $cardinalityValidation,
} from './model';
export {
  validateCardinality,
  getImpactMessage,
  parseMaxToNumber,
} from './lib/validation';
export type { CardinalityValidation } from './lib/validation';
