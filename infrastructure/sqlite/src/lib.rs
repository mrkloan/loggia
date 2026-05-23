pub mod health_repository;

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn establish_connection_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    tracing::info!("Database connection pool established and migrations applied");
    Ok(pool)
}
