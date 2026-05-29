//! Domain error types and result aliases.
//!
//! This module provides a unified error handling mechanism for the domain layer,
//! categorizing errors by their business context rather than technical implementation.

use std::fmt;

/// Represents all possible error types that can occur within domain operations.
///
/// # Variants
///
/// - `Validation`: Errors related to input validation failures (e.g., invalid user input)
/// - `Database`: Errors originating from database operations
/// - `NotFound`: Errors when a requested resource cannot be found
/// - `Internal`: Unexpected internal errors that violate domain invariants
///
/// # Design Rationale
///
/// By categorizing errors by business concern rather than technical source,
/// we maintain clear separation between domain logic and infrastructure concerns.
/// This allows error handling to remain consistent regardless of the underlying
/// persistence or external service implementation.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", content = "message")]
pub enum DomainError {
    /// Input validation failed due to business rule violation.
    Validation(String),
    /// A database operation failed, indicating an infrastructure issue.
    Database(String),
    /// The requested resource or entity was not found.
    NotFound(String),
    /// An unexpected internal error occurred that violates domain assumptions.
    Internal(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::Validation(msg) => write!(f, "Validation error: {}", msg),
            DomainError::Database(msg) => write!(f, "Database error: {}", msg),
            DomainError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DomainError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

/// A convenience type alias for domain operations that can fail.
///
/// This type represents the result of any domain operation, where success
/// contains a value of type `T` and failure contains a `DomainError`.
pub type DomainResult<T> = Result<T, DomainError>;
