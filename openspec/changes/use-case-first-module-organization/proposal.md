## Why

All three crates are currently organized by technical layers (`models/`, `ports/`, `services/`, `extractors/`), scattering related code across directories and forcing readers to navigate multiple locations to understand a single capability. Reorganizing around capabilities co-locates everything belonging to a use case, making the codebase easier to navigate, extend, and reason about.

## What Changes

- **domain**: Replace `models/`, `ports/`, and `services/` directories with capability modules (`health/`, `identity/`); keep `errors.rs` as a flat cross-cutting file.
- **infrastructure/sqlite**: Replace flat repository files with capability modules (`health/`).
- **application/api**: Replace `http/extractors/` and flat handler files with capability modules (`http/health/`, `http/identity/`).
- **Architecture spec**: Update the organizing principle from technical layers to use-case-first capability modules; update directory layout, naming conventions, and the feature development playbook.

## Capabilities

### New Capabilities

None. This change is structural — no new product capabilities are introduced.

### Modified Capabilities

- `architecture`: The module organization rule changes from technical-layer grouping to use-case-first capability modules. File naming conventions within a capability shift from technical role names (`use_case.rs`, `service.rs`, `repository.rs`) to action-oriented names (`check_health.rs`). The directory layout and feature development playbook must be updated to reflect the new structure.

## Impact

- All three workspace crates are restructured; all intra-crate `use` paths must be updated.
- No public API, database schema, or runtime behavior changes.
- The architecture spec becomes the authoritative reference for the new module organization convention.
