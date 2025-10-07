use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationStatus {
    Created,
    Scheduled,
    Running,
    Completed,
    Cancelled,
    Failed,
    Disconnected,
}

impl Default for ObservationStatus {
    fn default() -> Self {
        ObservationStatus::Created
    }
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: ObservationStatus,
    pub replicant_tags: Vec<String>,
    pub number_of_replicants: i32,
    pub image_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub replicant_ids: Vec<String>,
    pub logs: Vec<LogEntry>,
    pub aggregated_metrics: VecDeque<nexus_common::proto::generic::ObservationMetrics>,
}

impl Observation {
    pub fn new(
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        replicant_tags: Vec<String>,
        number_of_replicants: i32,
        image_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            start_time,
            end_time,
            status: ObservationStatus::Created,
            replicant_tags,
            number_of_replicants,
            image_id,
            created_at: now,
            updated_at: now,
            replicant_ids: Vec::new(),
            logs: Vec::new(),
            aggregated_metrics: VecDeque::with_capacity(256),
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn update_status(&mut self, status: ObservationStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn add_replicant(&mut self, replicant_id: String) {
        if !self.replicant_ids.contains(&replicant_id) {
            self.replicant_ids.push(replicant_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_replicant(&mut self, replicant_id: &str) {
        self.replicant_ids.retain(|id| id != replicant_id);
        self.updated_at = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            ObservationStatus::Running | ObservationStatus::Scheduled
        )
    }

    pub fn is_completed(&self) -> bool {
        matches!(
            self.status,
            ObservationStatus::Completed | ObservationStatus::Cancelled | ObservationStatus::Failed
        )
    }

    pub fn update_aggregated_metrics(
        &mut self,
        metrics: nexus_common::proto::generic::ObservationMetrics,
    ) {
        if self.aggregated_metrics.len() >= 256 {
            self.aggregated_metrics.pop_front();
        }
        self.aggregated_metrics.push_back(metrics);
        self.updated_at = Utc::now();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.start_time >= self.end_time {
            return Err("Start time must be before end time".to_string());
        }

        if self.number_of_replicants <= 0 {
            return Err("Number of replicants must be positive".to_string());
        }

        if self.image_id.is_empty() {
            return Err("Image ID cannot be empty".to_string());
        }

        Ok(())
    }
}

// Conversion from ObservationStatus to protobuf Status
impl From<ObservationStatus> for nexus_common::proto::enums::Status {
    fn from(status: ObservationStatus) -> Self {
        match status {
            ObservationStatus::Created => nexus_common::proto::enums::Status::Standby,
            ObservationStatus::Scheduled => nexus_common::proto::enums::Status::Starting,
            ObservationStatus::Running => nexus_common::proto::enums::Status::Running,
            ObservationStatus::Completed => nexus_common::proto::enums::Status::Stopped,
            ObservationStatus::Cancelled => nexus_common::proto::enums::Status::Stopped,
            ObservationStatus::Failed => nexus_common::proto::enums::Status::Errored,
            ObservationStatus::Disconnected => nexus_common::proto::enums::Status::Disconnected,
        }
    }
}

// Conversion from protobuf Status to ObservationStatus
impl From<nexus_common::proto::enums::Status> for ObservationStatus {
    fn from(proto_status: nexus_common::proto::enums::Status) -> Self {
        match proto_status {
            nexus_common::proto::enums::Status::Unknown => ObservationStatus::Created,
            nexus_common::proto::enums::Status::Standby => ObservationStatus::Created,
            nexus_common::proto::enums::Status::Starting => ObservationStatus::Scheduled,
            nexus_common::proto::enums::Status::Running => ObservationStatus::Running,
            nexus_common::proto::enums::Status::Stopping => ObservationStatus::Completed,
            nexus_common::proto::enums::Status::Stopped => ObservationStatus::Completed,
            nexus_common::proto::enums::Status::Errored => ObservationStatus::Failed,
            nexus_common::proto::enums::Status::Maintenance => ObservationStatus::Created,
            nexus_common::proto::enums::Status::Disconnected => ObservationStatus::Disconnected,
        }
    }
}

// Conversion from Observation to protobuf Observation
impl From<Observation> for nexus_common::proto::generic::Observation {
    fn from(obs: Observation) -> Self {
        nexus_common::proto::generic::Observation {
            observation_id: obs.id,
            start_time: Some(prost_types::Timestamp {
                seconds: obs.start_time.timestamp(),
                nanos: obs.start_time.timestamp_subsec_nanos() as i32,
            }),
            end_time: Some(prost_types::Timestamp {
                seconds: obs.end_time.timestamp(),
                nanos: obs.end_time.timestamp_subsec_nanos() as i32,
            }),
            replicant_tags: obs.replicant_tags,
            number_of_replicants: obs.number_of_replicants,
            image_id: obs.image_id,
            status: nexus_common::proto::enums::Status::from(obs.status) as i32,
            created_at: Some(prost_types::Timestamp {
                seconds: obs.created_at.timestamp(),
                nanos: obs.created_at.timestamp_subsec_nanos() as i32,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: obs.updated_at.timestamp(),
                nanos: obs.updated_at.timestamp_subsec_nanos() as i32,
            }),
            replicant_ids: obs.replicant_ids,
        }
    }
}

// Conversion from protobuf Observation to domain Observation
impl From<nexus_common::proto::generic::Observation> for Observation {
    fn from(proto: nexus_common::proto::generic::Observation) -> Self {
        let id = if proto.observation_id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            proto.observation_id
        };

        let start_time = proto
            .start_time
            .and_then(|ts| DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
            .unwrap_or_else(Utc::now);

        let end_time = proto
            .end_time
            .and_then(|ts| DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
            .unwrap_or_else(|| Utc::now() + chrono::Duration::hours(1));

        let created_at = proto
            .created_at
            .and_then(|ts| DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
            .unwrap_or_else(Utc::now);

        let updated_at = proto
            .updated_at
            .and_then(|ts| DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
            .unwrap_or_else(Utc::now);

        let status = nexus_common::proto::enums::Status::try_from(proto.status)
            .map(ObservationStatus::from)
            .unwrap_or_default();

        Observation {
            id,
            start_time,
            end_time,
            status,
            replicant_tags: proto.replicant_tags,
            number_of_replicants: proto.number_of_replicants,
            image_id: proto.image_id,
            created_at,
            updated_at,
            replicant_ids: proto.replicant_ids,
            logs: Vec::new(),
            aggregated_metrics: VecDeque::new(),
        }
    }
}

impl From<LogLevel> for nexus_common::proto::enums::LogLevel {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => nexus_common::proto::enums::LogLevel::Trace,
            LogLevel::Debug => nexus_common::proto::enums::LogLevel::Debug,
            LogLevel::Info => nexus_common::proto::enums::LogLevel::Info,
            LogLevel::Warn => nexus_common::proto::enums::LogLevel::Warn,
            LogLevel::Error => nexus_common::proto::enums::LogLevel::Error,
        }
    }
}

impl From<&LogEntry> for nexus_common::proto::generic::LogEntry {
    fn from(log: &LogEntry) -> Self {
        nexus_common::proto::generic::LogEntry {
            timestamp: Some(prost_types::Timestamp {
                seconds: log.timestamp.timestamp(),
                nanos: log.timestamp.timestamp_subsec_nanos() as i32,
            }),
            level: nexus_common::proto::enums::LogLevel::from(log.level) as i32,
            message: format!("[{}] {}", log.source, log.message),
            observation_id: String::new(),
        }
    }
}
