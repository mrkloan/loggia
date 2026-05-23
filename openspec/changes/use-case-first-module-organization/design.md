## Context

The `loggia` workspace has three crates — `domain`, `infrastructure/sqlite`, `application/api` — all currently organized by technical layer. In `domain`, this means `models/`, `ports/`, and `services/` directories; in `application/api`, it means `http/extractors/` alongside flat handler files. Understanding any single capability (e.g. health check) requires visiting several directories. The new organizing principle groups all files belonging to a capability into a single module, named after the capability and its action.

## Goals / Non-Goals

**Goals:**
- Replace all technical-layer directories with capability modules in all three crates.
- Establish clear, enforceable naming conventions for files within a capability module.
- Update the architecture spec to make use-case-first organization the stated standard.

**Non-Goals:**
- Introducing new product capabilities.
- Changing any public API, database schema, or runtime behavior.
- Changing the hexagonal architecture principles or crate-level dependency rules.

## Decisions

### 1. Capability modules replace technical directories at every level

**Decision:** All technical groupings (`models/`, `ports/`, `services/`, `extractors/`) are removed. Each capability becomes a module directory. This applies uniformly to all three crates.

**Alternatives considered:**
- *Hybrid*: Keep `models/` for shared entities, use capability modules for ports/services. Rejected — partial adoption leaves the same navigation problem and creates ambiguity about where new code belongs.
- *Domain only*: Apply the rule only to `domain`. Rejected — the user-visible benefit is strongest when all layers mirror the same capability vocabulary.

### 2. Files within a capability are named after the action, not their technical role

**Decision:** Inside a capability module, files are named after what they *do*, not what they *are* technically. The use case, its outbound port, and the service implementation are co-located in a single action-named file (e.g. `check_health.rs`). Models are named after the entity they represent (e.g. `system_health.rs`).

**Rationale:** A file named `check_health.rs` immediately communicates intent. A file named `use_case.rs` or `service.rs` communicates structure — which is the exact coupling to technical convention we are eliminating.

**Alternatives considered:**
- *Separate files per role within capability*: `check_health.rs` (ports) + `health_service.rs` (impl). Rejected — reintroduces technical role naming and scatters the same use case again.
- *Everything in `mod.rs`*: Co-locate all capability code in the module file. Rejected — loses explicitness; `mod.rs` becomes opaque as capabilities grow.

### 3. `errors.rs` stays as a flat cross-cutting file

**Decision:** `errors.rs` is not placed inside a capability module. It is a domain-wide cross-cutting concern that all capabilities depend on.

**Rationale:** Placing it inside a module (e.g. `errors/mod.rs`) would imply it is a feature rather than shared infrastructure. Its flatness signals its special status.

### 4. `identity/` is the capability name in both `domain` and `api`

**Decision:** The `User` model lives in `domain/src/identity/mod.rs`. In `application/api`, the `AuthenticatedUser` extractor and the `/me` handler both live under `http/identity/`. The extractor (`authenticate.rs`) is co-located with the handler (`get_me.rs`) within `identity/` rather than in a shared `extractors/` directory.

**Rationale:** The extractor is an expression of the identity capability at the HTTP boundary. Any future protected endpoint imports `AuthenticatedUser` from `http::identity::authenticate` — a capability path, not a technical utility path.

## Risks / Trade-offs

- **Import churn**: Every `use` path in all three crates changes. → All paths are intra-workspace; Rust's compiler will catch every missed reference. No silent breakage.
- **Spec divergence**: The architecture spec's directory layout and playbook examples reference old paths. → The spec update is a task in this change; it is done before the code migration is considered complete.

## Migration Plan

1. Update `openspec/specs/architecture/spec.md` with the new organizing principle, directory layout, and naming conventions.
2. Restructure `domain` crate: create capability modules, move and consolidate files, update all `use` paths, verify tests pass.
3. Restructure `infrastructure/sqlite` crate: create capability modules, move files, update paths.
4. Restructure `application/api` crate: create capability modules, move files, update paths.
5. Run full workspace build and tests to confirm no regressions.

No rollback strategy required — this is a pure rename/reorganization with no runtime or schema changes.
