pub mod health_service;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::errors::DomainResult;
    use crate::ports::inbound::CheckHealthUseCase;
    use crate::ports::outbound::HealthRepository;
    use super::health_service::HealthService;

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
