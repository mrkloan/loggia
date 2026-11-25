//! Identity provider port for token validation.
//!
//! This module defines the `IdentityProvider` trait, a port in the Hexagonal
//! Architecture that abstracts identity token validation from the domain layer.
//!
//! # Design Rationale
//!
//! The domain layer defines *what* it needs (token validation producing a User)
//! without knowing *how* it's implemented. This allows the infrastructure layer
//! to provide concrete implementations (e.g., Vouch Proxy integration) while
//! the domain remains independent of external services.

use async_trait::async_trait;
use std::sync::Arc;

use crate::errors::DomainResult;
use crate::identity::User;

/// Port for identity token validation.
///
/// Implementations of this trait validate identity tokens and return a
/// domain `User` entity on success, or a domain error on failure.
///
/// # Usage
///
/// ```rust,ignore
/// struct MyIdentityProvider;
///
/// #[async_trait]
/// impl IdentityProvider for MyIdentityProvider {
///     async fn validate_token(&self, token: String) -> DomainResult<User> {
///         // Validate token, return User or error
///     }
/// }
/// ```
#[async_trait]
pub trait IdentityProvider: Send + Sync {
    /// Validates an identity token and returns the corresponding user.
    ///
    /// # Arguments
    ///
    /// * `token` - The identity token string to validate
    ///
    /// # Returns
    ///
    /// - `Ok(User)` if the token is valid and a user can be constructed
    /// - `Err(DomainError::Authentication)` if the token is invalid or expired
    /// - `Err(DomainError::PartnerUnavailable)` if the validation service is unreachable
    /// - `Err(DomainError::Validation)` if the token is valid but user data is invalid
    async fn validate_token(&self, token: String) -> DomainResult<User>;
}

/// Type alias for a boxed, thread-safe identity provider.
///
/// This type is used for storing providers in application state.
pub type ValidateIdentityUseCase = Arc<dyn IdentityProvider + Send + Sync>;
