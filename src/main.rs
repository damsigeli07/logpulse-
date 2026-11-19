use logpulse::analyzer::analyze_logs;
use logpulse::exporter::{export_csv, export_json};
use logpulse::pipeline::{parse_logs_parallel, parse_logs_sequential};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "LogPulse",
    version,
    about = "High-performance server log analytics CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        input: PathBuf,

        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(long)]
        parallel: bool,
    },
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
enum OutputFormat {
    Table,
    Json,
    Csv,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            input,
            format,
            output,
            parallel,
        } => {
            analyze_file(input, format, output, parallel)?;
        }
    }

    Ok(())
}

fn analyze_file(
    input: PathBuf,
    format: OutputFormat,
    output: Option<PathBuf>,
    parallel: bool,
) -> Result<()> {
    if !input.is_file() {
        anyhow::bail!("Input path is not a file: {}", input.display());
    }

    println!("Reading log file: {}", input.display());

    let content = fs::read_to_string(&input)?;

    let start = std::time::Instant::now();

    let (entries, invalid_lines) = if parallel {
        println!("Processing mode: Parallel");
        parse_logs_parallel(&content)
    } else {
        println!("Processing mode: Sequential");
        parse_logs_sequential(&content)
    };

    let processing_time = start.elapsed();

    println!("Processing time: {:.2?}", processing_time);
    println!("Invalid lines: {}", invalid_lines);

    let mut report = analyze_logs(&entries);
    report.invalid_log_count = invalid_lines;

    match format {
        OutputFormat::Table => {
            println!("\n========== LOGPULSE ANALYTICS ==========");

            println!("Total Requests      : {}", report.total_requests);
            println!("Successful Requests : {}", report.successful_requests);
            println!("Client Errors       : {}", report.client_errors);
            println!("Server Errors       : {}", report.server_errors);
            println!("Invalid Lines       : {}", invalid_lines);

            println!("\n========== PERFORMANCE ==========");

            println!("Error Rate      : {:.2}%", report.error_rate_percent);
            println!("Average Latency : {:.2} ms", report.average_latency_ms);
            println!("P95 Latency     : {} ms", report.p95_latency_ms);
            println!("P99 Latency     : {} ms", report.p99_latency_ms);

            println!("\n========== TOP ENDPOINTS ==========");

            for (endpoint, count) in &report.top_endpoints {
                println!("{:<25} {}", endpoint, count);
            }
        }

        OutputFormat::Json => {
            let path = output.unwrap_or_else(|| PathBuf::from("report.json"));

            export_json(&report, &path)?;

            println!("JSON report exported to: {}", path.display());
        }

        OutputFormat::Csv => {
            let path = output.unwrap_or_else(|| PathBuf::from("report.csv"));

            export_csv(&report, &path)?;

            println!("CSV report exported to: {}", path.display());
        }
    }

    Ok(())
}
