## ADDED Requirements

### Requirement: Cargo Workspace Definition
The codebase MUST be initialized as a Cargo Workspace containing a root `Cargo.toml` file that specifies `domain`, `infrastructure/sqlite`, and `application/api` as its workspace members.

#### Scenario: Workspace structure verification
- **WHEN** the project root Cargo.toml is parsed
- **THEN** it SHALL list "domain", "infrastructure/sqlite", and "application/api" in its members array

### Requirement: Strict Unidirectional Layer Dependencies
The compilation dependencies between workspace members SHALL point strictly inward. The `domain` crate SHALL have no dependencies on other workspace crates. The `infrastructure/sqlite` crate SHALL depend only on the `domain` crate. The `application/api` crate SHALL depend on both `domain` and `infrastructure/sqlite`.

#### Scenario: Verify compiler separation
- **WHEN** compilation of the "domain" crate is performed
- **THEN** it MUST succeed without importing any DB or API libraries

### Requirement: Core Ports and Services Placement
All domain entities, inbound port use-case traits, outbound port repository traits, and application service orchestrators SHALL be co-located inside the `domain` library crate.

#### Scenario: Retrieve ports and services
- **WHEN** another developer inspects the "domain" crate
- **THEN** it SHALL contain all entities, use-case traits, repository traits, and services

### Requirement: Async SQLite Persistence Adapter
The `infrastructure/sqlite` crate SHALL implement an async SQLite persistence adapter using SQLx, which executes all database operations asynchronously and automatically runs SQL schema migrations on application startup.

#### Scenario: Run startup migrations
- **WHEN** the "infrastructure/sqlite" adapter establishes a connection pool
- **THEN** it SHALL automatically run all migrations inside the migrations directory

### Requirement: API Executable and DI Composition Root
The `application/api` crate SHALL compile to an executable binary that uses Axum to expose a REST API health endpoint, and it SHALL serve as the dependency injection Composition Root that instantiates the SQLite database repository adapter and injects it into domain services.

#### Scenario: Serve health check endpoint
- **WHEN** a GET request is sent to "/health"
- **THEN** the system SHALL return an HTTP 200 OK status
