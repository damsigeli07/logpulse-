use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogEntry {
    pub ip: String,
    pub timestamp: String,
    pub method: String,
    pub endpoint: String,
    pub status_code: u16,
    pub response_size_bytes: u64,
    pub response_time_ms: u64,
}
