use nexus_common::proto::enums::LogLevel;
use nexus_common::proto::generic::LogEntry;
use prost_types::Timestamp;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{Level, Subscriber};
use tracing_subscriber::layer::{Context, Layer};

/// Tracing layer that forwards logs to server only when observation is active
pub struct LogForwarder {
    tx: mpsc::Sender<LogEntry>,
    enabled: Arc<AtomicBool>,
}

impl LogForwarder {
    pub fn new(tx: mpsc::Sender<LogEntry>) -> (Self, LogForwarderHandle) {
        let enabled = Arc::new(AtomicBool::new(false));
        let handle = LogForwarderHandle {
            enabled: Arc::clone(&enabled),
        };

        (Self { tx, enabled }, handle)
    }
}

impl<S> Layer<S> for LogForwarder
where
    S: Subscriber,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        // Only forward logs if observation is active
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        // Convert tracing level to proto LogLevel
        let level = match *event.metadata().level() {
            Level::TRACE => LogLevel::Trace,
            Level::DEBUG => LogLevel::Debug,
            Level::INFO => LogLevel::Info,
            Level::WARN => LogLevel::Warn,
            Level::ERROR => LogLevel::Error,
        };

        // Extract the message from the event
        let mut visitor = MessageVisitor::new();
        event.record(&mut visitor);

        if visitor.message.is_empty() {
            return;
        }

        // Create timestamp
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let timestamp = Some(Timestamp {
            seconds: now.as_secs() as i64,
            nanos: now.subsec_nanos() as i32,
        });

        // Create proto log entry
        let log_entry = LogEntry {
            timestamp,
            level: level as i32,
            message: visitor.message,
            observation_id: String::new(),
        };

        // Send to channel (non-blocking, drop if full)
        let _ = self.tx.try_send(log_entry);
    }
}

/// Handle to enable/disable log forwarding
#[derive(Clone)]
pub struct LogForwarderHandle {
    enabled: Arc<AtomicBool>,
}

impl LogForwarderHandle {
    /// Enable log forwarding (call when observation starts)
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    /// Disable log forwarding (call when observation ends)
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }
}

/// Visitor to extract the message from tracing event
struct MessageVisitor {
    message: String,
}

impl MessageVisitor {
    fn new() -> Self {
        Self {
            message: String::new(),
        }
    }
}

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
            // Remove surrounding quotes from debug formatting
            if self.message.starts_with('"') && self.message.ends_with('"') {
                self.message = self.message[1..self.message.len() - 1].to_string();
            }
        } else {
            // Append other fields to the message
            if !self.message.is_empty() {
                self.message.push_str(", ");
            }
            self.message
                .push_str(&format!("{} = {:?}", field.name(), value));
        }
    }
}
