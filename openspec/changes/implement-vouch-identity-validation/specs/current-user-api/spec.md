## MODIFIED Requirements

### Requirement: GET /me returns the authenticated user
The system SHALL expose a `GET /me` endpoint that returns the current user's identity as a JSON-serialized `User` entity. The endpoint SHALL require a valid `X-Identity-Token` header for authentication.

#### Scenario: Authenticated request to GET /me
- **WHEN** a `GET /me` request is made with a valid `X-Identity-Token` header
- **THEN** the system validates the token via `IdentityProvider`
- **AND** returns `200 OK` with body `{ "username": "<username>" }`

#### Scenario: Unauthenticated request to GET /me
- **WHEN** a `GET /me` request is made without the `X-Identity-Token` header
- **THEN** the system returns `401 Unauthorized` with body `{ "error": "missing_identity_token" }`

#### Scenario: Invalid token request to GET /me
- **WHEN** a `GET /me` request is made with an invalid `X-Identity-Token` header
- **THEN** the system returns `401 Unauthorized` with body `{ "error": "invalid_identity_token" }`

## REMOVED Requirements

### Requirement: GET /me returns the authenticated user
**Reason**: Replaced with updated requirement using X-Identity-Token header
**Migration**: Clients must use `X-Identity-Token` header instead of `X-Vouch-User`
