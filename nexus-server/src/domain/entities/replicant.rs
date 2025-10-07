use chrono::{DateTime, Utc};
use std::collections::VecDeque;

use super::{Hardware, Instance, Metrics, Status};

#[derive(Debug, Clone)]
pub struct Replicant {
    pub id: String,
    pub version: String,
    pub tags: Vec<String>,
    pub status: Status,
    pub hardware: Hardware,
    pub metrics: VecDeque<Metrics>,
    pub instance: Option<Instance>,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Replicant {
    pub fn new(id: String, version: String) -> Self {
        Self {
            id,
            version,
            tags: Vec::new(),
            status: Status::Unknown,
            hardware: Hardware::default(),
            metrics: VecDeque::with_capacity(256),
            instance: None,
            last_seen: Utc::now(),
            created_at: Utc::now(),
        }
    }

    pub fn update_metrics(&mut self, metrics: Metrics) {
        if self.metrics.len() >= 256 {
            self.metrics.pop_front();
        }
        self.metrics.push_back(metrics);
        self.last_seen = Utc::now();
    }

    pub fn update_hardware(&mut self, hardware: Hardware) {
        self.hardware = hardware;
        self.last_seen = Utc::now();
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        self.last_seen = Utc::now();
    }

    pub fn set_instance(&mut self, instance: Instance) {
        self.instance = Some(instance);
    }

    pub fn remove_instance(&mut self) {
        self.instance = None;
    }

    pub fn has_instance(&self) -> bool {
        self.instance.is_some()
    }

    pub fn get_instance_id(&self) -> Option<String> {
        self.instance.as_ref().map(|i| i.id.clone())
    }
}
