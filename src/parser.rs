use crate::models::LogEntry;

use anyhow::{Context, Result, anyhow};
use regex::Regex;
use std::sync::OnceLock;

// Compile the regular expression only once.
static LOG_PATTERN: OnceLock<Regex> = OnceLock::new();

pub fn parse_line(line: &str) -> Result<LogEntry> {
    // Get the compiled regex.
    let regex = LOG_PATTERN.get_or_init(|| {
        Regex::new(
            r#"^(?P<ip>\S+)\s+-\s+-\s+\[(?P<timestamp>[^\]]+)\]\s+"(?P<method>[A-Z]+)\s+(?P<endpoint>\S+)\s+HTTP/\d(?:\.\d)?"\s+(?P<status>\d{3})\s+(?P<bytes>\d+)\s+(?P<latency>\d+)$"#
        )
        .expect("Log regex should be valid")
    });

    // Match the raw log line.
    let captures = regex
        .captures(line)
        .ok_or_else(|| anyhow!("Invalid log format: {}", line))?;

    // Extract text fields.
    let ip = captures.name("ip").unwrap().as_str().to_string();

    let timestamp = captures.name("timestamp").unwrap().as_str().to_string();

    let method = captures.name("method").unwrap().as_str().to_string();

    let endpoint = captures.name("endpoint").unwrap().as_str().to_string();

    // Convert numeric fields.
    let status_code = captures
        .name("status")
        .unwrap()
        .as_str()
        .parse::<u16>()
        .context("Invalid HTTP status code")?;

    let response_size_bytes = captures
        .name("bytes")
        .unwrap()
        .as_str()
        .parse::<u64>()
        .context("Invalid response size")?;

    let response_time_ms = captures
        .name("latency")
        .unwrap()
        .as_str()
        .parse::<u64>()
        .context("Invalid response time")?;

    // Create and return the structured log entry.
    Ok(LogEntry {
        ip,
        timestamp,
        method,
        endpoint,
        status_code,
        response_size_bytes,
        response_time_ms,
    })
}

