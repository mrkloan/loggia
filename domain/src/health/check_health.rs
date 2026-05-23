use std::sync::{Arc, OnceLock};
use std::time::Instant;
use crate::errors::DomainResult;
use crate::health::system_health::SystemHealth;

static START_TIME: OnceLock<Instant> = OnceLock::new();

// Outbound port
#[async_trait::async_trait]
pub trait HealthRepository: Send + Sync {
    async fn check_db_connection(&self) -> DomainResult<bool>;
}

// Inbound port (use case)
#[async_trait::async_trait]
pub trait CheckHealthUseCase: Send + Sync {
    async fn execute(&self) -> DomainResult<SystemHealth>;
}

// Service implementation
pub struct HealthService {
    health_repo: Arc<dyn HealthRepository>,
}

impl HealthService {
    pub fn new(health_repo: Arc<dyn HealthRepository>) -> Self {
        let _ = START_TIME.get_or_init(Instant::now);
        Self { health_repo }
    }
}

#[async_trait::async_trait]
impl CheckHealthUseCase for HealthService {
    async fn execute(&self) -> DomainResult<SystemHealth> {
        let db_connected = self.health_repo.check_db_connection().await.unwrap_or(false);
        let uptime_seconds = START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0);

        let status = if db_connected {
            "OK".to_string()
        } else {
            "DEGRADED (Database Offline)".to_string()
        };

        Ok(SystemHealth::new(status, db_connected, uptime_seconds))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::errors::DomainResult;
    use super::{CheckHealthUseCase, HealthRepository, HealthService};

    struct AlwaysConnectedRepository;

    #[async_trait::async_trait]
    impl HealthRepository for AlwaysConnectedRepository {
        async fn check_db_connection(&self) -> DomainResult<bool> {
            Ok(true)
        }
    }

    struct AlwaysDisconnectedRepository;

    #[async_trait::async_trait]
    impl HealthRepository for AlwaysDisconnectedRepository {
        async fn check_db_connection(&self) -> DomainResult<bool> {
            Ok(false)
        }
    }

    #[tokio::test]
    async fn health_service_returns_ok_when_database_is_connected() {
        let repo = Arc::new(AlwaysConnectedRepository);
        let service = HealthService::new(repo);

        let result = service.execute().await.expect("health check should succeed");

        assert_eq!(result.status, "OK");
        assert!(result.database_connected);
    }

    #[tokio::test]
    async fn health_service_returns_degraded_when_database_is_offline() {
        let repo = Arc::new(AlwaysDisconnectedRepository);
        let service = HealthService::new(repo);

        let result = service.execute().await.expect("health check should succeed");

        assert!(result.status.contains("DEGRADED"));
        assert!(!result.database_connected);
    }
}
