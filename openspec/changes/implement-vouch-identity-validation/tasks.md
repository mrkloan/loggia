## 1. Domain Layer Updates

- [x] 1.1 Add `Authentication(String)` and `PartnerUnavailable(String)` variants to `DomainError` in `domain/src/errors.rs`
- [x] 1.2 Create `domain/src/identity/provider.rs` with `IdentityProvider` async trait
- [x] 1.3 Update `domain/src/identity/mod.rs` to export the `provider` module
- [x] 1.4 Add `async-trait` dependency to `domain/Cargo.toml`

## 2. Infrastructure Layer - Vouch Adapter

- [x] 2.1 Create `infrastructure/vouch/Cargo.toml` with dependencies: `domain`, `reqwest`, `async-trait`, `url`
- [x] 2.2 Create `infrastructure/vouch/src/lib.rs` with `VouchIdentityProvider` struct
- [x] 2.3 Implement `IdentityProvider` for `VouchIdentityProvider` in `infrastructure/vouch/src/lib.rs`
- [x] 2.4 Add `VouchIdentityProvider::new()` constructor that reads `VOUCH_VALIDATE_URL` and `VOUCH_COOKIE` env vars
- [x] 2.5 Implement token validation logic: call `/validate` endpoint, handle response, extract `X-Vouch-User` header
- [x] 2.6 Map Vouch responses to domain errors (`Authentication`, `PartnerUnavailable`, `Validation`)

## 3. Workspace Updates

- [x] 3.1 Add `infrastructure/vouch` as a workspace member in root `Cargo.toml`
- [x] 3.2 Add `reqwest` and `url` to workspace dependencies in root `Cargo.toml`

## 4. Application Layer - State and Router

- [x] 4.1 Create `AppState` struct in `application/api/src/main.rs` with `health_use_case` and `identity_provider` fields
- [x] 4.2 Implement `Has<Arc<dyn IdentityProvider + Send + Sync>>` for `AppState`
- [x] 4.3 Update `application/api/src/main.rs` to create `VouchIdentityProvider` and `AppState`
- [x] 4.4 Update `application/api/src/http/mod.rs` router to accept `AppState` instead of just `health_use_case`
- [x] 4.5 Add `application/api/src/http/identity/constants.rs` or add `X_IDENTITY_TOKEN` constant to existing module

## 5. Application Layer - Authentication Extractor

- [x] 5.1 Replace `application/api/src/http/identity/authenticate.rs` with new extractor implementation
- [x] 5.2 New `AuthenticatedUser` extractor extracts `X-Identity-Token` header
- [x] 5.3 New extractor calls `IdentityProvider::validate_token()` via state
- [x] 5.4 Map domain errors to HTTP responses (401 for `Authentication`/`Validation`, 502 for `PartnerUnavailable`)
- [x] 5.5 Return `401` with `{"error": "missing_identity_token"}` when header is missing

## 6. Application Layer - Handler Updates

- [x] 6.1 Update `application/api/src/http/identity/get_me.rs` imports to use new `AuthenticatedUser`
- [x] 6.2 Verify all other handlers using authentication are updated (none currently exist beyond `/me`)

## 7. Cargo Dependencies

- [x] 7.1 Add `reqwest` and `url` dependencies to `application/api/Cargo.toml`

## 8. Verification

- [x] 8.1 Verify workspace builds successfully with `cargo build --workspace`
- [x] 8.2 Run existing tests to ensure no regressions
- [x] 8.3 Test `/health` endpoint is accessible without authentication
- [x] 8.4 Test `/me` endpoint returns 401 without `X-Identity-Token` header
- [x] 8.5 Test `/me` endpoint returns 401 with invalid `X-Identity-Token` header

## 9. Unit Tests for VouchIdentityProvider

- [x] 9.1 Add `mockito` dev-dependency to `infrastructure/vouch/Cargo.toml`
- [x] 9.2 Add test helper `VouchIdentityProvider::new_test()` for creating test instances
- [x] 9.3 Add test for successful token validation with user header
- [x] 9.4 Add test for successful validation with trimmed username
- [x] 9.5 Add test for missing `X-Vouch-User` header error
- [x] 9.6 Add test for empty `X-Vouch-User` header error
- [x] 9.7 Add test for 401 Unauthorized response
- [x] 9.8 Add test for 500 server error response
- [x] 9.9 Add test for 404 Not Found response
- [x] 9.10 Add test for correct cookie header formatting
- [x] 9.11 Add test for correct Host header
- [x] 9.12 Add test for connection error handling
- [x] 9.13 Run all vouch tests to verify implementation
