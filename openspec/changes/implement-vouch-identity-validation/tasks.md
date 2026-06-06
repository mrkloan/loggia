## 1. Domain Layer Updates

- [ ] 1.1 Add `Authentication(String)` and `PartnerUnavailable(String)` variants to `DomainError` in `domain/src/errors.rs`
- [ ] 1.2 Create `domain/src/identity/provider.rs` with `IdentityProvider` async trait
- [ ] 1.3 Update `domain/src/identity/mod.rs` to export the `provider` module
- [ ] 1.4 Add `async-trait` dependency to `domain/Cargo.toml`

## 2. Infrastructure Layer - Vouch Adapter

- [ ] 2.1 Create `infrastructure/vouch/Cargo.toml` with dependencies: `domain`, `reqwest`, `async-trait`, `url`
- [ ] 2.2 Create `infrastructure/vouch/src/lib.rs` with `VouchIdentityProvider` struct
- [ ] 2.3 Implement `IdentityProvider` for `VouchIdentityProvider` in `infrastructure/vouch/src/lib.rs`
- [ ] 2.4 Add `VouchIdentityProvider::new()` constructor that reads `VOUCH_VALIDATE_URL` and `VOUCH_COOKIE` env vars
- [ ] 2.5 Implement token validation logic: call `/validate` endpoint, handle response, extract `X-Vouch-User` header
- [ ] 2.6 Map Vouch responses to domain errors (`Authentication`, `PartnerUnavailable`, `Validation`)

## 3. Workspace Updates

- [ ] 3.1 Add `infrastructure/vouch` as a workspace member in root `Cargo.toml`
- [ ] 3.2 Add `reqwest` and `url` to workspace dependencies in root `Cargo.toml`

## 4. Application Layer - State and Router

- [ ] 4.1 Create `AppState` struct in `application/api/src/main.rs` with `health_use_case` and `identity_provider` fields
- [ ] 4.2 Implement `Has<Arc<dyn IdentityProvider + Send + Sync>>` for `AppState`
- [ ] 4.3 Update `application/api/src/main.rs` to create `VouchIdentityProvider` and `AppState`
- [ ] 4.4 Update `application/api/src/http/mod.rs` router to accept `AppState` instead of just `health_use_case`
- [ ] 4.5 Add `application/api/src/http/identity/constants.rs` or add `X_IDENTITY_TOKEN` constant to existing module

## 5. Application Layer - Authentication Extractor

- [ ] 5.1 Replace `application/api/src/http/identity/authenticate.rs` with new extractor implementation
- [ ] 5.2 New `AuthenticatedUser` extractor extracts `X-Identity-Token` header
- [ ] 5.3 New extractor calls `IdentityProvider::validate_token()` via state
- [ ] 5.4 Map domain errors to HTTP responses (401 for `Authentication`/`Validation`, 502 for `PartnerUnavailable`)
- [ ] 5.5 Return `401` with `{"error": "missing_identity_token"}` when header is missing

## 6. Application Layer - Handler Updates

- [ ] 6.1 Update `application/api/src/http/identity/get_me.rs` imports to use new `AuthenticatedUser`
- [ ] 6.2 Verify all other handlers using authentication are updated (none currently exist beyond `/me`)

## 7. Cargo Dependencies

- [ ] 7.1 Add `reqwest` and `url` dependencies to `application/api/Cargo.toml`

## 8. Verification

- [ ] 8.1 Verify workspace builds successfully with `cargo build --workspace`
- [ ] 8.2 Run existing tests to ensure no regressions
- [ ] 8.3 Test `/health` endpoint is accessible without authentication
- [ ] 8.4 Test `/me` endpoint returns 401 without `X-Identity-Token` header
- [ ] 8.5 Test `/me` endpoint returns 401 with invalid `X-Identity-Token` header
- [ ] 8.6 Test `/me` endpoint returns 200 with valid token (requires running Vouch Proxy)
