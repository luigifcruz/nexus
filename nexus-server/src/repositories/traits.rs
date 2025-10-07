use async_trait::async_trait;

use crate::domain::entities::{
    Hardware, Image, Instance, Metrics, Observation, ObservationStatus, Replicant, Status,
};

#[async_trait]
pub trait ReplicantRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Replicant>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_all(&self) -> Result<Vec<Replicant>, Box<dyn std::error::Error + Send + Sync>>;
    async fn save(
        &self,
        replicant: &Replicant,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update(
        &self,
        replicant: &Replicant,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_tags(
        &self,
        tags: &[String],
    ) -> Result<Vec<Replicant>, Box<dyn std::error::Error + Send + Sync>>;
    async fn update_status(
        &self,
        id: &str,
        status: Status,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update_last_seen(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait InstanceRepository: Send + Sync {
    async fn find_by_id(
        &self,
        replicant_id: &str,
        instance_id: &str,
    ) -> Result<Option<Instance>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_replicant_id(
        &self,
        replicant_id: &str,
    ) -> Result<Option<Instance>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_all(&self) -> Result<Vec<Instance>, Box<dyn std::error::Error + Send + Sync>>;
    async fn save(
        &self,
        instance: &Instance,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update(
        &self,
        instance: &Instance,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(
        &self,
        replicant_id: &str,
        instance_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update_status(
        &self,
        replicant_id: &str,
        instance_id: &str,
        status: Status,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update_command_status(
        &self,
        command_id: &str,
        success: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait MetricsRepository: Send + Sync {
    async fn save_metrics(
        &self,
        replicant_id: &str,
        metrics: &Metrics,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn find_latest(
        &self,
        replicant_id: &str,
        limit: usize,
    ) -> Result<Vec<Metrics>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_time_range(
        &self,
        replicant_id: &str,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Metrics>, Box<dyn std::error::Error + Send + Sync>>;
    async fn delete_old_metrics(
        &self,
        before: chrono::DateTime<chrono::Utc>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait HardwareRepository: Send + Sync {
    async fn save(
        &self,
        replicant_id: &str,
        hardware: &Hardware,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_replicant_id(
        &self,
        replicant_id: &str,
    ) -> Result<Option<Hardware>, Box<dyn std::error::Error + Send + Sync>>;
    async fn update(
        &self,
        replicant_id: &str,
        hardware: &Hardware,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait ObservationRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Observation>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_all(&self) -> Result<Vec<Observation>, Box<dyn std::error::Error + Send + Sync>>;
    async fn save(
        &self,
        observation: &Observation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update(
        &self,
        observation: &Observation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_status(
        &self,
        status: ObservationStatus,
    ) -> Result<Vec<Observation>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Observation>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Image>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_all(&self) -> Result<Vec<Image>, Box<dyn std::error::Error + Send + Sync>>;
    async fn save(&self, image: &Image) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
