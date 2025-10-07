use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};
use tonic::Status;
use uuid::Uuid;

use nexus_common::proto::replicant::{
    instance_dispatch_envelope::{self, Command, Payload},
    InstanceDispatchEnvelope, SignallerResponse,
};

#[derive(Debug, Clone)]
pub struct CreateInstanceCommand {
    pub instance_id: String,
    pub replicant_id: String,
    pub docker_image: String,
    pub docker_entrypoint: String,
    pub docker_args: Vec<String>,
    pub docker_env: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DestroyInstanceCommand {
    pub instance_id: String,
    pub replicant_id: String,
}

#[derive(Debug)]
pub struct ReplicantConnection {
    pub replicant_id: String,
    pub signaller_tx: Option<mpsc::Sender<Result<SignallerResponse, Status>>>,
    pub instance_dispatch_tx: Option<mpsc::Sender<Result<InstanceDispatchEnvelope, Status>>>,
    pub pending_commands: HashMap<String, oneshot::Sender<Result<(), String>>>,
}

impl ReplicantConnection {
    pub fn new(replicant_id: String) -> Self {
        Self {
            replicant_id,
            signaller_tx: None,
            instance_dispatch_tx: None,
            pending_commands: HashMap::new(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.signaller_tx.is_some() || self.instance_dispatch_tx.is_some()
    }
}

pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<String, ReplicantConnection>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_replicant_signaller(
        &self,
        replicant_id: String,
        tx: mpsc::Sender<Result<SignallerResponse, Status>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut connections = self.connections.write().await;
        let connection = connections
            .entry(replicant_id.clone())
            .or_insert_with(|| ReplicantConnection::new(replicant_id));

        connection.signaller_tx = Some(tx);
        Ok(())
    }

    pub async fn register_instance_dispatch(
        &self,
        replicant_id: String,
        tx: mpsc::Sender<Result<InstanceDispatchEnvelope, Status>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut connections = self.connections.write().await;
        let connection = connections
            .entry(replicant_id.clone())
            .or_insert_with(|| ReplicantConnection::new(replicant_id));

        connection.instance_dispatch_tx = Some(tx);
        Ok(())
    }

    pub async fn unregister_replicant(
        &self,
        replicant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut connections = self.connections.write().await;
        connections.remove(replicant_id);
        Ok(())
    }

    pub async fn is_replicant_connected(&self, replicant_id: &str) -> bool {
        let connections = self.connections.read().await;
        connections
            .get(replicant_id)
            .map(|c| c.is_connected())
            .unwrap_or(false)
    }

    pub async fn send_create_instance_command(
        &self,
        command: CreateInstanceCommand,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let command_id = Uuid::new_v4().to_string();

        let envelope = InstanceDispatchEnvelope {
            replicant_id: command.replicant_id.clone(),
            command_id: command_id.clone(),
            payload: Some(Payload::Command(Command {
                action: Some(instance_dispatch_envelope::command::Action::Create(
                    instance_dispatch_envelope::command::CreateInstanceRequest {
                        replicant_id: command.replicant_id.clone(),
                        docker_image: command.docker_image,
                        docker_entrypoint: command.docker_entrypoint,
                        docker_args: command.docker_args,
                        docker_env: command.docker_env,
                    },
                )),
            })),
        };

        self.send_instance_dispatch_envelope(&command.replicant_id, envelope)
            .await
    }

    pub async fn send_destroy_instance_command(
        &self,
        command: DestroyInstanceCommand,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let command_id = Uuid::new_v4().to_string();

        let envelope = InstanceDispatchEnvelope {
            replicant_id: command.replicant_id.clone(),
            command_id: command_id.clone(),
            payload: Some(Payload::Command(Command {
                action: Some(instance_dispatch_envelope::command::Action::Destroy(
                    instance_dispatch_envelope::command::DestroyInstanceRequest {
                        id: command.instance_id.clone(),
                    },
                )),
            })),
        };

        self.send_instance_dispatch_envelope(&command.replicant_id, envelope)
            .await
    }

    async fn send_instance_dispatch_envelope(
        &self,
        replicant_id: &str,
        envelope: InstanceDispatchEnvelope,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;

        let connection = connections
            .get(replicant_id)
            .ok_or_else(|| format!("Replicant {} not connected", replicant_id))?;

        let tx = connection.instance_dispatch_tx.as_ref().ok_or_else(|| {
            format!(
                "Instance dispatch channel not available for replicant {}",
                replicant_id
            )
        })?;

        tx.send(Ok(envelope))
            .await
            .map_err(|_| format!("Failed to send command to replicant {}", replicant_id))?;

        Ok(())
    }

    pub async fn send_create_instance_command_with_ack(
        &self,
        command: CreateInstanceCommand,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let command_id = Uuid::new_v4().to_string();
        let (ack_tx, ack_rx) = oneshot::channel();

        // Store the acknowledgment channel
        {
            let mut connections = self.connections.write().await;
            if let Some(connection) = connections.get_mut(&command.replicant_id) {
                connection
                    .pending_commands
                    .insert(command_id.clone(), ack_tx);
            } else {
                return Err(format!("Replicant {} not connected", command.replicant_id).into());
            }
        }

        let envelope = InstanceDispatchEnvelope {
            replicant_id: command.replicant_id.clone(),
            command_id: command_id.clone(),
            payload: Some(Payload::Command(Command {
                action: Some(instance_dispatch_envelope::command::Action::Create(
                    instance_dispatch_envelope::command::CreateInstanceRequest {
                        replicant_id: command.replicant_id.clone(),
                        docker_image: command.docker_image,
                        docker_entrypoint: command.docker_entrypoint,
                        docker_args: command.docker_args,
                        docker_env: command.docker_env,
                    },
                )),
            })),
        };

        // Send the command
        self.send_instance_dispatch_envelope(&command.replicant_id, envelope)
            .await?;

        // Wait for acknowledgment with timeout
        match tokio::time::timeout(std::time::Duration::from_secs(30), ack_rx).await {
            Ok(Ok(Ok(()))) => Ok(()),
            Ok(Ok(Err(msg))) => Err(msg.into()),
            Ok(Err(_)) => Err("Command acknowledgment channel closed".into()),
            Err(_) => {
                // Remove pending command on timeout
                let mut connections = self.connections.write().await;
                if let Some(connection) = connections.get_mut(&command.replicant_id) {
                    connection.pending_commands.remove(&command_id);
                }
                Err("Command acknowledgment timeout".into())
            }
        }
    }

    pub async fn handle_command_acknowledgment(
        &self,
        replicant_id: &str,
        command_id: &str,
        success: bool,
        message: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut connections = self.connections.write().await;

        if let Some(connection) = connections.get_mut(replicant_id) {
            if let Some(ack_tx) = connection.pending_commands.remove(command_id) {
                let result = if success {
                    Ok(())
                } else {
                    Err(message.unwrap_or_else(|| "Command failed".to_string()))
                };

                // Send acknowledgment (ignore if receiver dropped)
                let _ = ack_tx.send(result);
            }
        }

        Ok(())
    }

    pub async fn request_metrics(
        &self,
        replicant_ids: &[String],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;

        for replicant_id in replicant_ids {
            if let Some(connection) = connections.get(replicant_id) {
                if let Some(tx) = &connection.signaller_tx {
                    let _ = tx
                        .send(Ok(SignallerResponse {
                            push_metrics: true,
                            push_hardware: false,
                            push_status: false,
                        }))
                        .await;
                }
            }
        }

        Ok(())
    }

    pub async fn request_hardware(
        &self,
        replicant_ids: &[String],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;

        for replicant_id in replicant_ids {
            if let Some(connection) = connections.get(replicant_id) {
                if let Some(tx) = &connection.signaller_tx {
                    let _ = tx
                        .send(Ok(SignallerResponse {
                            push_metrics: false,
                            push_hardware: true,
                            push_status: false,
                        }))
                        .await;
                }
            }
        }

        Ok(())
    }

    pub async fn request_status(
        &self,
        replicant_ids: &[String],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;

        for replicant_id in replicant_ids {
            if let Some(connection) = connections.get(replicant_id) {
                if let Some(tx) = &connection.signaller_tx {
                    let _ = tx
                        .send(Ok(SignallerResponse {
                            push_metrics: false,
                            push_hardware: false,
                            push_status: true,
                        }))
                        .await;
                }
            }
        }

        Ok(())
    }

    pub async fn get_connected_replicants(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter(|c| c.is_connected())
            .map(|c| c.replicant_id.clone())
            .collect()
    }

    pub async fn send_keep_alive(
        &self,
        replicant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;

        if let Some(connection) = connections.get(replicant_id) {
            if let Some(tx) = &connection.signaller_tx {
                tx.send(Ok(SignallerResponse {
                    push_metrics: false,
                    push_hardware: false,
                    push_status: false,
                }))
                .await
                .map_err(|_| format!("Failed to send keep-alive to replicant {}", replicant_id))?;
            }
        }

        Ok(())
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
