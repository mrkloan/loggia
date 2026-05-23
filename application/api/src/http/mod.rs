pub mod health;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use domain::ports::inbound::CheckHealthUseCase;

pub fn router(health_use_case: Arc<dyn CheckHealthUseCase>) -> Router {
    Router::new()
        .route("/health", get(health::handle))
        .with_state(health_use_case)
}
