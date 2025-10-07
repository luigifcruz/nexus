use nexus_common::proto::generic::{
    CpuMetrics, GpuMetrics, HardwareMetrics, MemoryMetrics, NetworkMetrics, StorageMetrics,
};
use nvml_wrapper::Nvml;
use std::sync::Mutex;
use std::time::SystemTime;
use sysinfo::{Disks, Networks, System};

use crate::config::HardwareConfig;

struct StorageIoStats {
    read_bytes: u64,
    write_bytes: u64,
    timestamp: SystemTime,
}

pub struct MetricsCollector {
    system: System,
    disks: Disks,
    networks: Networks,
    nvml: Option<Nvml>,
    hardware_config: HardwareConfig,
    storage_device: Option<String>,
    last_io_stats: Mutex<Option<StorageIoStats>>,
}

impl MetricsCollector {
    pub fn new(hardware_config: HardwareConfig) -> Self {
        let nvml = Nvml::init().ok();
        let storage_device = Self::find_storage_device(&hardware_config.storage_path);

        Self {
            system: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            nvml,
            hardware_config,
            storage_device,
            last_io_stats: Mutex::new(None),
        }
    }

    pub fn collect(&mut self) -> HardwareMetrics {
        self.system.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);

        HardwareMetrics {
            timestamp: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
            cpu: Some(self.collect_cpu()),
            memory: Some(self.collect_memory()),
            storage: Some(self.collect_storage()),
            network: Some(self.collect_network()),
            gpu: Some(self.collect_gpu()),
            replicant_id: String::new(),
        }
    }

    fn collect_cpu(&self) -> CpuMetrics {
        let cpus = self.system.cpus();

        // Only collect metrics for the configured CPU threads
        let per_core_usage: Vec<f32> = self
            .hardware_config
            .cpu_threads
            .iter()
            .filter_map(|&thread_id| cpus.get(thread_id as usize).map(|cpu| cpu.cpu_usage()))
            .collect();

        let usage_percent = if !per_core_usage.is_empty() {
            per_core_usage.iter().sum::<f32>() / per_core_usage.len() as f32
        } else {
            0.0
        };

        CpuMetrics {
            usage_percent,
            temperature_celsius: 0.0, // sysinfo doesn't provide this easily
            per_core_usage,
        }
    }

    fn collect_memory(&self) -> MemoryMetrics {
        let total_bytes = self.system.total_memory();
        let used_bytes = self.system.used_memory();

        // Report based on configured memory size if smaller than actual
        let config_memory_bytes = (self.hardware_config.memory_size as u64) * 1024 * 1024;
        let reported_total = config_memory_bytes.min(total_bytes);
        let reported_used = used_bytes.min(reported_total);

        let usage_percent = if reported_total > 0 {
            (reported_used as f32 / reported_total as f32) * 100.0
        } else {
            0.0
        };

        MemoryMetrics {
            used_bytes: reported_used,
            total_bytes: reported_total,
            usage_percent,
        }
    }

    fn collect_storage(&self) -> StorageMetrics {
        // Find the disk that matches our configured storage path
        let storage_path = &self.hardware_config.storage_path;

        let (total_bytes, used_bytes) = self
            .disks
            .iter()
            .find(|disk| {
                disk.mount_point()
                    .to_str()
                    .map(|mp| storage_path.starts_with(mp))
                    .unwrap_or(false)
            })
            .map(|disk| {
                let total = disk.total_space();
                let used = total - disk.available_space();
                (total, used)
            })
            .unwrap_or((0, 0));

        let usage_percent = if total_bytes > 0 {
            (used_bytes as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };

        // Collect I/O bandwidth
        let (read_bps, write_bps) = self.collect_storage_io_bandwidth();

        StorageMetrics {
            used_bytes,
            total_bytes,
            usage_percent,
            read_bps,
            write_bps,
        }
    }

    fn collect_network(&self) -> NetworkMetrics {
        // Use the configured network interface
        let network_interface = &self.hardware_config.network_interface;

        let (rx_bytes, tx_bytes) = self
            .networks
            .iter()
            .find(|(name, _)| *name == network_interface)
            .map(|(_, network)| (network.received(), network.transmitted()))
            .unwrap_or((0, 0));

        NetworkMetrics {
            rx_bytes,
            tx_bytes,
            rx_bandwidth_bps: 0.0, // Would need rate calculation
            tx_bandwidth_bps: 0.0,
        }
    }

    fn collect_gpu(&self) -> GpuMetrics {
        if let Some(ref nvml) = self.nvml {
            // Try to find GPU by PCI ID
            let gpu_pcie_id = &self.hardware_config.gpu_pcie_id;

            // Try to match by PCI bus ID
            for i in 0..nvml.device_count().unwrap_or(0) {
                if let Ok(device) = nvml.device_by_index(i) {
                    if let Ok(pci_info) = device.pci_info() {
                        let pcie_id = format!("{:02x}:{:02x}.0", pci_info.bus, pci_info.device);

                        if pcie_id == *gpu_pcie_id || gpu_pcie_id.ends_with(&pcie_id) {
                            let utilization = device.utilization_rates().ok();
                            let memory = device.memory_info().ok();
                            let temperature = device
                                .temperature(
                                    nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu,
                                )
                                .ok();
                            let power = device.power_usage().ok();

                            return GpuMetrics {
                                compute_usage_percent: utilization
                                    .as_ref()
                                    .map(|u| u.gpu as f32)
                                    .unwrap_or(0.0),
                                memory_used_bytes: memory.as_ref().map(|m| m.used).unwrap_or(0),
                                memory_total_bytes: memory.as_ref().map(|m| m.total).unwrap_or(0),
                                memory_usage_percent: utilization
                                    .as_ref()
                                    .map(|u| u.memory as f32)
                                    .unwrap_or(0.0),
                                pcie_rx_bandwidth_bps: 0,
                                pcie_tx_bandwidth_bps: 0,
                                power_usage_watts: power.map(|p| p as f32 / 1000.0).unwrap_or(0.0),
                                temperature_celsius: temperature.map(|t| t as f32).unwrap_or(0.0),
                            };
                        }
                    }
                }
            }
        }

        GpuMetrics::default()
    }

    /// Find the block device for a given storage path
    fn find_storage_device(storage_path: &str) -> Option<String> {
        use std::process::Command;

        // Use df to find the device for the storage path
        let output = Command::new("df")
            .arg("--output=source")
            .arg(storage_path)
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8(output.stdout).ok()?;
        let device = stdout.lines().nth(1)?.trim();

        // Extract device name (e.g., /dev/nvme0n1p1 -> nvme0n1)
        // Handle partitions by removing partition number
        let device_name = device.trim_start_matches("/dev/");

        // Remove partition suffix (e.g., nvme0n1p1 -> nvme0n1, sda1 -> sda)
        let base_device = if device_name.contains("nvme") {
            // For NVMe: nvme0n1p1 -> nvme0n1
            device_name.split('p').next().unwrap_or(device_name)
        } else {
            // For regular disks: sda1 -> sda
            device_name.trim_end_matches(|c: char| c.is_ascii_digit())
        };

        Some(base_device.to_string())
    }

    /// Collect storage I/O bandwidth by reading /sys/block/*/stat
    fn collect_storage_io_bandwidth(&self) -> (u64, u64) {
        let device = match &self.storage_device {
            Some(d) => d,
            None => return (0, 0),
        };

        // Read I/O stats from /sys/block/<device>/stat
        let stats_path = format!("/sys/block/{}/stat", device);
        let stats_content = match std::fs::read_to_string(&stats_path) {
            Ok(content) => content,
            Err(_) => return (0, 0),
        };

        // Parse diskstats format (see https://www.kernel.org/doc/Documentation/iostats.txt)
        // Field 3: sectors read, Field 7: sectors written
        let fields: Vec<&str> = stats_content.split_whitespace().collect();
        if fields.len() < 7 {
            return (0, 0);
        }

        let sectors_read: u64 = fields[2].parse().unwrap_or(0);
        let sectors_written: u64 = fields[6].parse().unwrap_or(0);

        // Convert sectors to bytes (512 bytes per sector)
        let read_bytes = sectors_read * 512;
        let write_bytes = sectors_written * 512;

        let now = SystemTime::now();

        // Calculate rate based on previous measurement
        let mut last_stats = self.last_io_stats.lock().unwrap();

        let (read_bps, write_bps) = if let Some(ref prev) = *last_stats {
            let time_delta = now
                .duration_since(prev.timestamp)
                .unwrap_or_default()
                .as_secs_f64();

            if time_delta > 0.0 {
                let read_delta = read_bytes.saturating_sub(prev.read_bytes);
                let write_delta = write_bytes.saturating_sub(prev.write_bytes);

                let read_bps = (read_delta as f64 / time_delta) as u64;
                let write_bps = (write_delta as f64 / time_delta) as u64;

                (read_bps, write_bps)
            } else {
                (0, 0)
            }
        } else {
            // First measurement - no rate to calculate
            (0, 0)
        };

        // Store current stats for next calculation
        *last_stats = Some(StorageIoStats {
            read_bytes,
            write_bytes,
            timestamp: now,
        });

        (read_bps, write_bps)
    }
}
