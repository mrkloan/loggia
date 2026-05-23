use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use domain::ports::inbound::CheckHealthUseCase;

pub async fn handle(
    State(use_case): State<Arc<dyn CheckHealthUseCase>>,
) -> impl IntoResponse {
    match use_case.execute().await {
        Ok(health) => (StatusCode::OK, Json(health)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
