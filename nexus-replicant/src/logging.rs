use nexus_common::proto::generic::LogEntry;
use tokio::sync::mpsc;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;

use crate::log_forwarder::{LogForwarder, LogForwarderHandle};

pub struct LoggingSetup {
    pub unified_log_rx: mpsc::Receiver<LogEntry>,
    pub container_log_tx: mpsc::Sender<LogEntry>,
    pub log_handle: LogForwarderHandle,
}

/// Set up logging infrastructure with channels for application and container logs
pub fn setup(default_level: &str) -> Result<LoggingSetup, Box<dyn std::error::Error>> {
    // Create log forwarding channel for application logs
    let (app_log_tx, mut app_log_rx) = mpsc::channel(100);

    // Create container log forwarding channel
    let (container_log_tx, mut container_log_rx) = mpsc::channel(100);

    // Create unified log channel that merges both application and container logs
    let (unified_log_tx, unified_log_rx) = mpsc::channel(200);

    // Create log forwarder layer for application logs
    let (log_forwarder, log_handle) = LogForwarder::new(app_log_tx);

    // Spawn task to merge application and container logs
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(app_log) = app_log_rx.recv() => {
                    let _ = unified_log_tx.try_send(app_log);
                }
                Some(container_log) = container_log_rx.recv() => {
                    let _ = unified_log_tx.try_send(container_log);
                }
                else => break,
            }
        }
    });

    // Set up tracing with custom layer
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level)),
        )
        .finish()
        .with(log_forwarder);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(LoggingSetup {
        unified_log_rx,
        container_log_tx,
        log_handle,
    })
}
