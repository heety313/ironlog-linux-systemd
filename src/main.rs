use clap::Parser;
use ironlog::TcpLogger;
use log::{Level, LevelFilter};
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL for the TcpLogger (default: 127.0.0.1:5000)
    #[arg(short, long)]
    url: Option<String>,

    /// Application name (default: your-app-name)
    #[arg(short, long)]
    app_name: Option<String>,

    /// API key for future use
    #[arg(short, long)]
    api_key: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Initialize the TcpLogger with optional URL and app name
    let url = args.url.unwrap_or_else(|| "127.0.0.1:5000".to_string());
    let app_name = args.app_name.unwrap_or_else(|| "your-app-name".to_string());
    
    TcpLogger::init(&url, &app_name, LevelFilter::Debug)
        .expect("Failed to initialize TcpLogger");

    // Log the API key if provided (for demonstration purposes)
    if let Some(api_key) = args.api_key {
        log::info!("API key provided: {}", api_key);
    }

    // Check if running on Linux
    if !cfg!(target_os = "linux") {
        log::error!("This program is designed to run on Linux systems only.");
        std::process::exit(1);
    }

    // Check if journalctl is available
    if !Command::new("journalctl").output().is_ok() {
        log::error!("The 'journalctl' command is not available on this system.");
        std::process::exit(1);
    }

    // Spawn the journalctl process to follow logs in JSON format
    let mut child = Command::new("journalctl")
        .args(&["-f", "-o", "json"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start journalctl process");

    // Capture the stdout of the child process
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    // Iterate over each line (log entry) from journalctl
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Parse the JSON log entry
                if let Ok(json) = serde_json::from_str::<Value>(&line) {
                    // Extract the message
                    let message = json
                        .get("MESSAGE")
                        .and_then(|v| v.as_str())
                        .unwrap_or("No message");

                    // Extract the process name
                    let process_name = json
                        .get("SYSLOG_IDENTIFIER")
                        .or_else(|| json.get("_COMM"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");

                    // Extract the priority
                    let priority = json
                        .get("PRIORITY")
                        .and_then(|v| v.as_str())
                        .unwrap_or("6"); // Default to informational

                    // Map systemd priorities to log levels
                    let level = match priority {
                        "0" | "1" | "2" | "3" => Level::Error, // Emerg, Alert, Crit, Err
                        "4" => Level::Warn,                     // Warning
                        "5" => Level::Info,                     // Notice
                        "6" => Level::Info,                     // Informational
                        "7" => Level::Debug,                    // Debug
                        _ => Level::Info,                       // Default
                    };

                    // Log the message with the process name included
                    log::log!(level, "{} : {}", process_name, message);
                } else {
                    eprintln!("Failed to parse JSON: {}", line);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
