use std::sync::Arc;
use tracing::{debug, trace, warn};

use crate::domain::services::{ObservationMetricsAggregator, ObservationService, ReplicantService};
use crate::domain::DomainError;
use crate::transport::connections::ConnectionManager;

pub struct ObservationMetricsPublisher {
    observation_service: Arc<ObservationService>,
    replicant_service: Arc<ReplicantService>,
    connection_manager: Arc<ConnectionManager>,
    observation_metrics_aggregator: Arc<ObservationMetricsAggregator>,
}

impl ObservationMetricsPublisher {
    pub fn new(
        observation_service: Arc<ObservationService>,
        replicant_service: Arc<ReplicantService>,
        connection_manager: Arc<ConnectionManager>,
        observation_metrics_aggregator: Arc<ObservationMetricsAggregator>,
    ) -> Self {
        Self {
            observation_service,
            replicant_service,
            connection_manager,
            observation_metrics_aggregator,
        }
    }

    pub async fn initialize(&self) -> Result<(), DomainError> {
        debug!("Metrics publisher initialized...");
        Ok(())
    }

    pub async fn run(&self) -> Result<(), DomainError> {
        debug!("Publishing metrics...");

        self.request_metrics_from_replicants().await?;
        self.publish_observation_metrics().await?;

        Ok(())
    }

    async fn request_metrics_from_replicants(&self) -> Result<(), DomainError> {
        // Get all connected replicants
        let connected_replicants = self.connection_manager.get_connected_replicants().await;

        if connected_replicants.is_empty() {
            debug!("No replicants connected to request metrics from");
            return Ok(());
        }

        trace!(
            "Requesting metrics from {} replicants",
            connected_replicants.len()
        );

        // Request metrics from all connected replicants
        if let Err(e) = self
            .connection_manager
            .request_metrics(&connected_replicants)
            .await
        {
            warn!("Failed to request metrics from replicants: {}", e);
        }

        Ok(())
    }

    pub async fn terminate(&self) -> Result<(), DomainError> {
        debug!("Metrics publisher terminated...");
        Ok(())
    }

    async fn publish_observation_metrics(&self) -> Result<(), DomainError> {
        // Get all active observations
        let observations = self.observation_service.find_all().await?;

        for observation in observations {
            // Check if observation is active (Running or Scheduled)
            if !observation.is_active() {
                continue;
            }

            // Aggregate metrics for this observation
            match self
                .observation_metrics_aggregator
                .aggregate_observation_metrics(&observation.id)
                .await
            {
                Ok(aggregated_metrics) => {
                    trace!(
                        "Publishing aggregated metrics for observation {}",
                        observation.id
                    );

                    // Store in observation history
                    let _ = self
                        .observation_service
                        .update_aggregated_metrics(&observation.id, aggregated_metrics.clone())
                        .await;

                    // Publish to subscribers via service
                    let _ = self
                        .observation_service
                        .publish_observation_metrics(&observation.id, aggregated_metrics)
                        .await;
                }
                Err(e) => {
                    debug!(
                        "Failed to aggregate metrics for observation {}: {}",
                        observation.id, e
                    );
                }
            }
        }

        Ok(())
    }
}
