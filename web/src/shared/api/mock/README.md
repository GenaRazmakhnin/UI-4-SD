# Mock API Layer

## Overview

The mock API layer enables UI development without backend dependencies. It provides realistic data, simulated network delays, error handling, and localStorage persistence.

## Usage

### Enable Mock API

Set in `.env.development`:

```bash
VITE_USE_MOCK_API=true
```

### Switch to Real API

Set in `.env.development`:

```bash
VITE_USE_MOCK_API=false
VITE_API_BASE_URL=http://localhost:8080
```

## Mock Data

### Available Profiles

- **us-core-patient**: Simple US Core Patient profile with basic constraints
- **observation-with-slicing**: Observation profile demonstrating slicing on components
- **large-profile**: Profile with 500+ elements for performance testing

### Available Packages

- **hl7.fhir.r4.core** (4.0.1): FHIR R4 Core Package - 45.2 MB
- **hl7.fhir.us.core** (6.1.0): US Core Implementation Guide - 12.8 MB
- **hl7.fhir.uv.ipa** (1.0.0): International Patient Access - 3.4 MB

### Customizing Mock Data

Edit `web/src/shared/api/mock/fixtures.ts` to modify or add mock data:

```typescript
import { createMockProfile } from '@shared/api/mock/fixtures';

const myProfile = createMockProfile({
  name: 'MyCustomProfile',
  title: 'My Custom Profile',
  status: 'draft',
  // ... other overrides
});
```

## Simulated Behaviors

### Network Delays

- **Fast operations** (50-100ms): Undo/redo checks
- **Standard operations** (100-400ms): List fetches, element updates
- **Slow operations** (500-2000ms): Validation, package installation

Random jitter is added to all delays to simulate real network conditions.

### Error Simulation

- **Random failures**: 5% chance of network errors on most operations
- **Rate limiting**: 2% chance of rate limit errors
- **Not found errors**: When accessing non-existent resources
- **Validation errors**: For invalid data

### Data Persistence

All changes are automatically saved to localStorage:

- Profile modifications persist across page refreshes
- Undo/redo stacks are preserved
- Changes sync between browser tabs

To reset mock data:

```typescript
import { persistence } from '@shared/api/mock/persistence';

persistence.clear(); // Clear all mock data
```

## API Endpoints

### Profiles

```typescript
// List all profiles
const profiles = await api.profiles.list();

// Get single profile
const profile = await api.profiles.get('profile-id');

// Create profile
const newProfile = await api.profiles.create({ name: 'MyProfile', ... });

// Update profile
const updated = await api.profiles.update('profile-id', { title: 'New Title' });

// Delete profile
await api.profiles.delete('profile-id');

// Update element
const profile = await api.profiles.updateElement(
  'profile-id',
  'Patient.name',
  { min: 2 }
);
```

### Packages

```typescript
// List packages
const packages = await api.packages.list();

// Search packages
const results = await api.packages.search('core');

// Install package
const pkg = await api.packages.install('package-id');

// Uninstall package
await api.packages.uninstall('package-id');
```

### Search

```typescript
// Search resources
const resources = await api.search.resources('patient', filters);

// Search extensions
const extensions = await api.search.extensions('birthplace');

// Search value sets
const valueSets = await api.search.valueSets('gender');
```

### Validation

```typescript
// Validate profile
const result = await api.validation.validate('profile-id');
// Returns: { isValid: boolean, errors: [], warnings: [], info: [] }
```

### Export

```typescript
// Export to StructureDefinition JSON
const sd = await api.export.toSD('profile-id');

// Export to FHIR Shorthand
const fsh = await api.export.toFSH('profile-id');
```

### Undo/Redo

```typescript
// Check undo availability
const canUndo = await api.undo.canUndo('profile-id');

// Perform undo
const profile = await api.undo.undo('profile-id');

// Check redo availability
const canRedo = await api.undo.canRedo('profile-id');

// Perform redo
const profile = await api.undo.redo('profile-id');
```

## Using with TanStack Query

Import and use the provided hooks:

```typescript
import { useProfiles, useProfile, useUpdateProfile } from '@entities/profile/api/queries';

function ProfileList() {
  const { data: profiles, isLoading } = useProfiles();

  if (isLoading) return <div>Loading...</div>;

  return (
    <ul>
      {profiles?.map(profile => (
        <li key={profile.id}>{profile.title}</li>
      ))}
    </ul>
  );
}

function ProfileEditor({ profileId }: { profileId: string }) {
  const { data: profile } = useProfile(profileId);
  const updateProfile = useUpdateProfile();

  const handleSave = () => {
    updateProfile.mutate({
      id: profileId,
      data: { title: 'Updated Title' }
    });
  };

  return <div>{/* ... */}</div>;
}
```

## Testing Error States

To test specific error scenarios:

```typescript
import { errorSimulator } from '@shared/api/mock/errors';

// Throw validation error
throw errorSimulator.validationError('name', 'Name is required');

// Throw not found error
throw errorSimulator.notFoundError('Profile', 'unknown-id');

// Throw unauthorized error
throw errorSimulator.unauthorizedError();
```

## Configuration

Configure mock behavior via environment variables:

```bash
# Mock API toggle
VITE_USE_MOCK_API=true

# Feature flags
VITE_ENABLE_UNDO_REDO=true
VITE_ENABLE_FSH_EXPORT=true

# Performance settings
VITE_VIRTUALIZATION_THRESHOLD=100
VITE_DEBOUNCE_MS=300
```

## Architecture

```
web/src/shared/api/
├── mock/
│   ├── index.ts           # Main mock API implementation
│   ├── fixtures.ts        # Mock data (profiles, packages, etc.)
│   ├── utils.ts           # Delay & logging utilities
│   ├── errors.ts          # Error simulation
│   ├── persistence.ts     # localStorage persistence
│   └── README.md          # This file
├── real/
│   └── index.ts           # Real API implementation (stub)
├── client.ts              # Base HTTP client
└── index.ts               # API facade (switches between mock/real)
```

## Switching to Real Backend

When the backend is ready:

1. Set `VITE_USE_MOCK_API=false` in `.env.development`
2. Ensure `VITE_API_BASE_URL` points to your backend
3. Verify the real API implements the same interface

No code changes required in components!

## Troubleshooting

### Mock data not persisting

Check browser localStorage size limit. Clear old data:

```typescript
localStorage.clear();
```

### API calls failing

Check browser console for error messages. Verify environment variables are set correctly.

### Slow performance

Reduce mock delays in `fixtures.ts`:

```typescript
await simulateDelay(10, 50); // Faster delays for testing
```

## Development Tips

1. **Test error states**: The 5% error rate helps catch error handling bugs
2. **Test with large data**: Use `large-profile` to test performance
3. **Test persistence**: Refresh the page to verify state is saved
4. **Test concurrent edits**: Open multiple tabs to test sync behavior
5. **Monitor network tab**: Mock API calls are logged in dev mode

## Next Steps

- Add more realistic mock profiles
- Implement full undo/redo logic
- Add WebSocket support for real-time sync
- Add mock authentication/authorization
- Add request/response logging dashboard
