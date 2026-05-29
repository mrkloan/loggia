## Why

AI agents and developers performing Cargo.toml updates may not consistently use the latest available crate versions. Using `cargo search` provides a reliable, built-in method to query the latest version from the crates.io registry without requiring direct API calls.

## What Changes

- **New capability for AI agents**: Defined process for using `cargo search` to fetch the latest crate version when adding or updating Rust dependencies
- **Guidance specification**: Clear specifications on how to parse `cargo search` output to determine latest versions

## Capabilities

### New Capabilities
- `crate-version-fetching`: Process and requirements for AI agents to use `cargo search` to fetch the latest available version when adding or updating Rust dependencies

### Modified Capabilities

## Impact

- AI agents performing Cargo.toml updates
- Developers adding new Rust dependencies
- All crates in the project workspace
