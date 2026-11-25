//! HTTP router and handler modules.
//!
//! This module provides the HTTP routing configuration and handlers for the API.
//! It translates between HTTP requests/responses and domain use cases.
//!
//! # Architecture
//!
//! This is the **driving adapter** in Hexagonal Architecture:
//! - Receives HTTP requests from external clients
//! - Invokes domain use cases through inbound ports
//! - Transforms domain results into HTTP responses
//!
//! # Modules
//!
//! - `health`: Health check endpoint handlers
//! - `identity`: Identity/authorization endpoint handlers

pub mod health;
pub mod identity;

use axum::Router;
use axum::routing::get;
use crate::AppState;

/// Creates and configures the Axum router with all API endpoints.
///
/// This is the **composition root** for the HTTP layer. It:
/// 1. Creates a new Axum `Router`
/// 2. Registers all routes with their handlers
/// 3. Configures shared state (domain use cases, identity provider)
/// 4. Returns the configured router
///
/// # Arguments
///
/// * `state` - The application state containing all required services
///
/// # Returns
///
/// A configured `Router` instance with all API endpoints registered.
///
/// # Routes
///
/// - `GET /health` - System health check endpoint (public, no auth required)
/// - `GET /me` - Get current authenticated user endpoint (requires X-Identity-Token)
///
/// # Design Rationale
///
/// The router is configured as a pure function that takes application state as an argument.
/// This makes it:
/// - Easy to test (can pass mock implementations)
/// - Explicit about its dependencies
/// - Reusable in different contexts (e.g., integration tests, multiple servers)
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::check_health::handle))
        .route("/me", get(identity::get_me::handle))
        .with_state(state)
}
