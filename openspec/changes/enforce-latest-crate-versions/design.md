## Context

AI agents and developers updating Cargo.toml files need a reliable method to determine the latest available version of Rust crates. The `cargo search` command provides a built-in, standardized way to query the crates.io registry that is already available in any Rust toolchain.

## Goals / Non-Goals

**Goals:**
- Define a reliable process for using `cargo search` to fetch the latest crate version
- Ensure AI agents have clear guidance on parsing `cargo search` output
- Handle edge cases and ensure consistent behavior
- Provide a method that works without direct HTTP API calls

**Non-Goals:**
- CI/CD integration or automated enforcement
- Automatically updating existing dependencies
- Version constraint validation (^, ~, =, etc.)
- Workspace-level dependency coordination

## Decisions

### Use `cargo search` as the source of truth
AI agents MUST use the `cargo search <crate_name>` command to query the latest version. This leverages the official cargo tooling which queries the crates.io registry directly and provides consistent, reliable results.

**Alternatives considered:**
- Direct crates.io API calls: Requires HTTP client, parsing JSON, handling rate limits
- Local cargo metadata: May not have latest registry information
- GitHub releases: Not all crates publish to GitHub

### Parse `cargo search` output format
The `cargo search` command outputs in a consistent format: `crate_name = "version" # description`. AI agents MUST parse the first result's version field as the latest version.

### For exact crate lookup, use `cargo search <name> --limit 1`
When looking up a specific crate by name, agents SHALL use `--limit 1` to get only the most relevant result (the exact match or closest match).

### Handle "no exact match" cases
When the exact crate name doesn't exist, `cargo search` returns the closest matches. AI agents SHALL verify the first result's name matches the queried crate name exactly before using its version.

### Query before adding new dependencies
When adding a **new** dependency to Cargo.toml, AI agents MUST first run `cargo search <crate_name>` to get the latest version, rather than using a hardcoded or previously known version.

### Query before updating existing dependencies
When updating an **existing** dependency version, AI agents MUST run `cargo search <crate_name>` to verify the new version matches the latest available.

## Risks / Trade-offs

**[Risk] cargo search may not be installed** → `cargo search` is part of cargo itself and is available in any standard Rust installation. If unavailable, agents SHALL report an error.

**[Risk] Network failures** → If cargo cannot connect to the registry, agents SHALL report an error to the user.

**[Risk] Registry index stale** → The local cargo registry index may be stale. Agents SHALL run `cargo search` which will automatically update the index if needed.

## Open Questions

- Should agents also verify that the version satisfies MSRV (Minimum Supported Rust Version) requirements?
- Should there be a configuration option to prefer LTS versions over latest?
