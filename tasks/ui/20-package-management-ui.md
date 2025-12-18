# Task: Package Management UI

## Description

Implement the frontend UI for FHIR package management, connecting to the backend package API. This includes a package browser for discovering and installing packages, and resource search for finding extensions, value sets, and other resources from installed packages.

## Backend API Reference

The backend exposes the following endpoints:

### Package Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/packages` | List installed packages |
| GET | `/api/packages/search?q=` | Search registry for packages |
| POST | `/api/packages/:id/install` | Install package (SSE stream) |
| POST | `/api/packages/:id/uninstall` | Remove installed package |

### Resource Search
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/search/extensions?q=&package=` | Search extensions |
| GET | `/api/search/valuesets?q=` | Search value sets |
| GET | `/api/search/resources?q=&type=&package=` | Generic resource search |

## Requirements

### R1: Package Browser Page
- Display list of installed packages with:
  - Package name and version
  - FHIR version compatibility
  - Resource counts (profiles, extensions, valueSets, etc.)
  - Installation date
- Search/filter installed packages
- Uninstall button with confirmation dialog

### R2: Package Search & Install
- Search input for finding packages in registry
- Display search results with:
  - Package name, version, description
  - Publisher info
  - FHIR version compatibility
- "Install" button for each result
- Show installation progress with SSE events

### R3: Installation Progress UI
- Real-time progress display using SSE:
  - Download progress bar with percentage
  - Status messages (downloading, extracting, indexing)
  - Success/error notifications
- Handle SSE event types:
  ```typescript
  type InstallEvent =
    | { type: 'start'; data: { packageId: string; totalBytes?: number } }
    | { type: 'progress'; data: { packageId: string; downloadedBytes: number; totalBytes?: number; percentage: number } }
    | { type: 'extracting'; data: { packageId: string } }
    | { type: 'indexing'; data: { packageId: string } }
    | { type: 'complete'; data: { package: Package } }
    | { type: 'error'; data: { packageId: string; message: string; code: string } };
  ```

### R4: Resource Search Panel
- Unified search interface for finding resources
- Filter by resource type (Extension, ValueSet, Profile, etc.)
- Filter by package
- Display results with:
  - Resource name and URL
  - Description
  - Source package
- Click to view resource details or use in profile

### R5: Extension Picker Integration
- Searchable extension picker component
- Used when adding extensions to profiles
- Shows extension details (context, cardinality, type)
- "Add" action to include extension in profile

### R6: ValueSet Picker Integration
- Searchable value set picker for binding editor
- Shows value set details (expansion preview)
- Used when setting element bindings

## UI Components

### PackageBrowserPage
```typescript
// Main page component
interface PackageBrowserPageProps {
  // No props - uses React Query for data
}

// Features:
// - Tabs: "Installed" | "Browse Registry"
// - Search input
// - Package list/grid
// - Installation modal
```

### PackageCard
```typescript
interface PackageCardProps {
  package: Package;
  onUninstall?: () => void;
  onInstall?: () => void;
  isInstalling?: boolean;
}
```

### InstallProgressModal
```typescript
interface InstallProgressModalProps {
  packageId: string;
  onClose: () => void;
  onComplete: (pkg: Package) => void;
}

// Uses EventSource for SSE:
// const eventSource = new EventSource(`/api/packages/${packageId}/install`);
```

### ResourceSearchPanel
```typescript
interface ResourceSearchPanelProps {
  resourceType?: 'Extension' | 'ValueSet' | 'Profile' | 'CodeSystem';
  onSelect: (resource: SearchResult) => void;
  packages?: string[]; // Filter by packages
}
```

### ExtensionPicker
```typescript
interface ExtensionPickerProps {
  onSelect: (extension: Extension) => void;
  context?: string; // Filter by extension context
}
```

### ValueSetPicker
```typescript
interface ValueSetPickerProps {
  onSelect: (valueSet: ValueSet) => void;
  codeSystem?: string; // Filter by code system
}
```

## API Integration

### React Query Hooks

```typescript
// hooks/usePackages.ts
export function useInstalledPackages() {
  return useQuery({
    queryKey: ['packages'],
    queryFn: () => api.packages.list(),
  });
}

export function usePackageSearch(query: string) {
  return useQuery({
    queryKey: ['packages', 'search', query],
    queryFn: () => api.packages.search(query),
    enabled: query.length > 2,
  });
}

export function useUninstallPackage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (packageId: string) => api.packages.uninstall(packageId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['packages'] });
    },
  });
}

// hooks/useResourceSearch.ts
export function useExtensionSearch(query: string, packages?: string[]) {
  return useQuery({
    queryKey: ['search', 'extensions', query, packages],
    queryFn: () => api.search.extensions(query, { package: packages }),
    enabled: query.length > 0,
  });
}

export function useValueSetSearch(query: string) {
  return useQuery({
    queryKey: ['search', 'valuesets', query],
    queryFn: () => api.search.valueSets(query),
    enabled: query.length > 0,
  });
}
```

### SSE Hook for Installation

```typescript
// hooks/usePackageInstall.ts
export function usePackageInstall(packageId: string) {
  const [status, setStatus] = useState<'idle' | 'installing' | 'complete' | 'error'>('idle');
  const [progress, setProgress] = useState(0);
  const [message, setMessage] = useState('');
  const queryClient = useQueryClient();

  const install = useCallback(() => {
    setStatus('installing');
    setProgress(0);

    const eventSource = new EventSource(`/api/packages/${packageId}/install`, {
      method: 'POST',
    });

    eventSource.onmessage = (event) => {
      const data = JSON.parse(event.data);

      switch (data.type) {
        case 'start':
          setMessage('Starting installation...');
          break;
        case 'progress':
          setProgress(data.data.percentage);
          setMessage(`Downloading... ${data.data.percentage}%`);
          break;
        case 'extracting':
          setMessage('Extracting package...');
          break;
        case 'indexing':
          setMessage('Indexing resources...');
          break;
        case 'complete':
          setStatus('complete');
          setMessage('Installation complete!');
          queryClient.invalidateQueries({ queryKey: ['packages'] });
          eventSource.close();
          break;
        case 'error':
          setStatus('error');
          setMessage(data.data.message);
          eventSource.close();
          break;
      }
    };

    eventSource.onerror = () => {
      setStatus('error');
      setMessage('Connection lost');
      eventSource.close();
    };

    return () => eventSource.close();
  }, [packageId, queryClient]);

  return { status, progress, message, install };
}
```

## File Structure

```
web/src/
├── features/
│   └── packages/
│       ├── index.ts
│       ├── api/
│       │   └── packagesApi.ts
│       ├── hooks/
│       │   ├── usePackages.ts
│       │   ├── usePackageInstall.ts
│       │   └── useResourceSearch.ts
│       └── ui/
│           ├── PackageBrowserPage.tsx
│           ├── PackageCard.tsx
│           ├── PackageList.tsx
│           ├── InstallProgressModal.tsx
│           ├── ResourceSearchPanel.tsx
│           ├── ExtensionPicker.tsx
│           └── ValueSetPicker.tsx
├── pages/
│   └── packages/
│       └── ui/
│           └── PackagesPage.tsx
└── shared/
    └── api/
        └── real/
            └── index.ts  # Already has package API definitions
```

## Acceptance Criteria

- [ ] Package browser page displays installed packages
- [ ] Can search registry for new packages
- [ ] Can install packages with real-time progress
- [ ] Can uninstall packages with confirmation
- [ ] Resource search finds extensions/valueSets across packages
- [ ] Extension picker integrates with profile editor
- [ ] ValueSet picker integrates with binding editor
- [ ] Error handling for failed installations
- [ ] Loading states for all async operations
- [ ] Responsive design for different screen sizes

## Dependencies

- **Backend 10**: Package Management System (completed)
- **UI 09**: Binding Editor (for ValueSet picker integration)
- **UI 11**: Extension Picker (this task provides the implementation)

## Related Files

- `web/src/shared/api/real/index.ts` - API client (packages API already defined)
- `web/src/shared/types/package.ts` - Package type definitions
- `tasks/backend/10-package-management.md` - Backend implementation

## Priority

High - Required for working with external FHIR packages

## Notes

- SSE requires `POST` method but EventSource only supports GET
- Consider using `fetch` with `ReadableStream` for POST SSE
- Or change backend to support GET for install with query param
- Package installation can take time - consider background notifications
