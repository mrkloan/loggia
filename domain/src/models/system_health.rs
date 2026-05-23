#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemHealth {
    pub status: String,
    pub database_connected: bool,
    pub uptime_seconds: u64,
}

impl SystemHealth {
    pub fn new(status: String, database_connected: bool, uptime_seconds: u64) -> Self {
        Self {
            status,
            database_connected,
            uptime_seconds,
        }
    }
}
