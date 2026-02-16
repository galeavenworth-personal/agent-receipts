use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// Command wrapper that emits execution receipts as JSON
#[derive(Parser, Debug)]
#[command(name = "rcpt")]
#[command(about = "Execute commands and emit execution receipts as JSON")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a command and emit an execution receipt
    Run {
        /// Output path for the receipt JSON file
        #[arg(short, long, default_value = "receipt.json")]
        out: PathBuf,

        /// Command to execute
        #[arg(required = true, trailing_var_arg = true)]
        command: Vec<OsString>,
    },
}

/// Execution receipt containing command metadata and results
#[derive(Debug, Serialize, Deserialize)]
struct Receipt {
    /// Command that was executed
    command: String,
    /// Command arguments
    args: Vec<String>,
    /// Exit code of the command (None if terminated by signal)
    exit_code: Option<i32>,
    /// Standard output (stdout)
    stdout: String,
    /// Standard error (stderr)
    stderr: String,
    /// Start time (ISO 8601 timestamp)
    start_time: DateTime<Utc>,
    /// End time (ISO 8601 timestamp)
    end_time: DateTime<Utc>,
    /// Duration in milliseconds
    duration_ms: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { out, command } => {
            let receipt = execute_command(command)?;
            write_receipt(&out, &receipt)?;
            println!("Receipt written to: {}", out.display());
            
            // Exit with the same code as the wrapped command
            // Use 1 as default if exit code is not available (signal termination)
            std::process::exit(receipt.exit_code.unwrap_or(1));
        }
    }
}

fn execute_command(command_parts: Vec<OsString>) -> Result<Receipt> {
    if command_parts.is_empty() {
        anyhow::bail!("No command specified");
    }

    let cmd = &command_parts[0];
    let args = &command_parts[1..];

    let start_time = Utc::now();
    let start_instant = Instant::now();

    let output = Command::new(cmd)
        .args(args)
        .output()
        .context(format!("Failed to execute command: {}", cmd.to_string_lossy()))?;

    let duration = start_instant.elapsed();
    let end_time = Utc::now();

    let receipt = Receipt {
        command: cmd.to_string_lossy().to_string(),
        args: args.iter().map(|s| s.to_string_lossy().to_string()).collect(),
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        start_time,
        end_time,
        duration_ms: duration.as_millis() as u64,
    };

    Ok(receipt)
}

fn write_receipt(path: &Path, receipt: &Receipt) -> Result<()> {
    let json = serde_json::to_string_pretty(receipt)
        .context("Failed to serialize receipt to JSON")?;
    
    // Ensure the parent directory exists, if a parent is specified
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).context(format!(
                "Failed to create parent directories for {}",
                path.display()
            ))?;
        }
    }

    fs::write(path, json)
        .context(format!("Failed to write receipt to {}", path.display()))?;
    
    Ok(())
}
