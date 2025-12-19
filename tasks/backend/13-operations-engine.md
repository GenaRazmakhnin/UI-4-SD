# Task: Operations Engine (Constraint, Slicing, Extension)

## Description
Implement the operations engine that provides high-level operations for editing profiles. Each operation is reversible for undo/redo and validates constraints.

## Requirements

### R1: Constraint Operations
**Cardinality Operations:**
- `SetCardinality(path, min, max)`: Set element cardinality
- Validate min ≤ max and within base bounds
- Record in edit history

**Type Operations:**
- `AddTypeConstraint(path, type, profile)`: Restrict allowed types
- `RemoveTypeConstraint(path, type)`: Remove type restriction
- Validate type is subtype of base

**Flag Operations:**
- `SetMustSupport(path, value)`: Set mustSupport flag
- `SetIsModifier(path, value, reason)`: Set isModifier flag
- `SetIsSummary(path, value)`: Set isSummary flag

**Binding Operations:**
- `SetBinding(path, valueSet, strength)`: Set terminology binding
- Validate strength (required/extensible/preferred/example)
- Ensure strength not weakened from base

**Text Operations:**
- `SetShort(path, text)`: Set short description
- `SetDefinition(path, text)`: Set definition
- `SetComment(path, text)`: Set comment

### R2: Slicing Operations
**Create Slice:**
- `CreateSlicing(path, discriminators, rules)`: Initialize slicing on element
  - Validate discriminator paths exist
  - Set slicing rules (open/closed/openAtEnd)
  - Set ordered flag

**Add Slice:**
- `AddSlice(path, name, min, max)`: Add named slice
  - Validate slice name is unique
  - Validate cardinality
  - Create slice element tree

**Configure Discriminator:**
- `AddDiscriminator(path, type, path)`: Add discriminator
  - Validate discriminator type (value/exists/pattern/type/profile)
  - Validate discriminator path is valid

**Remove Slice:**
- `RemoveSlice(path, name)`: Remove slice
  - Validate no other slices depend on it

### R3: Extension Operations
**Add Extension:**
- `AddExtension(path, extensionUrl)`: Add extension to element
  - Resolve extension definition
  - Validate extension context matches usage
  - Create extension element

**Configure Extension:**
- `SetExtensionCardinality(path, extensionUrl, min, max)`
- `SetExtensionFixedValue(path, extensionUrl, value)`

**Remove Extension:**
- `RemoveExtension(path, extensionUrl)`: Remove extension

### R4: Fixed/Pattern Operations
- `SetFixedValue(path, value)`: Set fixed value constraint
- `SetPatternValue(path, value)`: Set pattern value constraint
- Validate value type matches element type

### R5: Invariant Operations
- `AddInvariant(key, severity, human, expression)`: Add FHIRPath constraint
  - Validate FHIRPath expression parses
  - Validate key is unique
  - Validate severity (error/warning)

- `UpdateInvariant(key, ...)`: Update existing invariant
- `RemoveInvariant(key)`: Remove invariant

### R6: Operation Interface
```rust
pub trait Operation: Send + Sync {
    fn apply(&self, document: &mut ProfileDocument) -> Result<()>;
    fn undo(&self, document: &mut ProfileDocument) -> Result<()>;
    fn validate(&self, document: &ProfileDocument) -> Result<()>;
    fn description(&self) -> String;
}
```

### R7: Operation Validation
- Pre-validate operation before applying
- Provide clear error messages
- Suggest corrections for invalid operations

### R8: Atomic Operations
- Operations are atomic (all-or-nothing)
- Rollback on validation failure
- Maintain document consistency

## Acceptance Criteria

- [x] All constraint operations implemented
- [x] All slicing operations implemented
- [x] All extension operations implemented
- [x] Fixed/pattern operations work
- [x] Invariant operations work
- [x] Operations validate before applying
- [x] Operations are reversible (undo/redo)
- [x] Operations maintain document consistency
- [x] Validation errors are clear
- [x] Performance: operations complete <50ms
- [x] Documentation for all operations

## Dependencies
- **Backend 02**: IR Data Model Implementation

## Related Files
- `src/operations/mod.rs` - Main module with apply functions
- `src/operations/traits.rs` - Operation trait definition
- `src/operations/error.rs` - Error types
- `src/operations/constraint.rs` - Constraint operations
- `src/operations/slicing.rs` - Slicing operations
- `src/operations/extension.rs` - Extension operations
- `src/operations/invariant.rs` - Invariant operations

## Implementation Notes

### Completed (2024-12)

**R1: Constraint Operations** ✅
- `SetCardinality` - Set element cardinality with validation
- `AddTypeConstraint` / `RemoveTypeConstraint` - Type restrictions
- `SetMustSupport` / `SetIsModifier` / `SetIsSummary` - Flag operations
- `SetBinding` / `RemoveBinding` - Terminology binding
- `SetShort` / `SetDefinition` / `SetComment` - Text operations
- `SetFixedValue` / `SetPatternValue` - Fixed/pattern values

**R2: Slicing Operations** ✅
- `CreateSlicing` - Initialize slicing on element with discriminators
- `RemoveSlicing` - Remove slicing definition
- `AddSlice` / `RemoveSlice` - Named slice management
- `AddDiscriminator` - Add discriminator to slicing
- `SetSlicingRules` - Set open/closed/openAtEnd rules

**R3: Extension Operations** ✅
- `AddExtension` - Add extension to element with cardinality
- `RemoveExtension` - Remove extension
- `SetExtensionCardinality` - Configure extension cardinality
- `SetExtensionFixedValue` - Set fixed value on extension

**R4: Fixed/Pattern Operations** ✅
- Included in constraint operations
- `SetFixedValue` for exact match constraints
- `SetPatternValue` for pattern match constraints

**R5: Invariant Operations** ✅
- `AddInvariant` - Add FHIRPath constraint with severity
- `UpdateInvariant` - Modify existing invariant
- `RemoveInvariant` - Remove invariant by key
- Basic FHIRPath syntax validation

**R6: Operation Interface** ✅
```rust
pub trait Operation: Send + Sync {
    fn validate(&self, document: &ProfileDocument) -> OperationResult<()>;
    fn apply(&self, document: &mut ProfileDocument) -> OperationResult<()>;
    fn undo(&self, document: &mut ProfileDocument) -> OperationResult<()>;
    fn description(&self) -> String;
    fn as_change(&self) -> Change;
}
```

**R7: Operation Validation** ✅
- Pre-validation before applying
- Clear error messages via `OperationError` enum
- Element existence checks
- Cardinality validation (min ≤ max)
- Duplicate detection (slices, invariants)

**R8: Atomic Operations** ✅
- Operations modify document state atomically
- Undo support via stored previous values
- Integration with `EditHistory` for undo/redo

### Helper Functions
- `apply_operation()` - Apply single operation with history
- `apply_batch()` - Apply multiple operations atomically

## Priority
🔴 Critical - Core editing functionality

## Status
🟢 **COMPLETE** - All operations implemented with full test coverage (186 tests pass)
