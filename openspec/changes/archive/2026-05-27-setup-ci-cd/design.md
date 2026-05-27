# Design: CI/CD pipeline (GitHub Actions)

## Overview
Use a single workflow file `.github/workflows/ci.yml` triggered on `push` and `pull_request`.
Jobs:
- `test` (runs on all branches/PRs): checkout, setup Rust, cache cargo, `cargo build` and `cargo test` for the workspace
- `package` (runs on all branches/PRs, depends on `test`): build the Docker image using `docker/build-push-action`; conditionally push to GHCR on main only (`push: ${{ github.ref == 'refs/heads/main' }}`)
- `publish-docs` (runs only on `refs/heads/main`, depends on `test`): generate `cargo doc` for all workspace crates and publish via the GitHub Pages artifact flow

```
  test ──▶ package  (build always, push on main only)
    └────▶ publish-docs  (main only)
```

## Workflow-level permissions
Set required permissions so the workflow can push packages and deploy Pages:
```
permissions:
  contents: read
  packages: write
  pages: write
  id-token: write
```

## SQLite bundled feature

> [!CAUTION]
> **Post-implementation erratum:** `sqlite-bundled` is **not** a valid sqlx 0.7 feature and causes a build failure. In sqlx 0.7, the `sqlite` feature already statically bundles SQLite by default (via `libsqlite3-sys` with `bundled`). No additional feature flag is needed. The correct dependency line is shown below.

```toml
sqlx = { version = "0.7.4", features = ["runtime-tokio", "sqlite", "macros"] }
```
The `sqlite` feature statically compiles SQLite into the binary, removing the runtime dependency on `libsqlite3`. Required for the distroless runtime image which has no system libraries. Also eliminates the need for `libsqlite3-dev` in the Docker builder stage.

## Docker image
- Registry: ghcr.io (GitHub Container Registry)
- Image name: `ghcr.io/${{ github.repository_owner }}/loggia-api`
- Tags: `${{ github.sha }}` and `latest`
- Single `package` job handles both build validation (all branches) and push (main only) via conditional `push: ${{ github.ref == 'refs/heads/main' }}`
- Use `docker/build-push-action` + `docker/login-action` with `GITHUB_TOKEN` and `packages: write` permission

## Dockerfile (place at `application/api/Dockerfile`) — multi-stage with layer caching
```Dockerfile
# builder stage
FROM rust:1.95.0 AS builder
WORKDIR /usr/src

# Copy manifests and lock file first for dependency layer caching
COPY Cargo.toml Cargo.lock ./
COPY domain/Cargo.toml domain/Cargo.toml
COPY infrastructure/sqlite/Cargo.toml infrastructure/sqlite/Cargo.toml
COPY application/api/Cargo.toml application/api/Cargo.toml

# Create stub source files so cargo can resolve the workspace and fetch/build dependencies
RUN mkdir -p domain/src infrastructure/sqlite/src application/api/src && \
    echo "" > domain/src/lib.rs && \
    echo "" > infrastructure/sqlite/src/lib.rs && \
    echo "fn main() {}" > application/api/src/main.rs

# Build dependencies only (this layer is cached unless Cargo.toml/Cargo.lock change)
RUN cargo build --release --package api

# Copy actual source code
COPY . .

# Rebuild with real sources (only our code recompiles, dependencies are cached)
RUN touch domain/src/lib.rs infrastructure/sqlite/src/lib.rs application/api/src/main.rs && \
    cargo build --release --package api

# runtime stage (distroless — minimal, production-focused)
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/target/release/api /usr/local/bin/api
USER 65532
EXPOSE 8080
CMD ["/usr/local/bin/api"]
```
Notes:
- No `apt-get install` needed: sqlx's `sqlite` feature bundles and compiles SQLite from C source using the C compiler already present in the `rust:1.95.0` image; no system `libsqlite3-dev` or `libssl-dev` required.
- The split COPY pattern caches the dependency build layer — only a `Cargo.toml` or `Cargo.lock` change triggers a full dependency rebuild.
- Runtime uses distroless to minimize image size (~20MB) and attack surface. Trade-off: no shell for debugging; all runtime libraries must be present or statically linked.

## Docs publishing
- Depends on `test` only (independent of `package` for faster feedback and decoupled failure domains)
- Run `cargo doc --workspace --no-deps --release` (covers all crates: `domain`, `sqlite`, `api`)
- Upload `target/doc` using `actions/upload-pages-artifact@v3`
- Deploy using `actions/deploy-pages@v4`

## Tooling choices
- `actions/checkout@v4`
- `dtolnay/rust-toolchain` — actively maintained replacement for the deprecated `actions-rs/toolchain@v1`; simpler configuration
- `Swatinem/rust-cache@v2` for cargo caching — handles Rust workspaces intelligently (keyed by lockfile, prunes old artifacts), avoids multi-GB `target/` cache bloat
- `docker/login-action@v3` + `docker/build-push-action@v6` for image build/push
- `actions/upload-pages-artifact@v3` and `actions/deploy-pages@v4` for docs

## Security and permissions
- Prefer `GITHUB_TOKEN` with `packages: write`, `pages: write`, and `id-token: write` permissions to avoid adding PATs. The `id-token: write` permission is required by `actions/deploy-pages` for OIDC token generation.
