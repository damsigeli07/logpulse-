
use logpulse::analyzer::analyze_logs;
use logpulse::models::LogEntry;
use logpulse::parser::parse_line;

#[test]
fn test_parser_and_analyzer_work_together() {
    let logs = vec![
        r#"192.168.1.10 - - [18/Nov/2026:08:10:00] "GET /api/users HTTP/1.1" 200 1234 100"#,
        r#"192.168.1.11 - - [18/Nov/2026:08:10:01] "POST /api/login HTTP/1.1" 201 500 200"#,
        r#"192.168.1.12 - - [18/Nov/2026:08:10:02] "GET /api/users HTTP/1.1" 404 300 300"#,
    ];

    let entries: Vec<LogEntry> = logs
        .iter()
        .map(|line| parse_line(line).expect("Valid log should parse"))
        .collect();

    let report = analyze_logs(&entries);

    assert_eq!(report.total_requests, 3);
    assert_eq!(report.successful_requests, 2);
    assert_eq!(report.client_errors, 1);
    assert_eq!(report.server_errors, 0);
    assert_eq!(report.top_endpoints[0].0, "/api/users");
    assert_eq!(report.top_endpoints[0].1, 2);
}

#[test]
fn test_parser_rejects_invalid_input() {
    let result = parse_line("This is an invalid log");

    assert!(result.is_err());
}

#[test]
fn test_analyzer_handles_empty_input() {
    let entries: Vec<LogEntry> = Vec::new();

    let report = analyze_logs(&entries);

    assert_eq!(report.total_requests, 0);
    assert_eq!(report.successful_requests, 0);
    assert_eq!(report.error_rate_percent, 0.0);
}
