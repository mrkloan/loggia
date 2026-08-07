//! Authentication extractor for Axum.
//!
//! This module provides the `AuthenticatedUser` extractor that validates
//! identity tokens from HTTP request headers using the `IdentityProvider` port.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::AppState;
use crate::http::identity::X_IDENTITY_TOKEN;
use domain::errors::DomainError;
use domain::identity::User;

/// A wrapper around a domain `User` that has been successfully authenticated.
///
/// This type is used as an extractor in Axum handlers to ensure that
/// only authenticated requests can reach the handler.
///
/// # Usage
///
/// ```rust,ignore
/// async fn my_handler(
///     State(app_state): State<AppState>,
///     AuthenticatedUser(user): AuthenticatedUser,
/// ) -> impl IntoResponse {
///     // user is guaranteed to be valid
///     (StatusCode::OK, Json(user))
/// }
/// ```
///
/// **Note:** This extractor requires `State<AppState>` to be extracted first,
/// as it needs access to the identity provider from the application state.
pub struct AuthenticatedUser(pub User);

/// Helper function to create a 401 Unauthorized response with a JSON body.
fn unauthorized_response(error: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": error })),
    )
        .into_response()
}

/// Helper function to create a 502 Bad Gateway response with a JSON body.
fn bad_gateway_response(error: &str) -> Response {
    (
        StatusCode::BAD_GATEWAY,
        Json(json!({ "error": error })),
    )
        .into_response()
}

/// Implementation of `FromRequestParts` for `AuthenticatedUser`.
///
/// This extractor performs authentication by:
/// 1. Extracting the `X-Identity-Token` header from the request
/// 2. Validating that the header exists and contains a valid UTF-8 string
/// 3. Calling the `IdentityProvider` to validate the token
/// 4. Returning the authenticated user or an error response
///
/// # Authentication Mechanism
///
/// The authentication uses the `IdentityProvider` port to validate tokens.
/// The API calls Vouch Proxy's `/validate` endpoint (via the infrastructure adapter)
/// to confirm the token's validity before processing any request.
///
/// # Design Rationale
///
/// Delegating token validation to an external provider:
/// - Enforces end-to-end security (API validates all tokens)
/// - Works for both public and internal request paths
/// - Keeps the application stateless
/// - Makes the application easier to test (provider can be mocked)
///
/// **Note:** This extractor must be used in handlers that also extract `State<AppState>`.
/// The AppState must be extracted first so that the extractor can access it.
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync + 'static,
{
    type Rejection = Response;

    /// Extracts and validates the `X-Identity-Token` header from the request.
    ///
    /// # Arguments
    ///
    /// * `parts` - The request parts (headers, method, URI, etc.)
    /// * `state` - The router state (AppState in our case)
    ///
    /// # Returns
    ///
    /// - `Ok(AuthenticatedUser)` if authentication succeeds
    /// - `Err(Response)` with `401 Unauthorized` if authentication fails
    ///
    /// # Failure Cases
    ///
    /// Returns `401 Unauthorized` if:
    /// - The `X-Identity-Token` header is missing
    /// - The token is invalid or expired (from IdentityProvider)
    /// - The username extracted from Vouch Proxy is invalid
    ///
    /// Returns `502 Bad Gateway` if:
    /// - Vouch Proxy is unreachable (PartnerUnavailable error)
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the header
        let token = parts
            .headers
            .get(X_IDENTITY_TOKEN)
            .ok_or_else(|| unauthorized_response("missing_identity_token"))?
            .to_str()
            .map_err(|_| unauthorized_response("missing_identity_token"))?
            .to_string();

        // Check if token is blank
        if token.trim().is_empty() {
            return Err(unauthorized_response("missing_identity_token"));
        }

        // Try to get AppState from the state parameter
        // This works when the router's state is AppState
        // We use Any to attempt downcasting
        use std::any::Any;
        
        let identity_provider = if let Some(app_state) = (state as &dyn Any).downcast_ref::<AppState>() {
            &app_state.identity_provider
        } else {
            // If downcast fails, try to get from extensions
            // This shouldn't happen in normal usage
            parts.extensions.get::<AppState>()
                .map(|app_state| &app_state.identity_provider)
                .ok_or_else(|| bad_gateway_response("internal_server_error"))?
        };

        // Validate the token using the identity provider
        match identity_provider.validate_token(token).await {
            Ok(user) => Ok(AuthenticatedUser(user)),
            Err(DomainError::Authentication(_)) => {
                Err(unauthorized_response("invalid_identity_token"))
            }
            Err(DomainError::PartnerUnavailable(_)) => {
                Err(bad_gateway_response("auth_service_unavailable"))
            }
            Err(DomainError::Validation(_)) => {
                // Invalid username from Vouch Proxy response
                Err(unauthorized_response("invalid_identity_token"))
            }
            Err(_) => {
                // Other domain errors shouldn't happen during auth, but handle gracefully
                Err(unauthorized_response("invalid_identity_token"))
            }
        }
    }
}
