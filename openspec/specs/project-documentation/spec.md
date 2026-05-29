# Project Documentation Specification

This specification defines the documentation standards and requirements for the `loggia` project, including workspace-level overview, Rust documentation exhaustiveness, comprehensiveness guidelines, and automated verification.

---

## Requirement: Workspace README.md Overview and Contributing Guidelines

The workspace root SHALL contain a `README.md` file that provides a high-level overview of the `loggia` project. It MUST describe the Hexagonal (Ports & Adapters) architecture, layer relationships, and provide simple, actionable contribution guidelines for developers. The README SHALL start with badges for CI status and project documentation, followed by the project's catch phrase "A loggia is a piece of architecture that opens up a building to the outside world." in a citation block. The README SHALL reference the UNLICENSE at <https://unlicense.org/> in its License section.

### Scenario: Workspace README contains essential sections
- **WHEN** the `README.md` at the workspace root is read
- **THEN** it SHALL start with a CI badge: [![CI](https://github.com/mrkloan/loggia/actions/workflows/ci.yml/badge.svg)](https://github.com/mrkloan/loggia/actions/workflows/ci.yml)
- **THEN** it SHALL contain a documentation badge: [![Documentation](https://img.shields.io/badge/docs-mrkloan.github.io/loggia-blue)](https://mrkloan.github.io/loggia/)
- **THEN** it SHALL start with a citation block containing "A loggia is a piece of architecture that opens up a building to the outside world."
- **THEN** it SHALL contain an "Overview" section outlining Hexagonal Architecture
- **THEN** it SHALL contain a note in the Overview section about the specification-driven workflow using OpenSpec
- **THEN** it SHALL reference `openspec/specs/` as the canonical source of truth for all project specifications
- **THEN** it SHALL contain a "Contributing Guidelines" section detailing how to add new features or run tests
- **THEN** it SHALL link directly to the formal architecture and capability specifications in `openspec/specs/` instead of duplicating their requirements or detailed design
- **THEN** it SHALL contain a "License" section that references the [UNLICENSE](https://unlicense.org/)

---

## Requirement: Rust Documentation Exhaustiveness

All production Rust crates in the workspace MUST enforce that all public items (modules, traits, structs, enums, functions, and methods) are documented. This SHALL be enforced at compile time using crate-level lints.

### Scenario: Compilation fails when a public item is undocumented
- **WHEN** a contributor defines a new public module, struct, trait, or function without doc comments
- **THEN** compiling the crate via Cargo SHALL fail with a `missing_docs` compilation error

---
## Requirement: Rust Documentation Comprehensiveness

Doc comments for all public items MUST focus on explaining the "why" (business rules, invariants, design rationale, and constraints) rather than the "how" (implementation details).

### Scenario: Public trait or model documentation explains the rationale
- **WHEN** a developer views the generated Rust documentation or doc comments for a domain port, entity, or service
- **THEN** the documentation SHALL explain the business context, invariants, and why the interface or entity is designed this way

---
## Requirement: Automated Rust Documentation Verification

The project's workspace setup SHALL support clean, warning-free generation of Rust documentation. Documentation checks MUST be easily runnable to verify correctness and completeness.

### Scenario: Verify warning-free cargo doc generation
- **WHEN** the command `cargo doc --workspace --no-deps --all-features` is executed with warnings treated as errors
- **THEN** all HTML documentation for workspace crates SHALL generate successfully without any rustdoc compiler warnings
