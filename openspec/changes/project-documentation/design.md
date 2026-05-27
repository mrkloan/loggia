## Context

Currently, the `loggia` codebase lacks a central `README.md` at the workspace root, making it harder for new contributors to understand the system architecture, workspace layout, and how to get started. Furthermore, while the project uses a highly modular Hexagonal (Ports & Adapters) architecture, there are no compile-time or automated checks to ensure the codebase remains thoroughly documented. To ensure high code quality, sustainability, and easy onboarding, we need to enforce comprehensive documentation standards.

## Goals / Non-Goals

**Goals:**
- Provide a robust, clean, and comprehensive `README.md` at the workspace root covering the architecture, workspace layout, and basic setup.
- Outline clear contributing guidelines for development, testing, and documentation standards.
- Enforce public documentation completeness across all production crates (`domain`, `infrastructure/sqlite`, `application/api`) at the compiler level via the `missing_docs` lint.
- Verify that Rust documentation compiles successfully with absolutely zero compiler warnings.
- Establish standards where public API/domain documentation focuses on the "why" (business context, invariants, invariants/reasons) instead of the "how" (code mechanics).

**Non-Goals:**
- Enforcing documentation completeness for private members or private helper functions (though internal comments are encouraged).
- Setting up external doc hosting or web publishing of the generated documentation.

## Decisions

### 1. Crate-Level Compiler Lint Enforcement
- **Decision**: Add `#![deny(missing_docs)]` at the root of all three production crates:
  - `domain/src/lib.rs`
  - `infrastructure/sqlite/src/lib.rs`
  - `application/api/src/main.rs` (and any other entrypoints)
- **Rationale**: Relying on manual code reviews to catch missing documentation is error-prone and slows down review cycles. Enforcing it at the compiler level guarantees that undocumented public APIs are caught immediately during development.
- **Alternatives Considered**: 
  - *`#![warn(missing_docs)]`*: Too easy to ignore in compiler noise.
  - *CI-only documentation checkers*: Slower feedback loops; compiler-level is immediate.

### 2. Standardizing "Explain the Why, Not the How"
- **Decision**: Establish strict guidelines in both `README.md` and the newly generated spec for how Rust doc comments (`///` and `//!`) should be authored. They must explain:
  - What invariants the entity or module maintains.
  - Why this struct, trait, or function exists in the domain.
  - What business requirement it directly implements.
- **Rationale**: The code itself explains *how* the logic works. Doc comments should add high-level context that cannot be derived directly from reading the code statements.
- **Alternatives Considered**: Allow generic, auto-generated style doc comments (e.g. `/// Creates a new system health check struct.`). This is rejected as it adds zero cognitive value.

### 3. Verification of Warning-Free Docs via Cargo Doc
- **Decision**: Define a standardized command for automated documentation verification:
  ```bash
  RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
  ```
- **Rationale**: Rustdoc-specific issues (e.g., broken intra-doc links, invalid Markdown in doc comments) are not caught by standard `cargo build` or `cargo check`. Forcing `RUSTDOCFLAGS="-D warnings"` ensures that all links are resolved and docs are perfect.
- **Alternatives Considered**: Standard `cargo doc`. However, this does not fail the build on doc compilation warnings.

### 4. Linking to Specs Instead of Duplication in README.md
- **Decision**: The root `README.md` will provide only high-level conceptual summaries (such as structural diagrams and basic layouts) and will link directly to the formal specification files (e.g., `openspec/specs/architecture/spec.md`) for detailed behavior, requirements, and playbook steps.
- **Rationale**: Duplicating specifications in both the `README.md` and the OpenSpec folders leads to a dual maintenance burden and inevitable synchronization drift. Linking ensures a single source of truth for the formal specifications.
- **Alternatives Considered**: Duplicating the entire Hexagonal playbooks and requirements directly into `README.md`. This was rejected to avoid specification out-of-sync issues.


## Risks / Trade-offs

- **[Risk] Developer Friction**: Forcing `#![deny(missing_docs)]` might be perceived as annoying during rapid prototyping.
  - *Mitigation*: The lint can be temporarily allowed with `#[allow(missing_docs)]` on experimental code, but must be removed before merging. The onboarding docs will clearly explain the value of this requirement.
- **[Risk] Doc Verbosity vs. Value**: Developers might write verbose but low-quality comments just to satisfy the compiler.
  - *Mitigation*: The contributing guidelines and specs will lay out concrete examples of "good" vs. "bad" doc comments, focusing on "why" vs. "how".
