use bollard::container::{
    Config as ContainerConfig, CreateContainerOptions, RemoveContainerOptions,
    StartContainerOptions, StopContainerOptions,
};
use bollard::image::CreateImageOptions;
use bollard::models::{DeviceRequest, HostConfig, Mount, MountTypeEnum};
use bollard::Docker;

use futures::StreamExt;
use nexus_common::proto::generic::LogEntry;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::config::HardwareConfig;
use crate::docker_logs;

pub struct DockerManager {
    docker: Docker,
    containers: HashMap<String, String>, // instance_id -> container_id
    hardware_config: HardwareConfig,
    log_tx: mpsc::Sender<LogEntry>,
}

impl DockerManager {
    pub async fn new(
        hardware_config: HardwareConfig,
        log_tx: mpsc::Sender<LogEntry>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self {
            docker,
            containers: HashMap::new(),
            hardware_config,
            log_tx,
        })
    }

    pub async fn create_instance(
        &mut self,
        replicant_id: &str,
        docker_image: &str,
        docker_entrypoint: &str,
        docker_args: Vec<String>,
        docker_env: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Creating instance {} with image {}, entrypoint: '{}', args: {:?}, env vars: {} provided",
            replicant_id,
            docker_image,
            if docker_entrypoint.is_empty() { "<default>" } else { docker_entrypoint },
            docker_args,
            docker_env.len()
        );

        // Remove any existing container with the same name
        self.cleanup_existing_container(replicant_id).await?;

        // Pull image if needed
        self.pull_image(docker_image).await?;

        // Create container labels
        let mut labels = HashMap::new();
        labels.insert("nexus.replicant_id".to_string(), replicant_id.to_string());
        labels.insert("nexus.docker_image".to_string(), docker_image.to_string());
        labels.insert(
            "nexus.component".to_string(),
            "replicant-instance".to_string(),
        );
        labels.insert(
            "nexus.created_at".to_string(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        );

        if !docker_entrypoint.is_empty() {
            labels.insert(
                "nexus.entrypoint".to_string(),
                docker_entrypoint.to_string(),
            );
        }

        let gpu_index = self.get_gpu_index_from_pci(&self.hardware_config.gpu_pcie_id)?;

        let device_requests = vec![DeviceRequest {
            driver: Some("nvidia".to_string()),
            count: None,
            device_ids: Some(vec![gpu_index.clone()]),
            capabilities: Some(vec![vec![
                "gpu".to_string(),
                "compute".to_string(),
                "utility".to_string(),
            ]]),
            options: Some(HashMap::new()),
        }];

        // Configure CPU pinning
        let cpuset_cpus = self
            .hardware_config
            .cpu_threads
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(",");

        // Configure storage mount
        let mounts = vec![Mount {
            target: Some("/workspace".to_string()),
            source: Some(self.hardware_config.storage_path.clone()),
            typ: Some(MountTypeEnum::BIND),
            read_only: Some(false),
            ..Default::default()
        }];

        // Configure memory limit (convert MB to bytes)
        let memory = (self.hardware_config.memory_size as i64) * 1024 * 1024;

        let host_config = HostConfig {
            device_requests: Some(device_requests),
            cpuset_cpus: Some(cpuset_cpus),
            mounts: Some(mounts),
            memory: Some(memory),
            memory_swap: Some(memory), // Same as memory to disable swap
            ..Default::default()
        };

        // Convert and validate docker_env to format expected by Docker API
        let mut env_vars: Vec<String> = Vec::new();

        // Add default environment variables
        env_vars.push(format!("NEXUS_REPLICANT_ID={}", replicant_id));
        env_vars.push(format!(
            "NEXUS_CREATED_AT={}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        // Validate and add user-provided environment variables
        for env_var in docker_env {
            if env_var.is_empty() {
                continue; // Skip empty environment variables
            }

            // Validate that env var is in KEY=VALUE format
            if !env_var.contains('=') {
                return Err(format!(
                    "Invalid environment variable format: '{}'. Expected KEY=VALUE format.",
                    env_var
                )
                .into());
            }

            // Check for duplicate keys
            let key = env_var.split('=').next().unwrap();
            if env_vars
                .iter()
                .any(|existing| existing.starts_with(&format!("{}=", key)))
            {
                return Err(format!("Duplicate environment variable key: '{}'", key).into());
            }

            env_vars.push(env_var);
        }

        let config = ContainerConfig {
            image: Some(docker_image.to_string()),
            entrypoint: if docker_entrypoint.is_empty() {
                None
            } else {
                Some(
                    docker_entrypoint
                        .split_whitespace()
                        .map(String::from)
                        .collect(),
                )
            },
            cmd: if docker_args.is_empty() {
                None
            } else {
                Some(docker_args)
            },
            env: Some(env_vars.clone()), // Always set env vars (we have at least the default ones)
            labels: Some(labels.clone()),
            host_config: Some(host_config),
            ..Default::default()
        };

        info!(
            "Container config for instance {}: CPU threads: {}, Memory: {}MB, GPU: {}, Storage mount: {}, Environment variables: {}",
            replicant_id,
            self.hardware_config.cpu_threads.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","),
            self.hardware_config.memory_size,
            gpu_index,
            self.hardware_config.storage_path,
            env_vars.len()
        );

        let options = CreateContainerOptions {
            name: format!("nexus-{}", replicant_id),
            platform: None,
        };

        let response = self.docker.create_container(Some(options), config).await?;
        let container_id = response.id;

        info!(
            "Created container {} for instance {} with name 'nexus-{}'",
            container_id, replicant_id, replicant_id
        );

        // Start container
        self.docker
            .start_container(&container_id, None::<StartContainerOptions<String>>)
            .await?;

        self.containers
            .insert(replicant_id.to_string(), container_id.clone());

        info!(
            "Successfully started container {} for instance {} - container is now running",
            container_id, replicant_id
        );

        // Start log streaming for the container
        docker_logs::start_log_streaming(
            self.docker.clone(),
            &container_id,
            replicant_id,
            self.log_tx.clone(),
        )
        .await?;

        Ok(())
    }

    async fn cleanup_existing_container(
        &mut self,
        replicant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let container_name = format!("nexus-{}", replicant_id);

        // Check if container exists by trying to inspect it
        match self.docker.inspect_container(&container_name, None).await {
            Ok(container_info) => {
                let container_id = &container_info.id.unwrap_or_else(|| container_name.clone());
                info!(
                    "Found existing container {} with name '{}', removing it",
                    container_id, container_name
                );

                // Stop container if it's running
                let stop_options = StopContainerOptions { t: 10 };
                if let Err(e) = self
                    .docker
                    .stop_container(container_id, Some(stop_options))
                    .await
                {
                    // Ignore errors if container is already stopped
                    info!("Container {} may already be stopped: {}", container_id, e);
                }

                // Remove container
                let remove_options = RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                };

                self.docker
                    .remove_container(container_id, Some(remove_options))
                    .await?;

                // Clean up from our tracking
                self.containers.remove(replicant_id);

                info!(
                    "Successfully removed existing container {} with name '{}'",
                    container_id, container_name
                );
            }
            Err(_) => {
                // Container doesn't exist, which is fine
            }
        }

        Ok(())
    }

    pub async fn destroy_instance(
        &mut self,
        replicant_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Destroying instance {} (container: {})",
            replicant_id,
            self.containers
                .get(replicant_id)
                .unwrap_or(&"<unknown>".to_string())
        );

        if let Some(container_id) = self.containers.get(replicant_id).cloned() {
            // Stop container
            let stop_options = StopContainerOptions {
                t: 10, // 10 second timeout
            };

            if let Err(e) = self
                .docker
                .stop_container(&container_id, Some(stop_options))
                .await
            {
                error!("Failed to stop container {}: {}", container_id, e);
            }

            // Remove container
            let remove_options = RemoveContainerOptions {
                force: true,
                ..Default::default()
            };

            self.docker
                .remove_container(&container_id, Some(remove_options))
                .await?;

            self.containers.remove(replicant_id);

            info!(
                "Successfully destroyed container {} for instance {} - container removed from system",
                container_id, replicant_id
            );
        }

        Ok(())
    }

    async fn pull_image(&self, image: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Pulling image {}", image);

        let options = Some(CreateImageOptions {
            from_image: image,
            ..Default::default()
        });

        let mut stream = self.docker.create_image(options, None, None);

        while let Some(result) = stream.next().await {
            match result {
                Ok(_info) => {
                    // Image pull progress
                }
                Err(e) => {
                    error!("Error pulling image: {}", e);
                    return Err(e.into());
                }
            }
        }

        info!("Successfully pulled image {}", image);
        Ok(())
    }

    fn get_gpu_index_from_pci(&self, pci_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        use std::process::Command;

        let output = Command::new("nvidia-smi")
            .args(&["--query-gpu=index,pci.bus_id", "--format=csv,noheader"])
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "nvidia-smi failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        let stdout = String::from_utf8(output.stdout)?;
        let needle = pci_id.trim().to_ascii_lowercase(); // e.g. "41:00.0"

        for line in stdout.lines() {
            let mut parts = line.split(',').map(|s| s.trim());
            if let (Some(index), Some(bus_id)) = (parts.next(), parts.next()) {
                let hay = bus_id.to_ascii_lowercase(); // e.g. "00000000:41:00.0"

                // exact match or suffix match (handles domain prefixes like "00000000:")
                if hay == needle || hay.ends_with(&needle) {
                    return Ok(index.to_string());
                }
            }
        }

        Err(format!(
            "GPU with PCI ID {} not found. Available: {}",
            pci_id, stdout
        )
        .into())
    }
}
