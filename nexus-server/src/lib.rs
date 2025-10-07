//! Nexus Server Library
//!
//! A clean-architecture server implementation for managing distributed compute resources.

pub mod app;
pub mod application;
pub mod domain;
pub mod repositories;
pub mod transport;

// Re-export commonly used items
pub use app::Container;
pub use domain::{
    entities::{Hardware, Instance, Metrics, Observation, ObservationStatus, Replicant, Status},
    services::{InstanceService, ObservationService, ReplicantService},
};
pub use repositories::{
    HardwareRepository, InstanceRepository, MetricsRepository, ObservationRepository,
    ReplicantRepository,
};
pub use transport::{
    connections::ConnectionManager,
    grpc::{MetaHandler, NexusHandler, ReplicantHandler},
};
