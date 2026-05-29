//! Health check HTTP handler.
//!
//! This module contains the HTTP handler for the health check endpoint.

use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use domain::health::check_health::CheckHealthUseCase;

/// HTTP handler for the health check endpoint.
///
/// This handler:
/// 1. Extracts the `CheckHealthUseCase` from the request state
/// 2. Executes the health check use case
/// 3. Transforms the domain result into an HTTP response
///
/// # Request
///
/// - Method: GET
/// - Path: /health
/// - State: Requires `CheckHealthUseCase` in Axum state
///
/// # Responses
///
/// - `200 OK`: Health check succeeded, returns `SystemHealth` as JSON
/// - `500 Internal Server Error`: Health check failed, returns error details as JSON
///
/// # Example
///
/// ```bash
/// curl http://localhost:8080/health
/// ```
///
/// Response:
/// ```json
/// {
///   "status": "OK",
///   "database_connected": true,
///   "uptime_seconds": 123
/// }
/// ```
///
/// # Design Rationale
///
/// This handler demonstrates the **Hexagonal Architecture** pattern:
/// - It depends on the domain's `CheckHealthUseCase` trait, not a concrete implementation
/// - It translates between HTTP concerns (status codes, JSON) and domain types
/// - It handles errors gracefully, converting domain errors to appropriate HTTP responses
/// - It has no knowledge of how the health check is implemented (SQLite, in-memory, etc.)
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
