//! # Domain Layer
//!
//! The core business logic crate implementing Hexagonal Architecture principles.
//! This crate contains entities, value objects, use cases, and ports (interfaces)
//! that define the contract between the domain and external adapters.
//!
//! ## Architecture
//!
//! This crate is the innermost layer of the Hexagonal Architecture:
//! - **Entities**: Business objects with identity (e.g., `User`)
//! - **Value Objects**: Immutable business concepts (e.g., `SystemHealth`)
//! - **Use Cases**: Business operations orchestrating domain logic
//! - **Ports**: Interfaces defining how external systems interact with the domain
//!
//! ## Key Design Principles
//!
//! - **Independence**: No dependencies on external frameworks, databases, or UI
//! - **Testability**: All business logic can be tested in isolation
//! - **Replaceability**: External adapters can be swapped without changing domain code
//!
//! ## Modules
//!
//! - `errors`: Domain error types and result aliases
//! - `health`: Health check domain logic (entities, use cases, ports)
//! - `identity`: User identity and authentication domain logic

#![deny(missing_docs)]

pub mod errors;
pub mod health;
pub mod identity;
