# Cargo Search Guide for AI Agents

This document provides complete guidance for AI agents on using `cargo search` to fetch the latest available version of Rust crates when updating Cargo.toml files.

## Overview

When adding or updating Rust dependencies in Cargo.toml, AI agents MUST use `cargo search` to determine the latest available version from the crates.io registry. This ensures consistency and prevents introducing outdated dependencies.

## Command Specification

### Primary Command

**Command:** `cargo search <crate_name> --limit 1`

**Purpose:** Query the crates.io registry for the latest version of a specific crate.

**Flags:**
- `--limit 1`: Returns only the top result (most relevant match)
- `--limit N`: Returns N results (use 1 for exact lookups)

### Output Format

The `cargo search` command outputs results in the following format:

```
<crate_name> = "<version>"    # <description>
... and <N> crates more (use --limit N to see more)
note: to learn more about a package, run `cargo info <name>`
```

**Example:**
```
serde = "1.0.228"    # A generic serialization/deserialization framework
... and 16418 crates more (use --limit N to see more)
note: to learn more about a package, run `cargo info <name>`
```

### Parsing the Output

To extract the version from the output:

1. Split the first line by `=` 
2. Take the right-hand side
3. Remove surrounding whitespace and quotes
4. Stop at the first `#` character (description separator)

**Python example:**
```python
import re

output = 'serde = "1.0.228"    # A generic serialization/deserialization framework'
match = re.match(r'\s*\w[\w-]*\s*=\s*"([^"]+)"', output)
if match:
    version = match.group(1)  # "1.0.228"
```

**Regex pattern:** `^\s*(\S+)\s*=\s*"([^"]+)"\s*(?:#.*)?$`
- Group 1: crate name
- Group 2: version string

## Version Lookup Process

### For Adding New Dependencies

1. Run: `cargo search <crate_name> --limit 1`
2. Parse the output to extract the version
3. Verify the crate name matches exactly (see Exact Match Verification)
4. Use the extracted version in the Cargo.toml dependency

**Example:**
```bash
$ cargo search tokio --limit 1
tokio = "1.35.0"    # An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.
... and 4 crates more (use --limit N to see more)
```
Use version: `"1.35.0"`

### For Updating Existing Dependencies

1. Run: `cargo search <crate_name> --limit 1`
2. Parse the output to extract the latest version
3. Verify the crate name matches exactly
4. Compare with current version in Cargo.toml
5. If different, update to the latest version

## Exact Match Verification

### The Problem

`cargo search` performs fuzzy matching. Searching for `my-crate` might return `my_crate` (hyphen vs underscore). AI agents MUST verify the result name matches the queried name exactly before using its version.

### Verification Process

1. Extract both the queried crate name and the result crate name
2. Compare them **exactly** (character-by-character)
3. Only use the version if they match exactly
4. If they don't match, report "No exact match found for '<queried_name>'"

### Examples

**Match:**
- Query: `serde` → Result: `serde = "1.0.228"` → ✅ USE
- Query: `tokio` → Result: `tokio = "1.35.0"` → ✅ USE

**No Match:**
- Query: `my-crate` → Result: `my_crate = "0.1.0"` → ❌ DO NOT USE (hyphen ≠ underscore)
- Query: `serde_json` → Result: `serde-json = "1.0.0"` → ❌ DO NOT USE

### Case Sensitivity

Crate names on crates.io are **case-insensitive** in the registry, but the canonical name uses a specific case. The `cargo search` output will show the canonical name. AI agents should:

1. Perform case-insensitive comparison for initial matching
2. Use the canonical name (as shown in output) for the actual dependency

**Example:**
- Query: `SERDE` → Result: `serde = "1.0.228"` → Use `serde` (canonical name) with version `1.0.228`

## Edge Cases and Error Handling

### No Results

**Condition:** `cargo search` returns no output (empty result)

**Behavior:**
- Report: "Crate '<name>' not found in the crates.io registry"
- Abort the dependency addition/update operation
- Suggest: Verify the crate name spelling

**Example:**
```bash
$ cargo search nonexistent-crate-xyz123 --limit 1
$ # (no output)
```

### Network Unavailable

**Condition:** `cargo search` fails with network error

**Error patterns to detect:**
- "Failed to update registry"
- "Network connection failed"
- "Unable to access crates.io"
- Timeout errors

**Behavior:**
- Report: "Network error: Unable to connect to crates.io registry"
- Report the specific error message from cargo
- Abort the operation
- Suggest: Check network connection and retry

### Cargo Command Not Available

**Condition:** `cargo` command is not found or not executable

**Error patterns to detect:**
- "command not found: cargo"
- "cargo: command not found"

**Behavior:**
- Report: "Cargo (Rust package manager) is not installed or not in PATH"
- Abort the operation
- Suggest: Install Rust toolchain from https://rust-lang.org

### Registry Index Stale

**Condition:** Local registry index is outdated

**Behavior:**
- `cargo search` will automatically attempt to update the index
- If update fails, see Network Unavailable
- If update succeeds but search still fails, treat as No Results

### Pre-release Versions

By default, `cargo search` returns the latest version including pre-releases (versions with hyphens, e.g., `2.0.0-rc.1`).

**Behavior:**
- AI agents SHALL use whatever version `cargo search` returns as the latest
- If the user specifically requests stable-only versions, additional filtering may be needed
- Document that the returned version may be a pre-release

**Example:**
```
tokio = "1.35.0"    # Stable version
some-crate = "2.0.0-rc.1"  # Pre-release version
```

Both are valid and should be used as-is from cargo search.

### Yanked Versions

**Important:** `cargo search` does NOT show yanked versions in its results by default. The registry automatically filters them out.

**Behavior:**
- AI agents can trust that versions returned by `cargo search` are NOT yanked
- No additional yanked version checking is needed
- This is different from the crates.io API which does show yanked versions

## Workflow Integration

### When to Trigger Version Lookup

AI agents MUST trigger a `cargo search` lookup in the following scenarios:

1. **Adding a new dependency** to Cargo.toml
   - Before writing the dependency line
   - Use the latest version from cargo search

2. **Updating an existing dependency version** in Cargo.toml
   - Before changing the version
   - Verify the new version matches the latest from cargo search

3. **User explicitly requests** the latest version of a crate
   - Run cargo search and report the result

### When NOT to Trigger Version Lookup

1. **Changing version constraints** without changing the version (e.g., `^1.0.0` → `~1.0.0`)
2. **Adding/removing features** from an existing dependency
3. **Changing dependency kind** (e.g., from normal to dev-dependency)
4. **Removing a dependency**

### Using the Result in Cargo.toml

Once the latest version is determined:

```toml
[dependencies]
serde = "1.0.228"
tokio = "1.35.0"
```

**Version constraint recommendations:**
- For libraries: Use `^` (caret) for compatible updates: `^1.0.228`
- For applications: Use exact version: `=1.0.228` or `1.0.228`
- Follow the project's existing conventions

### Complete Process Example

**Scenario:** Add `serde` dependency to a project

```
1. User requests: "Add serde dependency"
2. Agent runs: cargo search serde --limit 1
3. Output: serde = "1.0.228"    # A generic serialization/deserialization framework
4. Agent parses: version = "1.0.228"
5. Agent verifies: "serde" == "serde" (exact match)
6. Agent adds to Cargo.toml:
   [dependencies]
   serde = "^1.0.228"
7. Agent reports: "Added serde version 1.0.228 (latest from crates.io)"
```

**Scenario:** Update `tokio` to latest version

```
1. Current Cargo.toml has: tokio = "1.30.0"
2. User requests: "Update tokio to latest version"
3. Agent runs: cargo search tokio --limit 1
4. Output: tokio = "1.35.0"    # An event-driven, non-blocking I/O platform...
5. Agent parses: version = "1.35.0"
6. Agent verifies: "tokio" == "tokio" (exact match)
7. Agent compares: "1.30.0" < "1.35.0" (update needed)
8. Agent updates Cargo.toml: tokio = "^1.35.0"
9. Agent reports: "Updated tokio from 1.30.0 to 1.35.0 (latest)"
```

## Implementation Notes

### Command Execution

- Run `cargo search` in the project directory (where Cargo.toml exists)
- The command requires network access to crates.io
- First run may take longer as it downloads/updates the registry index

### Performance

- Subsequent searches are fast (local index)
- Index updates happen automatically in the background
- No rate limiting concerns (cargo handles this)

### Cross-Platform

- Works on all platforms where cargo is available
- No additional dependencies needed
- Uses the user's existing cargo configuration

## Testing Your Implementation

### Test Cases

1. **Existing popular crate:**
   ```bash
   cargo search serde --limit 1
   # Expected: serde = "<version>" (latest version)
   ```

2. **Non-existent crate:**
   ```bash
   cargo search this-definitely-does-not-exist-12345 --limit 1
   # Expected: no output
   ```

3. **Crate with hyphen in name:**
   ```bash
   cargo search regex --limit 1
   # Expected: regex = "<version>" (exact match)
   ```

4. **Partial match:**
   ```bash
   cargo search my-crate --limit 1
   # Expected: May return my_crate or similar (verify exact match!)
   ```

## Summary Checklist for AI Agents

- [ ] Always use `cargo search <name> --limit 1` for version lookups
- [ ] Parse output format: `name = "version" # description`
- [ ] Verify exact name match before using version
- [ ] Handle no results case
- [ ] Handle network errors gracefully
- [ ] Handle cargo not available case
- [ ] Use latest version in Cargo.toml updates
- [ ] Report errors clearly to users
