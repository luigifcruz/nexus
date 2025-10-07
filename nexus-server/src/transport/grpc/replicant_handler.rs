use serde::de;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, info, warn};

use nexus_common::proto::generic::Empty;
use nexus_common::proto::replicant::instance_dispatch_envelope::Payload;
use nexus_common::proto::replicant::replicant_server::Replicant;
use nexus_common::proto::replicant::{
    HardwareRequest, InstanceDispatchEnvelope, MetricsRequest, SignallerRequest, SignallerResponse,
    StatusRequest,
};

use crate::domain::entities::{LogEntry, LogLevel};
use crate::domain::services::{InstanceService, ObservationService, ReplicantService};
use crate::transport::connections::{ConnectionManager, SubscriptionManager};

pub use nexus_common::proto::replicant::replicant_server::ReplicantServer;
pub use nexus_common::proto::replicant::FILE_DESCRIPTOR_SET as ReplicantFileDescriptorSet;

pub struct ReplicantHandler {
    replicant_service: Arc<ReplicantService>,
    instance_service: Arc<InstanceService>,
    observation_service: Arc<ObservationService>,
    connection_manager: Arc<ConnectionManager>,
    subscription_manager: Arc<SubscriptionManager>,
}

impl ReplicantHandler {
    pub fn new(
        replicant_service: Arc<ReplicantService>,
        instance_service: Arc<InstanceService>,
        observation_service: Arc<ObservationService>,
        connection_manager: Arc<ConnectionManager>,
        subscription_manager: Arc<SubscriptionManager>,
    ) -> Self {
        Self {
            replicant_service,
            instance_service,
            observation_service,
            connection_manager,
            subscription_manager,
        }
    }
}

#[tonic::async_trait]
impl Replicant for ReplicantHandler {
    async fn push_metrics(
        &self,
        request: Request<MetricsRequest>,
    ) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let replicant_id = inner.replicant_id;

        if replicant_id.is_empty() {
            return Err(Status::invalid_argument("Missing replicant ID"));
        }

        if let Some(proto_metrics) = inner.metrics {
            let metrics = proto_metrics.into();

            if let Err(e) = self
                .replicant_service
                .update_metrics(&replicant_id, metrics)
                .await
            {
                warn!("Failed to update metrics for {}: {}", replicant_id, e);
                return Err(Status::internal(format!("Failed to update metrics: {}", e)));
            }
        }

        Ok(Response::new(Empty::default()))
    }

    async fn push_hardware(
        &self,
        request: Request<HardwareRequest>,
    ) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let replicant_id = inner.replicant_id;

        if replicant_id.is_empty() {
            return Err(Status::invalid_argument("Missing replicant ID"));
        }

        if let Some(proto_hardware) = inner.spec {
            let hardware = proto_hardware.into();

            match self
                .replicant_service
                .update_hardware(&replicant_id, hardware)
                .await
            {
                Ok(_) => info!("Hardware updated for replicant {}", replicant_id),
                Err(e) => {
                    warn!("Failed to update hardware for {}: {}", replicant_id, e);
                    return Err(Status::internal(format!(
                        "Failed to update hardware: {}",
                        e
                    )));
                }
            }
        }

        Ok(Response::new(Empty::default()))
    }

    async fn push_status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();
        let replicant_id = inner.replicant_id.clone();

        if replicant_id.is_empty() {
            return Err(Status::invalid_argument("Missing replicant ID"));
        }

        let replicant_status = inner.status().into();

        if let Err(e) = self
            .replicant_service
            .update_status(&replicant_id, replicant_status)
            .await
        {
            warn!("Failed to update status for {}: {}", replicant_id, e);
            return Err(Status::internal(format!("Failed to update status: {}", e)));
        }

        // Process log entry if present
        if let Some(proto_log) = inner.log {
            // Convert proto log level to domain log level
            let level = match proto_log.level() {
                nexus_common::proto::enums::LogLevel::Trace => LogLevel::Trace,
                nexus_common::proto::enums::LogLevel::Debug => LogLevel::Debug,
                nexus_common::proto::enums::LogLevel::Info => LogLevel::Info,
                nexus_common::proto::enums::LogLevel::Warn => LogLevel::Warn,
                nexus_common::proto::enums::LogLevel::Error => LogLevel::Error,
            };

            // Convert proto timestamp to chrono DateTime
            let timestamp = proto_log
                .timestamp
                .as_ref()
                .and_then(|ts| chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
                .unwrap_or_else(chrono::Utc::now);

            // Find observation for this replicant
            match self
                .observation_service
                .find_by_replicant_id(&replicant_id)
                .await
            {
                Ok(Some(observation)) => {
                    let log_entry = LogEntry {
                        timestamp,
                        level,
                        message: proto_log.message.clone(),
                        source: replicant_id.clone(),
                    };

                    if let Err(e) = self
                        .observation_service
                        .append_log(&observation.id, log_entry)
                        .await
                    {
                        warn!("Failed to append log for {}: {}", replicant_id, e);
                    }

                    self.subscription_manager
                        .publish_log(&observation.id, proto_log);
                }
                Ok(None) => {}
                Err(e) => warn!("Failed to find observation for {}: {}", replicant_id, e),
            }
        }

        Ok(Response::new(Empty::default()))
    }

    type SignallerStream = ReceiverStream<Result<SignallerResponse, Status>>;

    async fn signaller(
        &self,
        request: Request<SignallerRequest>,
    ) -> Result<Response<Self::SignallerStream>, Status> {
        let inner = request.into_inner();

        // Check the replicant version
        let version = inner.version;
        if version != "0.1.0" {
            return Err(Status::invalid_argument(format!(
                "Unsupported replicant `version` expected {}!",
                "0.1.0"
            )));
        }

        // Check replicant ID
        let replicant_id = inner.replicant_id;

        if replicant_id.is_empty() {
            return Err(Status::invalid_argument("Missing replicant ID"));
        }

        if let Err(e) = self
            .replicant_service
            .register_replicant(replicant_id.clone(), version.clone())
            .await
        {
            error!("Failed to register replicant {}: {}", replicant_id, e);
            return Err(Status::internal(format!(
                "Failed to register replicant: {}",
                e
            )));
        }

        info!("Replicant {} connected (version {})", replicant_id, version);

        let (tx, rx) = mpsc::channel(4);

        if let Err(e) = self
            .connection_manager
            .register_replicant_signaller(replicant_id.clone(), tx.clone())
            .await
        {
            error!("Failed to register signaller for {}: {}", replicant_id, e);
            return Err(Status::internal(format!(
                "Failed to register signaller: {}",
                e
            )));
        }

        // Request initial hardware, status, and metrics on connect
        let replicant_ids = vec![replicant_id.clone()];
        let _ = self
            .connection_manager
            .request_hardware(&replicant_ids)
            .await;
        let _ = self.connection_manager.request_status(&replicant_ids).await;
        let _ = self
            .connection_manager
            .request_metrics(&replicant_ids)
            .await;

        // Install keep-alive timer
        {
            let connection_manager = Arc::clone(&self.connection_manager);
            let replicant_service = Arc::clone(&self.replicant_service);
            let id_replicant_clone = replicant_id.clone();

            tokio::spawn(async move {
                let mut tick = tokio::time::interval(std::time::Duration::from_secs(5));

                loop {
                    tick.tick().await;

                    if connection_manager
                        .send_keep_alive(&id_replicant_clone)
                        .await
                        .is_err()
                    {
                        // Connection lost, unregister and mark as disconnected
                        let _ = connection_manager
                            .unregister_replicant(&id_replicant_clone)
                            .await;
                        let _ = replicant_service
                            .update_status(
                                &id_replicant_clone,
                                crate::domain::entities::Status::Disconnected,
                            )
                            .await;
                        break;
                    }
                }
            });
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type InstanceDispatchStream = ReceiverStream<Result<InstanceDispatchEnvelope, Status>>;

    async fn instance_dispatch(
        &self,
        request: Request<Streaming<InstanceDispatchEnvelope>>,
    ) -> Result<Response<Self::InstanceDispatchStream>, Status> {
        let mut inbound = request.into_inner();

        // Wait for the first message to register method
        let first = inbound
            .message()
            .await
            .map_err(|e| Status::unknown(format!("stream read error: {e}")))?
            .ok_or_else(|| Status::invalid_argument("empty stream: missing initial envelope"))?;

        let replicant_id = first.replicant_id.clone();

        if replicant_id.is_empty() {
            return Err(Status::invalid_argument(
                "missing replicant_id in first envelope",
            ));
        }

        // Check if replicant is connected
        if !self
            .connection_manager
            .is_replicant_connected(&replicant_id)
            .await
        {
            return Err(Status::permission_denied("replicant not registered"));
        }

        // Create channel for outbound messages
        let (tx, rx) = mpsc::channel(16);

        // Register instance dispatch channel with connection manager
        if let Err(e) = self
            .connection_manager
            .register_instance_dispatch(replicant_id.clone(), tx.clone())
            .await
        {
            return Err(Status::internal(format!(
                "Failed to register instance dispatch: {}",
                e
            )));
        }

        // Process first message if it contains an acknowledgment
        if let Some(Payload::Ack(ack)) = &first.payload {
            self.handle_acknowledgment(&replicant_id, &first.command_id, ack)
                .await?;
        }

        // Install inbound listener to process acknowledgments
        {
            let connection_manager = Arc::clone(&self.connection_manager);
            let instance_service = Arc::clone(&self.instance_service);
            let replicant_id = replicant_id.clone();

            tokio::spawn(async move {
                loop {
                    let msg = match inbound.message().await {
                        Ok(Some(m)) => m,
                        Ok(None) => break,
                        Err(e) => {
                            warn!("Signaller stream error: {}", e);
                            break;
                        }
                    };

                    // Handle acknowledgment
                    if let Some(Payload::Ack(ack)) = &msg.payload {
                        let success = ack.success;
                        let message = if ack.message.is_empty() {
                            None
                        } else {
                            Some(ack.message.clone())
                        };

                        // Update command status in instance service
                        let _ = instance_service
                            .handle_command_acknowledgment(
                                &msg.command_id,
                                success,
                                message.clone(),
                            )
                            .await;

                        // Notify connection manager of acknowledgment
                        let _ = connection_manager
                            .handle_command_acknowledgment(
                                &replicant_id,
                                &msg.command_id,
                                success,
                                message,
                            )
                            .await;
                    }
                }
            });
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

impl ReplicantHandler {
    async fn handle_acknowledgment(
        &self,
        replicant_id: &str,
        command_id: &str,
        ack: &nexus_common::proto::generic::Ack,
    ) -> Result<(), Status> {
        let success = ack.success;
        let message = if ack.message.is_empty() {
            None
        } else {
            Some(ack.message.clone())
        };

        // Update command status in instance service
        if let Err(e) = self
            .instance_service
            .handle_command_acknowledgment(command_id, success, message.clone())
            .await
        {
            return Err(Status::internal(format!(
                "Failed to handle acknowledgment: {}",
                e
            )));
        }

        // Notify connection manager of acknowledgment
        if let Err(e) = self
            .connection_manager
            .handle_command_acknowledgment(replicant_id, command_id, success, message)
            .await
        {
            return Err(Status::internal(format!(
                "Failed to handle acknowledgment: {}",
                e
            )));
        }

        Ok(())
    }
}
