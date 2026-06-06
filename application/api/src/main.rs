//! # API Application Layer
//!
//! This crate is the **application layer** (driving adapter) in Hexagonal Architecture.
//! It orchestrates the interaction between external drivers (HTTP) and the domain layer.
//!
//! ## Architecture Role
//!
//! The application layer:
//! - Receives external requests (HTTP, CLI, messages)
//! - Coordinates use cases from the domain layer
//! - Delegates to infrastructure adapters for persistence and external services
//! - Translates between external formats (HTTP JSON) and domain types
//!
//! ## Dependencies
//!
//! - `domain`: For business logic and use cases
//! - `sqlite`: For SQLite persistence adapter
//! - `axum`: For HTTP server and routing
//! - `tokio`: For async runtime
//! - `tracing`: For observability and logging

#![deny(missing_docs)]

mod http;

use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use domain::health::check_health::HealthService;
use domain::identity::provider::IdentityProviderRef;
use sqlite::health::SqliteHealthRepository;
use vouch::VouchIdentityProvider;

/// Application state containing all services and providers needed by the API.
///
/// This struct is used as the router state and provides access to:
/// - Health check use case
/// - Identity provider for token validation
#[derive(Clone)]
pub struct AppState {
    /// The health check use case from the domain layer.
    pub health_use_case: Arc<dyn domain::health::check_health::CheckHealthUseCase>,
    /// The identity provider for validating identity tokens.
    pub identity_provider: IdentityProviderRef,
}

impl AppState {
    /// Creates a new `AppState` with the given services.
    pub fn new(
        health_use_case: Arc<dyn domain::health::check_health::CheckHealthUseCase>,
        identity_provider: IdentityProviderRef,
    ) -> Self {
        Self {
            health_use_case,
            identity_provider,
        }
    }
}

/// Application entry point.
///
/// This function:
/// 1. Initializes the tracing/observability subsystem
/// 2. Establishes the database connection pool
/// 3. Wires up the domain services with infrastructure adapters
/// 4. Configures the HTTP router
/// 5. Starts the HTTP server
///
/// # Environment Variables
///
/// - `DATABASE_URL`: SQLite database URL (default: "sqlite:loggia.db")
/// - `RUST_LOG`: Logging level (controlled by tracing-subscriber)
///
/// # Design Rationale
///
/// The main function follows the **dependency injection** pattern:
/// - Domain services depend on traits (ports), not concrete implementations
/// - Infrastructure adapters implement those traits
/// - The composition root (this function) wires everything together
///
/// This ensures that:
/// - Domain logic remains independent of infrastructure
/// - Testing can use mock implementations
/// - Configuration changes don't require code changes
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

    // Create identity provider
    let identity_provider: IdentityProviderRef = Arc::new(
        VouchIdentityProvider::new()
            .expect("Failed to create VouchIdentityProvider: invalid configuration"),
    );

    // Create application state
    let app_state = AppState::new(health_service, identity_provider);

    let app = http::router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind TCP listener");

    tracing::info!("API server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
