use tokio::sync::broadcast;

use nexus_common::proto::generic::{HardwareMetrics, LogEntry, ObservationMetrics};

use crate::domain::entities::{Observation, Replicant};

#[derive(Debug, Clone)]
pub enum SubscriptionEvent {
    ObservationCreated(Observation),
    ObservationUpdated(Observation),
    ObservationDeleted(String),
    ReplicantCreated(Replicant),
    ReplicantUpdated(Replicant),
    ReplicantDeleted(String),
    ObservationMetrics(ObservationMetrics),
    HardwareMetrics(HardwareMetrics),
    Log(LogEntry),
}

#[derive(Clone)]
pub struct SubscriptionManager {
    event_tx: broadcast::Sender<SubscriptionEvent>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(1000);
        Self { event_tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SubscriptionEvent> {
        self.event_tx.subscribe()
    }

    pub fn notify_observation_created(&self, observation: Observation) {
        let _ = self
            .event_tx
            .send(SubscriptionEvent::ObservationCreated(observation));
    }

    pub fn notify_observation_updated(&self, observation: Observation) {
        let _ = self
            .event_tx
            .send(SubscriptionEvent::ObservationUpdated(observation));
    }

    pub fn notify_observation_deleted(&self, observation_id: String) {
        let _ = self
            .event_tx
            .send(SubscriptionEvent::ObservationDeleted(observation_id));
    }

    pub fn notify_replicant_created(&self, replicant: Replicant) {
        let _ = self
            .event_tx
            .send(SubscriptionEvent::ReplicantCreated(replicant));
    }

    pub fn notify_replicant_updated(&self, replicant: Replicant) {
        let _ = self
            .event_tx
            .send(SubscriptionEvent::ReplicantUpdated(replicant));
    }

    pub fn notify_replicant_deleted(&self, replicant_id: String) {
        let _ = self
            .event_tx
            .send(SubscriptionEvent::ReplicantDeleted(replicant_id));
    }

    pub fn publish_observation_metrics(
        &self,
        observation_id: &str,
        mut metrics: ObservationMetrics,
    ) {
        metrics.observation_id = observation_id.to_string();
        let _ = self
            .event_tx
            .send(SubscriptionEvent::ObservationMetrics(metrics));
    }

    pub fn publish_hardware_metrics(&self, replicant_id: &str, mut metrics: HardwareMetrics) {
        metrics.replicant_id = replicant_id.to_string();
        let _ = self
            .event_tx
            .send(SubscriptionEvent::HardwareMetrics(metrics));
    }

    pub fn publish_log(&self, observation_id: &str, mut log_entry: LogEntry) {
        log_entry.observation_id = observation_id.to_string();
        let _ = self.event_tx.send(SubscriptionEvent::Log(log_entry));
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}
