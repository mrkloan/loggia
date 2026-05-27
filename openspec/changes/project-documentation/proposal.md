## Why

The project currently lacks a central `README.md` at the workspace root, which hampers onboarding and clarity around project layout and architecture. Additionally, there are no established standards or automated verifications to ensure that all Rust production code is thoroughly documented with an emphasis on "why" (business rules and rationale) rather than "how" (implementation details).

## What Changes

- **Add Root README**: Create `README.md` at the workspace root containing a high-level overview of the project, its Hexagonal Architecture, workspace layout, and simple contributing guidelines.
- **Establish Documentation Standards**: Formally define standards for exhaustiveness and comprehensiveness (explaining the why rather than the how) for all production Rust code.
- **Automated Rust Documentation Checks**: Configure crates/workspace to enforce complete documentation (`missing_docs` lints) and verify that Rust doc can be successfully generated without warnings.

## Capabilities

### New Capabilities
- `project-documentation`: Defines specification and requirements for project overview docs (`README.md`), contributing guidelines, and exhaustive Rust doc standards and verification.

### Modified Capabilities

## Impact

- **Root directory**: A new `README.md` file will be created.
- **Rust crates (`domain`, `infrastructure/sqlite`, `application/api`)**: Crate configuration (`lib.rs` / `main.rs`) will be updated to enforce `#[deny(missing_docs)]` or standard documentation checks, and developer workflows will now require `cargo doc` compilation checks.
