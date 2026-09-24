//! Identity and authentication endpoint handlers.
//!
//! This module contains HTTP handlers for identity-related endpoints,
//! including authentication and user information retrieval.

pub mod authenticate;
pub mod get_me;

/// The HTTP header name for identity tokens.
///
/// This constant is used throughout the API layer for extracting identity tokens
/// from incoming requests. It is not exported outside the api crate to maintain
/// encapsulation of HTTP concerns.
pub(crate) const X_IDENTITY_TOKEN: &str = "X-Identity-Token";
