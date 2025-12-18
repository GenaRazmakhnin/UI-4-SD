# Task: Toolchain Alignment

## Status: COMPLETED

## Description
Align the Rust toolchain and MSRV (Minimum Supported Rust Version) with the `maki` project, which uses Rust edition 2024. This ensures compatibility and prevents build issues.

## Requirements

### R1: Rust Toolchain Configuration
- [x] Configure `rust-toolchain.toml` to pin the Rust version to match `maki`
- [x] Set edition to "2024" in workspace `Cargo.toml`
- [x] Set MSRV to 1.85 or higher

### R2: Workspace Setup
- [x] Create workspace root `Cargo.toml` with proper resolver and edition settings
- [x] Configure `.cargo/config.toml` with path to `maki-core` dependency
- [x] Document toolchain requirements in README

### R3: Dependency Path Configuration
- [x] Set up local path dependency to `maki-core` crate
- [x] Verify the dependency resolves correctly
- [x] Ensure all features needed from `maki-core` are available

### R4: Verification
- [x] Build succeeds with the configured toolchain
- [x] All `maki-core` APIs are accessible
- [x] No toolchain-related warnings or errors

## Acceptance Criteria

- [x] `rust-toolchain.toml` exists and specifies correct version
- [x] Workspace `Cargo.toml` sets edition = "2024" and rust-version = "1.85"
- [x] `.cargo/config.toml` correctly configures maki-core path
- [x] `cargo build` succeeds without toolchain errors
- [x] `cargo check` passes for all workspace members
- [ ] CI/CD (if configured) uses the pinned toolchain (N/A - no CI configured yet)
- [x] Documentation includes toolchain setup instructions

## Implementation Notes

### Additional Work Required

During implementation, discovered that `maki-core` had compatibility issues with `octofhir-canonical-manager` version 0.1.20. The following fixes were applied:

1. **canonical-manager**: Added `set_package_priority` method to `SearchStorage` trait
2. **maki-core**: Updated to use `search_storage()` instead of deprecated `package_storage()` API
3. **maki**: Updated to use local path dependency for canonical-manager

### Files Created/Modified

**NITEN (this project):**
- `rust-toolchain.toml` - Rust 1.92 (Edition 2024 compatible)
- `Cargo.toml` - Package configuration with maki-core dependency
- `.cargo/config.toml` - Cargo configuration
- `src/lib.rs` - Library entry point
- `src/main.rs` - Binary entry point
- `src/config.rs` - Server configuration
- `src/error.rs` - Error types
- `src/server.rs` - HTTP server (Axum)
- `README.md` - Toolchain setup instructions

**canonical-manager (sibling):**
- `src/traits.rs` - Added `set_package_priority` to `SearchStorage` trait
- `src/sqlite_storage.rs` - Implemented trait method

**maki (sibling):**
- `Cargo.toml` - Updated to use local canonical-manager path
- `crates/maki-core/src/canonical/mod.rs` - Updated API calls

## Dependencies
None (this is the first task)

## Related Files
- `rust-toolchain.toml` (new)
- `Cargo.toml` (workspace root, new)
- `.cargo/config.toml` (new)
- `README.md` (new)

## Priority
🔴 Critical - Must be completed first

## Estimated Complexity
Low - 1-2 hours

## Actual Complexity
Medium - Required fixing upstream dependency API compatibility
