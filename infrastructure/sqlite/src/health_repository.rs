use std::sync::Arc;
use sqlx::SqlitePool;
use domain::errors::{DomainError, DomainResult};
use domain::ports::outbound::HealthRepository;

pub struct SqliteHealthRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteHealthRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl HealthRepository for SqliteHealthRepository {
    async fn check_db_connection(&self) -> DomainResult<bool> {
        sqlx::query("SELECT 1")
            .execute(self.pool.as_ref())
            .await
            .map(|_| true)
            .map_err(|e| DomainError::Database(e.to_string()))
    }
}
