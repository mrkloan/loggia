## Why

The current authentication mechanism trusts the `X-Vouch-User` header injected by an upstream proxy without direct validation. This creates a security gap where internal network requests cannot be verified, and the API has no direct assurance that tokens are valid. By implementing direct validation against Vouch Proxy's `/validate` endpoint, we enforce identity verification at the API layer for all requests, ensuring end-to-end security even on internal networks.

## What Changes

- **BREAKING**: Replace `X-Vouch-User` header with `X-Identity-Token` header for authentication
- Add `IdentityProvider` port to domain layer defining token validation contract
- Create new `vouch` infrastructure adapter implementing identity validation via Vouch Proxy `/validate` endpoint
- Replace header-presence extractor with full token validation extractor
- Add `Authentication` and `PartnerUnavailable` error variants to `DomainError`
- Remove legacy `X-Vouch-User` header support
- Add `reqwest` and `url` dependencies to workspace

## Capabilities

### New Capabilities
- `identity-validation`: Contract for token validation via external identity provider
- `vouch-integration`: Vouch Proxy adapter for identity validation

### Modified Capabilities
- `user-identity`: **BREAKING** - Change authentication header from `X-Vouch-User` to `X-Identity-Token`; add token validation requirement; update error responses
- `current-user-api`: **BREAKING** - Header requirement changes from `X-Vouch-User` to `X-Identity-Token`

## Impact

- `domain`: New `IdentityProvider` trait in `identity/provider.rs`; new error variants in `errors.rs`
- `infrastructure`: New `vouch` crate with `VouchIdentityProvider` implementation
- `application/api`: New extractor in `http/identity/authenticate.rs`; updated router and state management; new constant `X_IDENTITY_TOKEN`
- `workspace`: New crate member `infrastructure/vouch`; new dependencies `reqwest`, `url`
- All protected endpoints now validate tokens via Vouch Proxy (except `/health`)
