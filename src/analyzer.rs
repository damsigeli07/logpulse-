use crate::models::LogEntry;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct AnalysisReport {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub client_errors: usize,
    pub server_errors: usize,
    pub error_rate_percent: f64,
    pub average_latency_ms: f64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,
    pub top_endpoints: Vec<(String, usize)>,
}

pub fn analyze_logs(entries: &[LogEntry]) -> AnalysisReport {
    let total_requests = entries.len();

    let successful_requests = entries
        .iter()
        .filter(|entry| (200..300).contains(&entry.status_code))
        .count();

    let client_errors = entries
        .iter()
        .filter(|entry| (400..500).contains(&entry.status_code))
        .count();

    let server_errors = entries
        .iter()
        .filter(|entry| (500..600).contains(&entry.status_code))
        .count();

    let error_rate_percent = if total_requests > 0 {
        ((client_errors + server_errors) as f64 / total_requests as f64) * 100.0
    } else {
        0.0
    };

    let average_latency_ms = if total_requests > 0 {
        entries
            .iter()
            .map(|entry| entry.response_time_ms)
            .sum::<u64>() as f64
            / total_requests as f64
    } else {
        0.0
    };

    // Calculate latency percentiles.
    let mut latencies: Vec<u64> = entries.iter().map(|entry| entry.response_time_ms).collect();

    latencies.sort_unstable();

    let p95_latency_ms = percentile(&latencies, 95);
    let p99_latency_ms = percentile(&latencies, 99);

    // Count requests per endpoint.
    let mut endpoint_counts: HashMap<String, usize> = HashMap::new();

    for entry in entries {
        *endpoint_counts.entry(entry.endpoint.clone()).or_insert(0) += 1;
    }

    // Sort endpoints by request count.
    let mut top_endpoints: Vec<(String, usize)> = endpoint_counts.into_iter().collect();

    top_endpoints.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    AnalysisReport {
        total_requests,
        successful_requests,
        client_errors,
        server_errors,
        error_rate_percent,
        average_latency_ms,
        p95_latency_ms,
        p99_latency_ms,
        top_endpoints,
    }
}

fn percentile(sorted_values: &[u64], percentile: usize) -> u64 {
    if sorted_values.is_empty() {
        return 0;
    }

    let rank = (percentile * sorted_values.len()).div_ceil(100);

    sorted_values[rank.saturating_sub(1)]
}

