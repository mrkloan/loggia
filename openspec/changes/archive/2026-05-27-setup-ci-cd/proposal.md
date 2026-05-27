# Proposal: Setup CI/CD with GitHub Actions

## Summary
Add a GitHub Actions-based CI/CD pipeline with four jobs in a single workflow:

- **test** (all branches/PRs): run `cargo build` and `cargo test` for the entire workspace
- **package** (all branches/PRs, after test): build the Docker image for `application/api`; push to GitHub Container Registry (ghcr.io) on main only
- **publish-docs** (main only, after test): generate rustdoc for all workspace crates and deploy to GitHub Pages

```
test ──▶ package  (build always, push on main only)
  └────▶ publish-docs  (main only)
```

## Motivation
Automate testing and delivery, provide reproducible container images and published documentation for all workspace crates.

## Scope
- Add `.github/workflows/ci.yml`
- Add `application/api/Dockerfile` (multi-stage: `rust:1.95.0-slim` builder → `gcr.io/distroless/cc-debian12` runtime)
- Enable `sqlx` bundled SQLite feature so the binary has no runtime libsqlite3 dependency
- Push images to GHCR with `GITHUB_TOKEN` (no PATs required)
- Publish `cargo doc --workspace` output for all crates (`domain`, `sqlite`, `api`) to GitHub Pages

## Success criteria
- All branch builds run tests and Docker image builds successfully
- Main builds push a tagged image to GHCR
- Rust docs for all workspace crates are available on GitHub Pages after main builds

## Out of scope
Image signing, semantic versioning/tagging, multi-arch builds, and CI validation (verified manually by an engineer)
