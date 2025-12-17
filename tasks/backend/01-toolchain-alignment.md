# Task: Toolchain Alignment

## Description
Align the Rust toolchain and MSRV (Minimum Supported Rust Version) with the `maki` project, which uses Rust edition 2024. This ensures compatibility and prevents build issues.

## Requirements

### R1: Rust Toolchain Configuration
- Configure `rust-toolchain.toml` to pin the Rust version to match `maki`
- Set edition to "2024" in workspace `Cargo.toml`
- Set MSRV to 1.85 or higher

### R2: Workspace Setup
- Create workspace root `Cargo.toml` with proper resolver and edition settings
- Configure `.cargo/config.toml` with path to `maki-core` dependency
- Document toolchain requirements in README

### R3: Dependency Path Configuration
- Set up local path dependency to `maki-core` crate
- Verify the dependency resolves correctly
- Ensure all features needed from `maki-core` are available

### R4: Verification
- Build succeeds with the configured toolchain
- All `maki-core` APIs are accessible
- No toolchain-related warnings or errors

## Acceptance Criteria

- [ ] `rust-toolchain.toml` exists and specifies correct version
- [ ] Workspace `Cargo.toml` sets edition = "2024" and rust-version = "1.85"
- [ ] `.cargo/config.toml` correctly configures maki-core path
- [ ] `cargo build` succeeds without toolchain errors
- [ ] `cargo check` passes for all workspace members
- [ ] CI/CD (if configured) uses the pinned toolchain
- [ ] Documentation includes toolchain setup instructions

## Dependencies
None (this is the first task)

## Related Files
- `rust-toolchain.toml` (new)
- `Cargo.toml` (workspace root, new)
- `.cargo/config.toml` (new)
- `README.md` (update)

## Priority
🔴 Critical - Must be completed first

## Estimated Complexity
Low - 1-2 hours
