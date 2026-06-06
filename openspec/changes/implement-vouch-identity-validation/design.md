## Context

The codebase implements Hexagonal Architecture with three crates: `domain` (entities, ports, use cases), `infrastructure/sqlite` (outbound adapters), and `application/api` (Axum HTTP layer). Currently, authentication relies on trusting the `X-Vouch-User` header injected by an upstream Vouch Proxy. This is insufficient for internal service-to-service calls where the header cannot be guaranteed, and provides no direct validation at the API layer.

The existing identity flow uses an Axum extractor (`AuthenticatedUser`) that simply checks for the presence of the `X-Vouch-User` header and constructs a domain `User` entity from it. No validation of the header value occurs within the API.

## Goals / Non-Goals

**Goals:**
- Implement direct token validation via Vouch Proxy's `/validate` endpoint for all API requests
- Replace header-trust model with active validation for both public and internal request paths
- Maintain Hexagonal Architecture patterns (domain ports, infrastructure adapters)
- Support per-route authentication via Axum extractor pattern
- Add proper error handling for validation failures and service unavailability
- Keep `/health` endpoint publicly accessible without authentication

**Non-Goals:**
- Support for `X-Vouch-User` header (breaking change, removed entirely)
- Token caching or validation result caching
- Direct JWT parsing or signature verification in the API
- GitHub API integration
- User persistence or management
- Backward compatibility with existing authentication header

## Decisions

### 1. Per-route Extractor Pattern (Approach A)

**Decision**: Use Axum extractor pattern for per-route authentication rather than Tower middleware layers.

**Rationale**: The existing codebase uses extractors (`AuthenticatedUser`), and this approach provides fine-grained control at the handler level. Each handler explicitly declares its authentication requirement, making the code more explicit and easier to test. Middleware layers would require splitting routers and managing layer composition, adding complexity without clear benefit.

**Alternatives considered**:
- _Tower middleware with router splitting_: Would enforce auth on groups of routes, but requires explicit exclusions for public routes and adds indirection.

### 2. IdentityProvider as Domain Port

**Decision**: Introduce `IdentityProvider` trait in the domain layer as a port, with `validate_token(token: String) -> DomainResult<User>` as the sole method.

**Rationale**: This follows Hexagonal Architecture principles. The domain defines *what* it needs (token validation producing a User) without knowing *how* it's implemented. The infrastructure layer provides the concrete implementation (Vouch Proxy integration), and the application layer orchestrates the call.

**Alternatives considered**:
- _Put trait in application layer_: Would break the architectural separation, coupling domain concerns to HTTP layer.
- _Inline validation in extractor_: Would bypass the domain layer entirely, leaking infrastructure concerns into the application layer.

### 3. VouchIdentityProvider as Infrastructure Adapter

**Decision**: Create new `infrastructure/vouch` crate with `VouchIdentityProvider` struct implementing `IdentityProvider`.

**Rationale**: This isolates Vouch Proxy-specific logic (HTTP client, cookie handling, domain configuration) from the domain and application layers. The adapter is responsible for:
- Calling Vouch Proxy's `/validate` endpoint
- Setting required headers (Host with configured domain, Cookie with token)
- Extracting username from `X-Vouch-User` response header
- Mapping Vouch responses to domain errors

**Configuration**: Three environment variables:
- `VOUCH_VALIDATE_URL`: Full URL (default: `http://vouch-proxy:9090/validate`)
- `VOUCH_COOKIE`: Cookie name (default: `VouchCookie`)
- `VOUCH_DOMAIN`: Domain for Host header (default: `example.com`)

### 4. New Error Variants in DomainError

**Decision**: Add two new error variants to `domain::errors::DomainError`:
- `Authentication(String)`: Token validation failed (401 Unauthorized)
- `PartnerUnavailable(String)`: Vouch Proxy unreachable or unexpected error (502 Bad Gateway)

**Rationale**: These are domain-level concerns that affect business logic. `Authentication` represents a business rejection (invalid identity), while `PartnerUnavailable` represents a technical failure with an external dependency. Keeping these in the domain allows the application layer to handle them generically.

**Mapping to HTTP**:
- `Authentication(_)` → 401 with `{"error": "invalid_identity_token"}`
- `PartnerUnavailable(_)` → 502 with `{"error": "auth_service_unavailable"}`
- `Validation(_)` from User::new → 401 (invalid username from Vouch response)

### 5. Header Name as Internal Constant

**Decision**: Define `X_IDENTITY_TOKEN: &str = "X-Identity-Token"` as a private constant in the api crate (`application/api/src/http/identity/mod.rs` or similar).

**Rationale**: This prevents string duplication across the codebase and ensures consistency. The constant is not exported outside the api crate, maintaining encapsulation.

**Alternatives considered**:
- _Export from domain_: Would leak HTTP concerns into domain layer.
- _Hardcode in multiple places_: Error-prone and violates DRY.

### 6. reqwest as HTTP Client

**Decision**: Use `reqwest` with async support for calling Vouch Proxy's `/validate` endpoint.

**Rationale**: `reqwest` is the most idiomatic, modern, and widely-used HTTP client for Rust with Tokio. It provides:
- Async support out of the box
- Cookie handling (via manual Cookie header)
- Automatic connection pooling
- Good error types
- Broad ecosystem compatibility

### 7. Host Header from VOUCH_DOMAIN

**Decision**: Use a dedicated `VOUCH_DOMAIN` environment variable (default: `example.com`) for the Host header in validation requests, instead of extracting it from the URL.

**Rationale**: This simplifies configuration and makes the domain explicitly configurable. Vouch Proxy requires the Host header to match one of its configured domains, which may differ from the URL's host (e.g., when using a load balancer or proxy).

**Previous approach**: The `get_host()` method attempted to extract host and port from `VOUCH_VALIDATE_URL`, but this was complex and error-prone. Using a dedicated `VOUCH_DOMAIN` variable is clearer and more flexible.

### 8. Async Trait for IdentityProvider

**Decision**: Use `#[async_trait]` macro to make `IdentityProvider` object-safe for use with `Arc<dyn IdentityProvider + Send + Sync>`.

**Rationale**: The `validate_token` method must be async (calls external service), and we need to store the provider in router state as a trait object. The `async_trait` macro generates the necessary glue code for object safety.

## Risks / Trade-offs

**[Latency] Extra network hop per request** → Each authenticated request adds ~1-10ms for the Vouch Proxy `/validate` call. This is acceptable given the security benefits. No caching is implemented per spec requirements.

**[Complexity] More moving parts** → Adds new crate, new dependencies, new error handling paths. Mitigation: Follow existing patterns, keep changes isolated to new modules.

**[Breaking Change] Header name change** → `X-Vouch-User` → `X-Identity-Token`. All clients must update. Mitigation: Coordinate deployment, document in migration notes.

**[Vouch Proxy Dependency] Single point of failure** → If Vouch Proxy is down, all authenticated requests fail with 502. Mitigation: Monitor Vouch Proxy health, ensure high availability.

**[Username Extraction] Assumes X-Vouch-User header** → Relies on Vouch Proxy returning username in `X-Vouch-User` header. Mitigation: This is Vouch Proxy's standard behavior; if it changes, the adapter fails explicitly.

## Migration Plan

**Deployment:**
1. Deploy Vouch Proxy with `/validate` endpoint configured
2. Set `VOUCH_DOMAIN` to match a domain in `vouch.domains` (e.g., `example.com`)
3. Update infrastructure to inject `X-Identity-Token` header instead of `X-Vouch-User`
4. Deploy new API version with vouch adapter and updated extractor
5. Monitor for authentication failures

**Rollback:**
- Revert to previous API version that uses `X-Vouch-User` header
- Revert infrastructure to inject `X-Vouch-User` header
- No database migrations required (no schema changes)

## Open Questions

None - all design decisions have been resolved during exploration phase.
