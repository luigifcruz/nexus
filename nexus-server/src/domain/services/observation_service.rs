use std::sync::Arc;

use crate::domain::entities::{LogEntry, Observation, ObservationStatus};
use crate::domain::services::{InstanceService, ReplicantService};
use crate::domain::DomainError;
use crate::repositories::traits::ObservationRepository;
use crate::transport::connections::SubscriptionManager;
use chrono::{DateTime, Utc};

pub struct ObservationService {
    observation_repository: Arc<dyn ObservationRepository>,
    subscription_manager: Option<Arc<SubscriptionManager>>,
}

impl ObservationService {
    pub fn new(observation_repository: Arc<dyn ObservationRepository>) -> Self {
        Self {
            observation_repository,
            subscription_manager: None,
        }
    }

    pub fn with_subscription_manager(mut self, manager: Arc<SubscriptionManager>) -> Self {
        self.subscription_manager = Some(manager);
        self
    }

    pub async fn create_observation(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        replicant_tags: Vec<String>,
        number_of_replicants: i32,
        image_id: String,
    ) -> Result<Observation, DomainError> {
        let now = Utc::now();

        // Time validation
        if start_time <= now {
            return Err(DomainError::new("Start time must be in the future"));
        }

        if end_time <= now {
            return Err(DomainError::new("End time must be in the future"));
        }

        if start_time >= end_time {
            return Err(DomainError::new("Start time must be before end time"));
        }

        // Basic validation
        if number_of_replicants <= 0 {
            return Err(DomainError::new("Number of replicants must be positive"));
        }

        if image_id.is_empty() {
            return Err(DomainError::new("Image ID cannot be empty"));
        }

        // Create new observation
        let observation = Observation::new(
            start_time,
            end_time,
            replicant_tags,
            number_of_replicants,
            image_id,
        );

        // Validate the observation
        observation.validate().map_err(|e| DomainError::new(e))?;

        // Save to repository
        self.observation_repository
            .save(&observation)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        // Notify subscribers of new observation
        if let Some(manager) = &self.subscription_manager {
            manager.notify_observation_created(observation.clone());
        }

        Ok(observation)
    }

    pub async fn destroy_observation(&self, id: &str) -> Result<(), DomainError> {
        // Find the observation
        let mut observation = self
            .observation_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", id)))?;

        // Mark as cancelled
        observation.update_status(ObservationStatus::Cancelled);

        // Update in repository
        self.observation_repository
            .update(&observation)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        // Notify subscribers of observation destruction
        if let Some(manager) = &self.subscription_manager {
            manager.notify_observation_updated(observation);
        }

        Ok(())
    }

    pub async fn get_observation(&self, id: &str) -> Result<Option<Observation>, DomainError> {
        self.observation_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Observation>, DomainError> {
        self.observation_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_all(&self) -> Result<Vec<Observation>, DomainError> {
        self.observation_repository
            .find_all()
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_by_status(
        &self,
        status: ObservationStatus,
    ) -> Result<Vec<Observation>, DomainError> {
        self.observation_repository
            .find_by_status(status)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn find_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Observation>, DomainError> {
        self.observation_repository
            .find_by_time_range(start, end)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn update_status(
        &self,
        id: &str,
        status: ObservationStatus,
    ) -> Result<(), DomainError> {
        // Find the observation
        let mut observation = self
            .observation_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", id)))?;

        // Check if status actually changed
        let status_changed = observation.status != status;

        // Update status
        observation.update_status(status);

        // Save to repository
        self.observation_repository
            .update(&observation)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        // Only notify subscribers if status actually changed
        if status_changed {
            if let Some(manager) = &self.subscription_manager {
                manager.notify_observation_updated(observation);
            }
        }

        Ok(())
    }

    pub async fn add_replicant_to_observation(
        &self,
        observation_id: &str,
        replicant_id: String,
    ) -> Result<(), DomainError> {
        // Find the observation
        let mut observation = self
            .observation_repository
            .find_by_id(observation_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        // Add replicant
        observation.add_replicant(replicant_id);

        // Save to repository
        self.observation_repository
            .update(&observation)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        Ok(())
    }

    pub async fn validate_observation(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        number_of_replicants: i32,
        image_id: &str,
    ) -> Result<(), DomainError> {
        let now = Utc::now();

        // Time validation
        if start_time <= now {
            return Err(DomainError::new("Start time must be in the future"));
        }

        if end_time <= now {
            return Err(DomainError::new("End time must be in the future"));
        }

        if start_time >= end_time {
            return Err(DomainError::new("Start time must be before end time"));
        }

        if number_of_replicants <= 0 {
            return Err(DomainError::new("Number of replicants must be positive"));
        }

        if image_id.is_empty() {
            return Err(DomainError::new("Image ID cannot be empty"));
        }

        Ok(())
    }

    /// Start an observation by provisioning instances on selected replicants
    /// Returns list of (instance_id, replicant_id) tuples for command dispatch
    pub async fn start_observation(
        &self,
        observation_id: &str,
        available_replicant_ids: &[String],
        replicant_service: &ReplicantService,
        _instance_service: &InstanceService,
    ) -> Result<Vec<(String, String)>, DomainError> {
        // Get observation
        let observation = self
            .find_by_id(observation_id)
            .await?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        // Select replicants for this observation
        let selected_replicants = self
            .select_replicants_for_observation(
                &observation,
                available_replicant_ids,
                replicant_service,
            )
            .await?;

        // Validate we have enough replicants
        if selected_replicants.len() < observation.number_of_replicants as usize {
            return Err(DomainError::new(format!(
                "Not enough replicants available: need {}, found {}",
                observation.number_of_replicants,
                selected_replicants.len()
            )));
        }

        // Create instances (one per replicant, instance_id = replicant_id-observation_id)
        // Note: This method signature needs to be updated to accept image_id instead
        let mut commands = Vec::new();
        for replicant_id in selected_replicants
            .into_iter()
            .take(observation.number_of_replicants as usize)
        {
            let instance_id = format!("{}-{}", replicant_id, observation_id); // Make unique per observation

            // Instance creation will be handled differently - we pass image_id
            commands.push((instance_id, replicant_id));
        }

        // Update observation status to Running
        self.update_status(observation_id, ObservationStatus::Running)
            .await?;

        Ok(commands)
    }

    /// Select replicants for an observation based on tags and availability
    async fn select_replicants_for_observation(
        &self,
        observation: &Observation,
        available_ids: &[String],
        replicant_service: &ReplicantService,
    ) -> Result<Vec<String>, DomainError> {
        // Get available replicant entities
        let available = replicant_service.find_by_ids(available_ids).await?;

        // Filter by tags using "any tag" matching
        let matching = if observation.replicant_tags.is_empty() {
            // No tags specified, use all available
            available
        } else {
            // Match replicants that have at least one of the required tags
            available
                .into_iter()
                .filter(|r| {
                    observation
                        .replicant_tags
                        .iter()
                        .any(|tag| r.tags.contains(tag))
                })
                .collect()
        };

        Ok(matching.into_iter().map(|r| r.id).collect())
    }

    /// Complete an observation and return instances to destroy
    pub async fn complete_observation(
        &self,
        observation_id: &str,
        instance_service: &InstanceService,
    ) -> Result<Vec<(String, String)>, DomainError> {
        // Get observation
        let observation = self
            .find_by_id(observation_id)
            .await?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        // Get all instances for this observation's replicants
        let all_instances = instance_service.find_all().await?;
        let observation_instances: Vec<_> = all_instances
            .into_iter()
            .filter(|inst| observation.replicant_ids.contains(&inst.replicant_id))
            .map(|inst| (inst.id, inst.replicant_id))
            .collect();

        // Update observation status to Completed
        self.update_status(observation_id, ObservationStatus::Completed)
            .await?;

        // Return instances to destroy
        Ok(observation_instances)
    }

    /// Fail an observation and return instances to destroy
    pub async fn fail_observation(
        &self,
        observation_id: &str,
        instance_service: &InstanceService,
    ) -> Result<Vec<(String, String)>, DomainError> {
        let observation = self
            .find_by_id(observation_id)
            .await?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        let all_instances = instance_service.find_all().await?;
        let observation_instances: Vec<_> = all_instances
            .into_iter()
            .filter(|inst| observation.replicant_ids.contains(&inst.replicant_id))
            .collect();

        // Destroy all instances for this observation
        for instance in &observation_instances {
            if let Err(e) = instance_service
                .destroy_instance(&instance.replicant_id, &instance.id)
                .await
            {
                tracing::warn!(
                    "Failed to destroy instance {} during observation failure: {}",
                    instance.id,
                    e
                );
            }
        }

        self.update_status(observation_id, ObservationStatus::Failed)
            .await?;

        // Return instances for sending destroy commands to replicants
        Ok(observation_instances
            .into_iter()
            .map(|inst| (inst.id, inst.replicant_id))
            .collect())
    }

    /// Perform health check on a running observation
    /// Returns instances to destroy if observation failed, empty vec otherwise
    pub async fn health_check(
        &self,
        observation_id: &str,
        replicant_service: &ReplicantService,
        instance_service: &InstanceService,
    ) -> Result<Vec<(String, String)>, DomainError> {
        let observation = self
            .observation_repository
            .find_by_id(observation_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        // Check if observation has ended
        if observation.end_time <= chrono::Utc::now() {
            self.update_status(observation_id, ObservationStatus::Completed)
                .await?;
            return Ok(vec![]);
        }

        // Check all replicants assigned to this observation
        for replicant_id in &observation.replicant_ids {
            match replicant_service.find_by_id(replicant_id).await {
                Ok(Some(replicant)) => {
                    // Check if replicant is disconnected or in error state
                    if replicant.status.is_disconnected() || replicant.status.is_error() {
                        let instances = self
                            .fail_observation(observation_id, instance_service)
                            .await?;
                        return Ok(instances);
                    }

                    if let Some(instance) = &replicant.instance {
                        // Check if instance is in error state or stopped
                        if instance.status.is_error() || instance.status.is_inactive() {
                            let instances = self
                                .fail_observation(observation_id, instance_service)
                                .await?;
                            return Ok(instances);
                        }
                    } else if observation.status == ObservationStatus::Running {
                        // Observation is running but replicant has no instance - something failed
                        let instances = self
                            .fail_observation(observation_id, instance_service)
                            .await?;
                        return Ok(instances);
                    }
                }
                Ok(None) => {
                    // Replicant no longer exists - fail observation
                    let instances = self
                        .fail_observation(observation_id, instance_service)
                        .await?;
                    return Ok(instances);
                }
                Err(_) => {
                    // Error fetching replicant - continue checking others
                    continue;
                }
            }
        }

        Ok(vec![])
    }

    /// Find observation that contains a specific replicant
    pub async fn find_by_replicant_id(
        &self,
        replicant_id: &str,
    ) -> Result<Option<Observation>, DomainError> {
        let all_observations = self.find_all().await?;
        Ok(all_observations
            .into_iter()
            .filter(|obs| obs.replicant_ids.contains(&replicant_id.to_string()))
            .max_by_key(|obs| obs.start_time))
    }

    /// Append a log entry to an observation
    pub async fn append_log(
        &self,
        observation_id: &str,
        log_entry: LogEntry,
    ) -> Result<(), DomainError> {
        let mut observation = self
            .observation_repository
            .find_by_id(observation_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        observation.logs.push(log_entry.clone());

        self.observation_repository
            .update(&observation)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        Ok(())
    }

    /// Get aggregated metrics history for an observation
    pub async fn get_aggregated_metrics(
        &self,
        observation_id: &str,
        limit: usize,
    ) -> Result<Vec<nexus_common::proto::generic::ObservationMetrics>, DomainError> {
        let observation = self
            .observation_repository
            .find_by_id(observation_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        // Get last N metrics, reversed to get oldest first
        let metrics: Vec<_> = observation
            .aggregated_metrics
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect();

        // Reverse again to return chronological order
        Ok(metrics.into_iter().rev().collect())
    }

    /// Update aggregated metrics for an observation
    pub async fn update_aggregated_metrics(
        &self,
        observation_id: &str,
        metrics: nexus_common::proto::generic::ObservationMetrics,
    ) -> Result<(), DomainError> {
        let mut observation = self
            .observation_repository
            .find_by_id(observation_id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        observation.update_aggregated_metrics(metrics);

        self.observation_repository
            .update(&observation)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        Ok(())
    }

    /// Publish observation metrics to subscribers
    pub async fn publish_observation_metrics(
        &self,
        observation_id: &str,
        aggregated_metrics: nexus_common::proto::generic::ObservationMetrics,
    ) -> Result<(), DomainError> {
        if let Some(manager) = &self.subscription_manager {
            manager.publish_observation_metrics(observation_id, aggregated_metrics);
        }
        Ok(())
    }
}
