use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};

use crate::domain::entities::ObservationStatus;
use crate::domain::services::{
    ImageService, InstanceService, ObservationService, ReplicantService,
};
use crate::domain::DomainError;
use crate::transport::connections::{
    ConnectionManager, CreateInstanceCommand, DestroyInstanceCommand,
};

pub struct ObservationScheduler {
    observation_service: Arc<ObservationService>,
    instance_service: Arc<InstanceService>,
    replicant_service: Arc<ReplicantService>,
    image_service: Arc<ImageService>,
    connection_manager: Arc<ConnectionManager>,
}

impl ObservationScheduler {
    pub fn new(
        observation_service: Arc<ObservationService>,
        instance_service: Arc<InstanceService>,
        replicant_service: Arc<ReplicantService>,
        image_service: Arc<ImageService>,
        connection_manager: Arc<ConnectionManager>,
    ) -> Self {
        Self {
            observation_service,
            instance_service,
            replicant_service,
            image_service,
            connection_manager,
        }
    }

    pub async fn initialize(&self) -> Result<(), DomainError> {
        debug!("Observation scheduler initialized");
        Ok(())
    }

    pub async fn run(&self) -> Result<(), DomainError> {
        self.process_pending_observations().await?;
        self.monitor_running_observations().await?;
        self.cleanup_completed_observations().await?;
        Ok(())
    }

    pub async fn terminate(&self) -> Result<(), DomainError> {
        debug!("Observation scheduler terminated");
        Ok(())
    }

    /// Process observations that are ready to start
    async fn process_pending_observations(&self) -> Result<(), DomainError> {
        let now = chrono::Utc::now();

        // Find Created observations ready to start
        let mut created = self
            .observation_service
            .find_by_status(ObservationStatus::Created)
            .await?;
        let mut scheduled = self
            .observation_service
            .find_by_status(ObservationStatus::Scheduled)
            .await?;

        // Combine observations
        created.append(&mut scheduled);

        // Split into valid (ready to start) and expired (past end time)
        let (expired, pending): (Vec<_>, Vec<_>) =
            created.into_iter().partition(|obs| obs.end_time <= now);

        // Drop expired observations
        for observation in expired {
            warn!(
                "Dropping expired observation: id={} end_time={}",
                observation.id,
                observation.end_time.format("%Y-%m-%d %H:%M:%S UTC")
            );
            let _ = self
                .observation_service
                .fail_observation(&observation.id, &self.instance_service)
                .await;
        }

        // Filter observations ready to start
        let ready_to_start: Vec<_> = pending
            .into_iter()
            .filter(|obs| obs.start_time <= now)
            .collect();

        for observation in ready_to_start {
            info!("Starting observation: {}", observation.id);

            let available_replicants = self.connection_manager.get_connected_replicants().await;

            if available_replicants.is_empty() {
                warn!(
                    "No connected replicants available for observation {}. Will retry later.",
                    observation.id
                );
                self.observation_service
                    .update_status(&observation.id, ObservationStatus::Scheduled)
                    .await?;

                continue;
            }

            let commands = match self
                .observation_service
                .start_observation(
                    &observation.id,
                    &available_replicants,
                    &self.replicant_service,
                    &self.instance_service,
                )
                .await
            {
                Ok(cmds) => cmds,
                Err(e) => {
                    error!("Failed to start observation {}: {}", observation.id, e);
                    let _ = self
                        .observation_service
                        .fail_observation(&observation.id, &self.instance_service)
                        .await;
                    continue;
                }
            };

            info!(
                "Dispatching {} create commands for observation {}",
                commands.len(),
                observation.id
            );

            // Fetch the image for this observation
            let image = match self.image_service.get_image(&observation.image_id).await {
                Ok(Some(img)) => img,
                Ok(None) => {
                    error!(
                        "Image {} not found for observation {}",
                        observation.image_id, observation.id
                    );
                    let _ = self
                        .observation_service
                        .fail_observation(&observation.id, &self.instance_service)
                        .await;
                    continue;
                }
                Err(e) => {
                    error!(
                        "Failed to fetch image {} for observation {}: {}",
                        observation.image_id, observation.id, e
                    );
                    let _ = self
                        .observation_service
                        .fail_observation(&observation.id, &self.instance_service)
                        .await;
                    continue;
                }
            };

            for (_, replicant_id) in &commands {
                self.observation_service
                    .add_replicant_to_observation(&observation.id, replicant_id.clone())
                    .await?;
            }

            for (instance_id, replicant_id) in &commands {
                info!(
                    "Sending create instance command: instance_id={} replicant_id={} docker_image={}",
                    instance_id, replicant_id, image.docker_image
                );

                let command = CreateInstanceCommand {
                    instance_id: instance_id.clone(),
                    replicant_id: replicant_id.clone(),
                    docker_image: image.docker_image.clone(),
                    docker_entrypoint: image.docker_entrypoint.clone(),
                    docker_args: image.docker_args.clone(),
                    docker_env: image.docker_env.clone(),
                };

                if let Err(e) = self
                    .connection_manager
                    .send_create_instance_command(command)
                    .await
                {
                    error!(
                        "Failed to send create instance command for instance_id={}: {}",
                        instance_id, e
                    );
                    let _ = self
                        .observation_service
                        .fail_observation(&observation.id, &self.instance_service)
                        .await;
                } else {
                    continue;
                }
            }

            info!(
                "Successfully started observation {} with {} instances",
                observation.id,
                commands.len()
            );
        }

        Ok(())
    }

    /// Monitor running observations and request status updates
    async fn monitor_running_observations(&self) -> Result<(), DomainError> {
        let running = self
            .observation_service
            .find_by_status(ObservationStatus::Running)
            .await?;

        for observation in running {
            trace!("Monitoring observation: {}", observation.id);

            // Request status update from all replicants for this observation
            if let Err(e) = self
                .connection_manager
                .request_status(&observation.replicant_ids)
                .await
            {
                warn!(
                    "Failed to request status for observation {}: {}",
                    observation.id, e
                );
            }

            // Perform health check (checks for instance errors and end time)
            match self
                .observation_service
                .health_check(
                    &observation.id,
                    &self.replicant_service,
                    &self.instance_service,
                )
                .await
            {
                Ok(instances_to_destroy) => {
                    for (instance_id, replicant_id) in instances_to_destroy {
                        info!(
                            "Destroying instance {} for failed observation {}",
                            instance_id, observation.id
                        );

                        let command = DestroyInstanceCommand {
                            instance_id: instance_id.clone(),
                            replicant_id: replicant_id.clone(),
                        };

                        if let Err(e) = self
                            .connection_manager
                            .send_destroy_instance_command(command)
                            .await
                        {
                            warn!("Failed to destroy instance {}: {}", instance_id, e);
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to perform health check for observation {}: {}",
                        observation.id, e
                    );
                }
            }
        }

        Ok(())
    }

    /// Cleanup observations that have completed
    async fn cleanup_completed_observations(&self) -> Result<(), DomainError> {
        let completed = self
            .observation_service
            .find_by_status(ObservationStatus::Completed)
            .await?;

        for observation in completed {
            trace!("Checking cleanup for observation: {}", observation.id);

            let instances_to_destroy = self
                .observation_service
                .complete_observation(&observation.id, &self.instance_service)
                .await?;

            for (instance_id, replicant_id) in instances_to_destroy {
                debug!(
                    "Destroying instance {} for completed observation {}",
                    instance_id, observation.id
                );

                let command = DestroyInstanceCommand {
                    instance_id: instance_id.clone(),
                    replicant_id: replicant_id.clone(),
                };

                if let Err(e) = self
                    .connection_manager
                    .send_destroy_instance_command(command)
                    .await
                {
                    warn!("Failed to destroy instance {}: {}", instance_id, e);
                }
            }
        }

        Ok(())
    }
}
