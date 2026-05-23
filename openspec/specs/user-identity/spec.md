### Requirement: User is a transient domain entity
The system SHALL represent an authenticated caller as a `User` domain model containing a single `username` attribute. The `User` SHALL NOT be persisted to any database.

#### Scenario: User is constructed with a valid username
- **WHEN** `User::new` is called with a non-blank string
- **THEN** a `User` is returned with the trimmed username value

### Requirement: Username invariant — non-blank
The system SHALL reject construction of a `User` when the provided username is blank or consists only of whitespace, returning a `DomainError::Validation`.

#### Scenario: Empty username is rejected
- **WHEN** `User::new` is called with an empty string
- **THEN** a `DomainError::Validation` error is returned

#### Scenario: Whitespace-only username is rejected
- **WHEN** `User::new` is called with a string containing only whitespace characters
- **THEN** a `DomainError::Validation` error is returned

#### Scenario: Username is trimmed before validation
- **WHEN** `User::new` is called with a string that has leading or trailing whitespace but a non-blank core
- **THEN** a `User` is returned with the whitespace stripped from the username

### Requirement: API requests require the X-Vouch-User header
All protected API endpoints SHALL require the `X-Vouch-User` HTTP request header to be present and valid.

#### Scenario: Request missing the X-Vouch-User header
- **WHEN** a request is made to a protected endpoint without the `X-Vouch-User` header
- **THEN** the system returns `401 Unauthorized` with body `{ "error": "Missing X-Vouch-User header" }`

#### Scenario: Request with a blank X-Vouch-User header value
- **WHEN** a request is made to a protected endpoint with an `X-Vouch-User` header whose value is blank or whitespace-only
- **THEN** the system returns `401 Unauthorized` with body `{ "error": "Missing X-Vouch-User header" }`

#### Scenario: Request with a valid X-Vouch-User header
- **WHEN** a request is made to a protected endpoint with a non-blank `X-Vouch-User` header value
- **THEN** the request proceeds normally and the user identity is available to the handler

### Requirement: Health endpoint is exempt from authentication
The `GET /health` endpoint SHALL remain publicly accessible without requiring the `X-Vouch-User` header.

#### Scenario: Health check without authentication header
- **WHEN** a request is made to `GET /health` without the `X-Vouch-User` header
- **THEN** the system responds normally (not with 401 or 403)
