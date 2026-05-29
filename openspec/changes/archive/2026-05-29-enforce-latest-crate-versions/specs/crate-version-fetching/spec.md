## ADDED Requirements

### Requirement: Use cargo search for version lookup

When adding or updating a Rust dependency in Cargo.toml, AI agents SHALL use the `cargo search` command to fetch the latest available version from the crates.io registry.

#### Scenario: Querying latest version for a crate
- **WHEN** the agent needs to find the latest version of `serde`
- **THEN** the agent SHALL run `cargo search serde --limit 1`

#### Scenario: Querying latest version before adding new dependency
- **WHEN** adding a new dependency to Cargo.toml
- **THEN** the agent SHALL first run `cargo search <crate_name> --limit 1` to get the latest version

#### Scenario: Querying latest version before updating existing dependency
- **WHEN** updating an existing dependency version in Cargo.toml
- **THEN** the agent SHALL run `cargo search <crate_name> --limit 1` to verify the new version is the latest available

### Requirement: Parse cargo search output format

AI agents SHALL correctly parse the output of `cargo search --limit 1`. The output format is: `crate_name = "version" # description`

#### Scenario: Parsing valid cargo search output
- **WHEN** `cargo search serde --limit 1` outputs `serde = "1.0.200" # A serialization framework`
- **THEN** the agent SHALL extract `"1.0.200"` as the latest version

#### Scenario: Parsing when version has prerelease tag
- **WHEN** `cargo search some_crate --limit 1` outputs `some_crate = "2.0.0-rc.1" # A crate`
- **THEN** the agent SHALL extract `"2.0.0-rc.1"` as the latest version

### Requirement: Verify exact crate name match

AI agents SHALL verify that the first result from `cargo search` is an exact match for the queried crate name before using its version.

#### Scenario: Exact match found
- **WHEN** `cargo search serde --limit 1` returns `serde = "1.0.200" # ...`
- **THEN** the agent SHALL use version `1.0.200`

#### Scenario: No exact match found
- **WHEN** `cargo search my-crate --limit 1` returns `my_crate = "1.0.0" # ...` (different name)
- **THEN** the agent SHALL NOT use this result and SHALL report that the exact crate was not found

### Requirement: Handle no results

If `cargo search` returns no results for a given crate name, AI agents SHALL report that the crate does not exist in the registry.

#### Scenario: Crate does not exist
- **WHEN** `cargo search nonexistent-crate --limit 1` returns no output
- **THEN** the agent SHALL report that the crate was not found in the registry

#### Scenario: Crate name has typos
- **WHEN** `cargo search serdee --limit 1` returns no results
- **THEN** the agent SHALL report that no crate matching that name was found

### Requirement: Handle network errors

If `cargo search` fails due to network issues, AI agents SHALL report the error to the user and abort the operation.

#### Scenario: Network connection failed
- **WHEN** `cargo search serde --limit 1` fails with a network error
- **THEN** the agent SHALL report the network error and stop

#### Scenario: Registry unavailable
- **WHEN** cargo cannot connect to the crates.io registry
- **THEN** the agent SHALL report the registry is unavailable and stop

### Requirement: Use latest version from search result

AI agents SHALL use the version from the first exact matching result of `cargo search --limit 1` as the authoritative latest version for that crate.

#### Scenario: Using latest version for dependency addition
- **WHEN** agent runs `cargo search tokio --limit 1` and gets `tokio = "1.35.0"`
- **THEN** the agent SHALL use `"1.35.0"` as the version when adding tokio to Cargo.toml

#### Scenario: Verifying update uses latest version
- **WHEN** agent is updating a dependency and `cargo search some_crate --limit 1` returns `some_crate = "2.0.0"`
- **THEN** the agent SHALL verify the update version matches `2.0.0`
