use crate::errors::DomainResult;
use crate::models::system_health::SystemHealth;

#[async_trait::async_trait]
pub trait CheckHealthUseCase: Send + Sync {
    async fn execute(&self) -> DomainResult<SystemHealth>;
}
