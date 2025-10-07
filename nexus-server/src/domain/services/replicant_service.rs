use std::sync::Arc;

use crate::domain::entities::{Hardware, Metrics, Replicant, Status};
use crate::domain::DomainError;
use crate::repositories::traits::{HardwareRepository, MetricsRepository, ReplicantRepository};
use crate::transport::connections::SubscriptionManager;

pub struct ReplicantService {
    replicant_repository: Arc<dyn ReplicantRepository>,
    metrics_repository: Arc<dyn MetricsRepository>,
    hardware_repository: Arc<dyn HardwareRepository>,
    subscription_manager: Option<Arc<SubscriptionManager>>,
}

impl ReplicantService {
    pub fn new(
        replicant_repository: Arc<dyn ReplicantRepository>,
        metrics_repository: Arc<dyn MetricsRepository>,
        hardware_repository: Arc<dyn HardwareRepository>,
    ) -> Self {
        Self {
            replicant_repository,
            metrics_repository,
            hardware_repository,
            subscription_manager: None,
        }
    }

    pub fn with_subscription_manager(mut self, manager: Arc<SubscriptionManager>) -> Self {
        self.subscription_manager = Some(manager);
        self
    }

    pub async fn register_replicant(
        &self,
        id: String,
        version: String,
    ) -> Result<Replicant, DomainError> {
        if let Some(existing) = self.replicant_repository.find_by_id(&id).await? {
            self.replicant_repository.update_last_seen(&id).await?;
            return Ok(existing);
        }

        let replicant = Replicant::new(id.clone(), version);
        self.replicant_repository.save(&replicant).await?;

        if let Some(manager) = &self.subscription_manager {
            manager.notify_replicant_created(replicant.clone());
        }

        Ok(replicant)
    }

    pub async fn update_metrics(
        &self,
        replicant_id: &str,
        metrics: Metrics,
    ) -> Result<(), DomainError> {
        self.metrics_repository
            .save_metrics(replicant_id, &metrics)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        self.replicant_repository
            .update_last_seen(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        if let Some(mut replicant) = self
            .replicant_repository
            .find_by_id(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
        {
            replicant.update_metrics(metrics.clone());
            self.replicant_repository
                .update(&replicant)
                .await
                .map_err(|e| DomainError::new(e.to_string()))?;
        }

        if let Some(manager) = &self.subscription_manager {
            manager.publish_hardware_metrics(replicant_id, metrics.into());
        }

        Ok(())
    }

    pub async fn update_hardware(
        &self,
        replicant_id: &str,
        hardware: Hardware,
    ) -> Result<(), DomainError> {
        self.hardware_repository
            .save(replicant_id, &hardware)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        self.replicant_repository
            .update_last_seen(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        if let Some(mut replicant) = self
            .replicant_repository
            .find_by_id(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
        {
            replicant.update_hardware(hardware);
            self.replicant_repository
                .update(&replicant)
                .await
                .map_err(|e| DomainError::new(e.to_string()))?;

            if let Some(manager) = &self.subscription_manager {
                manager.notify_replicant_updated(replicant);
            }
        }

        Ok(())
    }

    pub async fn update_status(
        &self,
        replicant_id: &str,
        replicant_status: Status,
    ) -> Result<(), DomainError> {
        // Update replicant status
        self.replicant_repository
            .update_status(replicant_id, replicant_status)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        self.replicant_repository
            .update_last_seen(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        if let Some(manager) = &self.subscription_manager {
            if let Some(replicant) = self.replicant_repository.find_by_id(replicant_id).await? {
                manager.notify_replicant_updated(replicant);
            }
        }

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Replicant>, DomainError> {
        self.replicant_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_by_ids(&self, ids: &[String]) -> Result<Vec<Replicant>, DomainError> {
        let all = self.find_all().await?;
        Ok(all.into_iter().filter(|r| ids.contains(&r.id)).collect())
    }

    pub async fn find_all(&self) -> Result<Vec<Replicant>, DomainError> {
        self.replicant_repository
            .find_all()
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_by_tags(&self, tags: &[String]) -> Result<Vec<Replicant>, DomainError> {
        self.replicant_repository
            .find_by_tags(tags)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn get_metrics(
        &self,
        replicant_id: &str,
        limit: usize,
    ) -> Result<Vec<Metrics>, DomainError> {
        self.metrics_repository
            .find_latest(replicant_id, limit)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn get_hardware(&self, replicant_id: &str) -> Result<Option<Hardware>, DomainError> {
        self.hardware_repository
            .find_by_replicant_id(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn remove_replicant(&self, id: &str) -> Result<(), DomainError> {
        self.replicant_repository
            .delete(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn publish_latest_metrics(&self, replicant_id: &str) -> Result<(), DomainError> {
        let metrics_list = self.get_metrics(replicant_id, 1).await?;

        if let Some(manager) = &self.subscription_manager {
            for metrics in metrics_list {
                manager.publish_hardware_metrics(replicant_id, metrics.into());
            }
        }

        Ok(())
    }
}
