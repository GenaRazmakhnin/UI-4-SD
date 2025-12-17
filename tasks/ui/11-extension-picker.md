# Task: Extension Picker Feature

## Description
Implement the extension picker for discovering and adding extensions to elements with context validation.

## Requirements

### R1: Extension Search Dialog
- Modal dialog for extension search
- Search across all loaded packages
- Filter by context (resource type, element path)
- Show extension metadata
- Context compatibility warnings

### R2: Search Interface
- Full-text search in extension names/descriptions
- Filter by package
- Filter by context type
- Sort by relevance
- Pagination for results

### R3: Extension Details
Display for each extension:
- Name and title
- Description
- Canonical URL
- Context rules
- Cardinality
- Value type
- Package source

### R4: Context Validation
- Check if extension context matches current element
- Show compatibility status
- Warn if incompatible
- Allow override with confirmation

### R5: Quick Actions
- Add extension to current element
- View extension definition
- Open extension in new tab
- Copy extension URL

### R6: Recent & Favorites
- Track recently used extensions
- Favorite extensions
- Quick access list

### R7: Extension Configuration
After adding extension:
- Set cardinality
- Set fixed value (if simple extension)
- Configure slicing for multiple extensions

## Acceptance Criteria
- [ ] Extension picker opens
- [ ] Search finds extensions
- [ ] Filters work correctly
- [ ] Context validation works
- [ ] Extension details display
- [ ] Add extension to element works
- [ ] Recent extensions tracked
- [ ] Favorites can be saved
- [ ] Configuration dialog works
- [ ] Changes persist to backend
- [ ] Unit tests pass

## Dependencies
- **UI 03**: Mock Data Layer
- **UI 12**: Search UI
- **Backend 11**: Search API

## Priority
🟡 High - Beta feature

## Estimated Complexity
High - 2 weeks
