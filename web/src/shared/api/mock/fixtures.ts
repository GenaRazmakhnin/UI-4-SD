import type {
  BaseResource,
  CreateArtifactInput,
  CreatedArtifact,
  ElementNode,
  Extension,
  Package,
  Profile,
  Project,
  ProjectResourceKind,
  ProjectResourceMetadata,
  ProjectTreeNode,
  ProjectTreeRoot,
  ValidationResult,
  ValueSet,
  ValueSetExpansion,
} from '@shared/types';

// FHIR R4 Core Patient Profile (Base Resource)
export const fhirCorePatient: Profile = {
  id: 'patient',
  url: 'http://hl7.org/fhir/StructureDefinition/Patient',
  name: 'Patient',
  title: 'Patient',
  status: 'active',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/DomainResource',
  derivation: 'specialization',
  description:
    'Demographics and other administrative information about an individual or animal receiving care or other health-related services.',
  elements: [
    {
      id: 'Patient',
      path: 'Patient',
      short: 'Information about an individual or animal receiving health care services',
      definition:
        'Demographics and other administrative information about an individual or animal receiving care or other health-related services.',
      min: 0,
      max: '*',
      isModified: false,
      children: [
        {
          id: 'Patient.identifier',
          path: 'Patient.identifier',
          short: 'An identifier for this patient',
          definition: 'An identifier for this patient.',
          min: 0,
          max: '*',
          type: [{ code: 'Identifier' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.active',
          path: 'Patient.active',
          short: 'Whether this patient record is in active use',
          definition:
            'Whether this patient record is in active use. Many systems use this property to mark as non-current patients.',
          min: 0,
          max: '1',
          type: [{ code: 'boolean' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.name',
          path: 'Patient.name',
          short: 'A name associated with the patient',
          definition: 'A name associated with the individual.',
          min: 0,
          max: '*',
          type: [{ code: 'HumanName' }],
          isModified: false,
          children: [
            {
              id: 'Patient.name.use',
              path: 'Patient.name.use',
              short: 'usual | official | temp | nickname | anonymous | old | maiden',
              min: 0,
              max: '1',
              type: [{ code: 'code' }],
              binding: {
                strength: 'required',
                valueSet: 'http://hl7.org/fhir/ValueSet/name-use',
              },
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.name.text',
              path: 'Patient.name.text',
              short: 'Text representation of the full name',
              min: 0,
              max: '1',
              type: [{ code: 'string' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.name.family',
              path: 'Patient.name.family',
              short: 'Family name (often called surname)',
              min: 0,
              max: '1',
              type: [{ code: 'string' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.name.given',
              path: 'Patient.name.given',
              short: 'Given names (not always first)',
              min: 0,
              max: '*',
              type: [{ code: 'string' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.name.prefix',
              path: 'Patient.name.prefix',
              short: 'Parts that come before the name',
              min: 0,
              max: '*',
              type: [{ code: 'string' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.name.suffix',
              path: 'Patient.name.suffix',
              short: 'Parts that come after the name',
              min: 0,
              max: '*',
              type: [{ code: 'string' }],
              isModified: false,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.telecom',
          path: 'Patient.telecom',
          short: 'A contact detail for the individual',
          definition:
            'A contact detail (e.g. a telephone number or an email address) by which the individual may be contacted.',
          min: 0,
          max: '*',
          type: [{ code: 'ContactPoint' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.gender',
          path: 'Patient.gender',
          short: 'male | female | other | unknown',
          definition:
            'Administrative Gender - the gender that the patient is considered to have for administration and record keeping purposes.',
          min: 0,
          max: '1',
          type: [{ code: 'code' }],
          binding: {
            strength: 'required',
            valueSet: 'http://hl7.org/fhir/ValueSet/administrative-gender',
          },
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.birthDate',
          path: 'Patient.birthDate',
          short: 'The date of birth for the individual',
          definition: 'The date of birth for the individual.',
          min: 0,
          max: '1',
          type: [{ code: 'date' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.deceased[x]',
          path: 'Patient.deceased[x]',
          short: 'Indicates if the individual is deceased or not',
          definition: 'Indicates if the individual is deceased or not.',
          min: 0,
          max: '1',
          type: [{ code: 'boolean' }, { code: 'dateTime' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.address',
          path: 'Patient.address',
          short: 'An address for the individual',
          definition: 'An address for the individual.',
          min: 0,
          max: '*',
          type: [{ code: 'Address' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.maritalStatus',
          path: 'Patient.maritalStatus',
          short: 'Marital (civil) status of a patient',
          definition: "This field contains a patient's most recent marital (civil) status.",
          min: 0,
          max: '1',
          type: [{ code: 'CodeableConcept' }],
          binding: {
            strength: 'extensible',
            valueSet: 'http://hl7.org/fhir/ValueSet/marital-status',
          },
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.multipleBirth[x]',
          path: 'Patient.multipleBirth[x]',
          short: 'Whether patient is part of a multiple birth',
          definition:
            'Indicates whether the patient is part of a multiple (boolean) or indicates the actual birth order (integer).',
          min: 0,
          max: '1',
          type: [{ code: 'boolean' }, { code: 'integer' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.photo',
          path: 'Patient.photo',
          short: 'Image of the patient',
          definition: 'Image of the patient.',
          min: 0,
          max: '*',
          type: [{ code: 'Attachment' }],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.contact',
          path: 'Patient.contact',
          short: 'A contact party (e.g. guardian, partner, friend) for the patient',
          definition: 'A contact party (e.g. guardian, partner, friend) for the patient.',
          min: 0,
          max: '*',
          type: [{ code: 'BackboneElement' }],
          isModified: false,
          children: [
            {
              id: 'Patient.contact.relationship',
              path: 'Patient.contact.relationship',
              short: 'The kind of relationship',
              min: 0,
              max: '*',
              type: [{ code: 'CodeableConcept' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.contact.name',
              path: 'Patient.contact.name',
              short: 'A name associated with the contact person',
              min: 0,
              max: '1',
              type: [{ code: 'HumanName' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.contact.telecom',
              path: 'Patient.contact.telecom',
              short: 'A contact detail for the person',
              min: 0,
              max: '*',
              type: [{ code: 'ContactPoint' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.contact.address',
              path: 'Patient.contact.address',
              short: 'Address for the contact person',
              min: 0,
              max: '1',
              type: [{ code: 'Address' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.contact.gender',
              path: 'Patient.contact.gender',
              short: 'male | female | other | unknown',
              min: 0,
              max: '1',
              type: [{ code: 'code' }],
              binding: {
                strength: 'required',
                valueSet: 'http://hl7.org/fhir/ValueSet/administrative-gender',
              },
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.contact.organization',
              path: 'Patient.contact.organization',
              short: 'Organization that is associated with the contact',
              min: 0,
              max: '1',
              type: [
                {
                  code: 'Reference',
                  targetProfile: ['http://hl7.org/fhir/StructureDefinition/Organization'],
                },
              ],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.contact.period',
              path: 'Patient.contact.period',
              short: 'The period during which this contact person is valid',
              min: 0,
              max: '1',
              type: [{ code: 'Period' }],
              isModified: false,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.communication',
          path: 'Patient.communication',
          short: 'A language which may be used to communicate with the patient',
          definition:
            'A language which may be used to communicate with the patient about his or her health.',
          min: 0,
          max: '*',
          type: [{ code: 'BackboneElement' }],
          isModified: false,
          children: [
            {
              id: 'Patient.communication.language',
              path: 'Patient.communication.language',
              short: 'The language which can be used to communicate',
              min: 1,
              max: '1',
              type: [{ code: 'CodeableConcept' }],
              binding: {
                strength: 'preferred',
                valueSet: 'http://hl7.org/fhir/ValueSet/languages',
              },
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.communication.preferred',
              path: 'Patient.communication.preferred',
              short: 'Language preference indicator',
              min: 0,
              max: '1',
              type: [{ code: 'boolean' }],
              isModified: false,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.generalPractitioner',
          path: 'Patient.generalPractitioner',
          short: "Patient's nominated primary care provider",
          definition: "Patient's nominated care provider.",
          min: 0,
          max: '*',
          type: [
            {
              code: 'Reference',
              targetProfile: ['http://hl7.org/fhir/StructureDefinition/Organization'],
            },
            {
              code: 'Reference',
              targetProfile: ['http://hl7.org/fhir/StructureDefinition/Practitioner'],
            },
            {
              code: 'Reference',
              targetProfile: ['http://hl7.org/fhir/StructureDefinition/PractitionerRole'],
            },
          ],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.managingOrganization',
          path: 'Patient.managingOrganization',
          short: 'Organization that is the custodian of the patient record',
          definition: 'Organization that is the custodian of the patient record.',
          min: 0,
          max: '1',
          type: [
            {
              code: 'Reference',
              targetProfile: ['http://hl7.org/fhir/StructureDefinition/Organization'],
            },
          ],
          isModified: false,
          children: [],
        },
        {
          id: 'Patient.link',
          path: 'Patient.link',
          short: 'Link to another patient resource that concerns the same actual person',
          definition: 'Link to another patient resource that concerns the same actual patient.',
          min: 0,
          max: '*',
          type: [{ code: 'BackboneElement' }],
          isModified: false,
          children: [
            {
              id: 'Patient.link.other',
              path: 'Patient.link.other',
              short: 'The other patient or related person resource',
              min: 1,
              max: '1',
              type: [
                {
                  code: 'Reference',
                  targetProfile: ['http://hl7.org/fhir/StructureDefinition/Patient'],
                },
                {
                  code: 'Reference',
                  targetProfile: ['http://hl7.org/fhir/StructureDefinition/RelatedPerson'],
                },
              ],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.link.type',
              path: 'Patient.link.type',
              short: 'replaced-by | replaces | refer | seealso',
              min: 1,
              max: '1',
              type: [{ code: 'code' }],
              binding: {
                strength: 'required',
                valueSet: 'http://hl7.org/fhir/ValueSet/link-type',
              },
              isModified: false,
              children: [],
            },
          ],
        },
      ],
    },
  ],
  isDirty: false,
};

// Mock Profile: US Core Patient (Simple)
export const usCorePatient: Profile = {
  id: 'us-core-patient',
  url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient',
  name: 'USCorePatientProfile',
  title: 'US Core Patient Profile',
  status: 'active',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
  derivation: 'constraint',
  description:
    'Defines constraints and extensions on the Patient resource for the minimal set of data to query and retrieve patient demographic information.',
  elements: [
    {
      id: 'Patient',
      path: 'Patient',
      min: 0,
      max: '*',
      isModified: false,
      children: [
        {
          id: 'Patient.extension',
          path: 'Patient.extension',
          min: 2,
          max: '*',
          short: 'Patient extensions (sliced by URL pattern)',
          slicing: {
            discriminator: [{ type: 'pattern', path: 'url' }],
            rules: 'open',
            ordered: false,
          },
          isModified: true,
          children: [
            {
              id: 'Patient.extension:ethnicity',
              path: 'Patient.extension',
              sliceName: 'ethnicity',
              min: 1,
              max: '1',
              short: '(USCDI) US Core ethnicity Extension',
              isModified: true,
              children: [
                {
                  id: 'Patient.extension:ethnicity.url',
                  path: 'Patient.extension.url',
                  min: 1,
                  max: '1',
                  short: 'Extension URL (pattern-sliced)',
                  definition:
                    'Slice discriminated by pattern on url = http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity',
                  type: [{ code: 'uri' }],
                  isModified: true,
                  children: [],
                },
                {
                  id: 'Patient.extension:ethnicity.value[x]',
                  path: 'Patient.extension.value[x]',
                  min: 0,
                  max: '1',
                  short: 'Ethnicity value',
                  type: [{ code: 'CodeableConcept' }],
                  isModified: true,
                  children: [],
                },
              ],
            },
            {
              id: 'Patient.extension:race',
              path: 'Patient.extension',
              sliceName: 'race',
              min: 1,
              max: '1',
              short: '(USCDI) US Core Race Extension',
              isModified: true,
              children: [
                {
                  id: 'Patient.extension:race.url',
                  path: 'Patient.extension.url',
                  min: 1,
                  max: '1',
                  short: 'Extension URL (pattern-sliced)',
                  definition:
                    'Slice discriminated by pattern on url = http://hl7.org/fhir/us/core/StructureDefinition/us-core-race',
                  type: [{ code: 'uri' }],
                  isModified: true,
                  children: [],
                },
                {
                  id: 'Patient.extension:race.value[x]',
                  path: 'Patient.extension.value[x]',
                  min: 0,
                  max: '1',
                  short: 'Race value',
                  type: [{ code: 'CodeableConcept' }],
                  isModified: true,
                  children: [],
                },
              ],
            }
          ]
        },
        {
          id: 'Patient.identifier',
          path: 'Patient.identifier',
          min: 1,
          max: '*',
          short: 'An identifier for this patient',
          definition: 'An identifier for this patient.',
          mustSupport: true,
          isModified: true,
          children: [],
        },
        {
          id: 'Patient.name',
          path: 'Patient.name',
          min: 1,
          max: '*',
          short: 'A name associated with the patient',
          mustSupport: true,
          isModified: true,
          children: [
            {
              id: 'Patient.name.family',
              path: 'Patient.name.family',
              min: 1,
              max: '1',
              short: 'Family name',
              mustSupport: true,
              isModified: true,
              children: [],
            },
            {
              id: 'Patient.name.given',
              path: 'Patient.name.given',
              min: 1,
              max: '*',
              short: 'Given names',
              mustSupport: true,
              isModified: true,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.gender',
          path: 'Patient.gender',
          min: 1,
          max: '1',
          short: 'male | female | other | unknown',
          mustSupport: true,
          isModified: true,
          binding: {
            strength: 'required',
            valueSet: 'http://hl7.org/fhir/ValueSet/administrative-gender',
          },
          children: [],
        },
      ],
    },
  ],
  isDirty: false,
};

// Mock Profile: Chile Core Patient (CL Core)
export const clCorePatient: Profile = {
  id: 'cl-core-patient',
  url: 'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/CorePacienteCl',
  name: 'CorePacienteCl',
  title: 'Core Paciente CL',
  status: 'active',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
  derivation: 'constraint',
  elements: [
    {
      id: 'Patient',
      path: 'Patient',
      min: 0,
      max: '*',
      isModified: false,
      children: [
        {
          id: 'Patient.extension',
          path: 'Patient.extension',
          min: 0,
          max: '*',
          short: 'Extensiones de paciente (slicing por URL)',
          slicing: {
            discriminator: [{ path: 'url', type: 'value' }],
            rules: 'open',
            ordered: false,
          },
          isModified: true,
          children: [
            {
              id: 'Patient.extension:nacionalidad',
              path: 'Patient.extension',
              sliceName: 'nacionalidad',
              min: 0,
              max: '2147483647',
              short: 'Nacionalidad (Código de País)',
              isModified: true,
              children: [
                {
                  id: 'Patient.extension:nacionalidad.url',
                  path: 'Patient.extension.url',
                  min: 1,
                  max: '1',
                  short: 'URL de extensión nacionalidad',
                  definition:
                    'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/CodigoPaises',
                  type: [{ code: 'uri' }],
                  isModified: true,
                  children: [],
                },
              ],
            },
            {
              id: 'Patient.extension:SexoBiologico',
              path: 'Patient.extension',
              sliceName: 'SexoBiologico',
              min: 0,
              max: '1',
              short: 'Extensión Sexo Biológico',
              isModified: true,
              children: [
                {
                  id: 'Patient.extension:SexoBiologico.url',
                  path: 'Patient.extension.url',
                  min: 1,
                  max: '1',
                  short: 'URL de extensión Sexo Biológico',
                  definition:
                    'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/SexoBiologico',
                  type: [{ code: 'uri' }],
                  isModified: true,
                  children: [],
                },
              ],
            },
            {
              id: 'Patient.extension:IdentidadDeGenero',
              path: 'Patient.extension',
              sliceName: 'IdentidadDeGenero',
              min: 0,
              max: '1',
              short: 'Extensión Identidad de Género',
              isModified: true,
              children: [
                {
                  id: 'Patient.extension:IdentidadDeGenero.url',
                  path: 'Patient.extension.url',
                  min: 1,
                  max: '1',
                  short: 'URL de extensión Identidad de Género',
                  definition:
                    'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/IdentidadDeGenero',
                  type: [{ code: 'uri' }],
                  isModified: true,
                  children: [],
                },
              ],
            },
          ],
        },
        {
          id: 'Patient.gender',
          path: 'Patient.gender',
          min: 0,
          max: '1',
          short: 'Sexo Registrado. (male | female | other | unknown)',
          definition: 'Sexo Registrado',
          mustSupport: true,
          isModified: true,
          children: [],
        },
        {
          id: 'Patient.name',
          path: 'Patient.name',
          min: 0,
          max: '*',
          short:
            'Nombres y Apellidos del Paciente considerando, según el caso: 1er Nombre, Nombres, 1er Apellido y 2o Apellido',
          definition:
            'Nombre del Paciente considerando, según el caso: 1er Nombre, Nombres, 1er Apellido y 2o Apellido',
          slicing: {
            discriminator: [{ path: 'use', type: 'value' }],
            rules: 'open',
            ordered: false,
            description:
              'Este slice se genera para diferenciar el nombre registrado Versus el nombre social',
          },
          isModified: true,
          children: [
            {
              id: 'Patient.name.use',
              path: 'Patient.name.use',
              min: 0,
              max: '1',
              short: "usual | official | temp | nickname | anonymous | old | maiden",
              definition: 'Identifies the purpose for this name.',
              comment:
                "Applications can assume that a name is current unless it explicitly says that it is temporary or old.",
              binding: {
                strength: 'required',
                valueSet: 'http://hl7.org/fhir/ValueSet/name-use',
              },
              type: [{ code: 'code' }],
              isModified: false,
              children: [],
            },
            {
              id: 'Patient.name.text',
              path: 'Patient.name.text',
              min: 0,
              max: '1',
              short: 'Text representation of the full name',
              definition:
                'Specifies the entire name as it should be displayed e.g. on an application UI.',
              isModified: false,
              type: [{ code: 'string' }],
              children: [],
            },
            {
              id: 'Patient.name.given',
              path: 'Patient.name.given',
              min: 0,
              max: '*',
              short: "Given names (not always 'first'). Includes middle names",
              definition: 'Given name.',
              isModified: false,
              type: [{ code: 'string' }],
              children: [],
            },
            {
              id: 'Patient.name.family',
              path: 'Patient.name.family',
              min: 0,
              max: '1',
              short: "Family name (often called 'Surname')",
              definition:
                'The part of a name that links to the genealogy. In some cultures the family name of a son is the first name of his father.',
              isModified: false,
              type: [{ code: 'string' }],
              children: [],
            },
            {
              id: 'Patient.name.prefix',
              path: 'Patient.name.prefix',
              min: 0,
              max: '*',
              short: 'Parts that come before the name',
              definition:
                'Title or part of the name that appears at the start (academic, legal, nobility, etc.).',
              isModified: false,
              type: [{ code: 'string' }],
              children: [],
            },
            {
              id: 'Patient.name.suffix',
              path: 'Patient.name.suffix',
              min: 0,
              max: '*',
              short: 'Parts that come after the name',
              definition:
                'Part of the name acquired as a title that appears at the end of the name.',
              isModified: false,
              type: [{ code: 'string' }],
              children: [],
            },
            {
              id: 'Patient.name.period',
              path: 'Patient.name.period',
              min: 0,
              max: '1',
              short: 'Time period when name was/is in use',
              definition: 'Indicates the period of time when this name was valid for the person.',
              isModified: false,
              type: [{ code: 'Period' }],
              children: [],
            },
            {
              id: 'Patient.name:NombreSocial',
              path: 'Patient.name',
              sliceName: 'NombreSocial',
              min: 0,
              max: '1',
              short: 'Nombre social del paciente',
              definition: 'Nombre con el cual se identifica al paciente sin ser este oficial.',
              mustSupport: true,
              isModified: true,
              children: [
                {
                  id: 'Patient.name:NombreSocial.use',
                  path: 'Patient.name.use',
                  min: 1,
                  max: '1',
                  short: 'Uso del nombre',
                  comment:
                    'Para ser considerado como nombre social, el use DEBE ser "usual".',
                  definition:
                    'Uso se fuerza a usual para identificar el slice de nombre social.',
                  mustSupport: true,
                  isModified: true,
                  children: [],
                },
                {
                  id: 'Patient.name:NombreSocial.given',
                  path: 'Patient.name.given',
                  min: 1,
                  max: '*',
                  short: 'Nombre Social',
                  definition: 'Nombre Social del Paciente',
                  mustSupport: true,
                  isModified: true,
                  children: [],
                },
              ],
            },
            {
              id: 'Patient.name:NombreOficial',
              path: 'Patient.name',
              sliceName: 'NombreOficial',
              min: 0,
              max: '1',
              short: 'Nombre registrado oficialmente',
              definition: 'Determinación del nombre registrado oficialmente del Paciente',
              mustSupport: true,
              isModified: true,
              children: [
                {
                  id: 'Patient.name:NombreOficial.use',
                  path: 'Patient.name.use',
                  min: 1,
                  max: '1',
                  short: 'Uso del nombre oficial',
                  comment:
                    'Para ser considerado como nombre oficial, el use DEBE ser "official".',
                  definition:
                    'Slice corresponde al nombre registrado al nacer, use se fuerza a official.',
                  mustSupport: true,
                  isModified: true,
                  children: [],
                },
                {
                  id: 'Patient.name:NombreOficial.given',
                  path: 'Patient.name.given',
                  min: 1,
                  max: '*',
                  short: 'Primer nombre y nombres del Paciente',
                  definition:
                    'Todos los nombres de los pacientes no necesariamente solo el primer nombre.',
                  mustSupport: true,
                  isModified: true,
                  children: [],
                },
                {
                  id: 'Patient.name:NombreOficial.family',
                  path: 'Patient.name.family',
                  min: 1,
                  max: '1',
                  short: '1er Apellido',
                  definition:
                    'Primer apellido registrado al nacer o inscrito legalmente en el Registro Civil.',
                  mustSupport: true,
                  isModified: true,
                  children: [
                    {
                      id: 'Patient.name:NombreOficial.family.extension',
                      path: 'Patient.name.family.extension',
                      min: 0,
                      max: '*',
                      short: 'Extensión para el segundo apellido',
                      slicing: {
                        discriminator: [{ path: 'url', type: 'value' }],
                        rules: 'open',
                        ordered: false,
                      },
                      isModified: true,
                      children: [
                        {
                          id: 'Patient.name:NombreOficial.family.extension:segundoApellido',
                          path: 'Patient.name.family.extension',
                          sliceName: 'segundoApellido',
                          min: 0,
                          max: '1',
                          short: 'Extensión segundo apellido',
                          definition:
                            'Extensión para la declaración de un segundo apellido.',
                          isModified: true,
                          children: [
                            {
                              id: 'Patient.name:NombreOficial.family.extension:segundoApellido.url',
                              path: 'Patient.name.family.extension.url',
                              min: 1,
                              max: '1',
                              short: 'URL de extensión segundo apellido',
                              definition:
                                'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/SegundoApellido',
                              type: [{ code: 'uri' }],
                              isModified: true,
                              children: [],
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
          ],
        },
        {
          id: 'Patient.address',
          path: 'Patient.address',
          min: 0,
          max: '*',
          type: [{ code: 'Address' }],
          mustSupport: true,
          isModified: true,
          children: [],
        },
        {
          id: 'Patient.identifier',
          path: 'Patient.identifier',
          min: 0,
          max: '*',
          short:
            'Listados de Id de Paciente. De poseer una CI con RUN vigente, este DEBE ser ingresado',
          definition:
            'Este es el listado de Identificaciones de un paciente. Se procura como R2 el RUN, pero en caso de no existir ese identificador se debe ocupar otro nacional u otro otorgado por país extranjero',
          comment:
            'La identificación implica el ingreso del tipo de documento, el país de origen y el valor.',
          mustSupport: true,
          isModified: true,
          children: [
            {
              id: 'Patient.identifier.use',
              path: 'Patient.identifier.use',
              min: 0,
              max: '1',
              comment:
                'Se definirá como official en primera etapa; se abrirá en siguientes iteraciones.',
              definition:
                'De contar el Paciente con una Cédula de Identidad Nacional, se sugiere el uso de esta como identificador.',
              mustSupport: true,
              isModified: true,
              children: [],
            },
            {
              id: 'Patient.identifier.type',
              path: 'Patient.identifier.type',
              min: 0,
              max: '1',
              short: 'Tipo de documento de Id (Extensible)',
              definition:
                'Tipo de documento de Id definido en el Sistema de Codificación V2-0203 de HL7.',
              comment:
                'Pacientes sin documento local deben especificar el de origen; sin Id usar MR (registro clínico).',
              binding: {
                strength: 'extensible',
                valueSet: 'https://hl7chile.cl/fhir/ig/clcore/ValueSet/VSTiposDocumentos',
                description: 'Tipos de documentos de identificación',
              },
              mustSupport: true,
              isModified: true,
              children: [
                {
                  id: 'Patient.identifier.type.coding',
                  path: 'Patient.identifier.type.coding',
                  min: 0,
                  max: '*',
                  mustSupport: true,
                  isModified: true,
                  children: [
                    {
                      id: 'Patient.identifier.type.coding.code',
                      path: 'Patient.identifier.type.coding.code',
                      min: 0,
                      max: '1',
                      short: 'Código de Tipo de Documento',
                      definition: 'Código de Tipo de Documento',
                      mustSupport: true,
                      isModified: true,
                      children: [],
                    },
                    {
                      id: 'Patient.identifier.type.coding.system',
                      path: 'Patient.identifier.type.coding.system',
                      min: 0,
                      max: '1',
                      short: 'Sistema de identificación de tipos de documentos',
                      comment:
                        'Ejemplo: Cédula chilena NNCL; Pasaporte usa PPT según VS.',
                      definition:
                        'Sistema mediante el cual se obtienen los códigos para un determinado tipo de documento.',
                      mustSupport: true,
                      isModified: true,
                      children: [],
                    },
                    {
                      id: 'Patient.identifier.type.coding.display',
                      path: 'Patient.identifier.type.coding.display',
                      min: 0,
                      max: '1',
                      short: 'Glosa del Código Documento',
                      definition: 'Glosa del Código Documento',
                      mustSupport: true,
                      isModified: true,
                      children: [],
                    },
                  ],
                },
                {
                  id: 'Patient.identifier.type.extension',
                  path: 'Patient.identifier.type.extension',
                  min: 0,
                  max: '*',
                  short: 'País de Origen del Documento de Id',
                  slicing: {
                    discriminator: [{ path: 'url', type: 'value' }],
                    rules: 'open',
                    ordered: false,
                  },
                  definition:
                    'Se usa esta extensión para agregarle al tipo de documento el país de origen.',
                  isModified: true,
                  children: [
                    {
                      id: 'Patient.identifier.type.extension:paisEmisionDocumento',
                      path: 'Patient.identifier.type.extension',
                      sliceName: 'paisEmisionDocumento',
                      min: 0,
                      max: '1',
                      short: 'País de emisión del documento',
                      type: [{ code: 'Extension' }],
                      mustSupport: true,
                      isModified: true,
                      children: [
                        {
                          id: 'Patient.identifier.type.extension:paisEmisionDocumento.url',
                          path: 'Patient.identifier.type.extension.url',
                          min: 1,
                          max: '1',
                          short: 'URL extensión país emisión',
                          definition:
                            'Extensión para país de origen del documento de identificación.',
                          type: [{ code: 'uri' }],
                          isModified: true,
                          children: [],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            {
              id: 'Patient.identifier.value',
              path: 'Patient.identifier.value',
              min: 0,
              max: '1',
              short: 'Número o valor de identificación',
              definition: 'Número o valor de identificación',
              isModified: true,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.birthDate',
          path: 'Patient.birthDate',
          min: 0,
          max: '1',
          short: 'Fecha de nacimiento del Paciente.',
          definition: 'Fecha de nacimiento del Paciente.',
          mustSupport: true,
          isModified: true,
          children: [],
        },
        {
          id: 'Patient.communication',
          path: 'Patient.communication',
          min: 0,
          max: '*',
          short: 'Lenguaje en el cual se puede comunicar con el paciente',
          definition: 'Lenguaje en el cual se puede comunicar con el paciente',
          mustSupport: true,
          isModified: true,
          children: [
            {
              id: 'Patient.communication.language',
              path: 'Patient.communication.language',
              min: 0,
              max: '1',
              short: 'Lenguaje específico',
              definition: 'Código del lenguaje específico',
              binding: {
                strength: 'required',
                valueSet: 'https://hl7chile.cl/fhir/ig/clcore/ValueSet/VSCodigoslenguaje',
                description: 'Lenguajes soportados',
              },
              mustSupport: true,
              isModified: true,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.telecom',
          path: 'Patient.telecom',
          min: 0,
          max: '*',
          short: 'Detalles de contacto del Paciente',
          definition:
            'Detalles del contacto de un paciente comúnmente el o los más usados (teléfono, email, etc.)',
          mustSupport: true,
          isModified: true,
          children: [
            {
              id: 'Patient.telecom.use',
              path: 'Patient.telecom.use',
              min: 0,
              max: '1',
              short: 'home | work | temp | old | mobile',
              definition: 'Propósito para el contacto definido',
              binding: {
                strength: 'required',
                valueSet: 'http://hl7.org/fhir/ValueSet/contact-point-use',
                description: 'Uso del punto de contacto',
              },
              mustSupport: true,
              isModified: true,
              children: [],
            },
            {
              id: 'Patient.telecom.value',
              path: 'Patient.telecom.value',
              min: 0,
              max: '1',
              short: 'Dato del contacto del paciente',
              definition:
                'Valor del contacto como por ejemplo el número de teléfono fijo, móvil o el email del Paciente.',
              mustSupport: true,
              isModified: true,
              children: [],
            },
            {
              id: 'Patient.telecom.system',
              path: 'Patient.telecom.system',
              min: 0,
              max: '1',
              definition:
                'Forma de telecomunicación para el punto de contacto: qué sistema de comunicación se requiere.',
              binding: {
                strength: 'required',
                valueSet: 'http://hl7.org/fhir/ValueSet/contact-point-system',
                description: 'Sistema de contacto',
              },
              mustSupport: true,
              isModified: true,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.generalPractitioner',
          path: 'Patient.generalPractitioner',
          min: 0,
          max: '*',
          short: 'Proveedor de Salud designado como principal',
          definition: 'Proveedor de Salud designado como principal',
          type: [
            {
              code: 'Reference',
              targetProfile: [
                'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/CoreOrganizacionCl|1.9.4',
                'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/CorePrestadorCl|1.9.4',
                'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/CoreRolClinicoCl|1.9.4',
              ],
            },
          ],
          mustSupport: true,
          isModified: true,
          children: [
            {
              id: 'Patient.generalPractitioner.display',
              path: 'Patient.generalPractitioner.display',
              min: 0,
              max: '1',
              short: 'Texto alternativo a la referencia',
              definition: 'Texto alternativo a la referencia',
              mustSupport: true,
              isModified: true,
              children: [],
            },
            {
              id: 'Patient.generalPractitioner.reference',
              path: 'Patient.generalPractitioner.reference',
              min: 0,
              max: '1',
              short: 'URI de referencia a la Organización o a un Médico',
              definition: 'URI de referencia a la Organización o a un Médico',
              mustSupport: true,
              isModified: true,
              children: [],
            },
          ],
        },
        {
          id: 'Patient.contact',
          path: 'Patient.contact',
          min: 0,
          max: '*',
          short: 'Contacto, tutor legal o representante del Paciente',
          definition: 'Contacto, tutor legal o representante del Paciente',
          mustSupport: true,
          isModified: true,
          children: [
            {
              id: 'Patient.contact.name',
              path: 'Patient.contact.name',
              min: 0,
              max: '1',
              short: 'Nombre del Contacto',
              definition: 'Nombre del contacto asociado al paciente',
              mustSupport: true,
              isModified: true,
              children: [
                {
                  id: 'Patient.contact.name.use',
                  path: 'Patient.contact.name.use',
                  min: 1,
                  max: '1',
                  short: 'Uso del nombre del contacto',
                  comment: 'El use DEBE ser "official".',
                  definition: 'Nombre registrado oficialmente en el Registro Civil',
                  mustSupport: true,
                  isModified: true,
                  children: [],
                },
                {
                  id: 'Patient.contact.name.given',
                  path: 'Patient.contact.name.given',
                  min: 1,
                  max: '*',
                  short: 'Primer nombre y nombres del Contacto o Representante Legal',
                  definition: 'Todos los nombres no necesariamente solo el Primero.',
                  mustSupport: true,
                  isModified: true,
                  children: [],
                },
                {
                  id: 'Patient.contact.name.family',
                  path: 'Patient.contact.name.family',
                  min: 1,
                  max: '1',
                  short: '1er Apellido',
                  definition:
                    'Primer apellido registrado al nacer o inscrito legalmente en el Registro Civil.',
                  mustSupport: true,
                  isModified: true,
                  children: [
                    {
                      id: 'Patient.contact.name.family.extension',
                      path: 'Patient.contact.name.family.extension',
                      min: 0,
                      max: '*',
                      short: 'Extensión para 2o apellido',
                      slicing: {
                        discriminator: [{ path: 'url', type: 'value' }],
                        rules: 'open',
                        ordered: false,
                      },
                      definition: 'Extensión para la declaración de un segundo apellido',
                      isModified: true,
                      children: [
                        {
                          id: 'Patient.contact.name.family.extension:segundoApellido',
                          path: 'Patient.contact.name.family.extension',
                          sliceName: 'segundoApellido',
                          min: 0,
                          max: '1',
                          short: 'Extensión segundo apellido',
                          type: [{ code: 'Extension' }],
                          mustSupport: true,
                          isModified: true,
                          children: [
                            {
                              id: 'Patient.contact.name.family.extension:segundoApellido.url',
                              path: 'Patient.contact.name.family.extension.url',
                              min: 1,
                              max: '1',
                              short: 'URL extensión segundo apellido',
                              definition:
                                'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/SegundoApellido',
                              type: [{ code: 'uri' }],
                              isModified: true,
                              children: [],
                            },
                          ],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
            {
              id: 'Patient.contact.extension',
              path: 'Patient.contact.extension',
              min: 0,
              max: '*',
              slicing: {
                discriminator: [{ path: 'url', type: 'value' }],
                rules: 'open',
                ordered: false,
              },
              isModified: true,
              children: [
                {
                  id: 'Patient.contact.extension:IdContacto',
                  path: 'Patient.contact.extension',
                  sliceName: 'IdContacto',
                  min: 0,
                  max: '2147483647',
                  short: 'Identificación del Contacto',
                  definition:
                    'Extensión para declarar identificación del contacto y la procedencia de esta.',
                  type: [{ code: 'Extension' }],
                  mustSupport: true,
                  isModified: true,
                  children: [
                    {
                      id: 'Patient.contact.extension:IdContacto.url',
                      path: 'Patient.contact.extension.url',
                      min: 1,
                      max: '1',
                      short: 'URL extensión IdContacto',
                      definition:
                        'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/IdContacto',
                      type: [{ code: 'uri' }],
                      isModified: true,
                      children: [],
                    },
                  ],
                },
              ],
            },
            {
              id: 'Patient.contact.relationship',
              path: 'Patient.contact.relationship',
              min: 0,
              max: '*',
              short: 'Relación legal o de parentesco entre el contacto y el paciente',
              binding: {
                strength: 'required',
                valueSet: 'https://hl7chile.cl/fhir/ig/clcore/ValueSet/VSContactoRelacion',
                description: 'Relación entre contacto y paciente',
              },
              mustSupport: true,
              isModified: true,
              children: [
                {
                  id: 'Patient.contact.relationship.coding',
                  path: 'Patient.contact.relationship.coding',
                  min: 0,
                  max: '*',
                  mustSupport: true,
                  isModified: true,
                  children: [],
                },
              ],
            },
          ],
        },
      ],
    },
  ],
  isDirty: false,
};

// Mock Profile: With Slicing (Observation)
export const observationWithSlicing: Profile = {
  id: 'observation-with-slicing',
  url: 'http://example.org/StructureDefinition/observation-with-slicing',
  name: 'ObservationWithSlicing',
  title: 'Observation with Component Slicing',
  status: 'draft',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Observation',
  derivation: 'constraint',
  elements: [
    {
      id: 'Observation',
      path: 'Observation',
      min: 0,
      max: '*',
      isModified: false,
      children: [
        {
          id: 'Observation.component',
          path: 'Observation.component',
          min: 2,
          max: '*',
          short: 'Component observations (sliced)',
          slicing: {
            discriminator: [{ type: 'pattern', path: 'code' }],
            rules: 'open',
            ordered: false,
          },
          isModified: true,
          children: [
            {
              id: 'Observation.component:systolic',
              path: 'Observation.component',
              sliceName: 'systolic',
              min: 1,
              max: '1',
              short: 'Systolic blood pressure',
              isModified: true,
              children: [],
            },
            {
              id: 'Observation.component:diastolic',
              path: 'Observation.component',
              sliceName: 'diastolic',
              min: 1,
              max: '1',
              short: 'Diastolic blood pressure',
              isModified: true,
              children: [],
            },
          ],
        },
      ],
    },
  ],
  isDirty: false,
};

// Generate large element tree for performance testing
function generateLargeElementTree(count: number): ElementNode[] {
  const root: ElementNode = {
    id: 'Patient',
    path: 'Patient',
    min: 0,
    max: '*',
    isModified: false,
    children: [],
  };

  let elementCount = 1;
  const paths = ['identifier', 'name', 'telecom', 'address', 'contact', 'communication'];

  while (elementCount < count) {
    for (const path of paths) {
      if (elementCount >= count) return [root];
      root.children.push({
        id: `Patient.${path}[${elementCount}]`,
        path: `Patient.${path}`,
        min: 0,
        max: '*',
        isModified: Math.random() > 0.7,
        children: [],
      });
      elementCount++;
    }
  }

  return [root];
}

// Mock Profile: Large Profile (500+ elements)
export const largeProfile: Profile = {
  id: 'large-profile',
  url: 'http://example.org/StructureDefinition/large-profile',
  name: 'LargeProfile',
  title: 'Large Profile for Performance Testing',
  status: 'draft',
  fhirVersion: '4.0.1',
  baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
  derivation: 'constraint',
  elements: generateLargeElementTree(500),
  isDirty: false,
};

// Mock Packages
export const mockPackages: Package[] = [
  {
    id: 'hl7.fhir.r4.core',
    name: 'hl7.fhir.r4.core',
    version: '4.0.1',
    description:
      'FHIR R4 Core Package - Contains all base resources, datatypes, and extensions defined in the FHIR R4 specification.',
    fhirVersion: '4.0.1',
    installed: true,
    size: '45.2 MB',
    publisher: 'HL7 International',
    license: 'CC0-1.0',
    homepage: 'https://hl7.org/fhir/R4/',
    canonical: 'http://hl7.org/fhir',
    downloadCount: 1250000,
    publishedDate: '2019-10-30',
    latestVersion: '4.0.1',
    hasUpdate: false,
    resourceCounts: {
      profiles: 145,
      extensions: 245,
      valueSets: 890,
      codeSystems: 420,
      searchParameters: 1200,
      operationDefinitions: 48,
      capabilityStatements: 2,
      total: 2950,
    },
    versions: [
      { version: '4.0.1', fhirVersion: '4.0.1', publishedDate: '2019-10-30', size: '45.2 MB' },
      { version: '4.0.0', fhirVersion: '4.0.0', publishedDate: '2018-12-27', size: '44.8 MB' },
    ],
    dependencies: [],
  },
  {
    id: 'hl7.fhir.us.core',
    name: 'hl7.fhir.us.core',
    version: '6.1.0',
    description:
      'US Core Implementation Guide - Defines the minimum conformance requirements for accessing patient data in the US healthcare system.',
    fhirVersion: '4.0.1',
    installed: true,
    size: '12.8 MB',
    publisher: 'HL7 US Realm Steering Committee',
    license: 'CC0-1.0',
    homepage: 'https://hl7.org/fhir/us/core/',
    canonical: 'http://hl7.org/fhir/us/core',
    downloadCount: 850000,
    publishedDate: '2023-06-15',
    latestVersion: '7.0.0',
    hasUpdate: true,
    resourceCounts: {
      profiles: 42,
      extensions: 18,
      valueSets: 95,
      codeSystems: 12,
      searchParameters: 35,
      operationDefinitions: 3,
      capabilityStatements: 2,
      total: 207,
    },
    versions: [
      { version: '7.0.0', fhirVersion: '4.0.1', publishedDate: '2024-05-01', size: '14.2 MB' },
      { version: '6.1.0', fhirVersion: '4.0.1', publishedDate: '2023-06-15', size: '12.8 MB' },
      { version: '6.0.0', fhirVersion: '4.0.1', publishedDate: '2023-03-26', size: '12.5 MB' },
      { version: '5.0.1', fhirVersion: '4.0.1', publishedDate: '2022-06-13', size: '11.2 MB' },
    ],
    dependencies: [
      { name: 'hl7.fhir.r4.core', version: '4.0.1', isInstalled: true },
      { name: 'hl7.terminology.r4', version: '5.3.0', isInstalled: false },
    ],
  },
  {
    id: 'hl7.fhir.uv.ipa',
    name: 'hl7.fhir.uv.ipa',
    version: '1.0.0',
    description:
      'International Patient Access - A minimal set of FHIR resources to access basic patient data across borders and organizational boundaries.',
    fhirVersion: '4.0.1',
    installed: false,
    size: '3.4 MB',
    publisher: 'HL7 International',
    license: 'CC0-1.0',
    homepage: 'https://hl7.org/fhir/uv/ipa/',
    canonical: 'http://hl7.org/fhir/uv/ipa',
    downloadCount: 125000,
    publishedDate: '2023-03-24',
    latestVersion: '1.0.0',
    hasUpdate: false,
    resourceCounts: {
      profiles: 12,
      extensions: 4,
      valueSets: 8,
      codeSystems: 2,
      searchParameters: 15,
      operationDefinitions: 2,
      capabilityStatements: 1,
      total: 44,
    },
    versions: [
      { version: '1.0.0', fhirVersion: '4.0.1', publishedDate: '2023-03-24', size: '3.4 MB' },
      {
        version: '1.0.0-ballot',
        fhirVersion: '4.0.1',
        publishedDate: '2022-09-01',
        size: '3.2 MB',
      },
    ],
    dependencies: [{ name: 'hl7.fhir.r4.core', version: '4.0.1', isInstalled: true }],
  },
  {
    id: 'hl7.fhir.uv.sdc',
    name: 'hl7.fhir.uv.sdc',
    version: '3.0.0',
    description:
      'Structured Data Capture - Provides additional constraints and extensions for improved questionnaire handling and data capture.',
    fhirVersion: '4.0.1',
    installed: false,
    size: '8.7 MB',
    publisher: 'HL7 International',
    license: 'CC0-1.0',
    homepage: 'https://hl7.org/fhir/uv/sdc/',
    canonical: 'http://hl7.org/fhir/uv/sdc',
    downloadCount: 320000,
    publishedDate: '2023-12-01',
    latestVersion: '3.0.0',
    hasUpdate: false,
    resourceCounts: {
      profiles: 28,
      extensions: 65,
      valueSets: 24,
      codeSystems: 8,
      searchParameters: 12,
      operationDefinitions: 8,
      capabilityStatements: 2,
      total: 147,
    },
    versions: [
      { version: '3.0.0', fhirVersion: '4.0.1', publishedDate: '2023-12-01', size: '8.7 MB' },
      { version: '2.7.0', fhirVersion: '4.0.1', publishedDate: '2022-08-01', size: '8.1 MB' },
    ],
    dependencies: [{ name: 'hl7.fhir.r4.core', version: '4.0.1', isInstalled: true }],
  },
  {
    id: 'hl7.fhir.eu.laboratory',
    name: 'hl7.fhir.eu.laboratory',
    version: '0.1.0',
    description:
      'European Laboratory Report - HL7 Europe Laboratory Report Implementation Guide for cross-border laboratory result exchange.',
    fhirVersion: '4.0.1',
    installed: false,
    size: '5.2 MB',
    publisher: 'HL7 Europe',
    license: 'CC0-1.0',
    homepage: 'https://hl7.eu/fhir/laboratory/',
    canonical: 'http://hl7.eu/fhir/laboratory',
    downloadCount: 45000,
    publishedDate: '2024-01-15',
    latestVersion: '0.1.0',
    hasUpdate: false,
    resourceCounts: {
      profiles: 18,
      extensions: 12,
      valueSets: 35,
      codeSystems: 6,
      searchParameters: 8,
      operationDefinitions: 0,
      capabilityStatements: 1,
      total: 80,
    },
    versions: [
      { version: '0.1.0', fhirVersion: '4.0.1', publishedDate: '2024-01-15', size: '5.2 MB' },
    ],
    dependencies: [
      { name: 'hl7.fhir.r4.core', version: '4.0.1', isInstalled: true },
      { name: 'hl7.fhir.uv.ips', version: '1.1.0', isInstalled: false },
    ],
  },
  {
    id: 'hl7.fhir.uv.ips',
    name: 'hl7.fhir.uv.ips',
    version: '1.1.0',
    description:
      'International Patient Summary - A minimal and non-exhaustive patient summary for unplanned, cross-border care.',
    fhirVersion: '4.0.1',
    installed: false,
    size: '9.8 MB',
    publisher: 'HL7 International',
    license: 'CC0-1.0',
    homepage: 'https://hl7.org/fhir/uv/ips/',
    canonical: 'http://hl7.org/fhir/uv/ips',
    downloadCount: 280000,
    publishedDate: '2023-05-12',
    latestVersion: '1.1.0',
    hasUpdate: false,
    resourceCounts: {
      profiles: 35,
      extensions: 22,
      valueSets: 68,
      codeSystems: 15,
      searchParameters: 18,
      operationDefinitions: 4,
      capabilityStatements: 2,
      total: 164,
    },
    versions: [
      { version: '1.1.0', fhirVersion: '4.0.1', publishedDate: '2023-05-12', size: '9.8 MB' },
      { version: '1.0.0', fhirVersion: '4.0.1', publishedDate: '2022-01-28', size: '9.2 MB' },
    ],
    dependencies: [{ name: 'hl7.fhir.r4.core', version: '4.0.1', isInstalled: true }],
  },
  {
    id: 'hl7.terminology.r4',
    name: 'hl7.terminology.r4',
    version: '5.4.0',
    description:
      'HL7 Terminology Package - Contains terminology resources (CodeSystems and ValueSets) published by HL7.',
    fhirVersion: '4.0.1',
    installed: false,
    size: '28.5 MB',
    publisher: 'HL7 International',
    license: 'CC0-1.0',
    homepage: 'https://terminology.hl7.org/',
    canonical: 'http://terminology.hl7.org',
    downloadCount: 680000,
    publishedDate: '2024-03-01',
    latestVersion: '5.4.0',
    hasUpdate: false,
    resourceCounts: {
      profiles: 0,
      extensions: 0,
      valueSets: 520,
      codeSystems: 380,
      searchParameters: 0,
      operationDefinitions: 0,
      capabilityStatements: 0,
      total: 900,
    },
    versions: [
      { version: '5.4.0', fhirVersion: '4.0.1', publishedDate: '2024-03-01', size: '28.5 MB' },
      { version: '5.3.0', fhirVersion: '4.0.1', publishedDate: '2023-09-15', size: '27.8 MB' },
    ],
    dependencies: [],
  },
];

// Mock Package Resources
export const mockPackageResources: Record<string, import('@shared/types').PackageResource[]> = {
  'hl7.fhir.us.core': [
    {
      id: 'us-core-patient',
      url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient',
      name: 'USCorePatientProfile',
      title: 'US Core Patient Profile',
      description:
        'Defines constraints on the Patient resource for the minimal set of data to query and retrieve patient demographic information.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '6.1.0',
    },
    {
      id: 'us-core-practitioner',
      url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitioner',
      name: 'USCorePractitionerProfile',
      title: 'US Core Practitioner Profile',
      description:
        'Defines constraints on the Practitioner resource for the minimal set of data to query and retrieve practitioner information.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '6.1.0',
    },
    {
      id: 'us-core-observation-lab',
      url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation-lab',
      name: 'USCoreLaboratoryResultObservationProfile',
      title: 'US Core Laboratory Result Observation Profile',
      description:
        'Defines constraints on Observation resources for the results of laboratory tests.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '6.1.0',
    },
    {
      id: 'us-core-condition-problems',
      url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-condition-problems-health-concerns',
      name: 'USCoreConditionProblemsHealthConcernsProfile',
      title: 'US Core Condition Problems and Health Concerns Profile',
      description:
        'Defines constraints on Condition for representing problems and health concerns.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '6.1.0',
    },
    {
      id: 'us-core-medication-request',
      url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-medicationrequest',
      name: 'USCoreMedicationRequestProfile',
      title: 'US Core MedicationRequest Profile',
      description:
        'Defines constraints on MedicationRequest for prescriptions and medication orders.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '6.1.0',
    },
    {
      id: 'us-core-race',
      url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-race',
      name: 'USCoreRaceExtension',
      title: 'US Core Race Extension',
      description:
        'Concepts classifying the person into a named category of humans sharing common history, traits, or nationality.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '6.1.0',
    },
    {
      id: 'us-core-ethnicity',
      url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity',
      name: 'USCoreEthnicityExtension',
      title: 'US Core Ethnicity Extension',
      description:
        'Concepts classifying the person into a named category of humans sharing a common heritage.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '6.1.0',
    },
    {
      id: 'us-core-birthsex-vs',
      url: 'http://hl7.org/fhir/us/core/ValueSet/birthsex',
      name: 'BirthSexValueSet',
      title: 'Birth Sex',
      description:
        'Codes for assigning sex at birth as specified by the Office of the National Coordinator for Health IT (ONC).',
      resourceType: 'ValueSet',
      status: 'active',
      version: '6.1.0',
    },
  ],
  'hl7.fhir.r4.core': [
    {
      id: 'patient',
      url: 'http://hl7.org/fhir/StructureDefinition/Patient',
      name: 'Patient',
      title: 'Patient',
      description:
        'Demographics and other administrative information about an individual or animal receiving care.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '4.0.1',
    },
    {
      id: 'observation',
      url: 'http://hl7.org/fhir/StructureDefinition/Observation',
      name: 'Observation',
      title: 'Observation',
      description:
        'Measurements and simple assertions made about a patient, device or other subject.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '4.0.1',
    },
    {
      id: 'condition',
      url: 'http://hl7.org/fhir/StructureDefinition/Condition',
      name: 'Condition',
      title: 'Condition',
      description:
        'A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '4.0.1',
    },
    {
      id: 'medication',
      url: 'http://hl7.org/fhir/StructureDefinition/Medication',
      name: 'Medication',
      title: 'Medication',
      description:
        'This resource is primarily used for the identification and definition of a medication.',
      resourceType: 'StructureDefinition',
      status: 'active',
      version: '4.0.1',
    },
  ],
};

// Mock Validation Results
export const mockValidationResults: Record<string, ValidationResult> = {
  'us-core-patient': {
    isValid: true,
    errors: [],
    warnings: [],
    info: [
      {
        severity: 'info',
        message: 'Profile is valid and ready for use',
        path: '',
      },
    ],
  },
  'observation-with-slicing': {
    isValid: false,
    errors: [
      {
        severity: 'error',
        message: 'Cardinality constraint violation: min (2) cannot be greater than max (1)',
        path: 'Observation.component',
        line: 42,
      },
    ],
    warnings: [
      {
        severity: 'warning',
        message: 'Slicing discriminator may be ambiguous',
        path: 'Observation.component',
        line: 40,
      },
    ],
    info: [],
  },
};

export const defaultValidationResult: ValidationResult = {
  isValid: true,
  errors: [],
  warnings: [],
  info: [],
};

// Mock Extensions
export const mockExtensions: Extension[] = [
  {
    id: 'patient-birthPlace',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-birthPlace',
    name: 'birthPlace',
    title: 'Birth Place',
    status: 'active',
    description:
      "The registered place of birth of the patient. A sytem may use the address.text if they don't store the birthPlace address in discrete elements.",
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['Address'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-birthTime',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-birthTime',
    name: 'birthTime',
    title: 'Birth Time',
    status: 'active',
    description:
      'The time of day that the patient was born. This includes the date to ensure that the timezone information can be communicated effectively.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient.birthDate',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['dateTime'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-mothersMaidenName',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-mothersMaidenName',
    name: 'mothersMaidenName',
    title: 'Mothers Maiden Name',
    status: 'active',
    description:
      "Mother's maiden (unmarried) name, commonly collected to help verify patient identity.",
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['string'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-nationality',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-nationality',
    name: 'nationality',
    title: 'Nationality',
    status: 'active',
    description:
      'The nationality of the patient. This is a complex extension that includes both a code and a period.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '*',
    isComplex: true,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-disability',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-disability',
    name: 'disability',
    title: 'Disability',
    status: 'active',
    description:
      'A code that identifies the disability or disabilities that affect how the person functions in everyday life.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '*',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-religion',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-religion',
    name: 'religion',
    title: 'Religion',
    status: 'active',
    description: "The patient's professed religious affiliations.",
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'patient-cadavericDonor',
    url: 'http://hl7.org/fhir/StructureDefinition/patient-cadavericDonor',
    name: 'cadavericDonor',
    title: 'Cadaveric Donor',
    status: 'active',
    description:
      'Flag indicating whether the patient authorized the donation of body parts after death.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['boolean'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'us-core-race',
    url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-race',
    name: 'race',
    title: 'US Core Race Extension',
    status: 'active',
    description:
      'Concepts classifying the person into a named category of humans sharing common history, traits, geographical origin or nationality. The race codes used to represent these concepts are based upon the CDC Race and Ethnicity Code Set.',
    publisher: 'HL7 US Realm Steering Committee',
    package: 'hl7.fhir.us.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
      {
        type: 'element',
        expression: 'RelatedPerson',
      },
      {
        type: 'element',
        expression: 'Practitioner',
      },
      {
        type: 'element',
        expression: 'Person',
      },
    ],
    min: 0,
    max: '1',
    isComplex: true,
    fhirVersion: '4.0.1',
    date: '2020-07-21T00:00:00+00:00',
    experimental: false,
  },
  {
    id: 'us-core-ethnicity',
    url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity',
    name: 'ethnicity',
    title: 'US Core Ethnicity Extension',
    status: 'active',
    description:
      'Concepts classifying the person into a named category of humans sharing a common real or presumed heritage, history, ancestry, or country of origin. The ethnicity codes used to represent these concepts are based upon the CDC Race and Ethnicity Code Set.',
    publisher: 'HL7 US Realm Steering Committee',
    package: 'hl7.fhir.us.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
      {
        type: 'element',
        expression: 'RelatedPerson',
      },
      {
        type: 'element',
        expression: 'Practitioner',
      },
      {
        type: 'element',
        expression: 'Person',
      },
    ],
    min: 0,
    max: '1',
    isComplex: true,
    fhirVersion: '4.0.1',
    date: '2020-07-21T00:00:00+00:00',
    experimental: false,
  },
  {
    id: 'us-core-birthsex',
    url: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex',
    name: 'birthsex',
    title: 'US Core Birth Sex Extension',
    status: 'active',
    description:
      "A code classifying the person's sex assigned at birth as specified by the Office of the National Coordinator for Health IT (ONC).",
    publisher: 'HL7 US Realm Steering Committee',
    package: 'hl7.fhir.us.core',
    context: [
      {
        type: 'element',
        expression: 'Patient',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['code'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2020-07-21T00:00:00+00:00',
    experimental: false,
  },
  {
    id: 'observation-bodyPosition',
    url: 'http://hl7.org/fhir/StructureDefinition/observation-bodyPosition',
    name: 'bodyPosition',
    title: 'Body Position',
    status: 'active',
    description: 'The position of the body when the observation was made.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Observation',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'observation-delta',
    url: 'http://hl7.org/fhir/StructureDefinition/observation-delta',
    name: 'delta',
    title: 'Observation Delta',
    status: 'active',
    description:
      'The qualitative change in the value relative to the previous measurement. Usually only recorded if the change is clinically significant.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Observation',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['CodeableConcept'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'resource-effectivePeriod',
    url: 'http://hl7.org/fhir/StructureDefinition/resource-effectivePeriod',
    name: 'effectivePeriod',
    title: 'Effective Period',
    status: 'active',
    description:
      'The period during which the resource content was or is planned to be in active use. Allows establishing a transition period from one resource to another.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'Resource',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['Period'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'humanname-fathers-family',
    url: 'http://hl7.org/fhir/StructureDefinition/humanname-fathers-family',
    name: 'fathersFamily',
    title: "Father's Family Name",
    status: 'active',
    description:
      'Indicates the family name of the father. Useful in cultures where the family name is derived from both parents.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'HumanName.family',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['string'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
  {
    id: 'humanname-mothers-family',
    url: 'http://hl7.org/fhir/StructureDefinition/humanname-mothers-family',
    name: 'mothersFamily',
    title: "Mother's Family Name",
    status: 'active',
    description:
      'Indicates the family name of the mother. Useful in cultures where the family name is derived from both parents.',
    publisher: 'HL7 International',
    package: 'hl7.fhir.r4.core',
    context: [
      {
        type: 'element',
        expression: 'HumanName.family',
      },
    ],
    min: 0,
    max: '1',
    valueTypes: ['string'],
    isComplex: false,
    fhirVersion: '4.0.1',
    date: '2019-11-01T09:29:23+11:00',
    experimental: false,
  },
];

// Mock ValueSets
export const mockValueSets: ValueSet[] = [
  {
    url: 'http://hl7.org/fhir/ValueSet/administrative-gender',
    name: 'AdministrativeGender',
    title: 'Administrative Gender',
    status: 'active',
    description: 'The gender of a person used for administrative purposes',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://hl7.org/fhir/administrative-gender',
        },
      ],
    },
  },
  {
    url: 'http://hl7.org/fhir/ValueSet/marital-status',
    name: 'MaritalStatus',
    title: 'Marital Status Codes',
    status: 'active',
    description:
      'This value set defines the set of codes that can be used to indicate the marital status of a person',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        },
      ],
    },
  },
  {
    url: 'http://hl7.org/fhir/ValueSet/observation-status',
    name: 'ObservationStatus',
    title: 'Observation Status',
    status: 'active',
    description: 'Codes providing the status of an observation',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://hl7.org/fhir/observation-status',
        },
      ],
    },
  },
  {
    url: 'http://hl7.org/fhir/ValueSet/contact-point-system',
    name: 'ContactPointSystem',
    title: 'Contact Point System',
    status: 'active',
    description: 'Telecommunications form for contact point',
    publisher: 'HL7 International',
    compose: {
      include: [
        {
          system: 'http://hl7.org/fhir/contact-point-system',
        },
      ],
    },
  },
  {
    url: 'http://loinc.org/vs/LL715-4',
    name: 'LaboratoryTestResults',
    title: 'Laboratory Test Results',
    status: 'active',
    description: 'LOINC codes for laboratory test results',
    publisher: 'Regenstrief Institute',
    compose: {
      include: [
        {
          system: 'http://loinc.org',
        },
      ],
    },
  },
  {
    url: 'http://snomed.info/sct/ValueSet/clinical-findings',
    name: 'ClinicalFindings',
    title: 'SNOMED CT Clinical Findings',
    status: 'active',
    description: 'SNOMED CT Clinical Findings',
    publisher: 'SNOMED International',
    compose: {
      include: [
        {
          system: 'http://snomed.info/sct',
        },
      ],
    },
  },
];

// Mock ValueSet Expansions
export const mockValueSetExpansions: Record<string, ValueSetExpansion> = {
  'http://hl7.org/fhir/ValueSet/administrative-gender': {
    total: 4,
    contains: [
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'male',
        display: 'Male',
      },
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'female',
        display: 'Female',
      },
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'other',
        display: 'Other',
      },
      {
        system: 'http://hl7.org/fhir/administrative-gender',
        code: 'unknown',
        display: 'Unknown',
      },
    ],
  },
  'http://hl7.org/fhir/ValueSet/marital-status': {
    total: 8,
    contains: [
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'A',
        display: 'Annulled',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'D',
        display: 'Divorced',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'I',
        display: 'Interlocutory',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'L',
        display: 'Legally Separated',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'M',
        display: 'Married',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'P',
        display: 'Polygamous',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'S',
        display: 'Never Married',
      },
      {
        system: 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus',
        code: 'W',
        display: 'Widowed',
      },
    ],
  },
  'http://hl7.org/fhir/ValueSet/observation-status': {
    total: 7,
    contains: [
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'registered',
        display: 'Registered',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'preliminary',
        display: 'Preliminary',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'final',
        display: 'Final',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'amended',
        display: 'Amended',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'corrected',
        display: 'Corrected',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'cancelled',
        display: 'Cancelled',
      },
      {
        system: 'http://hl7.org/fhir/observation-status',
        code: 'entered-in-error',
        display: 'Entered in Error',
      },
    ],
  },
};

// Mock Search Results
export const mockSearchResults = {
  resources: [
    {
      id: 'patient',
      url: 'http://hl7.org/fhir/StructureDefinition/Patient',
      name: 'Patient',
      title: 'Patient Resource',
      description: 'Demographics and other administrative information about an individual',
      type: 'resource' as const,
    },
    {
      id: 'observation',
      url: 'http://hl7.org/fhir/StructureDefinition/Observation',
      name: 'Observation',
      title: 'Observation Resource',
      description: 'Measurements and simple assertions',
      type: 'resource' as const,
    },
  ],
  extensions: [
    {
      id: 'patient-birthPlace',
      url: 'http://hl7.org/fhir/StructureDefinition/patient-birthPlace',
      name: 'birthPlace',
      title: 'Birth Place',
      description: 'The registered place of birth of the patient',
      type: 'extension' as const,
    },
  ],
  valueSets: [
    {
      id: 'administrative-gender',
      url: 'http://hl7.org/fhir/ValueSet/administrative-gender',
      name: 'AdministrativeGender',
      title: 'Administrative Gender',
      description: 'The gender of a person used for administrative purposes',
      type: 'valueset' as const,
    },
  ],
};

// Export mock profiles as array and by ID
export const mockProfiles: Profile[] = [
  fhirCorePatient,
  usCorePatient,
  clCorePatient,
  observationWithSlicing,
  largeProfile,
];

export const mockProfilesById: Record<string, Profile> = {
  [fhirCorePatient.id]: fhirCorePatient,
  [usCorePatient.id]: usCorePatient,
  [clCorePatient.id]: clCorePatient,
  [observationWithSlicing.id]: observationWithSlicing,
  [largeProfile.id]: largeProfile,
};

// Default profile for editor
export const defaultProfile = fhirCorePatient;

// Mock SD/FSH Export
export const mockSDExport: Record<string, string> = {
  'us-core-patient': JSON.stringify(usCorePatient, null, 2),
  'cl-core-patient': JSON.stringify(clCorePatient, null, 2),
};

export const mockFSHExport: Record<string, string> = {
  'us-core-patient': `
Profile: USCorePatientProfile
Parent: Patient
Id: us-core-patient
Title: "US Core Patient Profile"
Description: "Defines constraints and extensions on the Patient resource..."
* identifier 1..* MS
* name 1..* MS
* name.family 1..1 MS
* name.given 1..* MS
* gender 1..1 MS
`.trim(),
  'cl-core-patient': `
Profile: CorePacienteCl
Parent: Patient
Id: cl-core-patient
Title: "Core Paciente CL"
Description: "Perfil core chileno para el recurso Patient (mock)."
// Nota: generación real de FSH debe incluir slicing y constraints completos.
`.trim(),
};

// Project file tree fixtures
function makeFileNode(params: {
  root: ProjectTreeRoot;
  path: string;
  name: string;
  resourceId?: string;
  resourceType?: string;
  resourceKind?: ProjectResourceKind;
  canonicalUrl?: string;
}): ProjectTreeNode {
  return {
    path: params.path,
    name: params.name,
    kind: 'file',
    root: params.root,
    resourceId: params.resourceId,
    resourceType: params.resourceType,
    resourceKind: params.resourceKind,
    canonicalUrl: params.canonicalUrl,
    children: [],
  };
}

function makeFolder(
  root: ProjectTreeRoot,
  path: string,
  name: string,
  children: ProjectTreeNode[]
): ProjectTreeNode {
  return { path, name, root, kind: 'folder', children };
}

function generateProfileFolder(params: {
  root: ProjectTreeRoot;
  folderPath: string;
  folderName: string;
  count: number;
  prefix: string;
}): ProjectTreeNode {
  const children: ProjectTreeNode[] = Array.from({ length: params.count }).map((_, index) => {
    const suffix = (index + 1).toString().padStart(3, '0');
    const resourceId = `${params.prefix}-${suffix}`;

    return makeFileNode({
      root: params.root,
      path: `${params.folderPath}/${resourceId}.json`,
      name: `${resourceId}.json`,
      resourceId,
      resourceType: 'StructureDefinition',
      resourceKind: 'profile',
      canonicalUrl: `http://example.org/${params.prefix}/${resourceId}`,
    });
  });

  return makeFolder(params.root, params.folderPath, params.folderName, children);
}

function createBaseProjectTree(projectId: string): ProjectTreeNode[] {
  const irRoot = makeFolder('IR', 'IR', 'IR', [
    makeFolder('IR', 'IR/profiles', 'profiles', [
      makeFileNode({
        root: 'IR',
        path: 'IR/profiles/cl-core-patient.json',
        name: 'cl-core-patient.json',
        resourceId: 'cl-core-patient',
        resourceType: 'StructureDefinition',
        resourceKind: 'profile',
        canonicalUrl: 'https://hl7chile.cl/fhir/ig/clcore/StructureDefinition/CorePacienteCl',
      }),
      makeFileNode({
        root: 'IR',
        path: 'IR/profiles/us-core-patient.json',
        name: 'us-core-patient.json',
        resourceId: 'us-core-patient',
        resourceType: 'StructureDefinition',
        resourceKind: 'profile',
        canonicalUrl: 'http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient',
      }),
    ]),
    makeFolder('IR', 'IR/input', 'input', [
      makeFileNode({
        root: 'IR',
        path: 'IR/input/implementation-guide.json',
        name: 'implementation-guide.json',
        resourceId: `${projectId}-ig`,
        resourceType: 'ImplementationGuide',
        resourceKind: 'instance',
        canonicalUrl: `http://example.org/${projectId}/ImplementationGuide`,
      }),
      makeFileNode({
        root: 'IR',
        path: 'IR/input/examples-manifest.json',
        name: 'examples-manifest.json',
        resourceKind: 'other',
      }),
    ]),
    makeFolder('IR', 'IR/examples', 'examples', [
      makeFileNode({
        root: 'IR',
        path: 'IR/examples/patient-bundle.json',
        name: 'patient-bundle.json',
        resourceId: 'patient-bundle',
        resourceType: 'Bundle',
        resourceKind: 'instance',
      }),
      makeFileNode({
        root: 'IR',
        path: 'IR/examples/device-alert.json',
        name: 'device-alert.json',
        resourceId: 'device-alert',
        resourceType: 'Communication',
        resourceKind: 'instance',
      }),
    ]),
  ]);

  const sdRoot = makeFolder('SD', 'SD', 'SD', [
    makeFolder('SD', 'SD/profiles', 'profiles', [
      makeFileNode({
        root: 'SD',
        path: 'SD/profiles/patient-profile.json',
        name: 'patient-profile.json',
        resourceId: 'patient-profile',
        resourceType: 'StructureDefinition',
        resourceKind: 'profile',
        canonicalUrl: `http://example.org/${projectId}/StructureDefinition/patient-profile`,
      }),
      makeFileNode({
        root: 'SD',
        path: 'SD/profiles/observation-vitals.json',
        name: 'observation-vitals.json',
        resourceId: 'observation-vitals',
        resourceType: 'StructureDefinition',
        resourceKind: 'profile',
        canonicalUrl: `http://example.org/${projectId}/StructureDefinition/observation-vitals`,
      }),
      makeFileNode({
        root: 'SD',
        path: 'SD/profiles/device-alert.json',
        name: 'device-alert.json',
        resourceId: 'device-alert-profile',
        resourceType: 'StructureDefinition',
        resourceKind: 'profile',
        canonicalUrl: `http://example.org/${projectId}/StructureDefinition/device-alert`,
      }),
    ]),
    makeFolder('SD', 'SD/extensions', 'extensions', [
      makeFileNode({
        root: 'SD',
        path: 'SD/extensions/practitioner-role.json',
        name: 'practitioner-role.json',
        resourceId: 'practitioner-role-ext',
        resourceType: 'StructureDefinition',
        resourceKind: 'extension',
        canonicalUrl: `http://example.org/${projectId}/StructureDefinition/practitioner-role`,
      }),
      makeFileNode({
        root: 'SD',
        path: 'SD/extensions/device-udi.json',
        name: 'device-udi.json',
        resourceId: 'device-udi-ext',
        resourceType: 'StructureDefinition',
        resourceKind: 'extension',
        canonicalUrl: `http://example.org/${projectId}/StructureDefinition/device-udi`,
      }),
    ]),
    makeFolder('SD', 'SD/value-sets', 'value-sets', [
      makeFileNode({
        root: 'SD',
        path: 'SD/value-sets/vitals-locations.json',
        name: 'vitals-locations.json',
        resourceId: 'vitals-locations',
        resourceType: 'ValueSet',
        resourceKind: 'valueset',
        canonicalUrl: `http://example.org/${projectId}/ValueSet/vitals-locations`,
      }),
      makeFileNode({
        root: 'SD',
        path: 'SD/value-sets/device-alert-codes.json',
        name: 'device-alert-codes.json',
        resourceId: 'device-alert-codes',
        resourceType: 'CodeSystem',
        resourceKind: 'codesystem',
        canonicalUrl: `http://example.org/${projectId}/CodeSystem/device-alert-codes`,
      }),
    ]),
  ]);

  const fshRoot = makeFolder('FSH', 'FSH', 'FSH', [
    makeFolder('FSH', 'FSH/profiles', 'profiles', [
      makeFileNode({
        root: 'FSH',
        path: 'FSH/profiles/patient-profile.fsh',
        name: 'patient-profile.fsh',
        resourceKind: 'profile',
      }),
      makeFileNode({
        root: 'FSH',
        path: 'FSH/profiles/observation-vitals.fsh',
        name: 'observation-vitals.fsh',
        resourceKind: 'profile',
      }),
    ]),
    makeFolder('FSH', 'FSH/examples', 'examples', [
      makeFileNode({
        root: 'FSH',
        path: 'FSH/examples/patient-example.fsh',
        name: 'patient-example.fsh',
        resourceKind: 'example',
      }),
      makeFileNode({
        root: 'FSH',
        path: 'FSH/examples/device-alert-example.fsh',
        name: 'device-alert-example.fsh',
        resourceKind: 'example',
      }),
    ]),
  ]);

  return [irRoot, sdRoot, fshRoot];
}

function createLargeProjectTree(projectId: string, profileCount = 520): ProjectTreeNode[] {
  const base = createBaseProjectTree(projectId);
  const sdRoot = base.find((node) => node.root === 'SD');

  if (sdRoot) {
    const clinicalFolder = generateProfileFolder({
      root: 'SD',
      folderPath: 'SD/profiles/clinical',
      folderName: 'clinical',
      count: profileCount,
      prefix: `${projectId}-clinical-profile`,
    });

    sdRoot.children = [clinicalFolder, ...sdRoot.children];
  }

  return base;
}

export const mockProjectTrees: Record<string, ProjectTreeNode[]> = {
  default: createBaseProjectTree('default'),
  'clinical-quality-suite': createBaseProjectTree('clinical-quality-suite'),
  'regional-care-ig': createBaseProjectTree('regional-care-ig'),
  'device-data-pilot': createBaseProjectTree('device-data-pilot'),
  'research-catalog': createLargeProjectTree('research-catalog'),
};

export function cloneProjectTree(tree: ProjectTreeNode[]): ProjectTreeNode[] {
  return tree.map((node) => ({
    ...node,
    children: cloneProjectTree(node.children),
  }));
}

export function findResourceNodeById(
  nodes: ProjectTreeNode[],
  resourceId: string
): ProjectTreeNode | null {
  for (const node of nodes) {
    if (node.kind === 'file' && node.resourceId === resourceId) {
      return node;
    }
    const child = findResourceNodeById(node.children, resourceId);
    if (child) {
      return child;
    }
  }
  return null;
}

export function toResourceMetadata(
  projectId: string,
  node: ProjectTreeNode
): ProjectResourceMetadata {
  return {
    projectId,
    resourceId: node.resourceId ?? node.name,
    resourceType: node.resourceType,
    resourceKind: node.resourceKind ?? 'other',
    path: node.path,
    name: node.name,
    canonicalUrl: node.canonicalUrl,
    root: node.root,
  };
}

const slugify = (value: string) =>
  value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/(^-|-$)+/g, '');

function ensureFolder(
  nodes: ProjectTreeNode[],
  root: ProjectTreeRoot,
  path: string,
  name: string
): ProjectTreeNode {
  const parts = path.split('/').filter(Boolean);
  if (parts.length === 0) throw new Error('Invalid folder path');

  let current =
    nodes.find((node) => node.path === parts[0]) ??
    (() => {
      const folder: ProjectTreeNode = {
        path: parts[0],
        name: parts[0],
        root,
        kind: 'folder',
        children: [],
      };
      nodes.push(folder);
      return folder;
    })();

  for (let i = 1; i < parts.length; i++) {
    const nextPath = parts.slice(0, i + 1).join('/');
    let child = current.children.find((node) => node.path === nextPath);
    if (!child) {
      child = {
        path: nextPath,
        name: i === parts.length - 1 ? name : parts[i],
        root,
        kind: 'folder',
        children: [],
      };
      current.children.unshift(child);
    }
    current = child;
  }

  return current;
}

export function addMockArtifact(
  projectId: string,
  tree: ProjectTreeNode[],
  input: CreateArtifactInput
): CreatedArtifact {
  const folderPath =
    input.kind === 'valueset'
      ? 'SD/value-sets'
      : input.kind === 'extension'
        ? 'SD/extensions'
        : 'IR/input/profiles';

  const root: ProjectTreeRoot = input.kind === 'profile' ? 'IR' : 'SD';
  const folderName = folderPath.split('/').pop() || 'profiles';
  const folder = ensureFolder(tree, root, folderPath, folderName);

  const resourceId = input.id?.trim() || slugify(input.name || 'artifact');
  const fileName = `${resourceId}.json`;
  const resourceType = input.kind === 'valueset' ? 'ValueSet' : 'StructureDefinition';
  const resourceKind: ProjectResourceKind =
    input.kind === 'valueset' ? 'valueset' : input.kind === 'extension' ? 'extension' : 'profile';

  const canonicalUrl = `http://example.org/${projectId}/${resourceType}/${resourceId}`;
  const path = `${folderPath}/${fileName}`;

  const node: ProjectTreeNode = {
    path,
    name: fileName,
    root,
    kind: 'file',
    resourceId,
    resourceType,
    resourceKind,
    canonicalUrl,
    children: [],
  };

  folder.children.unshift(node);

  return {
    path,
    resourceId,
    resourceType,
    resourceKind,
    canonicalUrl,
  };
}

// Mock Projects
export const mockProjects: Project[] = [
  {
    id: 'clinical-quality-suite',
    name: 'Clinical Quality Suite',
    fhirVersion: '4.0.1',
    status: 'draft',
    templateId: 'implementation-guide',
    description: 'Quality measures and profiles for chronic care coordination.',
    packageId: 'org.example.cqs',
    canonicalBase: 'http://example.org/fhir/cqs',
    version: '1.3.0',
    publisher: 'FHIR Builders',
    path: '/workspace/cqs',
    createdAt: '2024-09-12T08:00:00.000Z',
    modifiedAt: '2024-12-18T17:30:00.000Z',
    lastOpenedAt: '2025-01-05T09:40:00.000Z',
    dependencies: [
      { packageId: 'hl7.fhir.r4.core', version: '4.0.1', name: 'FHIR R4 Core' },
      { packageId: 'hl7.fhir.us.core', version: '6.1.0', name: 'US Core IG' },
    ],
  },
  {
    id: 'regional-care-ig',
    name: 'Regional Care IG',
    fhirVersion: '4.3.0',
    status: 'draft',
    templateId: 'regional',
    description: 'Starter kit for regional implementation guides with IPS alignment.',
    packageId: 'org.example.rcig',
    canonicalBase: 'http://region.example.org/fhir',
    version: '0.6.2',
    publisher: 'Northwind Health',
    path: '/workspace/regional-care',
    createdAt: '2024-10-01T10:15:00.000Z',
    modifiedAt: '2024-12-20T15:05:00.000Z',
    lastOpenedAt: '2025-01-03T13:05:00.000Z',
    dependencies: [{ packageId: 'hl7.fhir.uv.ips', version: '1.1.0', name: 'IPS' }],
  },
  {
    id: 'device-data-pilot',
    name: 'Device Data Pilot',
    fhirVersion: '5.0.0',
    status: 'draft',
    templateId: 'blank',
    description: 'Lightweight pilot for ingesting device telemetry into FHIR.',
    packageId: 'org.example.ddp',
    canonicalBase: 'http://example.org/fhir/device',
    version: '0.3.0',
    publisher: 'Signal Labs',
    path: '/workspace/device-data',
    createdAt: '2024-11-12T09:12:00.000Z',
    modifiedAt: '2024-12-28T10:45:00.000Z',
    lastOpenedAt: '2025-01-06T08:10:00.000Z',
    dependencies: [{ packageId: 'hl7.fhir.uv.ipa', version: '1.0.0', name: 'IPA' }],
  },
  {
    id: 'research-catalog',
    name: 'Research Catalog',
    fhirVersion: '4.0.1',
    status: 'published',
    templateId: 'custom',
    description: 'Research profile catalog with observation and questionnaire focus.',
    packageId: 'org.example.research',
    canonicalBase: 'http://example.org/fhir/research',
    version: '0.9.0',
    publisher: 'Open Health Labs',
    path: '/workspace/research',
    createdAt: '2024-08-06T11:00:00.000Z',
    modifiedAt: '2024-12-10T16:10:00.000Z',
    lastOpenedAt: '2025-01-02T18:25:00.000Z',
    dependencies: [{ packageId: 'hl7.fhir.uv.sdc', version: '3.0.0', name: 'SDC' }],
  },
];

export const mockProjectsById: Record<string, Project> = Object.fromEntries(
  mockProjects.map((project) => [project.id, project])
);

export function createMockProject(
  overrides: Partial<Project> & Pick<Project, 'name' | 'fhirVersion'>
): Project {
  const slug = overrides.name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/(^-|-$)/g, '');
  const now = new Date().toISOString();

  const project: Project = {
    id: overrides.id ?? `${slug || 'project'}-${Math.random().toString(36).slice(2, 8)}`,
    name: overrides.name,
    fhirVersion: overrides.fhirVersion,
    status: overrides.status ?? 'draft',
    templateId: overrides.templateId ?? 'blank',
    description:
      overrides.description ?? 'Custom project created from the mock API while backend is offline.',
    packageId: overrides.packageId,
    canonicalBase: overrides.canonicalBase ?? `http://example.org/${slug || 'project'}`,
    version: overrides.version ?? '0.1.0',
    publisher: overrides.publisher ?? 'FHIR Profile Builder',
    path: overrides.path ?? `/workspace/${slug || 'project'}`,
    createdAt: overrides.createdAt ?? now,
    modifiedAt: overrides.modifiedAt ?? now,
    lastOpenedAt: overrides.lastOpenedAt ?? now,
    dependencies: overrides.dependencies ?? [],
  };

  mockProjects.unshift(project);
  mockProjectsById[project.id] = project;

  return project;
}

// Mock undo/redo stacks
export const mockUndoStack: Record<string, unknown[]> = {};
export const mockRedoStack: Record<string, unknown[]> = {};

// FHIR R4 Base Resource Types for profile creation
export const mockBaseResources: BaseResource[] = [
  {
    name: 'Account',
    url: 'http://hl7.org/fhir/StructureDefinition/Account',
    title: 'Account',
    description: 'A financial tool for tracking value accrued for a particular purpose.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'ActivityDefinition',
    url: 'http://hl7.org/fhir/StructureDefinition/ActivityDefinition',
    title: 'ActivityDefinition',
    description: 'A resource describing the definition of a requested or planned activity.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'AllergyIntolerance',
    url: 'http://hl7.org/fhir/StructureDefinition/AllergyIntolerance',
    title: 'Allergy Intolerance',
    description: 'Risk of harmful or undesirable physiological response from a substance or agent.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Appointment',
    url: 'http://hl7.org/fhir/StructureDefinition/Appointment',
    title: 'Appointment',
    description:
      'A booking of a healthcare event among patient(s), practitioner(s), and device(s).',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'CarePlan',
    url: 'http://hl7.org/fhir/StructureDefinition/CarePlan',
    title: 'Care Plan',
    description: 'Describes the intention of how one or more practitioners care for a patient.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Claim',
    url: 'http://hl7.org/fhir/StructureDefinition/Claim',
    title: 'Claim',
    description:
      'A provider-issued list of professional services and products for reimbursement or communication.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Condition',
    url: 'http://hl7.org/fhir/StructureDefinition/Condition',
    title: 'Condition',
    description: 'A clinical condition, problem, diagnosis, or other health event.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Coverage',
    url: 'http://hl7.org/fhir/StructureDefinition/Coverage',
    title: 'Coverage',
    description: 'Financial instrument providing reimbursement for health care services.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Device',
    url: 'http://hl7.org/fhir/StructureDefinition/Device',
    title: 'Device',
    description: 'A type of manufactured item used in the provision of healthcare.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'DiagnosticReport',
    url: 'http://hl7.org/fhir/StructureDefinition/DiagnosticReport',
    title: 'Diagnostic Report',
    description: 'The findings and interpretation of diagnostic tests.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Encounter',
    url: 'http://hl7.org/fhir/StructureDefinition/Encounter',
    title: 'Encounter',
    description: 'An interaction between a patient and healthcare provider(s).',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Goal',
    url: 'http://hl7.org/fhir/StructureDefinition/Goal',
    title: 'Goal',
    description: 'Describes the intended objective(s) for a patient, group or organization.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Immunization',
    url: 'http://hl7.org/fhir/StructureDefinition/Immunization',
    title: 'Immunization',
    description: 'Immunization event information.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Location',
    url: 'http://hl7.org/fhir/StructureDefinition/Location',
    title: 'Location',
    description: 'Details and position information for a physical place.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Medication',
    url: 'http://hl7.org/fhir/StructureDefinition/Medication',
    title: 'Medication',
    description: 'Definition of a medication for the purposes of prescribing, dispensing, etc.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'MedicationRequest',
    url: 'http://hl7.org/fhir/StructureDefinition/MedicationRequest',
    title: 'Medication Request',
    description: 'An order or request for a medication.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Observation',
    url: 'http://hl7.org/fhir/StructureDefinition/Observation',
    title: 'Observation',
    description:
      'Measurements and simple assertions made about a patient, device or other subject.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Organization',
    url: 'http://hl7.org/fhir/StructureDefinition/Organization',
    title: 'Organization',
    description: 'A formally or informally recognized grouping of people or organizations.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Patient',
    url: 'http://hl7.org/fhir/StructureDefinition/Patient',
    title: 'Patient',
    description:
      'Demographics and other administrative information about an individual or animal receiving care.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Practitioner',
    url: 'http://hl7.org/fhir/StructureDefinition/Practitioner',
    title: 'Practitioner',
    description:
      'A person who is directly or indirectly involved in the provisioning of healthcare.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Procedure',
    url: 'http://hl7.org/fhir/StructureDefinition/Procedure',
    title: 'Procedure',
    description: 'An action that is performed on a patient to diagnose, treat, or otherwise.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Questionnaire',
    url: 'http://hl7.org/fhir/StructureDefinition/Questionnaire',
    title: 'Questionnaire',
    description: 'A structured set of questions intended to guide data collection.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'QuestionnaireResponse',
    url: 'http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse',
    title: 'Questionnaire Response',
    description: 'A structured set of questions and their answers.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'RelatedPerson',
    url: 'http://hl7.org/fhir/StructureDefinition/RelatedPerson',
    title: 'Related Person',
    description:
      'Information about a person that is involved in the care of a patient but not a direct target of care.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'ServiceRequest',
    url: 'http://hl7.org/fhir/StructureDefinition/ServiceRequest',
    title: 'Service Request',
    description: 'A record of a request for a procedure or diagnostic to be performed.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
  {
    name: 'Specimen',
    url: 'http://hl7.org/fhir/StructureDefinition/Specimen',
    title: 'Specimen',
    description: 'A sample to be used for analysis.',
    packageName: 'hl7.fhir.r4.core',
    packageVersion: '4.0.1',
  },
];

// Factory function for creating mock profiles
export function createMockProfile(overrides?: Partial<Profile>): Profile {
  return {
    id: `profile-${Date.now()}`,
    url: `http://example.org/StructureDefinition/profile-${Date.now()}`,
    name: `CustomProfile${Date.now()}`,
    title: 'Custom Profile',
    status: 'draft',
    fhirVersion: '4.0.1',
    baseDefinition: 'http://hl7.org/fhir/StructureDefinition/Patient',
    derivation: 'constraint',
    elements: [],
    isDirty: false,
    ...overrides,
  };
}
