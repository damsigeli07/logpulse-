use crate::analyzer::AnalysisReport;
use anyhow::Result;
use serde::Serialize;
use std::path::Path;

// Export the complete report as JSON.
pub fn export_json(report: &AnalysisReport, output_path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;

    std::fs::write(output_path, json)?;

    Ok(())
}

// A simple CSV representation.
#[derive(Serialize)]
struct CsvRecord {
    category: String,
    name: String,
    value: String,
}

// Export report statistics as CSV.
pub fn export_csv(report: &AnalysisReport, output_path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(output_path)?;

    // Summary statistics.
    let records = vec![
        CsvRecord {
            category: "summary".to_string(),
            name: "total_requests".to_string(),
            value: report.total_requests.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "invalid_log_count".to_string(),
            value: report.invalid_log_count.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "successful_requests".to_string(),
            value: report.successful_requests.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "client_errors".to_string(),
            value: report.client_errors.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "server_errors".to_string(),
            value: report.server_errors.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "error_rate_percent".to_string(),
            value: report.error_rate_percent.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "average_latency_ms".to_string(),
            value: report.average_latency_ms.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "p95_latency_ms".to_string(),
            value: report.p95_latency_ms.to_string(),
        },
        CsvRecord {
            category: "summary".to_string(),
            name: "p99_latency_ms".to_string(),
            value: report.p99_latency_ms.to_string(),
        },
    ];

    for record in records {
        writer.serialize(record)?;
    }

    // Endpoint statistics.
    for (endpoint, count) in &report.top_endpoints {
        writer.serialize(CsvRecord {
            category: "endpoint".to_string(),
            name: endpoint.clone(),
            value: count.to_string(),
        })?;
    }

    writer.flush()?;

    Ok(())
}
