//! User identity domain module.
//!
//! This module contains the domain logic for user identity and authentication.
//! It defines the `User` entity and its business rules.

use crate::errors::{DomainError, DomainResult};

/// Represents a user in the system.
///
/// This is an **entity** in the domain layer, meaning it has a unique identity
/// (the username) and encapsulates business rules about user validation.
///
/// # Business Rules
///
/// - A username must not be blank or whitespace-only
/// - Usernames are automatically trimmed of leading/trailing whitespace
/// - A `User` instance, once created, guarantees these invariants are satisfied
///
/// # Design Rationale
///
/// By encapsulating username validation in the `new` constructor, we ensure
/// that every `User` instance in the system is valid by construction.
/// This follows the domain-driven design principle of making invalid states
/// unrepresentable.
#[derive(Debug, Clone, serde::Serialize)]
pub struct User {
    /// The user's username, guaranteed to be non-empty and trimmed.
    username: String,
}

impl User {
    /// Creates a new `User` entity with the given username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username string to validate and use
    ///
    /// # Returns
    ///
    /// - `Ok(User)` if the username is valid (non-empty after trimming)
    /// - `Err(DomainError::Validation)` if the username is blank or whitespace-only
    ///
    /// # Business Logic
    ///
    /// This constructor enforces the business rule that usernames must not be blank.
    /// It automatically trims leading and trailing whitespace from the input.
    pub fn new(username: String) -> DomainResult<Self> {
        let username = username.trim().to_string();
        if username.is_empty() {
            return Err(DomainError::Validation(
                "Username cannot be blank".to_string(),
            ));
        }
        Ok(Self { username })
    }

    /// Returns a reference to the user's username.
    ///
    /// # Returns
    ///
    /// The username as a string slice, guaranteed to be non-empty.
    pub fn username(&self) -> &str {
        &self.username
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_username_is_accepted() {
        let user = User::new("alice".to_string()).unwrap();
        assert_eq!(user.username(), "alice");
    }

    #[test]
    fn username_is_trimmed() {
        let user = User::new("  alice  ".to_string()).unwrap();
        assert_eq!(user.username(), "alice");
    }

    #[test]
    fn empty_username_is_rejected() {
        let err = User::new("".to_string()).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn whitespace_only_username_is_rejected() {
        let err = User::new("   ".to_string()).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }
}
