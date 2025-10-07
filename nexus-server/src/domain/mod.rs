pub mod entities;
pub mod services;

use std::fmt;

pub use entities::{
    CpuMetrics, CpuType, GpuMetrics, GpuType, Hardware, Instance, MemoryMetrics, Metrics,
    NetworkMetrics, NetworkSpeed, NetworkType, Observation, ObservationStatus, Replicant, Status,
    StorageMetrics,
};

pub use services::{InstanceService, ObservationService, ReplicantService};

#[derive(Debug, Clone)]
pub struct DomainError {
    pub message: String,
}

impl DomainError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DomainError {}

impl From<Box<dyn std::error::Error>> for DomainError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        DomainError::new(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for DomainError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        DomainError::new(err.to_string())
    }
}

impl From<String> for DomainError {
    fn from(message: String) -> Self {
        DomainError::new(message)
    }
}

impl From<&str> for DomainError {
    fn from(message: &str) -> Self {
        DomainError::new(message)
    }
}
