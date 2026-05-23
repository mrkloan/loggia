## Why

Establish a highly-scalable, modular, and type-safe foundation for Rust development by introducing a multi-crate Cargo Workspace structured under Hexagonal Architecture (Ports and Adapters) with SQLite (SQLx) async persistence. This establishes clear boundaries between layers and documents a scalable blueprint for future feature additions (such as CLI apps or workers) without layer-leakage or redundant code.

## What Changes

- Create a root `Cargo.toml` to define the Cargo Workspace and manage shared dependencies.
- Create a consolidated `domain` library crate containing business entities (`models/`), inbound/outbound traits (`ports/`), and application orchestrators (`services/`).
- Create an `infrastructure/sqlite` library crate containing the SQLite database repository implementation, SQLx configuration, and database migrations.
- Create an `application/api` binary crate containing the Axum web server, HTTP route controllers, and the Dependency Injection (DI) Composition Root.
- Populate default health and basic endpoint structures to verify compilation and dependency mapping.

## Capabilities

### New Capabilities
- `workspace-architecture`: Defines the multi-crate workspace structure, dependency rule, and contribution rules for loggia's hexagonal layers.

### Modified Capabilities
<!-- None -->

## Impact

This is a greenfield initialization that restructures the root directory. It establishes the workspace boundaries, sets up third-party crate dependencies (e.g. `tokio`, `axum`, `sqlx`, `uuid`), and guarantees compile-time enforcement of the layer boundaries.
