/**
 * FHIR primitive and complex types with their inheritance
 */
export const FHIR_TYPE_HIERARCHY: Record<
  string,
  { parent?: string; isPrimitive: boolean }
> = {
  // Primitive types
  boolean: { isPrimitive: true },
  integer: { parent: 'decimal', isPrimitive: true },
  string: { isPrimitive: true },
  decimal: { isPrimitive: true },
  uri: { isPrimitive: true },
  url: { parent: 'uri', isPrimitive: true },
  canonical: { parent: 'uri', isPrimitive: true },
  base64Binary: { isPrimitive: true },
  instant: { isPrimitive: true },
  date: { isPrimitive: true },
  dateTime: { isPrimitive: true },
  time: { isPrimitive: true },
  code: { parent: 'string', isPrimitive: true },
  oid: { parent: 'uri', isPrimitive: true },
  id: { parent: 'string', isPrimitive: true },
  markdown: { parent: 'string', isPrimitive: true },
  unsignedInt: { parent: 'integer', isPrimitive: true },
  positiveInt: { parent: 'integer', isPrimitive: true },
  uuid: { parent: 'uri', isPrimitive: true },

  // Complex types
  Quantity: { isPrimitive: false },
  SimpleQuantity: { parent: 'Quantity', isPrimitive: false },
  Duration: { parent: 'Quantity', isPrimitive: false },
  Distance: { parent: 'Quantity', isPrimitive: false },
  Count: { parent: 'Quantity', isPrimitive: false },
  Money: { parent: 'Quantity', isPrimitive: false },
  Age: { parent: 'Quantity', isPrimitive: false },
  Range: { isPrimitive: false },
  Period: { isPrimitive: false },
  Ratio: { isPrimitive: false },
  RatioRange: { isPrimitive: false },
  Coding: { isPrimitive: false },
  CodeableConcept: { isPrimitive: false },
  Identifier: { isPrimitive: false },
  HumanName: { isPrimitive: false },
  Address: { isPrimitive: false },
  ContactPoint: { isPrimitive: false },
  Reference: { isPrimitive: false },
  Attachment: { isPrimitive: false },
  Annotation: { isPrimitive: false },
  Signature: { isPrimitive: false },
  SampledData: { isPrimitive: false },
};

/**
 * Get all parent types of a given type
 */
export function getParentTypes(typeCode: string): string[] {
  const parents: string[] = [];
  let current = typeCode;

  while (FHIR_TYPE_HIERARCHY[current]?.parent) {
    const parent = FHIR_TYPE_HIERARCHY[current].parent!;
    parents.push(parent);
    current = parent;
  }

  return parents;
}

/**
 * Check if childType is a subtype of parentType
 */
export function isSubtype(childType: string, parentType: string): boolean {
  if (childType === parentType) return true;
  const parents = getParentTypes(childType);
  return parents.includes(parentType);
}

/**
 * Get all subtypes of a given type
 */
export function getSubtypes(parentType: string): string[] {
  const subtypes: string[] = [];

  Object.entries(FHIR_TYPE_HIERARCHY).forEach(([typeCode, info]) => {
    if (info.parent === parentType) {
      subtypes.push(typeCode);
      // Recursively get subtypes of subtypes
      subtypes.push(...getSubtypes(typeCode));
    }
  });

  return subtypes;
}
