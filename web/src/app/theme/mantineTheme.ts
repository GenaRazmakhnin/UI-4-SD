import {
  createTheme,
  defaultCssVariablesResolver,
  type CSSVariablesResolver,
  type MantineColorsTuple,
} from '@mantine/core';

function tokenColorTuple(prefix: string): MantineColorsTuple {
  return [
    `var(--color-${prefix}-50)`,
    `var(--color-${prefix}-100)`,
    `var(--color-${prefix}-200)`,
    `var(--color-${prefix}-300)`,
    `var(--color-${prefix}-400)`,
    `var(--color-${prefix}-500)`,
    `var(--color-${prefix}-600)`,
    `var(--color-${prefix}-700)`,
    `var(--color-${prefix}-800)`,
    `var(--color-${prefix}-900)`,
  ] as const;
}

const darkTuple: MantineColorsTuple = [
  'var(--color-neutral-950)',
  'var(--color-neutral-900)',
  'var(--color-neutral-800)',
  'var(--color-neutral-700)',
  'var(--color-neutral-600)',
  'var(--color-neutral-500)',
  'var(--color-neutral-400)',
  'var(--color-neutral-300)',
  'var(--color-neutral-200)',
  'var(--color-neutral-100)',
] as const;

export const mantineTheme = createTheme({
  fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
  fontFamilyMonospace: '"JetBrains Mono", "Fira Code", "Consolas", "Monaco", monospace',
  primaryColor: 'brand',
  primaryShade: { light: 6, dark: 6 },
  defaultRadius: 'md',
  spacing: {
    xs: 'var(--spacing-half)',
    sm: 'var(--spacing-x1)',
    md: 'var(--spacing-x2)',
    lg: 'var(--spacing-x3)',
    xl: 'var(--spacing-x4)',
  },
  radius: {
    xs: 'var(--corner-corner-xs)',
    sm: 'var(--corner-corner-s)',
    md: 'var(--corner-corner-m)',
    lg: 'var(--corner-corner-l)',
    xl: 'var(--corner-corner-l)',
  },
  colors: {
    brand: tokenColorTuple('brand'),
    gray: tokenColorTuple('neutral'),
    dark: darkTuple,
    red: tokenColorTuple('red'),
    blue: tokenColorTuple('blue'),
    green: tokenColorTuple('green'),
    yellow: tokenColorTuple('yellow'),
  },
});

export const mantineCssVariablesResolver: CSSVariablesResolver = (theme) => {
  const base = defaultCssVariablesResolver(theme);

  return {
    ...base,
    light: {
      ...base.light,
      '--mantine-color-body': 'var(--color-surface-0)',
      '--mantine-color-text': 'var(--color-elements-readable)',
    },
    dark: {
      ...base.dark,
      '--mantine-color-body': 'var(--color-neutral-950)',
      '--mantine-color-text': 'var(--color-elements-readable-inv)',
    },
  };
};

