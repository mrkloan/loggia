pub mod health_repository;

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

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
