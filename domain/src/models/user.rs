use crate::errors::{DomainError, DomainResult};

#[derive(Debug, Clone, serde::Serialize)]
pub struct User {
    username: String,
}

impl User {
    pub fn new(username: String) -> DomainResult<Self> {
        let username = username.trim().to_string();
        if username.is_empty() {
            return Err(DomainError::Validation(
                "Username cannot be blank".to_string(),
            ));
        }
        Ok(Self { username })
    }

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
