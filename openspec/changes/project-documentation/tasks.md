## 1. Documentation Setup

- [ ] 1.1 Create `README.md` at the workspace root containing the project overview, architectural philosophy (Hexagonal Architecture), and directory layout.
- [ ] 1.2 Detail simple contributing guidelines in `README.md` with step-by-step developer setup (build, run, test) and documentation requirements.

## 2. Compile-Time Documentation Enforcement

- [ ] 2.1 Add `#![deny(missing_docs)]` to `domain/src/lib.rs` and thoroughly document all public modules, models, use cases, ports, and services in the `domain` crate.
- [ ] 2.2 Add `#![deny(missing_docs)]` to `infrastructure/sqlite/src/lib.rs` and document all public components (database connection pool, repository implementations) in the `sqlite` crate.
- [ ] 2.3 Add `#![deny(missing_docs)]` to `application/api/src/main.rs` and `application/api/src/http` modules to document any public items/handlers in the `api` crate.

## 3. Verification & CI Checking

- [ ] 3.1 Verify clean workspace compilation under the new documentation rules using `cargo check`.
- [ ] 3.2 Verify warning-free cargo doc generation by running `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features`.
