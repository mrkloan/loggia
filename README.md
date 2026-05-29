# Loggia

[![CI](https://github.com/mrkloan/loggia/actions/workflows/ci.yml/badge.svg)](https://github.com/mrkloan/loggia/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/badge/docs-mrkloan.github.io/loggia-blue)](https://mrkloan.github.io/loggia/)

> A loggia is a piece of architecture that opens up a building to the outside world.

A Rust-based backend service built on Hexagonal Architecture (Ports & Adapters) principles.

## Overview

Loggia implements a clean separation of concerns through **Hexagonal Architecture**, which organizes the codebase into distinct layers that communicate through well-defined interfaces:

- **Domain Layer** (`domain/`): Contains the core business logic, entities, and use cases. This is the innermost layer and has no dependencies on external frameworks or databases.
- **Application Layer** (`application/`): Orchestrates use cases and coordinates between domain and infrastructure. Contains HTTP handlers and API routing.
- **Infrastructure Layer** (`infrastructure/`): Provides concrete implementations of domain ports (interfaces). Currently includes SQLite database persistence.

This architecture ensures that:
- Business logic remains pure and testable without external dependencies
- External adapters (SQLite, HTTP, etc.) can be swapped without affecting domain logic
- The domain layer defines interfaces (ports) that infrastructure implements (adapters)

**Specification-Driven Development**: This project uses a specification-driven workflow with OpenSpec. All formal specifications, requirements, and design decisions are maintained in [openspec/specs/](openspec/specs/), which serves as the canonical source of truth for project specifications.

For detailed architecture specifications, see: [openspec/specs/architecture/spec.md](openspec/specs/architecture/spec.md)

## Contributing Guidelines

### Developer Setup

#### Prerequisites
- Rust 1.70+ (recommended: latest stable)
- Cargo (comes with Rust)
- SQLite3 development libraries

#### Build

Clone the repository and build the entire workspace:

```bash
# Clone the repository
git clone <repository-url>
cd loggia

# Build all crates in the workspace
cargo build --workspace
```

#### Run

To run the API server:

```bash
# Export database URL (optional, defaults to sqlite:loggia.db)
export DATABASE_URL="sqlite:loggia.db"

# Run the API server
cargo run --package api
```

The server will start on `0.0.0.0:8080` with the following endpoints:
- `GET /health` - Health check endpoint
- `GET /me` - Get current authenticated user (requires `X-Vouch-User` header)

#### Test

Run all workspace tests:

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test --package domain
cargo test --package sqlite
cargo test --package api
```

#### Check Documentation

Verify that all documentation is complete and warning-free:

```bash
# Check compilation with documentation lints
cargo check --workspace

# Generate documentation and treat warnings as errors
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

# Open generated docs in browser
cargo doc --workspace --no-deps --all-features --open
```

### Documentation Requirements

This project enforces comprehensive documentation standards:

1. **All public items MUST be documented** - Traits, structs, enums, functions, and methods that are `pub` must have doc comments (`///` or `//!`)
2. **Focus on the "why", not the "how"** - Documentation should explain:
   - Business rules and invariants
   - Design rationale and constraints
   - What the entity represents in the domain
   - Why the interface exists
3. **Compiler enforcement** - All production crates use `#![deny(missing_docs)]` to ensure documentation completeness at compile time
4. **No compiler warnings** - Documentation must compile without warnings using `RUSTDOCFLAGS="-D warnings"`

For detailed documentation standards, see: [openspec/specs/project-documentation/spec.md](openspec/specs/project-documentation/spec.md)

### Code Organization Principles

When adding new features:

1. **Start from the domain** - Define entities, use cases, and ports first
2. **Implement infrastructure adapters** - Create concrete implementations of domain ports
3. **Wire up in application** - Connect use cases to HTTP handlers or other drivers
4. **Keep dependencies flowing inward** - Domain should never depend on application or infrastructure

## License

This project is licensed under the [UNLICENSE](https://unlicense.org/).
