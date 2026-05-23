## 1. Architecture Spec

- [x] 1.1 Update the directory layout diagram in `openspec/specs/architecture/spec.md` to reflect capability modules across all three crates
- [x] 1.2 Update the feature development playbook (Step 1–5) to use capability module structure and action-oriented file naming
- [x] 1.3 Remove references to `models/`, `ports/`, `services/`, and `extractors/` directories from the spec

## 2. Domain Crate

- [x] 2.1 Create `domain/src/health/` capability module with `mod.rs`, `system_health.rs`, and `check_health.rs` (consolidating `SystemHealth` model, `CheckHealthUseCase`, `HealthRepository`, `HealthService`, and tests)
- [x] 2.2 Create `domain/src/identity/` capability module with `mod.rs` (containing `User` and its tests)
- [x] 2.3 Update `domain/src/lib.rs` to declare `health` and `identity` modules in place of `models`, `ports`, and `services`
- [x] 2.4 Delete `domain/src/models/`, `domain/src/ports/`, and `domain/src/services/` directories
- [x] 2.5 Verify `domain` crate builds and all tests pass

## 3. Infrastructure Crate

- [x] 3.1 Create `infrastructure/sqlite/src/health/` capability module with `check_health.rs` (containing `SqliteHealthRepository`)
- [x] 3.2 Update `infrastructure/sqlite/src/lib.rs` to declare the `health` module and re-export `SqliteHealthRepository`
- [x] 3.3 Delete `infrastructure/sqlite/src/health_repository.rs`
- [x] 3.4 Verify `infrastructure/sqlite` crate builds and all tests pass

## 4. Application Crate

- [x] 4.1 Create `application/api/src/http/health/` capability module with `check_health.rs` (containing the health handler)
- [x] 4.2 Create `application/api/src/http/identity/` capability module with `authenticate.rs` (containing `AuthenticatedUser` extractor) and `get_me.rs` (containing the `/me` handler)
- [x] 4.3 Update `application/api/src/http/mod.rs` to declare `health` and `identity` modules and wire routes
- [x] 4.4 Delete `application/api/src/http/health.rs`, `application/api/src/http/me.rs`, and `application/api/src/http/extractors/` directory
- [x] 4.5 Verify `application/api` crate builds and all tests pass

## 5. Final Verification

- [x] 5.1 Run full workspace build (`cargo build --workspace`) and confirm no errors
- [x] 5.2 Run full workspace test suite (`cargo test --workspace`) and confirm no regressions
