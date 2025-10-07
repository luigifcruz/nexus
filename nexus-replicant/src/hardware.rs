use nexus_common::proto::enums::{CpuType, GpuType, NetworkSpeed, NetworkType};
use nexus_common::proto::generic::HardwareSpec;
use std::path::Path;
use tracing::info;

use crate::config::HardwareConfig;

pub fn print_hardware_config(config: &HardwareConfig) {
    info!("Hardware configuration:");
    info!("  GPU: {} (PCIe: {})", config.gpu_type, config.gpu_pcie_id);
    info!(
        "  CPU: {} (Socket: {}, Threads: {:?})",
        config.cpu_type, config.cpu_socket, config.cpu_threads
    );
    info!("  Memory: {} GB", config.memory_size / 1024);
    info!(
        "  Storage: {} GB (Path: {})",
        config.storage_size / 1024,
        config.storage_path
    );
    info!(
        "  Network: {} {} (Interface: {}, PCIe: {})",
        config.network_type, config.network_speed, config.network_interface, config.network_pcie_id
    );
}

pub fn validate_hardware(config: &HardwareConfig) -> Result<(), Box<dyn std::error::Error>> {
    validate_gpu(&config.gpu_pcie_id)?;
    validate_network_interface(&config.network_interface)?;
    validate_storage_path(&config.storage_path)?;
    validate_cpu_threads(&config.cpu_threads)?;
    Ok(())
}

fn validate_gpu(pcie_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sysfs_path = format!("/sys/bus/pci/devices/0000:{}", pcie_id);
    if !Path::new(&sysfs_path).exists() {
        return Err(format!("GPU PCIe device {} not found at {}", pcie_id, sysfs_path).into());
    }
    Ok(())
}

fn validate_network_interface(interface: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sysfs_path = format!("/sys/class/net/{}", interface);
    if !Path::new(&sysfs_path).exists() {
        return Err(format!("Network interface {} not found", interface).into());
    }
    Ok(())
}

fn validate_storage_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err(format!("Storage path {} does not exist", path).into());
    }
    if !Path::new(path).is_dir() {
        return Err(format!("Storage path {} is not a directory", path).into());
    }
    Ok(())
}

fn validate_cpu_threads(threads: &[i32]) -> Result<(), Box<dyn std::error::Error>> {
    if threads.is_empty() {
        return Err("CPU threads list cannot be empty".into());
    }

    let max_cpu = std::fs::read_to_string("/sys/devices/system/cpu/present")
        .and_then(|content| {
            let parts: Vec<&str> = content.trim().split('-').collect();
            if parts.len() == 2 {
                Ok(parts[1].parse::<i32>().unwrap_or(0))
            } else {
                Ok(0)
            }
        })
        .unwrap_or(0);

    for &thread in threads {
        if thread < 0 || thread > max_cpu {
            return Err(format!("CPU thread {} is out of range (0-{})", thread, max_cpu).into());
        }
    }

    Ok(())
}

pub fn build_hardware_spec(config: &HardwareConfig) -> HardwareSpec {
    let gpu_type = match config.gpu_type.as_str() {
        "NVIDIA_RTX6000_ADA" => GpuType::NvidiaRtx6000Ada,
        "NVIDIA_A4000" => GpuType::NvidiaA4000,
        _ => GpuType::NvidiaRtx6000Ada,
    };

    let cpu_type = match config.cpu_type.as_str() {
        "AMD_EPYC_MILAN" => CpuType::AmdEpycMilan,
        "AMD_EPYC_GENOA" => CpuType::AmdEpycGenoa,
        _ => CpuType::AmdEpycMilan,
    };

    let network_type = match config.network_type.as_str() {
        "CONNECTX6" => NetworkType::Connectx6,
        "CONNECTX7" => NetworkType::Connectx7,
        _ => NetworkType::Connectx6,
    };

    let network_speed = match config.network_speed.as_str() {
        "GBE1" => NetworkSpeed::Gbe1,
        "GBE10" => NetworkSpeed::Gbe10,
        "GBE25" => NetworkSpeed::Gbe25,
        "GBE40" => NetworkSpeed::Gbe40,
        "GBE100" => NetworkSpeed::Gbe100,
        "GBE200" => NetworkSpeed::Gbe200,
        _ => NetworkSpeed::Gbe100,
    };

    HardwareSpec {
        gpu_type: gpu_type.into(),
        gpu_pcie_id: config.gpu_pcie_id.clone(),
        cpu_type: cpu_type.into(),
        cpu_socket: config.cpu_socket.clone(),
        cpu_threads: config.cpu_threads.clone(),
        memory_size: config.memory_size,
        storage_size: config.storage_size,
        storage_path: config.storage_path.clone(),
        network_type: network_type.into(),
        network_speed: network_speed.into(),
        network_pcie_id: config.network_pcie_id.clone(),
        network_interface: config.network_interface.clone(),
        network_mac: config.network_mac.clone(),
    }
}
