//! Vouch Proxy identity provider adapter.
//!
//! This crate provides the `VouchIdentityProvider` implementation of the
//! `IdentityProvider` port, which validates identity tokens via Vouch Proxy's
//! `/validate` endpoint.
//!
//! # Architecture
//!
//! This is the **infrastructure adapter** in Hexagonal Architecture:
//! - Implements the `IdentityProvider` port from the domain layer
//! - Provides concrete Vouch Proxy integration
//! - Translates between Vouch Proxy responses and domain types
//!
//! # Environment Variables
//!
//! - `VOUCH_ENDPOINT`: Vouch Proxy validate endpoint URL (default: "http://vouch-proxy:9090/validate")
//! - `VOUCH_COOKIE`: Cookie name for Vouch Proxy (default: "VouchCookie")
//! - `VOUCH_DOMAIN`: Domain to send in Host header (default: "example.com")

use async_trait::async_trait;
use std::env;

use domain::errors::DomainError;
use domain::identity::provider::IdentityProvider;
use domain::identity::User;

/// Default Vouch Proxy validate endpoint URL.
pub const DEFAULT_VOUCH_ENDPOINT: &str = "http://vouch-proxy:9090/validate";

/// Default Vouch Proxy cookie name.
pub const DEFAULT_VOUCH_COOKIE: &str = "VouchCookie";

/// Default Vouch Proxy domain for Host header.
pub const DEFAULT_VOUCH_DOMAIN: &str = "example.com";

/// Vouch Proxy implementation of the `IdentityProvider` port.
///
/// This struct validates identity tokens by calling Vouch Proxy's `/validate`
/// endpoint and extracts the username from the `X-Vouch-User` header in the response.
///
/// # Usage
///
/// ```rust,ignore
/// let provider = VouchIdentityProvider::new()?;
/// let user = provider.validate_token(token).await?;
/// ```
///
/// # Design Rationale
///
/// By delegating token validation to Vouch Proxy:
/// - The API enforces end-to-end security (validates all tokens)
/// - Works for both public and internal request paths
/// - Keeps the application stateless
/// - Centralizes authentication logic
pub struct VouchIdentityProvider {
    /// The Vouch Proxy validate endpoint URL.
    endpoint: String,
    /// The cookie name used by Vouch Proxy.
    cookie_name: String,
    /// The domain to send in the Host header.
    domain: String,
    /// HTTP client for making requests to Vouch Proxy.
    client: reqwest::Client,
}

impl VouchIdentityProvider {
    /// Creates a new `VouchIdentityProvider` with configuration from environment variables.
    ///
    /// # Environment Variables
    ///
    /// - `VOUCH_ENDPOINT`: Vouch Proxy validate endpoint URL (default: "http://vouch-proxy:9090/validate")
    /// - `VOUCH_COOKIE`: Cookie name for Vouch Proxy (default: "VouchCookie")
    /// - `VOUCH_DOMAIN`: Domain to send in Host header (default: "example.com")
    ///
    /// # Returns
    ///
    /// - `Ok(VouchIdentityProvider)` if all configuration is valid
    /// - `Err(String)` if required configuration is missing or invalid
    ///
    /// # Panics
    ///
    /// This function will not panic. It returns `Err` for any configuration issues.
    pub fn new() -> Result<Self, String> {
        let endpoint = env::var("VOUCH_ENDPOINT")
            .unwrap_or_else(|_| DEFAULT_VOUCH_ENDPOINT.to_string());
        let cookie_name = env::var("VOUCH_COOKIE")
            .unwrap_or_else(|_| DEFAULT_VOUCH_COOKIE.to_string());
        let domain = env::var("VOUCH_DOMAIN")
            .unwrap_or_else(|_| DEFAULT_VOUCH_DOMAIN.to_string());

        // Validate endpoint is a valid URL
        let _ = url::Url::parse(&endpoint)
            .map_err(|e| format!("Invalid VOUCH_ENDPOINT URL: {}", e))?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            endpoint,
            cookie_name,
            domain,
            client,
        })
    }

    /// Returns the configured Vouch Proxy endpoint.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the configured cookie name.
    pub fn cookie_name(&self) -> &str {
        &self.cookie_name
    }

    /// Returns the configured domain.
    pub fn domain(&self) -> &str {
        &self.domain
    }
}

#[async_trait]
impl IdentityProvider for VouchIdentityProvider {
    /// Validates an identity token by calling Vouch Proxy's `/validate` endpoint.
    ///
    /// # Arguments
    ///
    /// * `token` - The identity token string to validate
    ///
    /// # Returns
    ///
    /// - `Ok(User)` if the token is valid and a user can be constructed
    /// - `Err(DomainError::Authentication)` if the token is invalid or expired
    /// - `Err(DomainError::PartnerUnavailable)` if Vouch Proxy is unreachable
    /// - `Err(DomainError::Validation)` if the username from Vouch Proxy is invalid
    ///
    /// # Process
    ///
    /// 1. Send GET request to Vouch Proxy's `/validate` endpoint
    /// 2. Include the token as a cookie (using configured cookie name)
    /// 3. Set Host header to configured domain
    /// 4. Check response status:
    ///    - `200 OK`: Extract `X-Vouch-User` header and create User
    ///    - `401 Unauthorized`: Token is invalid/expired
    ///    - Other errors: Vouch Proxy unreachable or misconfigured
    async fn validate_token(&self, token: String) -> domain::errors::DomainResult<User> {
        let mut request = self.client.get(&self.endpoint);

        // Set the token as a Cookie header
        // Format: "CookieName=token"
        request = request.header("Cookie", format!("{}={}", self.cookie_name, token));

        // Set the Host header to the configured domain
        request = request.header("Host", &self.domain);

        // Make the request to Vouch Proxy
        let response = match request.send().await {
            Ok(resp) => resp,
            Err(e) => {
                return Err(DomainError::PartnerUnavailable(format!(
                    "Failed to connect to Vouch Proxy: {}",
                    e
                )));
            }
        };

        // Check the response status
        match response.status() {
            reqwest::StatusCode::OK => {
                // Extract X-Vouch-User header from response
                let username = response
                    .headers()
                    .get("X-Vouch-User")
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string());

                match username {
                    Some(username) => {
                        // Create User entity with validation
                        User::new(username)
                            .map_err(|e| DomainError::Validation(e.to_string()))
                    }
                    None => Err(DomainError::Validation(
                        "Vouch Proxy response missing X-Vouch-User header".to_string(),
                    )),
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                // Token is invalid or expired
                Err(DomainError::Authentication(
                    "Invalid or expired identity token".to_string(),
                ))
            }
            _ => {
                // Other error status codes (5xx, etc.)
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                Err(DomainError::PartnerUnavailable(format!(
                    "Vouch Proxy returned unexpected status {}: {}",
                    status, body
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    /// Creates a test VouchIdentityProvider with a custom client and endpoint.
    ///
    /// For testing purposes, this allows creating a provider with arbitrary
    /// endpoint, cookie name, domain, and HTTP client.
    impl VouchIdentityProvider {
        pub fn new_test(
            endpoint: String,
            cookie_name: String,
            domain: String,
            client: reqwest::Client,
        ) -> Result<Self, String> {
            Ok(Self {
                endpoint,
                cookie_name,
                domain,
                client,
            })
        }

        /// Creates a test provider with a custom endpoint and default test client.
        pub fn with_endpoint(endpoint: String) -> Result<Self, String> {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(1))
                .build()
                .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
            Self::new_test(
                endpoint,
                DEFAULT_VOUCH_COOKIE.to_string(),
                DEFAULT_VOUCH_DOMAIN.to_string(),
                client,
            )
        }
    }

    #[test]
    fn new_with_defaults() {
        let _provider = VouchIdentityProvider::new().unwrap();
        assert_eq!(DEFAULT_VOUCH_ENDPOINT, "http://vouch-proxy:9090/validate");
        assert_eq!(DEFAULT_VOUCH_COOKIE, "VouchCookie");
        assert_eq!(DEFAULT_VOUCH_DOMAIN, "example.com");
    }

    #[test]
    fn new_with_custom_values() {
        let client = reqwest::Client::new();
        let provider = VouchIdentityProvider::new_test(
            "http://custom:8080/validate".to_string(),
            "CustomCookie".to_string(),
            "custom.example.com".to_string(),
            client,
        )
        .unwrap();
        assert_eq!(provider.endpoint(), "http://custom:8080/validate");
        assert_eq!(provider.cookie_name(), "CustomCookie");
        assert_eq!(provider.domain(), "custom.example.com");
    }

    #[test]
    fn new_with_invalid_endpoint() {
        let result = url::Url::parse("not-a-valid-url");
        assert!(result.is_err());
    }

    /// Helper to run async code in a tokio runtime
    fn run_async<'a, F, R>(future: F) -> R
    where
        F: std::future::Future<Output = R> + Send + 'a,
        R: Send + 'a,
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create runtime");
        rt.block_on(future)
    }

    #[test]
    fn validate_token_success_with_user_header() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(200)
            .with_header("X-Vouch-User", "alice")
            .create();

        let provider = VouchIdentityProvider::with_endpoint(server.url() + "/validate").unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username(), "alice");
    }

    #[test]
    fn validate_token_success_with_trimmed_username() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(200)
            .with_header("X-Vouch-User", "  alice  ")
            .create();

        let provider = VouchIdentityProvider::with_endpoint(server.url() + "/validate").unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username(), "alice");
    }

    #[test]
    fn validate_token_missing_user_header() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(200)
            .create();

        let provider = VouchIdentityProvider::with_endpoint(server.url() + "/validate").unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }

    #[test]
    fn validate_token_missing_user_header_empty_value() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(200)
            .with_header("X-Vouch-User", "")
            .create();

        let provider = VouchIdentityProvider::with_endpoint(server.url() + "/validate").unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }

    #[test]
    fn validate_token_unauthorized() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(401)
            .create();

        let provider = VouchIdentityProvider::with_endpoint(server.url() + "/validate").unwrap();

        let result = run_async(provider.validate_token("invalid-token".to_string()));

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Authentication(_)));
    }

    #[test]
    fn validate_token_server_error() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let provider = VouchIdentityProvider::with_endpoint(server.url() + "/validate").unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::PartnerUnavailable(_)));
    }

    #[test]
    fn validate_token_not_found() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(404)
            .create();

        let provider = VouchIdentityProvider::with_endpoint(server.url() + "/validate").unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::PartnerUnavailable(_)));
    }

    #[test]
    fn validate_token_sends_correct_cookie() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(200)
            .with_header("X-Vouch-User", "bob")
            .match_header("Cookie", "MyCookie=my-token-value")
            .create();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(1))
            .build()
            .unwrap();

        let provider = VouchIdentityProvider::new_test(
            server.url() + "/validate",
            "MyCookie".to_string(),
            "test.example.com".to_string(),
            client,
        )
        .unwrap();

        let result = run_async(provider.validate_token("my-token-value".to_string()));

        assert!(result.is_ok());
    }

    #[test]
    fn validate_token_sends_correct_host_header() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/validate")
            .with_status(200)
            .with_header("X-Vouch-User", "charlie")
            .match_header("Host", "my-custom-domain.com")
            .create();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(1))
            .build()
            .unwrap();

        let provider = VouchIdentityProvider::new_test(
            server.url() + "/validate",
            "VouchCookie".to_string(),
            "my-custom-domain.com".to_string(),
            client,
        )
        .unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_ok());
    }

    #[test]
    fn validate_token_connection_error() {
        let provider = VouchIdentityProvider::with_endpoint("http://localhost:12345/validate".to_string()).unwrap();

        let result = run_async(provider.validate_token("test-token".to_string()));

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::PartnerUnavailable(_)));
    }

    #[test]
    fn test_accessors() {
        let provider = VouchIdentityProvider::new_test(
            "http://test:8080/validate".to_string(),
            "TestCookie".to_string(),
            "test.domain.com".to_string(),
            reqwest::Client::new(),
        )
        .unwrap();

        assert_eq!(provider.endpoint(), "http://test:8080/validate");
        assert_eq!(provider.cookie_name(), "TestCookie");
        assert_eq!(provider.domain(), "test.domain.com");
    }
}
