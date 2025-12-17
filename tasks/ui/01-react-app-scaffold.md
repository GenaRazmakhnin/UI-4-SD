# Task: React App Scaffold with FSD Architecture

## 📋 Description

Set up the React application with Feature-Sliced Design (FSD) architecture, Vite build system, and foundational libraries (Effector, TanStack Query, Mantine UI) to create a robust, maintainable, and scalable foundation for the FHIR Profile Builder.

**Reference**: IMPLEMENTATION_PLAN.md Section 13 "FSD Architecture", Section 15 "Technology Stack"

## 🎯 Context from Implementation Plan

This implements the foundational architecture described in:
- **FSD Architecture** (Section 13): Feature-Sliced Design with strict layer boundaries
- **Technology Stack** (Section 15): React + TypeScript + Vite + Effector ecosystem
- **CSS Modules** (Section 18): Component-scoped styling with CSS Modules
- **Parallel Development** (Section 20): Architecture that enables independent feature development

## 📐 Requirements

### R1: Project Initialization with Vite

**Complete Vite Setup**:
```bash
# Initialize Vite project
bunx create-vite web --template react-ts
cd web

# Install core dependencies
bun install

# Install state management
bun add effector effector-react patronum effector-storage

# Install data fetching
bun add @tanstack/react-query @tanstack/react-query-devtools

# Install routing
bun add @tanstack/react-router @tanstack/router-devtools

# Install UI library
bun add @mantine/core @mantine/hooks @mantine/form @mantine/notifications @mantine/modals @mantine/spotlight
bun add @tabler/icons-react

# Install forms and validation
bun add react-hook-form @hookform/resolvers zod

# Install utilities
bun add clsx class-variance-authority nanoid date-fns

# Install virtualization
bun add react-window react-window-infinite-loader
bun add -D @types/react-window

# Install code editor
bun add @monaco-editor/react monaco-editor

# Install dev dependencies
bun add -D @types/node
bun add -D @biomejs/biome # Linter and formatter
bun add -D vite-tsconfig-paths
```

**Package.json Scripts**:
```json
{
  "name": "fhir-profile-builder-ui",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "biome check .",
    "lint:fix": "biome check --write .",
    "format": "biome format --write .",
    "format:check": "biome format .",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "effector": "^23.2.2",
    "effector-react": "^23.2.1",
    "patronum": "^2.2.0",
    "effector-storage": "^7.1.0",
    "@tanstack/react-query": "^5.56.2",
    "@tanstack/react-query-devtools": "^5.56.2",
    "@tanstack/react-router": "^1.58.3",
    "@tanstack/router-devtools": "^1.58.3",
    "@mantine/core": "^8.0.0",
    "@mantine/hooks": "^8.0.0",
    "@mantine/form": "^8.0.0",
    "@mantine/notifications": "^8.0.0",
    "@mantine/modals": "^8.0.0",
    "@mantine/spotlight": "^8.0.0",
    "@tabler/icons-react": "^3.17.0",
    "react-hook-form": "^7.53.0",
    "@hookform/resolvers": "^3.9.0",
    "zod": "^3.23.8",
    "clsx": "^2.1.1",
    "class-variance-authority": "^0.7.0",
    "nanoid": "^5.0.7",
    "date-fns": "^4.1.0",
    "react-window": "^1.8.10",
    "react-window-infinite-loader": "^1.0.9",
    "@monaco-editor/react": "^4.6.0",
    "monaco-editor": "^0.52.0"
  },
  "devDependencies": {
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0",
    "@types/react-window": "^1.8.8",
    "@types/node": "^22.7.5",
    "@vitejs/plugin-react": "^4.3.2",
    "@biomejs/biome": "^1.9.4",
    "typescript": "^5.6.2",
    "vite": "^7.0.0",
    "vite-tsconfig-paths": "^5.0.1"
  }
}
```

### R2: FSD Directory Structure

**Complete FSD Implementation** (Reference: IMPLEMENTATION_PLAN.md Section 13):
```bash
# Create complete FSD structure
mkdir -p web/src/{app,pages,widgets,features,entities,shared}

# App layer
mkdir -p web/src/app/{providers,routes,styles}

# Pages layer
mkdir -p web/src/pages/{editor,project-browser,settings}

# Widgets layer
mkdir -p web/src/widgets/{element-tree,inspector-panel,diagnostics-panel,preview-panel,package-browser,quick-constraints}

# Features layer
mkdir -p web/src/features/{edit-cardinality,edit-binding,edit-flags,edit-type-constraint,add-slice,search-extensions,search-packages,undo-redo}

# Entities layer
mkdir -p web/src/entities/{profile,element,package,validation,project}

# Shared layer
mkdir -p web/src/shared/{ui,api,lib,config,types}

# Create public API index files
touch web/src/{app,pages,widgets,features,entities,shared}/index.ts
```

**Directory Structure**:
```
web/
├── public/
│   └── vite.svg
├── src/
│   ├── app/                         # Application layer
│   │   ├── providers/
│   │   │   ├── EffectorProvider.tsx
│   │   │   ├── QueryProvider.tsx
│   │   │   ├── RouterProvider.tsx
│   │   │   ├── MantineProvider.tsx
│   │   │   └── index.tsx
│   │   ├── routes/
│   │   │   ├── index.tsx
│   │   │   └── routes.ts
│   │   ├── styles/
│   │   │   ├── globals.css
│   │   │   ├── variables.css
│   │   │   └── reset.css
│   │   ├── App.tsx
│   │   └── index.ts
│   ├── pages/                       # Pages layer
│   │   ├── editor/
│   │   │   ├── ui/
│   │   │   │   ├── ProfileEditorPage.tsx
│   │   │   │   └── ProfileEditorPage.module.css
│   │   │   ├── model/
│   │   │   │   └── index.ts
│   │   │   └── index.ts
│   │   ├── project-browser/
│   │   │   ├── ui/
│   │   │   │   └── ProjectBrowserPage.tsx
│   │   │   └── index.ts
│   │   └── settings/
│   │       └── index.ts
│   ├── widgets/                     # Widgets layer
│   │   ├── element-tree/
│   │   │   ├── ui/
│   │   │   │   ├── ElementTree.tsx
│   │   │   │   ├── ElementRow.tsx
│   │   │   │   └── ElementTree.module.css
│   │   │   ├── model/
│   │   │   │   └── index.ts
│   │   │   ├── lib/
│   │   │   │   └── useTreeKeyboard.ts
│   │   │   └── index.ts
│   │   ├── inspector-panel/
│   │   │   └── index.ts
│   │   ├── diagnostics-panel/
│   │   │   └── index.ts
│   │   └── preview-panel/
│   │       └── index.ts
│   ├── features/                    # Features layer
│   │   ├── edit-cardinality/
│   │   │   ├── ui/
│   │   │   │   ├── CardinalityEditor.tsx
│   │   │   │   └── CardinalityEditor.module.css
│   │   │   ├── model/
│   │   │   │   └── index.ts
│   │   │   ├── lib/
│   │   │   │   └── validation.ts
│   │   │   └── index.ts
│   │   ├── edit-binding/
│   │   │   └── index.ts
│   │   └── add-slice/
│   │       └── index.ts
│   ├── entities/                    # Entities layer
│   │   ├── profile/
│   │   │   ├── model/
│   │   │   │   ├── index.ts
│   │   │   │   └── types.ts
│   │   │   ├── api/
│   │   │   │   └── profileApi.ts
│   │   │   ├── ui/
│   │   │   │   └── ProfileCard.tsx
│   │   │   └── index.ts
│   │   ├── element/
│   │   │   └── index.ts
│   │   └── package/
│   │       └── index.ts
│   └── shared/                      # Shared layer
│       ├── ui/
│       │   ├── Button/
│       │   │   ├── Button.tsx
│       │   │   └── Button.module.css
│       │   ├── Badge/
│       │   ├── Input/
│       │   └── index.ts
│       ├── api/
│       │   ├── client.ts
│       │   ├── baseQuery.ts
│       │   └── index.ts
│       ├── lib/
│       │   ├── cn.ts
│       │   ├── formatters.ts
│       │   └── index.ts
│       ├── config/
│       │   ├── env.ts
│       │   └── index.ts
│       └── types/
│           ├── fhir.ts
│           └── index.ts
├── index.html
├── package.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── biome.json
└── README.md
```

### R3: Vite Configuration

**Complete vite.config.ts**:
```typescript
// web/vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tsconfigPaths from 'vite-tsconfig-paths';
import path from 'path';

export default defineConfig({
  plugins: [
    react(),
    tsconfigPaths(),
  ],

  resolve: {
    alias: {
      '@app': path.resolve(__dirname, './src/app'),
      '@pages': path.resolve(__dirname, './src/pages'),
      '@widgets': path.resolve(__dirname, './src/widgets'),
      '@features': path.resolve(__dirname, './src/features'),
      '@entities': path.resolve(__dirname, './src/entities'),
      '@shared': path.resolve(__dirname, './src/shared'),
    },
  },

  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },

  build: {
    outDir: 'dist',
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          // Vendor chunks for better caching
          'react-vendor': ['react', 'react-dom'],
          'effector-vendor': ['effector', 'effector-react', 'patronum'],
          'ui-vendor': ['@mantine/core', '@mantine/hooks'],
          'query-vendor': ['@tanstack/react-query'],
          'router-vendor': ['@tanstack/react-router'],
          'monaco-vendor': ['@monaco-editor/react', 'monaco-editor'],
        },
      },
    },
    chunkSizeWarningLimit: 1000,
  },

  css: {
    modules: {
      localsConvention: 'camelCase',
      generateScopedName: '[name]__[local]___[hash:base64:5]',
    },
  },

  optimizeDeps: {
    include: [
      'react',
      'react-dom',
      'effector',
      'effector-react',
      '@mantine/core',
      '@mantine/hooks',
    ],
  },
});
```

### R4: TypeScript Configuration

**tsconfig.json**:
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedIndexedAccess": true,
    "noImplicitOverride": true,
    "forceConsistentCasingInFileNames": true,

    /* Path Aliases */
    "baseUrl": ".",
    "paths": {
      "@app/*": ["./src/app/*"],
      "@pages/*": ["./src/pages/*"],
      "@widgets/*": ["./src/widgets/*"],
      "@features/*": ["./src/features/*"],
      "@entities/*": ["./src/entities/*"],
      "@shared/*": ["./src/shared/*"]
    },

    /* Additional Options */
    "incremental": true,
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

**tsconfig.node.json**:
```json
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
```

### R5: Biome Configuration

**biome.json**:
```json
{
  "$schema": "https://biomejs.dev/schemas/1.9.4/schema.json",
  "vcs": {
    "enabled": true,
    "clientKind": "git",
    "useIgnoreFile": true
  },
  "files": {
    "ignoreUnknown": false,
    "ignore": ["node_modules", "dist", "build", ".next"]
  },
  "formatter": {
    "enabled": true,
    "formatWithErrors": false,
    "indentStyle": "space",
    "indentWidth": 2,
    "lineWidth": 100,
    "lineEnding": "lf"
  },
  "organizeImports": {
    "enabled": true
  },
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true,
      "complexity": {
        "noExtraBooleanCast": "error",
        "noMultipleSpacesInRegularExpressionLiterals": "error",
        "noUselessCatch": "error",
        "noUselessTypeConstraint": "error",
        "noWith": "error"
      },
      "correctness": {
        "noConstAssign": "error",
        "noConstantCondition": "error",
        "noEmptyCharacterClassInRegex": "error",
        "noEmptyPattern": "error",
        "noGlobalObjectCalls": "error",
        "noInvalidConstructorSuper": "error",
        "noInvalidNewBuiltin": "error",
        "noNonoctalDecimalEscape": "error",
        "noPrecisionLoss": "error",
        "noSelfAssign": "error",
        "noSetterReturn": "error",
        "noSwitchDeclarations": "error",
        "noUndeclaredVariables": "error",
        "noUnreachable": "error",
        "noUnreachableSuper": "error",
        "noUnsafeFinally": "error",
        "noUnsafeOptionalChaining": "error",
        "noUnusedLabels": "error",
        "noUnusedVariables": "error",
        "useIsNan": "error",
        "useValidForDirection": "error",
        "useYield": "error"
      },
      "style": {
        "noArguments": "error",
        "noVar": "error",
        "useConst": "error",
        "useTemplate": "error"
      },
      "suspicious": {
        "noAsyncPromiseExecutor": "error",
        "noCatchAssign": "error",
        "noClassAssign": "error",
        "noCompareNegZero": "error",
        "noControlCharactersInRegex": "error",
        "noDebugger": "error",
        "noDoubleEquals": "warn",
        "noDuplicateCase": "error",
        "noDuplicateClassMembers": "error",
        "noDuplicateObjectKeys": "error",
        "noDuplicateParameters": "error",
        "noEmptyBlockStatements": "error",
        "noExplicitAny": "warn",
        "noExtraNonNullAssertion": "error",
        "noFallthroughSwitchClause": "error",
        "noFunctionAssign": "error",
        "noGlobalAssign": "error",
        "noImportAssign": "error",
        "noMisleadingCharacterClass": "error",
        "noPrototypeBuiltins": "error",
        "noRedeclare": "error",
        "noShadowRestrictedNames": "error",
        "noUnsafeNegation": "error",
        "useGetterReturn": "error",
        "useValidTypeof": "error"
      }
    }
  },
  "javascript": {
    "formatter": {
      "jsxQuoteStyle": "double",
      "quoteProperties": "asNeeded",
      "trailingCommas": "es5",
      "semicolons": "always",
      "arrowParentheses": "always",
      "bracketSpacing": true,
      "bracketSameLine": false,
      "quoteStyle": "single",
      "attributePosition": "auto"
    }
  },
  "overrides": [
    {
      "include": ["*.ts", "*.tsx"],
      "linter": {
        "rules": {
          "correctness": {
            "noUndeclaredVariables": "off"
          }
        }
      }
    }
  ]
}
```

**Note**: FSD boundary enforcement will be implemented through custom linting rules or architectural documentation. Biome doesn't have a plugin system like ESLint, so layer boundaries should be enforced through:

1. Code review practices
2. Architecture documentation
3. Path alias usage patterns
4. Team conventions

### R6: Optional oxc Integration (Faster TypeScript Transpilation)

**oxc** can be used as a faster alternative to TypeScript compiler. To integrate oxc:

```bash
# Install oxc
bun add -D @oxc-transform/vite-plugin
```

**Update vite.config.ts** (optional):
```typescript
// web/vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import oxc from '@oxc-transform/vite-plugin';
import tsconfigPaths from 'vite-tsconfig-paths';
import path from 'path';

export default defineConfig({
  plugins: [
    react(),
    oxc(), // Use oxc for faster transpilation
    tsconfigPaths(),
  ],
  // ... rest of config
});
```

**Note**: Using oxc is optional but recommended for faster builds. Keep TypeScript compiler for type checking (`npm run typecheck`).

### R7: Application Root Component

**src/app/App.tsx**:
```typescript
// web/src/app/App.tsx
import { StrictMode } from 'react';
import { MantineProvider } from './providers/MantineProvider';
import { EffectorProvider } from './providers/EffectorProvider';
import { QueryProvider } from './providers/QueryProvider';
import { RouterProvider } from './providers/RouterProvider';
import './styles/globals.css';

export function App() {
  return (
    <StrictMode>
      <EffectorProvider>
        <QueryProvider>
          <MantineProvider>
            <RouterProvider />
          </MantineProvider>
        </QueryProvider>
      </EffectorProvider>
    </StrictMode>
  );
}
```

**src/app/providers/MantineProvider.tsx**:
```typescript
// web/src/app/providers/MantineProvider.tsx
import { MantineProvider as MantineUIProvider, createTheme } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { ModalsProvider } from '@mantine/modals';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';

const theme = createTheme({
  fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
  fontFamilyMonospace: '"Fira Code", "Consolas", "Monaco", monospace',
  primaryColor: 'blue',
  defaultRadius: 'md',
});

interface Props {
  children: React.ReactNode;
}

export function MantineProvider({ children }: Props) {
  return (
    <MantineUIProvider theme={theme} defaultColorScheme="light">
      <Notifications position="top-right" />
      <ModalsProvider>{children}</ModalsProvider>
    </MantineUIProvider>
  );
}
```

**src/app/providers/EffectorProvider.tsx**:
```typescript
// web/src/app/providers/EffectorProvider.tsx
import { Provider } from 'effector-react';

interface Props {
  children: React.ReactNode;
}

export function EffectorProvider({ children }: Props) {
  return <Provider>{children}</Provider>;
}
```

**src/app/providers/QueryProvider.tsx**:
```typescript
// web/src/app/providers/QueryProvider.tsx
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { useState } from 'react';

interface Props {
  children: React.ReactNode;
}

export function QueryProvider({ children }: Props) {
  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            staleTime: 1000 * 60 * 5, // 5 minutes
            gcTime: 1000 * 60 * 10, // 10 minutes
            retry: 1,
            refetchOnWindowFocus: false,
          },
          mutations: {
            retry: 0,
          },
        },
      })
  );

  return (
    <QueryClientProvider client={queryClient}>
      {children}
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  );
}
```

### R8: Shared Utilities

**src/shared/lib/cn.ts** (className utility):
```typescript
// web/src/shared/lib/cn.ts
import { clsx, type ClassValue } from 'clsx';

/**
 * Combines multiple class names, supporting conditional classes
 * @example cn('base', condition && 'conditional', styles.module)
 */
export function cn(...inputs: ClassValue[]) {
  return clsx(inputs);
}
```

**src/shared/config/env.ts**:
```typescript
// web/src/shared/config/env.ts
export const ENV = {
  API_BASE_URL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',
  NODE_ENV: import.meta.env.MODE,
  isDev: import.meta.env.DEV,
  isProd: import.meta.env.PROD,
} as const;
```

**src/shared/api/client.ts**:
```typescript
// web/src/shared/api/client.ts
import { ENV } from '@shared/config';

export class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string = ENV.API_BASE_URL) {
    this.baseUrl = baseUrl;
  }

  async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    const response = await fetch(url, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    });

    if (!response.ok) {
      throw new Error(`API Error: ${response.status} ${response.statusText}`);
    }

    return response.json();
  }

  async get<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'GET' });
  }

  async post<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async patch<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'PATCH',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async delete<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'DELETE' });
  }
}

export const apiClient = new ApiClient();
```

### R9: Global Styles

**src/app/styles/globals.css**:
```css
/* web/src/app/styles/globals.css */
@import './reset.css';
@import './variables.css';

* {
  box-sizing: border-box;
}

html,
body,
#root {
  height: 100%;
  margin: 0;
  padding: 0;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial,
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

code {
  font-family: 'Fira Code', 'Consolas', 'Monaco', monospace;
}

#root {
  display: flex;
  flex-direction: column;
}
```

**src/app/styles/variables.css**:
```css
/* web/src/app/styles/variables.css */
:root {
  /* Colors */
  --primary-color: #228be6;
  --primary-hover: #1c7ed6;

  --bg-primary: #ffffff;
  --bg-secondary: #f8f9fa;
  --bg-tertiary: #e9ecef;

  --text-primary: #212529;
  --text-secondary: #495057;
  --text-tertiary: #868e96;

  --border-color: #dee2e6;
  --hover-bg: #f1f3f5;
  --selected-bg: #e7f5ff;

  /* Status colors */
  --success-color: #40c057;
  --warning-color: #fab005;
  --error-color: #fa5252;
  --info-color: #339af0;

  /* Element status colors */
  --modified-color: #228be6;
  --new-color: #40c057;
  --inherited-color: #868e96;

  /* Spacing */
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;

  /* Border radius */
  --radius-sm: 4px;
  --radius-md: 8px;
  --radius-lg: 12px;

  /* Shadows */
  --shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.1);
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1);

  /* Font sizes */
  --font-xs: 11px;
  --font-sm: 13px;
  --font-md: 14px;
  --font-lg: 16px;
  --font-xl: 18px;

  /* Monospace font */
  --font-mono: 'Fira Code', 'Consolas', 'Monaco', monospace;
}
```

### R10: Environment Variables

**.env.example**:
```bash
# API Configuration
VITE_API_BASE_URL=http://localhost:8080

# Development
VITE_ENABLE_DEVTOOLS=true
```

**.env.development**:
```bash
VITE_API_BASE_URL=http://localhost:8080
VITE_ENABLE_DEVTOOLS=true
```

**.env.production**:
```bash
VITE_API_BASE_URL=
VITE_ENABLE_DEVTOOLS=false
```

## ✅ Acceptance Criteria

### Functional Requirements
- [ ] Vite project initializes and runs (`bun run dev`)
- [ ] App opens in browser at http://localhost:3000
- [ ] All FSD directories are created with correct structure
- [ ] Path aliases work correctly (@app, @pages, @widgets, etc.)
- [ ] Development server proxies API calls to backend (localhost:8080)
- [ ] Hot module replacement (HMR) works
- [ ] TypeScript compilation has no errors
- [ ] All providers render correctly (Effector, Query, Mantine)
- [ ] React DevTools show component tree
- [ ] TanStack Query DevTools visible in development
- [ ] Console has no errors or warnings

### Build Requirements
- [ ] Production build completes successfully (`bun run build`)
- [ ] Build output is optimized:
  - [ ] Initial bundle <500KB (gzipped)
  - [ ] Vendor chunks created correctly
  - [ ] Code splitting works for routes
  - [ ] Source maps generated
- [ ] Preview mode works (`bun run preview`)
- [ ] Build is reproducible (same inputs = same outputs)

### Code Quality Requirements
- [ ] Biome linter runs without errors (`bun run lint`)
- [ ] Biome formatter formats code consistently (`bun run format`)
- [ ] TypeScript strict mode enabled with no errors
- [ ] All imports use path aliases (no relative imports across layers)
- [ ] Public API pattern enforced (index.ts exports)
- [ ] FSD boundary rules documented and followed

### Performance Requirements
- [ ] Dev server starts in <5 seconds
- [ ] HMR updates in <1 second
- [ ] Production build completes in <30 seconds
- [ ] Page load time <2 seconds (initial)
- [ ] Time to interactive <3 seconds

### Accessibility Requirements (WCAG 2.1 AA)
- [ ] No accessibility violations in browser console
- [ ] Mantine components have proper ARIA attributes
- [ ] Focus management works correctly
- [ ] Keyboard navigation works

### Testing Requirements
- [ ] Can create new feature slice following FSD pattern
- [ ] Import between layers respects hierarchy
- [ ] CSS modules work and generate scoped classes
- [ ] API client makes successful requests to backend
- [ ] Environment variables load correctly

## 🔗 Dependencies

### Required Before
None (foundational task)

### Required For
- **ALL UI Tasks**: This is the foundation

### Integration Points
- **Backend Server**: Must run on localhost:8080 for API proxy

## 📚 API Contract

**Health Check**:
```typescript
// GET /api/health
{
  "status": "ok",
  "timestamp": "2025-12-18T10:30:00Z"
}
```

## 🧪 Testing Examples

**Test FSD Boundaries** (enforced through code review and documentation):
```typescript
// AVOID (feature importing from page)
// features/edit-cardinality/ui/CardinalityEditor.tsx
import { ProfileEditorPage } from '@pages/editor'; // ❌ Violates FSD hierarchy

// CORRECT (feature importing from entity)
// features/edit-cardinality/model/index.ts
import { $selectedElement } from '@entities/element'; // ✅ Follows FSD hierarchy
```

**Test Path Aliases**:
```typescript
// pages/editor/ui/ProfileEditorPage.tsx
import { ElementTree } from '@widgets/element-tree'; // ✅
import { CardinalityEditor } from '@features/edit-cardinality'; // ✅
import { $profile } from '@entities/profile'; // ✅
import { Button } from '@shared/ui'; // ✅
```

**Test CSS Modules**:
```typescript
// widgets/element-tree/ui/ElementTree.tsx
import styles from './ElementTree.module.css';

export function ElementTree() {
  return <div className={styles.container}>...</div>;
}
```

**Test API Client**:
```typescript
// Test in browser console
import { apiClient } from '@shared/api';

const health = await apiClient.get('/api/health');
console.log(health); // { status: 'ok', ... }
```

## 📖 Related Documentation

- **IMPLEMENTATION_PLAN.md Section 13**: FSD Architecture specification
- **IMPLEMENTATION_PLAN.md Section 15**: Technology Stack details
- **IMPLEMENTATION_PLAN.md Section 18**: CSS Modules usage
- **IMPLEMENTATION_PLAN.md Section 20**: Parallel Development workflow

## 🎨 Priority

🔴 **Critical** - Foundation for all UI work, blocks everything

## ⏱️ Estimated Complexity

**High** - 1 week
- Day 1: Project initialization, dependencies, basic config
- Day 2: FSD structure, path aliases, Biome configuration
- Day 3: Providers setup, global styles, utilities
- Day 4: Build optimization, documentation
- Day 5: Testing, validation, polish
