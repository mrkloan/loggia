pub mod health;
pub mod identity;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use domain::health::check_health::CheckHealthUseCase;

pub fn router(health_use_case: Arc<dyn CheckHealthUseCase>) -> Router {
    Router::new()
        .route("/health", get(health::check_health::handle))
        .route("/me", get(identity::get_me::handle))
        .with_state(health_use_case)
}
