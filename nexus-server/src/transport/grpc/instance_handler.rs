use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, info, warn};

use nexus_common::proto::generic::Empty;
use nexus_common::proto::instance::instance_server::Instance as InstanceService;
use nexus_common::proto::instance::{
    MetricsRequest, RunnerNotifierEnvelope, SignallerRequest, SignallerResponse, StatusRequest,
};

use crate::domain::services::InstanceService as DomainInstanceService;
use crate::transport::connections::ConnectionManager;

pub struct InstanceHandler {
    instance_service: Arc<DomainInstanceService>,
    connection_manager: Arc<ConnectionManager>,
}

impl InstanceHandler {
    pub fn new(
        instance_service: Arc<DomainInstanceService>,
        connection_manager: Arc<ConnectionManager>,
    ) -> Self {
        Self {
            instance_service,
            connection_manager,
        }
    }
}

#[tonic::async_trait]
impl InstanceService for InstanceHandler {
    async fn push_metrics(
        &self,
        request: Request<MetricsRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        let instance_id = req.instance_id.clone();

        info!(
            "Received metrics from instance {}: {} metrics, {} labels",
            instance_id,
            req.metrics.len(),
            req.labels.len()
        );

        // Store metrics (placeholder - implement actual storage)
        // In production, you'd store these in a metrics database

        Ok(Response::new(Empty {}))
    }

    async fn push_status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        let instance_id = req.instance_id.clone();
        let status = req.status();

        info!(
            "Received status from instance {}: {:?}",
            instance_id, status
        );

        // Update instance status
        // Note: We need replicant_id to update, but the proto only has instance_id
        // In production, you'd look up the replicant_id from instance_id

        Ok(Response::new(Empty {}))
    }

    type SignallerStream = ReceiverStream<Result<SignallerResponse, Status>>;

    async fn signaller(
        &self,
        request: Request<SignallerRequest>,
    ) -> Result<Response<Self::SignallerStream>, Status> {
        let req = request.into_inner();
        let instance_id = req.instance_id.clone();

        info!(
            "Instance {} connected to signaller stream (version: {})",
            instance_id, req.version
        );

        let (tx, rx) = mpsc::channel(32);

        // Spawn task to send periodic signals to instance
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));

            loop {
                interval.tick().await;

                let response = SignallerResponse {
                    push_metrics: true,
                    push_status: true,
                };

                if tx.send(Ok(response)).await.is_err() {
                    warn!("Instance {} signaller stream closed", instance_id);
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type RunnerNotifierStream = ReceiverStream<Result<RunnerNotifierEnvelope, Status>>;

    async fn runner_notifier(
        &self,
        request: Request<Streaming<RunnerNotifierEnvelope>>,
    ) -> Result<Response<Self::RunnerNotifierStream>, Status> {
        let mut stream = request.into_inner();

        info!("Instance connected to runner notifier stream");

        let (tx, rx) = mpsc::channel(32);
        let instance_service = self.instance_service.clone();

        // Spawn task to handle incoming messages
        tokio::spawn(async move {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(envelope) => {
                        let command_id = envelope.command_id.clone();

                        // Check if this is an acknowledgment
                        if let Some(
                            nexus_common::proto::instance::runner_notifier_envelope::Payload::Ack(
                                ack,
                            ),
                        ) = envelope.payload
                        {
                            info!(
                                "Received ack for command {}: success={} message={}",
                                command_id, ack.success, ack.message
                            );

                            // Update command status
                            if let Err(e) = instance_service
                                .handle_command_acknowledgment(
                                    &command_id,
                                    ack.success,
                                    Some(ack.message),
                                )
                                .await
                            {
                                error!("Failed to handle command ack: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error reading from runner notifier stream: {}", e);
                        break;
                    }
                }
            }

            info!("Runner notifier stream ended");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
