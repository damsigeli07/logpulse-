use crate::models::LogEntry;
use crate::parser::parse_line;
use anyhow::Result;
use rayon::prelude::*;

// Sequential log processing.
pub fn parse_logs_sequential(content: &str) -> (Vec<LogEntry>, usize) {
    collect_results(
        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(parse_line),
    )
}

// Parallel log processing.
pub fn parse_logs_parallel(content: &str) -> (Vec<LogEntry>, usize) {
    let results: Vec<_> = content
        .par_lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect();

    collect_results(results)
}

fn collect_results(results: impl IntoIterator<Item = Result<LogEntry>>) -> (Vec<LogEntry>, usize) {
    let mut entries = Vec::new();
    let mut invalid_lines = 0;

    for result in results {
        match result {
            Ok(entry) => entries.push(entry),
            Err(_) => invalid_lines += 1,
        }
    }

    (entries, invalid_lines)
}
