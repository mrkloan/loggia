use crate::errors::DomainResult;

#[async_trait::async_trait]
pub trait HealthRepository: Send + Sync {
    async fn check_db_connection(&self) -> DomainResult<bool>;
}
