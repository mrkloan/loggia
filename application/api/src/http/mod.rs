pub mod extractors;
pub mod health;
pub mod me;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use domain::ports::inbound::CheckHealthUseCase;

pub fn router(health_use_case: Arc<dyn CheckHealthUseCase>) -> Router {
    Router::new()
        .route("/health", get(health::handle))
        .route("/me", get(me::handle))
        .with_state(health_use_case)
}
