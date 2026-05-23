## Context

The `loggia` project is a new Rust project aimed at providing a high-performance REST API. To prevent architectural drift, spaghetti code, and dependency leakage, we are implementing a multi-crate Cargo Workspace based on **Hexagonal Architecture (Ports and Adapters)**. This design details the concrete layout, dependency directions, and engineering decisions to construct this modular, testable skeleton.

## Goals / Non-Goals

**Goals:**
- Configure a multi-crate Cargo Workspace featuring `domain`, `infrastructure/sqlite`, and `application/api`.
- Enforce strict compile-time architectural boundaries: core business rules must be independent of delivery mechanisms (HTTP) and storage solutions (SQLite).
- Integrate **SQLx with SQLite** as the async persistence engine, configuring dynamic migrations on startup.
- Establish **Axum** as the HTTP delivery layer inside `application/api`, demonstrating dependency injection (Composition Root) using thread-safe dynamic dispatch (`Arc<dyn Trait>`).
- Verify the workspace by compiling and running a lightweight, functioning health check API.

**Non-Goals:**
- Implement real business domain features (such as authorization, user profiles, or task tracking).
- Set up continuous integration (CI) environments, Docker containers, or production cloud deployment pipelines.
- Integrate heavy Object-Relational Mapping (ORM) frameworks; all SQL interaction will rely on raw, compile-time verified SQLx queries.

## Decisions

### 1. Single Library Crate for Core Logic (`domain`)
We will combine pure domain entities, inbound ports (use cases), outbound ports (repositories), and core services into a single library crate: `domain`.
- *Alternatives Considered*: Having a separate `domain` crate and `application` crate.
- *Rationale*: Combining them into a single `domain` crate keeps the workspace simple and eliminates unnecessary crate-management boilerplate. Because the `domain` crate declares zero workspace dependencies, it remains completely clean of web and storage technology leaks.

### 2. Grouping Infrastructure Adapters & Executables in Subdirectories
Infrastructure adapters live in `infrastructure/` (e.g., `infrastructure/sqlite`), and binary entry points live in `application/` (e.g., `application/api`).
- *Alternatives Considered*: Flat root workspace.
- *Rationale*: Subdirectories bring absolute symmetry to Ports and Adapters. Outer layers are logically separated. It also makes future additions—such as a CLI application under `application/cli` or a worker under `application/worker`—incredibly simple and clean.

### 3. Dynamic Dispatch (`Arc<dyn Trait>`) for Dependency Injection
Dependency Injection (DI) will be implemented using trait objects and dynamic dispatch (e.g., `Arc<dyn UserRepository>`).
- *Alternatives Considered*: Static dispatch using generics and trait bounds (e.g. `UserService<R: UserRepository>`).
- *Rationale*: While static dispatch has zero runtime cost, it infects struct definitions with extensive generic cascades and type signatures, leading to complex and hard-to-maintain code, especially when routing via Axum's shared state. Dynamic dispatch has negligible overhead (nanoseconds) but provides clean, highly readable type signatures and simplifies composition.

### 4. SQLx Offline Verification Mode
SQLx queries will be compile-time checked. We will configure SQLx's offline mode using `sqlx-data.json`.
- *Alternatives Considered*: Disabling compile-time verification, or using a live database file during all builds.
- *Rationale*: Offline mode lets developers compile the project (and run `cargo check` / CI) without needing an active SQLite database file or database engine running on the host system.

## Risks / Trade-offs

- **[Risk] Compile-Time DB Check Overhead** → *Mitigation*: SQLx requires a valid database schema to compile queries. We mitigate this by checking in the generated `sqlx-data.json` offline file to the git repository. Contributors can compile the app immediately without any SQLite setup.
- **[Risk] Boilerplate Multi-Crate Setup** → *Mitigation*: Creating multiple crates requires more `Cargo.toml` configuration up front. However, this is a one-time setup cost that pays permanent dividends by using Rust's compiler to absolutely guarantee that database/HTTP concepts are never imported into core business logic.
