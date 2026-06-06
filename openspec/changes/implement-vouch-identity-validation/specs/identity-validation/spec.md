## ADDED Requirements

### Requirement: IdentityProvider port for token validation
The system SHALL provide an `IdentityProvider` port in the domain layer with a `validate_token` method that accepts a token string and returns a `DomainResult<User>`.

#### Scenario: Valid token returns User
- **WHEN** `IdentityProvider::validate_token` is called with a valid identity token
- **THEN** the system returns `Ok(User)` with the username extracted from the validated token

#### Scenario: Invalid token returns Authentication error
- **WHEN** `IdentityProvider::validate_token` is called with an invalid or expired token
- **THEN** the system returns `Err(DomainError::Authentication("invalid_identity_token"))`

#### Scenario: Validation service unavailable returns PartnerUnavailable error
- **WHEN** `IdentityProvider::validate_token` is called but the validation service is unreachable
- **THEN** the system returns `Err(DomainError::PartnerUnavailable("auth_service_unavailable"))`

### Requirement: IdentityProvider must be object-safe
The system SHALL allow `IdentityProvider` implementations to be stored as `Arc<dyn IdentityProvider + Send + Sync>` for use in async contexts.

#### Scenario: Trait object can be created and used
- **WHEN** an `IdentityProvider` implementation is created and stored as `Arc<dyn IdentityProvider + Send + Sync>`
- **THEN** the `validate_token` method can be called through the trait object

### Requirement: DomainError includes Authentication and PartnerUnavailable variants
The system SHALL extend `DomainError` with `Authentication(String)` and `PartnerUnavailable(String)` variants for identity validation failures.

#### Scenario: Authentication error can be constructed
- **WHEN** `DomainError::Authentication` is constructed with an error message
- **THEN** the error can be pattern-matched and the message retrieved

#### Scenario: PartnerUnavailable error can be constructed
- **WHEN** `DomainError::PartnerUnavailable` is constructed with an error message
- **THEN** the error can be pattern-matched and the message retrieved
