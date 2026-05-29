//! SQLite implementation of the HealthRepository port.
//!
//! This module contains the concrete SQLite adapter that implements the
//! `HealthRepository` trait from the domain layer.

use std::sync::Arc;
use sqlx::SqlitePool;
use domain::errors::{DomainError, DomainResult};
use domain::health::check_health::HealthRepository;

/// SQLite implementation of the domain's `HealthRepository` port.
///
/// This struct is an **adapter** in Hexagonal Architecture. It provides
/// the concrete SQLite implementation for checking database connectivity,
/// as defined by the domain layer's `HealthRepository` trait.
///
/// # Dependencies
///
/// - `SqlitePool`: A thread-safe connection pool to a SQLite database
///
/// # Design Rationale
///
/// By implementing the domain's `HealthRepository` trait, this adapter:
/// - Can be used anywhere the domain expects a `HealthRepository`
/// - Can be swapped for a different database implementation without changing domain code
/// - Maintains the separation between domain logic and infrastructure concerns
pub struct SqliteHealthRepository {
    /// Thread-safe connection pool to the SQLite database.
    pool: Arc<SqlitePool>,
}

impl SqliteHealthRepository {
    /// Creates a new `SqliteHealthRepository` with the given connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - A thread-safe SQLite connection pool
    ///
    /// # Returns
    ///
    /// A new repository instance ready to check database connectivity.
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

/// Implementation of the `HealthRepository` trait for SQLite.
///
/// This implementation checks database connectivity by executing a simple
/// `SELECT 1` query. If the query succeeds, the database is considered connected.
///
/// # Error Handling
///
/// Any SQLite errors are converted to `DomainError::Database` to maintain
/// the domain's error abstraction.
#[async_trait::async_trait]
impl HealthRepository for SqliteHealthRepository {
    /// Checks if the SQLite database is connected and responding.
    ///
    /// Executes a simple `SELECT 1` query against the database. This is the
    /// standard way to test database connectivity in SQL-based systems.
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if the query executes successfully
    /// - `Ok(false)` is not returned; any error results in `Err`
    /// - `Err(DomainError::Database)` if the query fails
    async fn check_db_connection(&self) -> DomainResult<bool> {
        sqlx::query("SELECT 1")
            .execute(self.pool.as_ref())
            .await
            .map(|_| true)
            .map_err(|e| DomainError::Database(e.to_string()))
    }
}
