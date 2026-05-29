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

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use domain::health::check_health::CheckHealthUseCase;

/// Creates and configures the Axum router with all API endpoints.
///
/// This is the **composition root** for the HTTP layer. It:
/// 1. Creates a new Axum `Router`
/// 2. Registers all routes with their handlers
/// 3. Configures shared state (domain use cases)
/// 4. Returns the configured router
///
/// # Arguments
///
/// * `health_use_case` - An implementation of the `CheckHealthUseCase` inbound port
///
/// # Returns
///
/// A configured `Router` instance with all API endpoints registered.
///
/// # Routes
///
/// - `GET /health` - System health check endpoint
/// - `GET /me` - Get current authenticated user endpoint
///
/// # Design Rationale
///
/// The router is configured as a pure function that takes domain use cases as arguments.
/// This makes it:
/// - Easy to test (can pass mock use cases)
/// - Explicit about its dependencies
/// - Reusable in different contexts (e.g., integration tests, multiple servers)
pub fn router(health_use_case: Arc<dyn CheckHealthUseCase>) -> Router {
    Router::new()
        .route("/health", get(health::check_health::handle))
        .route("/me", get(identity::get_me::handle))
        .with_state(health_use_case)
}
