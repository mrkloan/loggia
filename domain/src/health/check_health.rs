//! Health check use case implementation.
//!
//! This module contains the core domain logic for checking system health.
//! It defines the ports (interfaces) and the service that implements the use case.

use std::sync::{Arc, OnceLock};
use std::time::Instant;
use crate::errors::DomainResult;
use crate::health::system_health::SystemHealth;

/// Tracks when the service started for uptime calculation.
///
/// This is initialized on first access and remains constant for the lifetime of the process.
/// It provides a simple, thread-safe way to calculate service uptime.
static START_TIME: OnceLock<Instant> = OnceLock::new();

/// **Outbound Port**: Defines the contract for database connectivity checking.
///
/// This trait is an **outbound port** in Hexagonal Architecture terminology.
/// It defines how the domain layer interacts with external systems (infrastructure)
/// to verify database connectivity.
///
/// # Contract
///
/// Implementations must provide a method that checks if the database is reachable
/// and returns a boolean result wrapped in a `DomainResult`.
///
/// # Design Rationale
///
/// By defining this as a trait (interface) in the domain layer, we ensure that:
/// - The domain layer remains independent of any specific database technology
/// - Database connectivity checking can be tested with mock implementations
/// - Different persistence adapters (SQLite, PostgreSQL, etc.) can be used interchangeably
#[async_trait::async_trait]
pub trait HealthRepository: Send + Sync {
    /// Checks if the database connection is active and functional.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the database is reachable and responding.
    /// `Ok(false)` if the database is not responding.
    /// `Err(DomainError)` if there was an error during the check itself.
    async fn check_db_connection(&self) -> DomainResult<bool>;
}

/// **Inbound Port**: Defines the use case for checking system health.
///
/// This trait is an **inbound port** in Hexagonal Architecture terminology.
/// It represents a use case (business operation) that can be invoked by external drivers
/// such as HTTP handlers, CLI commands, or scheduled jobs.
///
/// # Contract
///
/// Implementations must provide an `execute` method that returns the current
/// system health status as a `SystemHealth` value object.
///
/// # Design Rationale
///
/// Separating the use case interface from its implementation allows:
/// - Multiple implementations (e.g., cached, real-time, mock)
/// - Easy testing through dependency injection
/// - Clear separation between "what" the use case does and "how" it does it
#[async_trait::async_trait]
pub trait CheckHealthUseCase: Send + Sync {
    /// Executes the health check use case.
    ///
    /// This method coordinates the health check operation by:
    /// 1. Checking database connectivity via the outbound `HealthRepository` port
    /// 2. Calculating service uptime
    /// 3. Determining the overall system status
    /// 4. Returning a complete `SystemHealth` value object
    ///
    /// # Returns
    ///
    /// A `DomainResult` containing a `SystemHealth` instance with the current system state.
    async fn execute(&self) -> DomainResult<SystemHealth>;
}

/// **Service**: The default implementation of the `CheckHealthUseCase` inbound port.
///
/// This struct implements the health check use case by coordinating between
/// the outbound `HealthRepository` port and the domain's `SystemHealth` value object.
///
/// # Dependencies
///
/// - `HealthRepository`: Used to check database connectivity (outbound port)
///
/// # Usage
///
/// ```rust,ignore
/// let health_repo = Arc::new(SqliteHealthRepository::new(pool));
/// let health_service = Arc::new(HealthService::new(health_repo));
/// let health = health_service.execute().await?;
/// ```
pub struct HealthService {
    /// The outbound port for checking database connectivity.
    health_repo: Arc<dyn HealthRepository>,
}

impl HealthService {
    /// Creates a new `HealthService` instance.
    ///
    /// # Arguments
    ///
    /// * `health_repo` - An implementation of the `HealthRepository` outbound port
    ///
    /// # Returns
    ///
    /// A new `HealthService` ready to execute health checks.
    ///
    /// # Side Effects
    ///
    /// Initializes the `START_TIME` static variable on first creation,
    /// which is used for uptime calculation.
    pub fn new(health_repo: Arc<dyn HealthRepository>) -> Self {
        let _ = START_TIME.get_or_init(Instant::now);
        Self { health_repo }
    }
}

/// Implementation of `CheckHealthUseCase` for `HealthService`.
///
/// This implementation:
/// 1. Delegates to the `HealthRepository` to check database connectivity
/// 2. Retrieves the service start time to calculate uptime
/// 3. Determines the overall system status based on database connectivity
/// 4. Constructs and returns a `SystemHealth` value object
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
