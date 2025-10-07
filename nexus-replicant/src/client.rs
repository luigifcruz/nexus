use futures::StreamExt;
use nexus_common::proto::generic::Ack;
use nexus_common::proto::generic::LogEntry;
use nexus_common::proto::replicant::instance_dispatch_envelope::{Command, Payload};
use nexus_common::proto::replicant::replicant_client::ReplicantClient as GrpcReplicantClient;
use nexus_common::proto::replicant::{
    HardwareRequest, InstanceDispatchEnvelope, MetricsRequest, StatusRequest,
};
use tokio::sync::mpsc;
use tonic::transport::Channel;
use tracing::{error, info, warn};

use crate::config::Config;
use crate::docker::DockerManager;
use crate::hardware::build_hardware_spec;
use crate::log_forwarder::LogForwarderHandle;
use crate::metrics::MetricsCollector;

pub struct ReplicantClient {
    config: Config,
    grpc_client: GrpcReplicantClient<Channel>,
    docker_manager: DockerManager,
    metrics_collector: MetricsCollector,
    instance_status: nexus_common::proto::enums::Status,
    status_tx: Option<mpsc::Sender<nexus_common::proto::enums::Status>>,
    log_forwarder: Option<LogForwarderHandle>,
}

impl ReplicantClient {
    pub async fn new(
        config: Config,
        log_tx: mpsc::Sender<LogEntry>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = format!("http://{}:{}", config.nexus.host, config.nexus.port);
        let grpc_client = GrpcReplicantClient::connect(endpoint).await?;

        info!(
            "Connected to nexus server at {}:{}",
            config.nexus.host, config.nexus.port
        );

        let docker_manager =
            DockerManager::new(config.replicant.hardware.clone(), log_tx.clone()).await?;
        let metrics_collector = MetricsCollector::new(config.replicant.hardware.clone());

        Ok(Self {
            config,
            grpc_client,
            docker_manager,
            metrics_collector,
            instance_status: nexus_common::proto::enums::Status::Standby,
            status_tx: None,
            log_forwarder: None,
        })
    }

    pub async fn run(
        &mut self,
        mut log_rx: mpsc::Receiver<LogEntry>,
        log_handle: LogForwarderHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting replicant client");

        let (signaller_tx, mut signaller_rx) = mpsc::channel(32);
        self.start_signaller(signaller_tx).await?;

        let (_instance_tx, mut instance_rx, instance_outbound_tx) =
            self.start_instance_dispatch().await?;

        let (status_tx, mut status_rx) = mpsc::channel(32);
        self.status_tx = Some(status_tx);

        self.log_forwarder = Some(log_handle);

        info!("Replicant client ready");

        loop {
            tokio::select! {
                Some(signal) = signaller_rx.recv() => {
                    self.handle_signaller_response(signal).await;
                }
                Some(envelope) = instance_rx.recv() => {
                    self.handle_instance_command(envelope, instance_outbound_tx.clone()).await;
                }
                Some(_status) = status_rx.recv() => {
                    if let Err(e) = self.push_status().await {
                        error!("Failed to push status update: {}", e);
                    }
                }
                Some(log_entry) = log_rx.recv() => {
                    if let Err(e) = self.push_status_with_log(Some(log_entry)).await {
                        error!("Failed to push log entry: {}", e);
                    }
                }
            }
        }
    }

    async fn start_signaller(
        &mut self,
        tx: mpsc::Sender<nexus_common::proto::replicant::SignallerResponse>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = nexus_common::proto::replicant::SignallerRequest {
            replicant_id: self.config.replicant.replicant_id.clone(),
            version: "0.1.0".to_string(),
        };

        let mut stream = self.grpc_client.signaller(request).await?.into_inner();
        info!("Signaller stream established");

        tokio::spawn(async move {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(response) => {
                        if tx.send(response).await.is_err() {
                            warn!("Failed to forward signaller response, channel closed");
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Signaller stream error: {}", e);
                        break;
                    }
                }
            }
            warn!("Signaller stream task ended");
        });

        Ok(())
    }

    async fn start_instance_dispatch(
        &mut self,
    ) -> Result<
        (
            mpsc::Sender<InstanceDispatchEnvelope>,
            mpsc::Receiver<InstanceDispatchEnvelope>,
            mpsc::Sender<InstanceDispatchEnvelope>,
        ),
        Box<dyn std::error::Error>,
    > {
        let (outbound_tx, outbound_rx) = mpsc::channel(32);
        let (inbound_tx, inbound_rx) = mpsc::channel(32);

        let initial_envelope = InstanceDispatchEnvelope {
            replicant_id: self.config.replicant.replicant_id.clone(),
            command_id: String::new(),
            payload: None,
        };

        outbound_tx.send(initial_envelope).await?;

        let outbound_stream = tokio_stream::wrappers::ReceiverStream::new(outbound_rx);

        let mut response_stream = self
            .grpc_client
            .instance_dispatch(outbound_stream)
            .await?
            .into_inner();

        let outbound_tx_clone = outbound_tx.clone();
        let inbound_tx_clone = inbound_tx.clone();
        tokio::spawn(async move {
            while let Some(result) = response_stream.next().await {
                match result {
                    Ok(envelope) => {
                        if inbound_tx_clone.send(envelope).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Instance dispatch stream error: {}", e);
                        break;
                    }
                }
            }
        });

        Ok((inbound_tx, inbound_rx, outbound_tx_clone))
    }

    async fn handle_signaller_response(
        &mut self,
        response: nexus_common::proto::replicant::SignallerResponse,
    ) {
        if response.push_metrics {
            if let Err(e) = self.push_metrics().await {
                error!("Failed to push metrics: {}", e);
            }
        }

        if response.push_hardware {
            if let Err(e) = self.push_hardware().await {
                error!("Failed to push hardware: {}", e);
            }
        }

        if response.push_status {
            if let Err(e) = self.push_status().await {
                error!("Failed to push status: {}", e);
            }
        }
    }

    async fn handle_instance_command(
        &mut self,
        envelope: InstanceDispatchEnvelope,
        outbound_tx: mpsc::Sender<InstanceDispatchEnvelope>,
    ) {
        let command_id = envelope.command_id.clone();

        if let Some(Payload::Command(command)) = envelope.payload {
            let result = self.execute_command(command).await;

            let ack = Ack {
                success: result.is_ok(),
                message: result.err().map(|e| e.to_string()).unwrap_or_default(),
            };

            if ack.success {
                info!("Command {} executed successfully", command_id);
            } else {
                error!(
                    "Command failed: command_id={} error={}",
                    command_id, ack.message
                );
            }

            let response = InstanceDispatchEnvelope {
                replicant_id: self.config.replicant.replicant_id.clone(),
                command_id: command_id.clone(),
                payload: Some(Payload::Ack(ack)),
            };

            if let Err(e) = outbound_tx.send(response).await {
                error!("Failed to send ack: command_id={} error={}", command_id, e);
            }
        } else {
            warn!(
                "Received envelope without command payload: command_id={}",
                command_id
            );
        }
    }

    async fn execute_command(
        &mut self,
        command: Command,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(action) = command.action {
            match action {
                nexus_common::proto::replicant::instance_dispatch_envelope::command::Action::Create(req) => {
                    self.create_instance(&req.replicant_id, &req.docker_image, &req.docker_entrypoint, req.docker_args, req.docker_env).await?;
                }
                nexus_common::proto::replicant::instance_dispatch_envelope::command::Action::Destroy(req) => {
                    self.destroy_instance(&req.id).await?;
                }
            }
        }

        Ok(())
    }

    async fn push_metrics(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let metrics = self.metrics_collector.collect();

        let request = MetricsRequest {
            replicant_id: self.config.replicant.replicant_id.clone(),
            metrics: Some(metrics),
        };

        self.grpc_client.push_metrics(request).await?;

        Ok(())
    }

    async fn push_hardware(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let hardware = build_hardware_spec(&self.config.replicant.hardware);

        let request = HardwareRequest {
            replicant_id: self.config.replicant.replicant_id.clone(),
            spec: Some(hardware),
        };

        self.grpc_client.push_hardware(request).await?;
        info!("Hardware spec published to server");

        Ok(())
    }

    async fn push_status(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.push_status_with_log(None).await
    }

    async fn push_status_with_log(
        &mut self,
        log: Option<LogEntry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = StatusRequest {
            replicant_id: self.config.replicant.replicant_id.clone(),
            status: nexus_common::proto::enums::Status::Running.into(),
            log,
        };

        self.grpc_client.push_status(request).await?;

        Ok(())
    }

    fn set_instance_status(&mut self, status: nexus_common::proto::enums::Status) {
        self.instance_status = status;
        if let Some(tx) = &self.status_tx {
            let _ = tx.try_send(status);
        }
    }

    async fn create_instance(
        &mut self,
        replicant_id: &str,
        docker_image: &str,
        docker_entrypoint: &str,
        docker_args: Vec<String>,
        docker_env: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Enable log forwarding when observation starts
        if let Some(handle) = &self.log_forwarder {
            handle.enable();
        }

        info!(
            "Creating instance: replicant_id={} docker_image={} entrypoint={} args={:?} env={:?}",
            replicant_id, docker_image, docker_entrypoint, docker_args, docker_env
        );

        self.set_instance_status(nexus_common::proto::enums::Status::Starting);

        match self
            .docker_manager
            .create_instance(
                replicant_id,
                docker_image,
                docker_entrypoint,
                docker_args,
                docker_env,
            )
            .await
        {
            Ok(_) => {
                info!(
                    "Successfully created instance: replicant_id={}",
                    replicant_id
                );
                self.set_instance_status(nexus_common::proto::enums::Status::Running);
                Ok(())
            }
            Err(e) => {
                error!(
                    "Failed to create instance: replicant_id={} error={}",
                    replicant_id, e
                );
                self.set_instance_status(nexus_common::proto::enums::Status::Errored);
                Err(e)
            }
        }
    }

    async fn destroy_instance(
        &mut self,
        replicant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Destroying instance: replicant_id={}", replicant_id);

        self.set_instance_status(nexus_common::proto::enums::Status::Stopping);

        match self.docker_manager.destroy_instance(replicant_id).await {
            Ok(_) => {
                info!(
                    "Successfully destroyed instance: replicant_id={}",
                    replicant_id
                );
                self.set_instance_status(nexus_common::proto::enums::Status::Standby);

                // Disable log forwarding when observation ends
                if let Some(handle) = &self.log_forwarder {
                    handle.disable();
                }

                Ok(())
            }
            Err(e) => {
                error!(
                    "Failed to destroy instance: replicant_id={} error={}",
                    replicant_id, e
                );
                self.set_instance_status(nexus_common::proto::enums::Status::Errored);
                Err(e)
            }
        }
    }
}
