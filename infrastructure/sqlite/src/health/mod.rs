//! SQLite health check adapter implementation.
//!
//! This module provides the SQLite-specific implementation of the domain's
//! `HealthRepository` port. It adapts the domain's abstract interface to
//! concrete SQLite database operations.
//!
//! # Architecture
//!
//! This is a classic **Hexagonal Architecture adapter**:
//! - Implements the `HealthRepository` trait from the domain layer
//! - Translates domain-level health check requests into SQLite-specific operations
//! - Remains completely replaceable without affecting domain logic

pub mod check_health;
pub use check_health::SqliteHealthRepository;
