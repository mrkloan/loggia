### Requirement: GET /me returns the authenticated user
The system SHALL expose a `GET /me` endpoint that returns the current user's identity as a JSON-serialized `User` entity.

#### Scenario: Authenticated request to GET /me
- **WHEN** a `GET /me` request is made with a valid `X-Vouch-User` header value `"alice"`
- **THEN** the system returns `200 OK` with body `{ "username": "alice" }`

#### Scenario: Unauthenticated request to GET /me
- **WHEN** a `GET /me` request is made without the `X-Vouch-User` header
- **THEN** the system returns `401 Unauthorized` (as per user-identity authentication requirement)
