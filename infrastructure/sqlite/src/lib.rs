//! # SQLite Infrastructure Adapter
//!
//! This crate provides the SQLite-specific implementation of domain layer ports.
//! It is an **adapter** in Hexagonal Architecture terminology, implementing
//! the interfaces defined by the domain layer.
//!
//! ## Architecture Role
//!
//! This is part of the **infrastructure layer** (outer layer) that provides
//! concrete implementations for the domain's abstraction. The domain layer
//! depends on this crate, but not vice versa - maintaining proper dependency direction.
//!
//! ## Modules
//!
//! - `health`: SQLite implementation of the `HealthRepository` port
//!
//! ## Usage
//!
//! ```rust,ignore
//! use sqlite::establish_connection_pool;
//!
//! let pool = establish_connection_pool("sqlite:loggia.db").await?;
//! let repo = Arc::new(SqliteHealthRepository::new(pool));
//! ```

#![deny(missing_docs)]

pub mod health;

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

/// Establishes a connection pool to a SQLite database and runs migrations.
///
/// This is the entry point for the SQLite infrastructure adapter. It:
/// 1. Parses the database URL
/// 2. Creates the database file if it doesn't exist
/// 3. Establishes a connection pool with configured limits
/// 4. Runs all pending database migrations
/// 5. Returns the configured pool
///
/// # Arguments
///
/// * `database_url` - The SQLite connection URL (e.g., "sqlite:loggia.db")
///
/// # Returns
///
/// - `Ok(SqlitePool)` on successful connection and migration
/// - `Err(sqlx::Error)` if connection or migration fails
///
/// # Design Rationale
///
/// Centralizing connection pool creation here ensures:
/// - Consistent pool configuration across the application
/// - Migrations are always run before the pool is used
/// - Connection errors are handled uniformly
///
/// The pool is configured with a maximum of 5 connections, which is appropriate
/// for a typical SQLite use case (single file, limited concurrency).
pub async fn establish_connection_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let connect_options = database_url
        .parse::<sqlx::sqlite::SqliteConnectOptions>()?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    tracing::info!("Database connection pool established and migrations applied");
    Ok(pool)
}
