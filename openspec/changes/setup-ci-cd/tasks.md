# Tasks: Implement CI/CD pipeline

- [ ] 1) enable-sqlite-bundled
  - In the workspace `Cargo.toml`, add `sqlite-bundled` to the sqlx features so the binary statically links SQLite (required for the distroless runtime image which has no system libsqlite3).

- [ ] 2) create-dockerfile
  - Create `application/api/Dockerfile` (multi-stage) per design.
  - Use split COPY pattern (manifests first, then sources) for Docker layer caching.
  - No `libsqlite3-dev` or `libssl-dev` needed at build time thanks to `sqlite-bundled`.

- [ ] 3) add-ci-workflow
  - Add `.github/workflows/ci.yml` with a `test` job (all branches/PRs) that:
    - checks out code
    - sets up Rust toolchain using `dtolnay/rust-toolchain`
    - caches cargo artifacts using `Swatinem/rust-cache`
    - runs `cargo build` and `cargo test` for the workspace

- [ ] 4) add-package-job
  - In the workflow, add a `package` job (all branches/PRs, `needs: test`) to:
    - checkout code
    - login to GHCR using `docker/login-action` and `GITHUB_TOKEN`
    - build the Docker image using `docker/build-push-action`
    - conditionally push with `push: ${{ github.ref == 'refs/heads/main' }}` (validates on branches, publishes on main)
    - tag as `ghcr.io/${{ github.repository_owner }}/loggia-api:${{ github.sha }}` and `:latest`

- [ ] 5) add-docs-deploy
  - Add a `publish-docs` job (main only, `needs: test` — independent of `package`) to:
    - set up Rust toolchain using `dtolnay/rust-toolchain`
    - run `cargo doc --workspace --no-deps --release` to generate docs for all crates (domain, sqlite, api)
    - upload `target/doc` with `actions/upload-pages-artifact@v3`
    - deploy with `actions/deploy-pages@v4`

Each task should be implemented in order; the `sqlite-bundled` feature must be enabled before the Dockerfile is created.
