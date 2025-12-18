export { TypeConstraintEditor } from './ui/TypeConstraintEditor';
export {
  typeConstraintChanged,
  targetProfileAdded,
  targetProfileRemoved,
  searchProfilesFx,
} from './model';
export {
  validateTypeConstraints,
  getRecommendedTypeConstraints,
} from './lib/validation';
export { isSubtype, getParentTypes, getSubtypes } from './lib/type-hierarchy';
export type { TypeValidation } from './lib/validation';
