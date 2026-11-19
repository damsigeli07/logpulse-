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
        .captures(line.trim())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_log() {
        let line =
            r#"192.168.1.10 - - [18/Nov/2026:08:10:00] "GET /api/users HTTP/1.1" 200 1234 145"#;

        let entry = parse_line(line).unwrap();

        assert_eq!(entry.ip, "192.168.1.10");
        assert_eq!(entry.method, "GET");
        assert_eq!(entry.endpoint, "/api/users");
        assert_eq!(entry.status_code, 200);
        assert_eq!(entry.response_size_bytes, 1234);
        assert_eq!(entry.response_time_ms, 145);
    }

    #[test]
    fn test_parse_invalid_log() {
        let line = "This is not a valid server log";

        assert!(parse_line(line).is_err());
    }

    #[test]
    fn test_parse_log_with_surrounding_whitespace() {
        let line =
            "  192.168.1.10 - - [18/Nov/2026:08:10:00] \"GET /api/users HTTP/1.1\" 200 1234 145  ";

        assert_eq!(parse_line(line).unwrap().endpoint, "/api/users");
    }

    #[test]
    fn test_parse_post_request() {
        let line =
            r#"192.168.1.12 - - [18/Nov/2026:08:10:04] "POST /api/login HTTP/1.1" 201 456 210"#;

        let entry = parse_line(line).unwrap();

        assert_eq!(entry.method, "POST");
        assert_eq!(entry.status_code, 201);
    }
}
