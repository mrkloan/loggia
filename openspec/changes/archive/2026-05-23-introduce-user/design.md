## Context

The codebase follows a hexagonal architecture with three crates: `domain` (models, ports, services), `infrastructure/sqlite` (outbound adapters), and `application/api` (Axum HTTP layer). Currently there is no concept of identity — any caller can reach the API anonymously.

The `X-Vouch-User` header is set by an upstream Vouch proxy, which handles actual authentication. The API's responsibility is to trust that header, validate its value, and expose the user identity downstream.

## Goals / Non-Goals

**Goals:**
- Introduce a `User` domain model as a transient, non-persisted entity
- Validate `User` invariants at construction time within the domain
- Enforce presence and validity of `X-Vouch-User` on all protected routes
- Expose `GET /me` to reflect the current user's identity
- Keep `/health` public and exempt from authentication

**Non-Goals:**
- Storing users in the database
- Authorization / permission checks
- User management (creation, update, deletion)
- Any form of token issuance or session handling

## Decisions

### 1. `User` lives in the domain crate

**Decision**: `domain/src/models/user.rs`

The `username` is explicitly called out as a future FK anchor for database rows. Keeping `User` in the domain ensures that services and infrastructure can reference it when needed, without coupling to the HTTP layer.

Alternatives considered:
- _Define `User` in the API layer_: Simpler short-term, but prevents domain services and repositories from ever referencing the identity without creating an upward dependency.

### 2. Invariant enforcement via private field + `new()` constructor

**Decision**: The `username` field is private. `User::new(username: String) -> DomainResult<Self>` trims whitespace and returns `DomainError::Validation` if the result is empty. A public `username()` accessor exposes the value.

This makes it impossible to construct an invalid `User` anywhere in the codebase — the domain owns the rule.

Alternatives considered:
- _Validate in the extractor only_: Leaks the rule into the HTTP layer; any future construction path bypasses the check.

### 3. Axum extractor as the enforcement mechanism

**Decision**: Implement `FromRequestParts` for `User`. Any handler that declares `User` as a parameter automatically enforces the contract. No global middleware needed.

Error mapping:
- Header absent → `401 Unauthorized` + `{ "error": "Missing X-Vouch-User header" }`
- Header present, value fails validation → `401 Unauthorized` + `{ "error": "Missing X-Vouch-User header" }`

Alternatives considered:
- _Tower middleware_: Would enforce on every route including `/health`, requiring explicit exclusions. Extractor-based enforcement is more composable and idiomatic in Axum.

### 4. Router split: public vs. protected

**Decision**: The `http::router` function continues to own all routes. `/health` is registered without the `User` extractor; `GET /me` uses it. No separate middleware layer required at this stage.

## Risks / Trade-offs

- **Implicit contract**: Developers must remember to include `User` as an extractor parameter on new protected handlers. There is no compile-time enforcement of "all routes must be protected". → Mitigation: document the convention; revisit with middleware if the surface area grows.
- **Header trust**: The API fully trusts the `X-Vouch-User` value injected by the proxy. If the proxy is bypassed, any string becomes a valid identity. → Mitigation: out of scope for this change; network-level controls are assumed.
