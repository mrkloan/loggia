## ADDED Requirements

### Requirement: Use-case-first module organization
All workspace crates SHALL organize their source code into capability modules rather than technical-layer directories. A capability module is a directory named after a business or cross-cutting concern (e.g. `health/`, `identity/`). Technical grouping directories (e.g. `models/`, `ports/`, `services/`, `extractors/`) SHALL NOT exist at any level within a crate.

The sole exception is `errors.rs` in the `domain` crate, which SHALL remain a flat file at the crate root as a cross-cutting concern shared by all capabilities.

#### Scenario: No technical-layer directories in domain
- **WHEN** the `domain` crate source tree is inspected
- **THEN** it SHALL NOT contain directories named `models`, `ports`, or `services`
- **THEN** it SHALL contain only capability module directories and the flat `errors.rs` file alongside `lib.rs`

#### Scenario: No technical-layer directories in infrastructure
- **WHEN** the `infrastructure/sqlite` crate source tree is inspected
- **THEN** it SHALL NOT contain flat repository files at the crate root (other than `lib.rs`)
- **THEN** it SHALL organize persistence adapters into capability module directories

#### Scenario: No technical-layer directories in application
- **WHEN** the `application/api` crate source tree is inspected
- **THEN** it SHALL NOT contain an `extractors/` directory or flat handler files under `http/`
- **THEN** it SHALL organize handlers and request-scoped extractors into capability module directories under `http/`

### Requirement: Action-oriented file naming within capability modules
Files within a capability module SHALL be named after the action or entity they represent, not after their technical role. Names such as `use_case.rs`, `service.rs`, `repository.rs`, and `inbound.rs` are forbidden. The use case trait, outbound port trait, and service implementation for a given use case SHALL be co-located in a single file named after the action (e.g. `check_health.rs`). Domain entity files SHALL be named after the entity (e.g. `system_health.rs`).

#### Scenario: Use case file is action-named
- **WHEN** a developer adds a new use case to a capability module
- **THEN** the file containing the use case trait, its outbound port(s), and service implementation SHALL be named after the action (e.g. `<verb>_<noun>.rs`)

#### Scenario: No role-named files inside a capability
- **WHEN** any capability module in any crate is inspected
- **THEN** it SHALL NOT contain files named `use_case.rs`, `service.rs`, `repository.rs`, `inbound.rs`, or `outbound.rs`

### Requirement: Capability naming symmetry across layers
A capability that spans multiple crates SHALL use the same capability name in each crate. The `domain` capability name SHALL be the authoritative reference; `infrastructure` and `application` layers SHALL mirror it.

#### Scenario: Health capability name is consistent across crates
- **WHEN** the source trees of `domain`, `infrastructure/sqlite`, and `application/api` are inspected
- **THEN** the health capability SHALL appear as `health/` in each crate that implements it

#### Scenario: Identity capability name is consistent across layers
- **WHEN** the source trees of `domain` and `application/api` are inspected
- **THEN** the identity capability SHALL appear as `identity/` in both crates

## MODIFIED Requirements

### Requirement: Core Ports and Services Placement
All domain entities, inbound port use-case traits, outbound port repository traits, and application service orchestrators SHALL be co-located inside the `domain` library crate, organized into capability modules. Each capability module SHALL contain all artifacts belonging to that capability: its entity model(s), use case trait(s), outbound port trait(s), and service implementation(s).

#### Scenario: Retrieve ports and services
- **WHEN** another developer inspects the `domain` crate
- **THEN** it SHALL contain all entities, use-case traits, repository traits, and services
- **THEN** all artifacts for a single capability SHALL reside within the same capability module directory
