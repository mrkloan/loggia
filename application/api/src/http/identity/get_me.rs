//! Get current authenticated user endpoint.
//!
//! This module contains the HTTP handler for retrieving the current user's identity.

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use crate::http::identity::authenticate::AuthenticatedUser;

/// HTTP handler for the "get me" endpoint.
///
/// Returns the authenticated user's information as JSON.
///
/// # Request
///
/// - Method: GET
/// - Path: /me
/// - Headers: Requires `X-Identity-Token` header for authentication
///
/// # Responses
///
/// - `200 OK`: Authentication succeeded, returns user information as JSON
/// - `401 Unauthorized`: Authentication failed (handled by the `AuthenticatedUser` extractor)
///
/// # Example
///
/// ```bash
/// curl -H "X-Identity-Token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." http://localhost:8080/me
/// ```
///
/// Response:
/// ```json
/// {
///   "username": "alice"
/// }
/// ```
///
/// # Design Rationale
///
/// This handler is intentionally simple because:
/// - Authentication is handled by the `AuthenticatedUser` extractor
/// - The domain `User` entity already guarantees valid data
/// - There's no additional business logic needed - just return what we have
pub async fn handle(AuthenticatedUser(user): AuthenticatedUser) -> impl IntoResponse {
    (StatusCode::OK, Json(user))
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::Router;
    use domain::errors::DomainResult;
    use domain::health::check_health::CheckHealthUseCase;
    use domain::health::system_health::SystemHealth;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    /// Mock implementation of `CheckHealthUseCase` for testing.
    ///
    /// Returns a successful health check result with all systems operational.
    struct MockHealthUseCase;

    #[async_trait]
    impl CheckHealthUseCase for MockHealthUseCase {
        async fn execute(&self) -> DomainResult<SystemHealth> {
            Ok(SystemHealth::new("OK".to_string(), true, 0))
        }
    }

    /// Mock implementation of `IdentityProvider` for testing.
    ///
    /// Returns a mock user for any valid token.
    struct MockIdentityProvider;

    use domain::identity::User;

    #[async_trait]
    impl domain::identity::provider::IdentityProvider for MockIdentityProvider {
        async fn validate_token(&self, _token: String) -> DomainResult<User> {
            // For testing, return a mock user
            User::new("testuser".to_string())
        }
    }

    /// Creates a test router with mock services.
    ///
    /// This is used to test HTTP handlers in isolation without requiring
    /// a real database connection or Vouch Proxy.
    fn test_router() -> Router {
        use std::sync::Arc;
        use crate::AppState;
        use domain::identity::provider::ValidateIdentityUseCase;
        
        let health_use_case = Arc::new(MockHealthUseCase);
        let identity_provider: ValidateIdentityUseCase = Arc::new(MockIdentityProvider);
        let app_state = AppState::new(health_use_case, identity_provider);
        crate::http::router(app_state)
    }

    /// Helper function to extract JSON body from a response.
    ///
    /// Collects the response body and parses it as JSON.
    async fn body_json(body: Body) -> serde_json::Value {
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn get_me_with_valid_header_returns_user() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header("X-Identity-Token", "valid-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let json = body_json(response.into_body()).await;
        assert_eq!(json["username"], "testuser");
    }

    #[tokio::test]
    async fn get_me_without_header_returns_401() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let json = body_json(response.into_body()).await;
        assert_eq!(json["error"], "missing_identity_token");
    }

    #[tokio::test]
    async fn get_me_with_blank_header_returns_401() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header("X-Identity-Token", "   ")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let json = body_json(response.into_body()).await;
        assert_eq!(json["error"], "missing_identity_token");
    }

    #[tokio::test]
    async fn get_health_without_header_is_accessible() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
