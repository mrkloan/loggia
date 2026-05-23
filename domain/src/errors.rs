use std::fmt;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", content = "message")]
pub enum DomainError {
    Validation(String),
    Database(String),
    NotFound(String),
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

pub type DomainResult<T> = Result<T, DomainError>;
