use std::sync::Arc;

use crate::application::schedulers::{ObservationMetricsPublisher, ObservationScheduler};
use crate::domain::services::{
    ImageService, InstanceService, ObservationMetricsAggregator, ObservationService,
    ReplicantService,
};
use crate::domain::DomainError;
use crate::repositories::implementations::{
    InMemoryHardwareRepository, InMemoryImageRepository, InMemoryInstanceRepository,
    InMemoryMetricsRepository, InMemoryObservationRepository, InMemoryReplicantRepository,
};
use crate::repositories::traits::{
    HardwareRepository, ImageRepository, InstanceRepository, MetricsRepository,
    ObservationRepository, ReplicantRepository,
};
use crate::transport::connections::{ConnectionManager, SubscriptionManager};
use crate::transport::grpc::{InstanceHandler, MetaHandler, NexusHandler, ReplicantHandler};

pub struct Container {
    // Repositories
    replicant_repository: Arc<dyn ReplicantRepository>,
    instance_repository: Arc<dyn InstanceRepository>,
    metrics_repository: Arc<dyn MetricsRepository>,
    hardware_repository: Arc<dyn HardwareRepository>,
    observation_repository: Arc<dyn ObservationRepository>,
    image_repository: Arc<dyn ImageRepository>,

    // Domain Services
    replicant_service: Arc<ReplicantService>,
    instance_service: Arc<InstanceService>,
    observation_service: Arc<ObservationService>,
    image_service: Arc<ImageService>,
    observation_metrics_aggregator: Arc<ObservationMetricsAggregator>,

    connection_manager: Arc<ConnectionManager>,
    subscription_manager: Arc<SubscriptionManager>,

    // Application
    observation_scheduler: Arc<ObservationScheduler>,
    observation_metrics_publisher: Arc<ObservationMetricsPublisher>,

    // Configuration
    version: String,
    scheduler_interval: u64,
}

impl Container {
    pub fn new(version: String, scheduler_interval: u64) -> Result<Self, DomainError> {
        // Initialize repositories (using in-memory implementations)
        let replicant_repository: Arc<dyn ReplicantRepository> =
            Arc::new(InMemoryReplicantRepository::new());
        let instance_repository: Arc<dyn InstanceRepository> =
            Arc::new(InMemoryInstanceRepository::new());
        let metrics_repository: Arc<dyn MetricsRepository> =
            Arc::new(InMemoryMetricsRepository::new());
        let hardware_repository: Arc<dyn HardwareRepository> =
            Arc::new(InMemoryHardwareRepository::new());
        let observation_repository: Arc<dyn ObservationRepository> =
            Arc::new(InMemoryObservationRepository::new());
        let image_repository: Arc<dyn ImageRepository> = Arc::new(InMemoryImageRepository::new());

        let subscription_manager = Arc::new(SubscriptionManager::new());

        let replicant_service = Arc::new(
            ReplicantService::new(
                Arc::clone(&replicant_repository),
                Arc::clone(&metrics_repository),
                Arc::clone(&hardware_repository),
            )
            .with_subscription_manager(Arc::clone(&subscription_manager)),
        );

        let instance_service = Arc::new(
            InstanceService::new(
                Arc::clone(&instance_repository),
                Arc::clone(&replicant_repository),
            )
            .with_subscription_manager(Arc::clone(&subscription_manager)),
        );

        let observation_service = Arc::new(
            ObservationService::new(Arc::clone(&observation_repository))
                .with_subscription_manager(Arc::clone(&subscription_manager)),
        );

        let image_service = Arc::new(ImageService::new(Arc::clone(&image_repository)));

        let observation_metrics_aggregator = Arc::new(ObservationMetricsAggregator::new(
            Arc::clone(&observation_service),
            Arc::clone(&replicant_service),
        ));

        // Initialize transport
        let connection_manager = Arc::new(ConnectionManager::new());

        // Initialize application schedulers
        let observation_scheduler = Arc::new(ObservationScheduler::new(
            Arc::clone(&observation_service),
            Arc::clone(&instance_service),
            Arc::clone(&replicant_service),
            Arc::clone(&image_service),
            Arc::clone(&connection_manager),
        ));

        let observation_metrics_publisher = Arc::new(ObservationMetricsPublisher::new(
            Arc::clone(&observation_service),
            Arc::clone(&replicant_service),
            Arc::clone(&connection_manager),
            Arc::clone(&observation_metrics_aggregator),
        ));

        Ok(Self {
            replicant_repository,
            instance_repository,
            metrics_repository,
            hardware_repository,
            observation_repository,
            image_repository,
            replicant_service,
            instance_service,
            observation_service,
            image_service,
            observation_metrics_aggregator,
            connection_manager,
            subscription_manager,
            observation_scheduler,
            observation_metrics_publisher,
            version,
            scheduler_interval,
        })
    }

    // Getters for services
    pub fn replicant_service(&self) -> Arc<ReplicantService> {
        Arc::clone(&self.replicant_service)
    }

    pub fn instance_service(&self) -> Arc<InstanceService> {
        Arc::clone(&self.instance_service)
    }

    pub fn observation_service(&self) -> Arc<ObservationService> {
        Arc::clone(&self.observation_service)
    }

    pub fn image_service(&self) -> Arc<ImageService> {
        Arc::clone(&self.image_service)
    }

    pub fn observation_metrics_aggregator(&self) -> Arc<ObservationMetricsAggregator> {
        Arc::clone(&self.observation_metrics_aggregator)
    }

    pub fn connection_manager(&self) -> Arc<ConnectionManager> {
        Arc::clone(&self.connection_manager)
    }

    pub fn subscription_manager(&self) -> Arc<SubscriptionManager> {
        Arc::clone(&self.subscription_manager)
    }

    pub fn observation_scheduler(&self) -> Arc<ObservationScheduler> {
        Arc::clone(&self.observation_scheduler)
    }

    pub fn observation_metrics_publisher(&self) -> Arc<ObservationMetricsPublisher> {
        Arc::clone(&self.observation_metrics_publisher)
    }

    // Create gRPC handlers
    pub fn create_meta_handler(&self) -> MetaHandler {
        MetaHandler::new(self.version.clone())
    }

    pub fn create_replicant_handler(&self) -> ReplicantHandler {
        ReplicantHandler::new(
            Arc::clone(&self.replicant_service),
            Arc::clone(&self.instance_service),
            Arc::clone(&self.observation_service),
            Arc::clone(&self.connection_manager),
            Arc::clone(&self.subscription_manager),
        )
    }

    pub fn create_nexus_handler(&self) -> NexusHandler {
        NexusHandler::new(
            Arc::clone(&self.observation_service),
            Arc::clone(&self.replicant_service),
            Arc::clone(&self.instance_service),
            Arc::clone(&self.image_service),
            Arc::clone(&self.subscription_manager),
        )
    }

    pub fn create_instance_handler(&self) -> InstanceHandler {
        InstanceHandler::new(
            Arc::clone(&self.instance_service),
            Arc::clone(&self.connection_manager),
        )
    }

    // Getters for configuration
    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn scheduler_interval(&self) -> u64 {
        self.scheduler_interval
    }

    // Initialize all components
    pub async fn initialize(&self) -> Result<(), DomainError> {
        // Initialize scheduler
        self.observation_scheduler.initialize().await?;

        // Initialize metrics publisher
        self.observation_metrics_publisher.initialize().await?;

        tracing::info!("Container initialized successfully");
        Ok(())
    }

    // Run scheduled tasks
    pub async fn run_schedulers(&self) -> Result<(), DomainError> {
        // Run the observation scheduler
        self.observation_scheduler.run().await?;

        // Run the metrics publisher
        self.observation_metrics_publisher.run().await?;

        Ok(())
    }

    // Shutdown all components
    pub async fn shutdown(&self) -> Result<(), DomainError> {
        // Terminate scheduler
        self.observation_scheduler.terminate().await?;

        // Terminate metrics publisher
        self.observation_metrics_publisher.terminate().await?;

        // Disconnect all replicants
        let connected_replicants = self.connection_manager.get_connected_replicants().await;
        for replicant_id in connected_replicants {
            self.connection_manager
                .unregister_replicant(&replicant_id)
                .await
                .map_err(|e| DomainError::new(e.to_string()))?;
        }

        tracing::info!("Container shut down successfully");
        Ok(())
    }
}
