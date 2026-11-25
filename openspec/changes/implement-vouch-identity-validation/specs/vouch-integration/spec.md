## ADDED Requirements

### Requirement: VouchIdentityProvider implements IdentityProvider
The system SHALL provide a `VouchIdentityProvider` in the infrastructure layer that implements the `IdentityProvider` port by calling Vouch Proxy's `/validate` endpoint.

#### Scenario: Valid token causes call to Vouch Proxy /validate
- **WHEN** `VouchIdentityProvider::validate_token` is called with a token
- **THEN** the system makes a GET request to the configured Vouch Proxy `/validate` URL
- **AND** the request includes a Cookie header with the token
- **AND** the request includes a Host header matching the configured domain from `VOUCH_DOMAIN`

#### Scenario: Username extracted from X-Vouch-User response header
- **WHEN** Vouch Proxy returns 200 OK with a `X-Vouch-User` header
- **THEN** the system extracts the username from the `X-Vouch-User` header value
- **AND** the system returns `Ok(User)` with that username

#### Scenario: 401 from Vouch Proxy returns Authentication error
- **WHEN** Vouch Proxy returns 401 Unauthorized
- **THEN** the system returns `Err(DomainError::Authentication("invalid_identity_token"))`

#### Scenario: Connection error returns PartnerUnavailable error
- **WHEN** the request to Vouch Proxy fails with a connection error
- **THEN** the system returns `Err(DomainError::PartnerUnavailable("auth_service_unavailable"))`

#### Scenario: Missing X-Vouch-User header from Vouch Proxy returns PartnerUnavailable error
- **WHEN** Vouch Proxy returns 200 OK but without a `X-Vouch-User` header
- **THEN** the system returns `Err(DomainError::PartnerUnavailable("Missing X-Vouch-User header in Vouch response"))`

#### Scenario: Invalid username from X-Vouch-User header returns Validation error
- **WHEN** Vouch Proxy returns 200 OK with a `X-Vouch-User` header containing a blank or whitespace-only username
- **THEN** the system returns `Err(DomainError::Validation("Username cannot be blank"))`

### Requirement: VouchIdentityProvider is configurable via environment variables
The system SHALL allow configuration of `VouchIdentityProvider` through environment variables for the Vouch Proxy endpoint, cookie name, and domain.

#### Scenario: Default VOUCH_VALIDATE_URL is used
- **WHEN** `VOUCH_VALIDATE_URL` environment variable is not set
- **THEN** the system defaults to `http://vouch-proxy:9090/validate`

#### Scenario: Custom VOUCH_VALIDATE_URL is used
- **WHEN** `VOUCH_VALIDATE_URL` environment variable is set to a custom URL
- **THEN** the system uses that URL for validation requests

#### Scenario: Default VOUCH_COOKIE is used
- **WHEN** `VOUCH_COOKIE` environment variable is not set
- **THEN** the system defaults to `VouchCookie`

#### Scenario: Custom VOUCH_COOKIE is used
- **WHEN** `VOUCH_COOKIE` environment variable is set to a custom cookie name
- **THEN** the system uses that cookie name in the validation request

#### Scenario: Default VOUCH_DOMAIN is used
- **WHEN** `VOUCH_DOMAIN` environment variable is not set
- **THEN** the system defaults to `example.com`

#### Scenario: Custom VOUCH_DOMAIN is used
- **WHEN** `VOUCH_DOMAIN` environment variable is set to a custom domain
- **THEN** the system uses that domain in the Host header of validation requests
