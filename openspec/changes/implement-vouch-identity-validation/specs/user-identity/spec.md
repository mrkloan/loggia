## MODIFIED Requirements

### Requirement: API requests require the X-Identity-Token header
All protected API endpoints SHALL require the `X-Identity-Token` HTTP request header to be present and valid. The token SHALL be validated via the `IdentityProvider` port before the request is processed.

#### Scenario: Request missing the X-Identity-Token header
- **WHEN** a request is made to a protected endpoint without the `X-Identity-Token` header
- **THEN** the system returns `401 Unauthorized` with body `{ "error": "missing_identity_token" }`

#### Scenario: Request with a blank X-Identity-Token header value
- **WHEN** a request is made to a protected endpoint with an `X-Identity-Token` header whose value is blank or whitespace-only
- **THEN** the system returns `401 Unauthorized` with body `{ "error": "missing_identity_token" }`

#### Scenario: Request with an invalid X-Identity-Token header
- **WHEN** a request is made to a protected endpoint with an `X-Identity-Token` header containing an invalid or expired token
- **THEN** the system calls `IdentityProvider::validate_token` with the token
- **AND** returns `401 Unauthorized` with body `{ "error": "invalid_identity_token" }`

#### Scenario: Request with a valid X-Identity-Token header
- **WHEN** a request is made to a protected endpoint with a valid `X-Identity-Token` header value
- **THEN** the system calls `IdentityProvider::validate_token` with the token
- **AND** the request proceeds normally with the validated user identity available to the handler

### Requirement: Health endpoint is exempt from authentication
The `GET /health` endpoint SHALL remain publicly accessible without requiring the `X-Identity-Token` header.

#### Scenario: Health check without authentication header
- **WHEN** a request is made to `GET /health` without the `X-Identity-Token` header
- **THEN** the system responds normally with the health check result (not with 401 or 403)

## REMOVED Requirements

### Requirement: API requests require the X-Vouch-User header
**Reason**: Replaced by X-Identity-Token header with active validation
**Migration**: Clients must send `X-Identity-Token` header instead of `X-Vouch-User`

### Requirement: User is constructed with a valid username
**Reason**: Moved to identity-validation spec as part of the IdentityProvider contract
**Migration**: User construction now happens inside IdentityProvider implementation
