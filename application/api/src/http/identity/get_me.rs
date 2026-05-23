use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use crate::http::identity::authenticate::AuthenticatedUser;

pub async fn handle(AuthenticatedUser(user): AuthenticatedUser) -> impl IntoResponse {
    (StatusCode::OK, Json(user))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::Router;
    use domain::errors::DomainResult;
    use domain::health::system_health::SystemHealth;
    use domain::health::check_health::CheckHealthUseCase;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    struct MockHealthUseCase;

    #[async_trait::async_trait]
    impl CheckHealthUseCase for MockHealthUseCase {
        async fn execute(&self) -> DomainResult<SystemHealth> {
            Ok(SystemHealth::new("OK".to_string(), true, 0))
        }
    }

    fn test_router() -> Router {
        crate::http::router(Arc::new(MockHealthUseCase))
    }

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
                    .header("X-Vouch-User", "alice")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let json = body_json(response.into_body()).await;
        assert_eq!(json["username"], "alice");
    }

    #[tokio::test]
    async fn get_me_with_trimmed_username_returns_trimmed_value() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header("X-Vouch-User", "  bob  ")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let json = body_json(response.into_body()).await;
        assert_eq!(json["username"], "bob");
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
        assert_eq!(json["error"], "Missing X-Vouch-User header");
    }

    #[tokio::test]
    async fn get_me_with_blank_header_returns_401_with_validation_message() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header("X-Vouch-User", "   ")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let json = body_json(response.into_body()).await;
        assert_eq!(json["error"], "Missing X-Vouch-User header");
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
