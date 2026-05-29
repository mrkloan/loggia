//! Health check domain module.
//!
//! This module provides the domain logic for system health monitoring and verification.
//! It defines the core entities, use cases, and ports for health check functionality.
//!
//! # Architecture
//!
//! - `system_health`: Contains the `SystemHealth` value object representing the current system state
//! - `check_health`: Contains the use case, ports (interfaces), and service implementation
//!
//! The health check feature demonstrates Hexagonal Architecture principles:
//! - Domain defines `HealthRepository` port (outbound) for database connectivity checks
//! - Domain defines `CheckHealthUseCase` port (inbound) for the health check operation
//! - Infrastructure provides concrete implementations of these ports

pub mod system_health;
pub mod check_health;
