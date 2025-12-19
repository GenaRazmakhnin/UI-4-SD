import type React from 'react';
import { useMemo, useState } from 'react';
import profileSchema from './cl.core.profile.json';
import addressSchema from './r4.address.json';
import attachmentSchema from './r4.attachment.json';
import backboneElementSchema from './r4.backboneelement.json';
import codeableConceptSchema from './r4.codeableconcept.json';
import contactPointSchema from './r4.contactpoint.json';
import baseSchema from './r4.core.json';
import domainSchema from './r4.domainresource.json';
import humanNameSchema from './r4.humanname.json';
import identifierSchema from './r4.identifier.json';
import periodSchema from './r4.period.json';
import referenceSchema from './r4.reference.json';
import resourceSchema from './r4.resource.json';

type RawElement = {
  type?: string;
  array?: boolean;
  scalar?: boolean;
  short?: string;
  comment?: string;
  definition?: string;
  requirements?: string;
  summary?: boolean;
  modifier?: boolean;
  mustSupport?: boolean;
  binding?: Record<string, unknown>;
  elements?: Record<string, RawElement>;
  choices?: string[];
  choiceOf?: string;
  min?: number;
  max?: number | string;
  slicing?: {
    slices?: Record<
      string,
      {
        min?: number;
        max?: number | string;
        schema?: RawElement;
      }
    >;
  };
  required?: string[];
};

type RawSchema = {
  name: string;
  elements: Record<string, RawElement>;
  type?: string;
};

type TreeNode = {
  id: string;
  name: string;
  children?: TreeNode[];
  sliceVariants?: { id: string; label: string; children: TreeNode[] }[];
  meta: {
    type: string;
    min: number | string;
    max: number | string;
    isSummary?: boolean;
    isModifier?: boolean;
    mustSupport?: boolean;
    short?: string;
    desc?: string;
  };
};

const pickDescription = (meta?: RawElement) => {
  if (!meta) return undefined;
  return meta.short ?? meta.definition ?? meta.comment ?? meta.requirements;
};

const computeCardinality = (meta: RawElement, isRequired: boolean) => {
  const min = meta.min ?? (isRequired ? 1 : 0);
  const max = meta.max ?? (meta.array ? '*' : meta.scalar || meta.type || meta.choices ? 1 : 1);

  return { min, max };
};

const typeElementMaps: Record<string, Record<string, RawElement>> = [
  resourceSchema as RawSchema,
  domainSchema as RawSchema,
  baseSchema as RawSchema,
  humanNameSchema as RawSchema,
  addressSchema as RawSchema,
  identifierSchema as RawSchema,
  contactPointSchema as RawSchema,
  periodSchema as RawSchema,
  referenceSchema as RawSchema,
  attachmentSchema as RawSchema,
  codeableConceptSchema as RawSchema,
  backboneElementSchema as RawSchema,
].reduce<Record<string, Record<string, RawElement>>>((acc, schema) => {
  if (schema?.type && schema.elements) {
    acc[schema.type] = schema.elements;
  }
  return acc;
}, {});

const getTypeElements = (typeName?: string) => {
  if (!typeName) return {};
  return typeElementMaps[typeName] ?? {};
};

const mergeMeta = (base: RawElement | undefined, profile: RawElement | undefined): RawElement => {
  if (!base && !profile) return {};
  if (!base) return { ...profile };
  if (!profile) return { ...base };

  return {
    ...base,
    ...profile,
    elements:
      base.elements || profile.elements
        ? { ...(base.elements ?? {}), ...(profile.elements ?? {}) }
        : undefined,
  };
};

const buildNodes = (
  baseElements: Record<string, RawElement>,
  profileElements: Record<string, RawElement>,
  parentId: string,
  rootBaseElements: Record<string, RawElement>,
  requiredChildren: string[] = [],
  typeStack: string[] = []
): TreeNode[] => {
  const orderedNames = [
    ...Object.keys(baseElements),
    ...Object.keys(profileElements).filter((name) => !(name in baseElements)),
  ];

  return orderedNames.reduce<TreeNode[]>((acc, name) => {
    const baseMeta = baseElements[name];
    const profileMeta = profileElements[name];

    if (baseMeta?.choiceOf) return acc;

    if (name === 'extension') {
      const slices = profileMeta?.slicing?.slices;
      if (slices) {
        const mergedMeta = mergeMeta(baseMeta, profileMeta);
        const sliceBaseElements = {
          ...getTypeElements(mergedMeta.type),
          ...(mergedMeta.elements ?? {}),
        };
        Object.entries(slices).forEach(([sliceName, sliceMeta]) => {
          const sliceSchema = sliceMeta.schema ?? {};
          const sliceCardinality = computeCardinality(
            { ...mergedMeta, ...sliceSchema, min: sliceMeta.min, max: sliceMeta.max },
            false
          );
          const sliceChildren = buildNodes(
            sliceBaseElements,
            sliceSchema.elements ?? {},
            `${parentId}/${name}:${sliceName}`,
            rootBaseElements,
            sliceSchema.required ?? [],
            mergedMeta.type ? [...typeStack, mergedMeta.type] : typeStack
          );

          acc.push({
            id: `${parentId}/${name}:${sliceName}`,
            name: `${name}:${sliceName}`,
            children: sliceChildren.length ? sliceChildren : undefined,
            meta: {
              type: sliceSchema.type ?? mergedMeta.type ?? 'Extension',
              min: sliceCardinality.min,
              max: sliceCardinality.max,
              isSummary: sliceSchema.summary ?? mergedMeta.summary,
              isModifier: sliceSchema.modifier ?? mergedMeta.modifier,
              mustSupport: sliceSchema.mustSupport ?? mergedMeta.mustSupport,
              short: pickDescription(sliceSchema) ?? pickDescription(mergedMeta),
              desc:
                sliceSchema.definition ??
                sliceSchema.comment ??
                sliceSchema.requirements ??
                mergedMeta.definition,
            },
          });
        });
        return acc;
      }
    }

    const mergedMeta = mergeMeta(baseMeta, profileMeta);
    const nodeId = `${parentId}/${name}`;
    const { min, max } = computeCardinality(mergedMeta, requiredChildren.includes(name));
    const metaTypeName = mergedMeta.type ?? 'Element';
    const metaType = mergedMeta.choices ? 'union' : metaTypeName;

    const typeElements = metaType !== 'union' ? getTypeElements(metaTypeName) : {};
    const alreadyInStack = metaTypeName ? typeStack.includes(metaTypeName) : false;
    const baseForChildren = alreadyInStack
      ? {}
      : { ...typeElements, ...(mergedMeta.elements ?? {}) };

    const nestedBaseElements = baseForChildren;
    const nestedProfileElements = profileMeta?.elements ?? {};
    const nestedRequired = mergedMeta.required ?? [];
    const nextStack = metaTypeName ? [...typeStack, metaTypeName] : typeStack;

    const children: TreeNode[] = buildNodes(
      nestedBaseElements,
      nestedProfileElements,
      nodeId,
      rootBaseElements,
      nestedRequired,
      nextStack
    );
    let sliceVariants: TreeNode['sliceVariants'];

    if (mergedMeta.choices?.length) {
      mergedMeta.choices.forEach((choiceName) => {
        const choiceBase = rootBaseElements[choiceName];
        const choiceProfile = profileElements[choiceName];
        if (!choiceBase && !choiceProfile) return;
        const combinedMeta = mergeMeta(choiceBase, choiceProfile);
        const { min: choiceMin, max: choiceMax } = computeCardinality(combinedMeta, false);
        children.push({
          id: `${nodeId}/${choiceName}`,
          name: choiceName,
          meta: {
            type: combinedMeta.type ?? 'Element',
            min: choiceMin,
            max: choiceMax,
            isSummary: combinedMeta.summary,
            isModifier: combinedMeta.modifier,
            mustSupport: combinedMeta.mustSupport ?? profileElements?.[choiceName]?.mustSupport,
            short: pickDescription(combinedMeta),
            desc: combinedMeta.definition ?? combinedMeta.comment ?? combinedMeta.requirements,
          },
        });
      });
    }

    const slices = profileMeta?.slicing?.slices;
    if (slices && name !== 'extension') {
      sliceVariants = Object.entries(slices).map(([sliceName, sliceMeta]) => {
        const sliceSchema = sliceMeta.schema ?? {};
        const sliceBaseElements = baseForChildren;
        const sliceChildren = buildNodes(
          sliceBaseElements,
          sliceSchema.elements ?? {},
          `${nodeId}/${sliceName}`,
          rootBaseElements,
          sliceSchema.required ?? [],
          metaTypeName ? [...typeStack, metaTypeName] : typeStack
        );
        return {
          id: `${nodeId}:${sliceName}`,
          label: sliceName,
          children: sliceChildren,
        };
      });
    }

    acc.push({
      id: nodeId,
      name,
      children: children.length ? children : undefined,
      sliceVariants,
      meta: {
        type: metaType,
        min,
        max,
        isSummary: mergedMeta.summary,
        isModifier: mergedMeta.modifier,
        mustSupport: mergedMeta.mustSupport ?? profileMeta?.mustSupport,
        short: pickDescription(mergedMeta),
        desc: mergedMeta.definition ?? mergedMeta.comment ?? mergedMeta.requirements,
      },
    });

    return acc;
  }, []);
};

const buildTree = (): TreeNode => {
  const coreElements = (baseSchema as RawSchema).elements ?? {};
  const mergedBaseElements: Record<string, RawElement> = {
    ...coreElements,
  };
  const profileElements = (profileSchema as RawSchema).elements ?? {};
  const children = buildNodes(mergedBaseElements, profileElements, 'root', mergedBaseElements);

  return {
    id: 'root',
    name: (baseSchema as RawSchema).name ?? 'Resource',
    children,
    meta: {
      type: 'Resource',
      min: 0,
      max: '*',
      short: (baseSchema as RawSchema).name,
    },
  };
};

const typeBadgeColor: Record<string, string> = {
  Resource: '#4B5563',
  BackboneElement: '#2563EB',
  union: '#9333EA',
  boolean: '#1F2937',
  integer: '#065F46',
  code: '#1F2937',
  string: '#1F2937',
};

const ColumnHeader = () => (
  <div style={styles.header}>
    <div style={{ ...styles.cell, ...styles.nameCol }}>Name</div>
    <div style={{ ...styles.cell, ...styles.flagsCol }}>Flags</div>
    <div style={{ ...styles.cell, ...styles.cardCol }}>Card.</div>
    <div style={{ ...styles.cell, ...styles.typeCol }}>Type</div>
    <div style={{ ...styles.cell, ...styles.descCol }}>Description</div>
  </div>
);

const Flag = ({ label, tooltip }: { label: string; tooltip: string }) => (
  <span style={styles.flag} title={tooltip}>
    {label}
  </span>
);

const Row = ({
  node,
  level,
  index,
  expanded,
  onToggle,
  sliceSelection,
  onSliceChange,
}: {
  node: TreeNode;
  level: number;
  index: number;
  expanded: Set<string>;
  onToggle: (id: string) => void;
  sliceSelection: Record<string, string>;
  onSliceChange: (id: string, value: string) => void;
}) => {
  const hasBaseChildren = Boolean(node.children?.length);
  const hasSliceChildren = (node.sliceVariants ?? []).some((v) => v.children.length > 0);
  const hasChildren = hasBaseChildren || hasSliceChildren;
  const isOpen = expanded.has(node.id);
  const paddingLeft = level * 20;
  const hasSlices = node.sliceVariants && node.sliceVariants.length > 0;
  const selectedSlice = sliceSelection[node.id] ?? '__base';
  const isExtensionRow = node.name.startsWith('extension:');
  const backgroundColor = (() => {
    if (isExtensionRow && level === 1) return '#0d4f7b'; // lifted root extensions: blue
    return index % 2 === 0 ? '#0B1220' : '#0F172A';
  })();

  return (
    <div
      style={{
        ...styles.row,
        background: backgroundColor,
      }}
    >
      <div style={{ ...styles.cell, ...styles.nameCol, paddingLeft: paddingLeft + 12 }}>
        {hasChildren && (
          <button style={styles.chevron} onClick={() => onToggle(node.id)} aria-label="toggle">
            {isOpen ? '▾' : '▸'}
          </button>
        )}
        {!hasChildren && <span style={styles.placeholder} />}
        <span style={styles.fieldName}>{node.name}</span>
        {hasSlices && (
          <div style={styles.sliceSwitch}>
            {[{ id: '__base', label: 'R4' }, ...(node.sliceVariants ?? [])].map((variant) => (
              <button
                key={variant.id}
                style={{
                  ...styles.sliceButton,
                  ...(selectedSlice === variant.id ? styles.sliceButtonActive : {}),
                }}
                onClick={() => onSliceChange(node.id, variant.id)}
              >
                {variant.label}
              </button>
            ))}
          </div>
        )}
      </div>
      <div style={{ ...styles.cell, ...styles.flagsCol }}>
        {node.meta.mustSupport && <Flag label="S" tooltip="Must be supported" />}
        {node.meta.isSummary && <Flag label="Σ" tooltip="Summary" />}
        {node.meta.isModifier && <Flag label="?!" tooltip="Modifier element" />}
      </div>
      <div style={{ ...styles.cell, ...styles.cardCol }}>
        {node.meta.min}..{node.meta.max}
      </div>
      <div style={{ ...styles.cell, ...styles.typeCol }}>
        <span
          style={{
            ...styles.typePill,
            background: typeBadgeColor[node.meta.type] ?? '#E5E7EB',
          }}
        >
          {node.meta.type}
        </span>
      </div>
      <div style={{ ...styles.cell, ...styles.descCol }}>
        {node.meta.short || node.meta.desc || ''}
      </div>
    </div>
  );
};

export const FhirSchemaTree = () => {
  const tree = useMemo(() => buildTree(), []);
  const [expanded, setExpanded] = useState<Set<string>>(() => new Set<string>());
  const [sliceSelection, setSliceSelection] = useState<Record<string, string>>({});

  const toggle = (id: string) => {
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const visibleNodes = useMemo(() => {
    const rows: { node: TreeNode; level: number }[] = [];
    const walk = (n: TreeNode, level: number) => {
      rows.push({ node: n, level });
      if (!expanded.has(n.id)) return;

      const selectedVariantId = sliceSelection[n.id];
      const selectedVariant = n.sliceVariants?.find((v) => v.id === selectedVariantId);
      const children = selectedVariant?.children ?? n.children ?? [];

      children.forEach((child) => walk(child, level + 1));
    };
    walk(tree, 0);
    return rows;
  }, [expanded, tree, sliceSelection]);

  return (
    <div style={styles.container}>
      <ColumnHeader />
      {visibleNodes.map((row, idx) => (
        <Row
          key={row.node.id}
          node={row.node}
          level={row.level}
          index={idx}
          expanded={expanded}
          onToggle={toggle}
          sliceSelection={sliceSelection}
          onSliceChange={(id, value) => setSliceSelection((prev) => ({ ...prev, [id]: value }))}
        />
      ))}
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  container: {
    width: '100%',
    border: '1px solid #1F2937',
    borderRadius: 8,
    overflow: 'auto',
    fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
    fontSize: 12,
    color: '#E5E7EB',
    background: '#0B1220',
  },
  header: {
    display: 'grid',
    gridTemplateColumns: '390px 70px 70px 200px 1fr',
    padding: '12px 16px',
    background: '#111827',
    borderBottom: '1px solid #1F2937',
    fontWeight: 600,
    color: '#9CA3AF',
    position: 'sticky',
    top: 0,
    zIndex: 1,
  },
  row: {
    display: 'grid',
    gridTemplateColumns: '390px 70px 70px 200px 1fr',
    alignItems: 'center',
    padding: '8px 16px',
    borderBottom: '1px solid #1F2937',
  },
  cell: {
    minHeight: 20,
    display: 'flex',
    alignItems: 'center',
    gap: 8,
  },
  nameCol: {
    minWidth: 390,
  },
  flagsCol: {
    minWidth: 70,
  },
  cardCol: {
    minWidth: 70,
    fontVariantNumeric: 'tabular-nums',
  },
  typeCol: {
    minWidth: 200,
  },
  descCol: {
    minWidth: 200,
    color: '#9CA3AF',
  },
  chevron: {
    border: 'none',
    background: 'transparent',
    cursor: 'pointer',
    fontSize: 12,
    padding: 0,
    width: 16,
    color: '#9CA3AF',
  },
  placeholder: {
    display: 'inline-block',
    width: 16,
  },
  fieldName: {
    fontWeight: 500,
    color: '#E5E7EB',
  },
  flag: {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    minWidth: 16,
    height: 16,
    padding: '0 4px',
    borderRadius: 4,
    background: '#1F2937',
    color: '#E5E7EB',
    fontWeight: 600,
    cursor: 'help',
  },
  typePill: {
    padding: '2px 8px',
    borderRadius: 999,
    color: '#fff',
    fontWeight: 600,
    fontSize: 11,
    textTransform: 'capitalize',
  },
  sliceSwitch: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: 4,
    marginLeft: 8,
  },
  sliceButton: {
    border: '1px solid #1F2937',
    background: '#0F172A',
    color: '#D1D5DB',
    borderRadius: 6,
    padding: '2px 6px',
    fontSize: 11,
    cursor: 'pointer',
  },
  sliceButtonActive: {
    background: '#2563EB',
    borderColor: '#1D4ED8',
    color: '#fff',
  },
};
