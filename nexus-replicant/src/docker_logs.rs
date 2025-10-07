use bollard::container::LogsOptions;
use bollard::Docker;
use futures::StreamExt;
use nexus_common::proto::enums::LogLevel;
use nexus_common::proto::generic::LogEntry;
use prost_types::Timestamp;
use tokio::sync::mpsc;
use tracing::{error, info};

/// Start streaming logs from a container to the log channel
pub async fn start_log_streaming(
    docker: Docker,
    container_id: &str,
    replicant_id: &str,
    log_tx: mpsc::Sender<LogEntry>,
) -> Result<(), Box<dyn std::error::Error>> {
    let container_id = container_id.to_string();
    let replicant_id = replicant_id.to_string();

    tokio::spawn(async move {
        let log_options = LogsOptions::<String> {
            follow: true,
            stdout: true,
            stderr: true,
            timestamps: true,
            ..Default::default()
        };

        let mut log_stream = docker.logs(&container_id, Some(log_options));

        info!(
            "Started log streaming for container {} ({})",
            container_id, replicant_id
        );

        while let Some(log_result) = log_stream.next().await {
            match log_result {
                Ok(log_output) => {
                    let message = log_output.to_string();
                    if message.trim().is_empty() {
                        continue;
                    }

                    // Create timestamp
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap();
                    let timestamp = Timestamp {
                        seconds: now.as_secs() as i64,
                        nanos: now.subsec_nanos() as i32,
                    };

                    // Determine log level based on stderr vs stdout
                    let level = match log_output {
                        bollard::container::LogOutput::StdErr { .. } => LogLevel::Error,
                        _ => LogLevel::Info,
                    };

                    // Strip Docker timestamp prefix (e.g. "2025-10-07T06:20:08.823422388Z ")
                    let clean_message = strip_docker_timestamp(&message);

                    let log_entry = LogEntry {
                        timestamp: Some(timestamp),
                        level: level as i32,
                        message: clean_message,
                        observation_id: "".to_string(), // Will be filled by server if needed
                    };

                    // Send log entry (non-blocking, drop if channel is full)
                    if let Err(_) = log_tx.try_send(log_entry) {
                        error!(
                            "Log channel is full, dropping container log for {}",
                            container_id
                        );
                    }
                }
                Err(e) => {
                    error!("Error reading logs from container {}: {}", container_id, e);
                    break;
                }
            }
        }

        info!(
            "Log streaming ended for container {} ({})",
            container_id, replicant_id
        );
    });

    Ok(())
}

/// Strip Docker timestamp prefix from log messages
/// Docker logs typically start with "2025-10-07T06:20:08.823422388Z " followed by the actual message
fn strip_docker_timestamp(message: &str) -> String {
    let trimmed = message.trim();

    // Look for pattern: timestamp followed by space
    // Docker timestamps are in format: YYYY-MM-DDTHH:MM:SS.nanosZ
    if let Some(space_pos) = trimmed.find(' ') {
        let potential_timestamp = &trimmed[..space_pos];

        // Check if it looks like a Docker timestamp (basic check for RFC3339-like format)
        if potential_timestamp.len() >= 20
            && potential_timestamp.contains('T')
            && potential_timestamp.ends_with('Z')
        {
            return trimmed[space_pos + 1..].to_string();
        }
    }

    // If no timestamp pattern found, return original trimmed message
    trimmed.to_string()
}
