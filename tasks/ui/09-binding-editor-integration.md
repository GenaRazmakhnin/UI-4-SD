# Binding Editor Integration Map

## 🏗️ Component Architecture

```
InspectorPanel (Inspector Panel Widget)
└── Tabs
    ├── Constraints Tab
    ├── Binding Tab ✨ UPDATED
    │   └── BindingEditor (Feature Component)
    │       ├── ValueSet URL Input
    │       ├── Binding Strength Selector
    │       ├── Description Field
    │       ├── ValueSetBrowser Modal
    │       │   └── ValueSet Search & Selection
    │       └── ExpansionPreview (Accordion)
    │           └── Code Display (20 codes max)
    ├── Slicing Tab
    └── Metadata Tab
```

## 🔗 Data Flow

```
User Action
    │
    ├─→ Select Element in Element Tree
    │       └─→ $selectedElement (Effector Store)
    │               └─→ InspectorPanel receives update
    │                       └─→ BindingTab shows BindingEditor
    │
    ├─→ Click "Search" Button
    │       └─→ Opens ValueSetBrowser Modal
    │               └─→ searchValueSetsFx (Effector Effect)
    │                       └─→ api.search.valueSets()
    │                               └─→ Returns ValueSet[]
    │                                       └─→ $searchResults Store
    │
    ├─→ Select ValueSet
    │       └─→ Populates URL & Description
    │               └─→ Auto-fetches expansion
    │                       └─→ fetchExpansionFx
    │                               └─→ api.terminology.expand()
    │                                       └─→ $expansions Store (cached)
    │
    └─→ Click "Apply Binding"
            └─→ bindingChanged Event
                    └─→ updateBindingFx
                            └─→ api.profiles.updateElement()
                                    └─→ Profile Updated
```

## 📦 State Management

### Effector Stores

| Store | Purpose | Source |
|-------|---------|--------|
| `$selectedElement` | Currently selected element | `@widgets/element-tree` |
| `$searchResults` | ValueSet search results | `binding-editor/model` |
| `$searchLoading` | Search loading state | `binding-editor/model` |
| `$expansions` | Cached ValueSet expansions | `binding-editor/model` |
| `$expansionLoading` | Expansion loading state | `binding-editor/model` |

### Events

| Event | Trigger | Action |
|-------|---------|--------|
| `bindingChanged` | User clicks "Apply Binding" | Updates element binding via API |
| `removeBinding` | User clicks "Remove Binding" | Removes binding from element |

### Effects

| Effect | Purpose | API Call |
|--------|---------|----------|
| `searchValueSetsFx` | Search for ValueSets | `api.search.valueSets()` |
| `fetchExpansionFx` | Expand ValueSet codes | `api.terminology.expand()` |
| `updateBindingFx` | Save binding to element | `api.profiles.updateElement()` |

## 🎨 UI Components

### Main Components

1. **BindingEditor** (`features/binding-editor/ui/BindingEditor.tsx`)
   - Main component with all binding configuration
   - Manages local state for form inputs
   - Validates binding strength changes
   - Shows/hides expansion preview

2. **ValueSetBrowser** (`features/binding-editor/ui/ValueSetBrowser.tsx`)
   - Modal dialog for searching ValueSets
   - Code system filtering
   - Interactive ValueSet cards
   - Integrates with search API

3. **ExpansionPreview** (`features/binding-editor/ui/ExpansionPreview.tsx`)
   - Displays expanded codes from ValueSet
   - Shows up to 20 codes with system info
   - Handles loading and error states
   - Auto-fetches on URL change

### Integration Point

**BindingTab** (`widgets/inspector-panel/ui/BindingTab.tsx`)
- Host component in Inspector Panel
- Checks if element can have binding
- Shows helpful message for new bindings
- Renders BindingEditor component

## 🔧 Validation Logic

### Binding Strength Hierarchy

```
required      ← Strongest (most restrictive)
    ↓
extensible
    ↓
preferred
    ↓
example       ← Weakest (least restrictive)
```

**Rules:**
- ✅ Can strengthen: preferred → required
- ❌ Cannot weaken: required → preferred
- ⚠️  Warns when strengthening significantly

### Bindable Element Types

- `code` - Single code value
- `Coding` - System + code + display
- `CodeableConcept` - Multiple codings
- `Quantity` - Quantity with unit code
- `string` - String constrained by ValueSet
- `uri` - URI constrained by ValueSet

## 📡 API Integration

### Mock API (Development)

Located in `web/src/shared/api/mock/`:

1. **ValueSet Fixtures** (`fixtures.ts`)
   - 6 mock ValueSets
   - 3 with full expansions
   - Realistic FHIR data

2. **Search API** (`index.ts`)
   ```typescript
   api.search.valueSets(query, options)
   // Returns: ValueSet[]
   // Options: { codeSystem?: string[] }
   ```

3. **Terminology API** (`index.ts`)
   ```typescript
   api.terminology.expand(valueSetUrl)
   // Returns: ValueSetExpansion
   // Includes: total, contains[], error?
   ```

### Real API (Production Ready)

Located in `web/src/shared/api/real/`:
- Endpoints defined and typed
- Ready for backend implementation
- Same interface as mock API

## ✅ Testing Checklist

### Manual Testing Steps

1. **Basic Binding Creation**
   - [ ] Select a `code` element in tree
   - [ ] Navigate to Binding tab
   - [ ] Enter ValueSet URL manually
   - [ ] Select binding strength
   - [ ] Click "Apply Binding"
   - [ ] Verify binding saved

2. **ValueSet Search**
   - [ ] Click search button
   - [ ] Enter search query
   - [ ] Verify results appear
   - [ ] Select a ValueSet
   - [ ] Verify URL and description populated

3. **Code System Filtering**
   - [ ] Open ValueSet browser
   - [ ] Select "SNOMED CT" filter
   - [ ] Search for "finding"
   - [ ] Verify only SNOMED ValueSets shown

4. **Expansion Preview**
   - [ ] Enter valid ValueSet URL
   - [ ] Expand "Preview ValueSet Expansion"
   - [ ] Verify codes display
   - [ ] Check code, display, system shown

5. **Binding Strength Validation**
   - [ ] Try weakening binding (should show error)
   - [ ] Try strengthening binding (should show warning)
   - [ ] Verify invalid combinations prevented

6. **Remove Binding**
   - [ ] Click "Remove Binding"
   - [ ] Confirm dialog
   - [ ] Verify binding removed
   - [ ] Verify form cleared

### Integration Points to Verify

- [ ] Element selection updates BindingEditor
- [ ] Binding changes trigger API calls
- [ ] Loading states display correctly
- [ ] Error states handle gracefully
- [ ] Expansion caching works (no duplicate fetches)

## 🚀 Future Enhancements

1. **Base Binding Lookup**
   - Fetch base definition bindings from FHIR core
   - Show base binding info in UI
   - Validate against base constraints

2. **Enhanced Search**
   - Full-text search in descriptions
   - Filter by publisher
   - Sort by relevance/date
   - Recent searches history

3. **Accessibility**
   - Replace clickable divs with buttons
   - Add keyboard navigation
   - ARIA labels and roles
   - Focus management

4. **Advanced Features**
   - Compare ValueSets side-by-side
   - View code hierarchy/taxonomy
   - Suggest binding based on element name
   - Bulk binding operations

## 📚 Key Files Reference

| File | Purpose | Lines |
|------|---------|-------|
| `features/binding-editor/ui/BindingEditor.tsx` | Main editor component | 195 |
| `features/binding-editor/ui/ValueSetBrowser.tsx` | Search modal | 160 |
| `features/binding-editor/ui/ExpansionPreview.tsx` | Code display | 105 |
| `features/binding-editor/model/index.ts` | State management | 120 |
| `features/binding-editor/lib/validation.ts` | Validation logic | 105 |
| `widgets/inspector-panel/ui/BindingTab.tsx` | Integration point | 52 |
| `shared/types/terminology.ts` | FHIR types | 95 |
| `shared/api/mock/fixtures.ts` | Mock data | +200 lines added |

## 🎓 Developer Notes

### Adding New Code Systems

To add a new code system filter:

```typescript
// In ValueSetBrowser.tsx
<Select
  data={[
    { value: 'http://snomed.info/sct', label: 'SNOMED CT' },
    { value: 'http://loinc.org', label: 'LOINC' },
    // Add new system here:
    { value: 'http://new-system.org', label: 'New System' },
  ]}
/>
```

### Adding New ValueSet Fixtures

```typescript
// In mock/fixtures.ts
export const mockValueSets: ValueSet[] = [
  // ... existing ValueSets
  {
    url: 'http://example.org/ValueSet/my-valueset',
    name: 'MyValueSet',
    title: 'My Custom ValueSet',
    status: 'active',
    description: 'Description here',
    publisher: 'Your Organization',
    compose: {
      include: [{ system: 'http://code-system.org' }],
    },
  },
];
```

### Customizing Binding Validation

```typescript
// In lib/validation.ts
export function canChangeBindingStrength(
  baseStrength: BindingConstraint['strength'] | undefined,
  newStrength: BindingConstraint['strength']
): BindingValidation {
  // Add custom validation logic here
}
```

---

**Last Updated:** 2024-12-18
**Status:** ✅ Complete and Integrated
**Next Task:** User acceptance testing
