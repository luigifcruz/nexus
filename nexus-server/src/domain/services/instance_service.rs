use std::sync::Arc;

use crate::domain::entities::{Instance, Replicant, Status};
use crate::domain::DomainError;
use crate::repositories::traits::{InstanceRepository, ReplicantRepository};
use crate::transport::connections::SubscriptionManager;

pub struct InstanceService {
    instance_repository: Arc<dyn InstanceRepository>,
    replicant_repository: Arc<dyn ReplicantRepository>,
    subscription_manager: Option<Arc<SubscriptionManager>>,
}

impl InstanceService {
    pub fn new(
        instance_repository: Arc<dyn InstanceRepository>,
        replicant_repository: Arc<dyn ReplicantRepository>,
    ) -> Self {
        Self {
            instance_repository,
            replicant_repository,
            subscription_manager: None,
        }
    }

    pub fn with_subscription_manager(mut self, manager: Arc<SubscriptionManager>) -> Self {
        self.subscription_manager = Some(manager);
        self
    }

    pub async fn create_instance(
        &self,
        instance_id: String,
        replicant_id: String,
        docker_image: String,
        docker_args: Vec<String>,
        tags: Vec<String>,
    ) -> Result<Instance, DomainError> {
        // Check if replicant exists
        let mut replicant = self
            .replicant_repository
            .find_by_id(&replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Replicant {} not found", replicant_id)))?;

        // Check if replicant already has an instance (one instance per replicant)
        if replicant.has_instance() {
            return Err(DomainError::new(format!(
                "Replicant {} already has an instance",
                replicant_id
            )));
        }

        // Create new instance
        let instance = Instance::new(
            instance_id.clone(),
            replicant_id.clone(),
            docker_image,
            docker_args,
            tags,
        );

        // Save instance to repository
        self.instance_repository
            .save(&instance)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        // Update replicant with instance
        replicant.set_instance(instance.clone());
        self.replicant_repository
            .update(&replicant)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        // Notify subscribers of new instance
        if let Some(manager) = &self.subscription_manager {
            manager.notify_replicant_updated(replicant);
        }

        Ok(instance)
    }

    pub async fn destroy_instance(
        &self,
        replicant_id: &str,
        instance_id: &str,
    ) -> Result<(), DomainError> {
        // Check if instance exists
        let instance = self
            .instance_repository
            .find_by_id(replicant_id, instance_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| {
                DomainError::new(format!(
                    "Instance {} not found for replicant {}",
                    instance_id, replicant_id
                ))
            })?;

        // Remove instance from repository
        self.instance_repository
            .delete(replicant_id, &instance.id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        if let Some(mut replicant) = self
            .replicant_repository
            .find_by_id(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
        {
            replicant.remove_instance();
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
        instance_id: &str,
        status: Status,
    ) -> Result<(), DomainError> {
        // Update instance status in repository
        self.instance_repository
            .update_status(replicant_id, instance_id, status)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        // Update replicant's last_seen
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

    pub async fn handle_command_acknowledgment(
        &self,
        command_id: &str,
        success: bool,
        _message: Option<String>,
    ) -> Result<(), DomainError> {
        // Update command status in repository
        self.instance_repository
            .update_command_status(command_id, success)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        Ok(())
    }

    pub async fn find_by_id(
        &self,
        replicant_id: &str,
        instance_id: &str,
    ) -> Result<Option<Instance>, DomainError> {
        self.instance_repository
            .find_by_id(replicant_id, instance_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_by_replicant_id(
        &self,
        replicant_id: &str,
    ) -> Result<Option<Instance>, DomainError> {
        self.instance_repository
            .find_by_replicant_id(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_all(&self) -> Result<Vec<Instance>, DomainError> {
        self.instance_repository
            .find_all()
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_all_for_replicant(
        &self,
        replicant_id: &str,
    ) -> Result<Vec<Instance>, DomainError> {
        // Since there's only one instance per replicant, return a vector with at most one element
        if let Some(instance) = self
            .instance_repository
            .find_by_replicant_id(replicant_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
        {
            Ok(vec![instance])
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn is_instance_running(
        &self,
        replicant_id: &str,
        instance_id: &str,
    ) -> Result<bool, DomainError> {
        if let Some(instance) = self
            .instance_repository
            .find_by_id(replicant_id, instance_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
        {
            Ok(instance.is_running())
        } else {
            Ok(false)
        }
    }

    pub async fn get_replicant_for_instance(
        &self,
        instance_id: &str,
    ) -> Result<Option<Replicant>, DomainError> {
        // Find all instances and check which one matches
        let instances = self
            .instance_repository
            .find_all()
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;
        for instance in instances {
            if instance.id == instance_id {
                return self
                    .replicant_repository
                    .find_by_id(&instance.replicant_id)
                    .await
                    .map_err(|e| DomainError::new(e.to_string()));
            }
        }
        Ok(None)
    }
}
