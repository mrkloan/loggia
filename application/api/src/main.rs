mod http;

use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use domain::health::check_health::HealthService;
use sqlite::health::SqliteHealthRepository;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:loggia.db".to_string());

    let pool = sqlite::establish_connection_pool(&database_url)
        .await
        .expect("Failed to establish database connection pool");

    let pool = Arc::new(pool);
    let health_repo = Arc::new(SqliteHealthRepository::new(pool));
    let health_service = Arc::new(HealthService::new(health_repo));

    let app = http::router(health_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind TCP listener");

    tracing::info!("API server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
