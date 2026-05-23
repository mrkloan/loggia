use std::sync::{Arc, OnceLock};
use std::time::Instant;
use crate::errors::DomainResult;
use crate::models::system_health::SystemHealth;
use crate::ports::inbound::CheckHealthUseCase;
use crate::ports::outbound::HealthRepository;

static START_TIME: OnceLock<Instant> = OnceLock::new();

pub struct HealthService {
    health_repo: Arc<dyn HealthRepository>,
}

impl HealthService {
    pub fn new(health_repo: Arc<dyn HealthRepository>) -> Self {
        // Initialize start time on first creation
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
