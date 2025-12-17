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

- [ ] All constraint operations implemented
- [ ] All slicing operations implemented
- [ ] All extension operations implemented
- [ ] Fixed/pattern operations work
- [ ] Invariant operations work
- [ ] Operations validate before applying
- [ ] Operations are reversible (undo/redo)
- [ ] Operations maintain document consistency
- [ ] Validation errors are clear
- [ ] Performance: operations complete <50ms
- [ ] Documentation for all operations

## Dependencies
- **Backend 02**: IR Data Model Implementation

## Related Files
- `crates/profile-builder/src/operations/mod.rs` (new)
- `crates/profile-builder/src/operations/constraint_ops.rs` (new)
- `crates/profile-builder/src/operations/slicing_ops.rs` (new)
- `crates/profile-builder/src/operations/extension_ops.rs` (new)
- `crates/profile-builder/src/operations/binding_ops.rs` (new)
- `crates/profile-builder/src/operations/invariant_ops.rs` (new)
- `crates/profile-builder/src/operations/validator.rs` (new)

## Priority
🔴 Critical - Core editing functionality

## Estimated Complexity
Very High - 3-4 weeks
