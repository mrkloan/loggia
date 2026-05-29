//! System health value object.
//!
//! This module contains the `SystemHealth` entity which represents the current
//! state of the system from a business perspective.

/// Represents the current health status of the system.
///
/// This is a **value object** in the domain layer, meaning it has no identity
/// and is defined entirely by its attributes. It captures the essential business
/// metrics needed to determine if the system is functioning correctly.
///
/// # Invariants
///
/// - `status` must accurately reflect the combination of `database_connected` and other system checks
/// - `uptime_seconds` must be monotonically increasing (or zero on first creation)
/// - If `database_connected` is true, the system is considered at least partially healthy
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemHealth {
    /// Human-readable status summary (e.g., "OK", "DEGRADED (Database Offline)").
    pub status: String,
    /// Whether the database connection is established and functional.
    pub database_connected: bool,
    /// Number of seconds the service has been running.
    pub uptime_seconds: u64,
}

impl SystemHealth {
    /// Creates a new `SystemHealth` instance.
    ///
    /// # Arguments
    ///
    /// * `status` - A string describing the overall system status
    /// * `database_connected` - Whether database connectivity check passed
    /// * `uptime_seconds` - Service uptime in seconds
    ///
    /// # Returns
    ///
    /// A new `SystemHealth` value object representing the current system state.
    pub fn new(status: String, database_connected: bool, uptime_seconds: u64) -> Self {
        Self {
            status,
            database_connected,
            uptime_seconds,
        }
    }
}
