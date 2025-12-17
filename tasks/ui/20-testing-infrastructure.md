# Task: UI Testing Infrastructure

## Description
Set up comprehensive testing infrastructure including unit tests, integration tests, and E2E tests.

## Requirements

### R1: Unit Testing Setup
- Vitest for unit tests
- React Testing Library
- Mock service worker (MSW)
- Test utilities and helpers

### R2: Integration Testing
- Test widget interactions
- Test state management
- Test API integration
- Test routing

### R3: E2E Testing
- Playwright or Cypress
- Test critical user flows
- Visual regression tests
- Cross-browser testing

### R4: Test Coverage
- Minimum 80% code coverage
- Coverage reports in CI
- Enforce coverage thresholds
- Track coverage over time

### R5: Test Organization
```
web/src/
├── features/
│   └── edit-cardinality/
│       ├── __tests__/
│       │   ├── CardinalityEditor.test.tsx
│       │   └── model.test.ts
│       └── ...
```

### R6: CI Integration
- Run tests on every PR
- Fail PR if tests fail
- Run E2E tests nightly
- Generate coverage reports

## Acceptance Criteria
- [ ] Vitest configured and working
- [ ] RTL configured and working
- [ ] MSW mocks API requests
- [ ] E2E framework set up
- [ ] Coverage reporting works
- [ ] CI runs all tests
- [ ] Documentation for testing

## Dependencies
- **UI 01**: React App Scaffold

## Priority
🔴 Critical - Quality assurance

## Estimated Complexity
Medium - 1 week
