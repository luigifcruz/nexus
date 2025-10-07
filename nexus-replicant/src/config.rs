use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub nexus: NexusConfig,
    pub replicant: ReplicantConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicantConfig {
    pub replicant_id: String,
    pub hardware: HardwareConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub gpu_type: String,
    pub gpu_pcie_id: String,
    pub cpu_type: String,
    pub cpu_socket: String,
    pub cpu_threads: Vec<i32>,
    pub memory_size: i32,
    pub storage_size: i32,
    pub storage_path: String,
    pub network_type: String,
    pub network_speed: String,
    pub network_interface: String,
    pub network_pcie_id: String,
    pub network_mac: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;

        if config.version != "1.0" {
            return Err("Unsupported config version".into());
        }

        Ok(config)
    }
}
