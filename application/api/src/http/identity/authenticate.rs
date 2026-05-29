//! Authentication middleware for Axum.
//!
//! This module provides the authentication extractor that validates
//! user identity from HTTP request headers.

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::identity::User;

/// A wrapper around a domain `User` that has been successfully authenticated.
///
/// This type is used as an extractor in Axum handlers to ensure that
/// only authenticated requests can reach the handler.
///
/// # Usage
///
/// ```rust,ignore
/// async fn my_handler(AuthenticatedUser(user): AuthenticatedUser) -> impl IntoResponse {
///     // user is guaranteed to be valid
///     (StatusCode::OK, Json(user))
/// }
/// ```
pub struct AuthenticatedUser(pub User);

/// Implementation of `FromRequestParts` for `AuthenticatedUser`.
///
/// This extractor performs authentication by:
/// 1. Extracting the `X-Vouch-User` header from the request
/// 2. Validating that the header exists and contains a valid UTF-8 string
/// 3. Creating a domain `User` entity from the username
/// 4. Returning the authenticated user or an error response
///
/// # Authentication Mechanism
///
/// The authentication relies on the `X-Vouch-User` header, which is set by
/// an external proxy (e.g., Vouch Proxy, OAuth2 proxy) that validates
/// the user's credentials before forwarding the request.
///
/// # Design Rationale
///
/// Delegating authentication to an external proxy:
/// - Reduces application complexity
/// - Allows for flexible authentication strategies (OAuth, SAML, etc.)
/// - Keeps the application stateless
/// - Makes the application easier to test (headers can be mocked)
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    /// Extracts and validates the `X-Vouch-User` header from the request.
    ///
    /// # Arguments
    ///
    /// * `parts` - The request parts (headers, method, URI, etc.)
    /// * `_state` - The application state (not used for header-based auth)
    ///
    /// # Returns
    ///
    /// - `Ok(AuthenticatedUser)` if authentication succeeds
    /// - `Err(Response)` with `401 Unauthorized` if authentication fails
    ///
    /// # Failure Cases
    ///
    /// Returns `401 Unauthorized` if:
    /// - The `X-Vouch-User` header is missing
    /// - The header value is not valid UTF-8
    /// - The username fails domain validation (blank/empty)
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header_value = parts
            .headers
            .get("X-Vouch-User")
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({ "error": "Missing X-Vouch-User header" })),
                )
                    .into_response()
            })?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({ "error": "Missing X-Vouch-User header" })),
                )
                    .into_response()
            })?
            .to_string();

        User::new(header_value)
            .map(AuthenticatedUser)
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({ "error": "Missing X-Vouch-User header" })),
                )
                    .into_response()
            })
    }
}
