use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct CpuMetrics {
    pub usage_percent: f32,
    pub temperature_celsius: f32,
    pub per_core_usage: Vec<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryMetrics {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Default)]
pub struct StorageMetrics {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub usage_percent: f32,
    pub read_bps: u64,
    pub write_bps: u64,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_bandwidth_bps: f32,
    pub tx_bandwidth_bps: f32,
}

#[derive(Debug, Clone, Default)]
pub struct GpuMetrics {
    pub compute_usage_percent: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub memory_usage_percent: f32,
    pub pcie_rx_bandwidth_bps: u64,
    pub pcie_tx_bandwidth_bps: u64,
    pub power_usage_watts: f32,
    pub temperature_celsius: f32,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub timestamp: DateTime<Utc>,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub storage: StorageMetrics,
    pub network: NetworkMetrics,
    pub gpu: GpuMetrics,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            cpu: CpuMetrics::default(),
            memory: MemoryMetrics::default(),
            storage: StorageMetrics::default(),
            network: NetworkMetrics::default(),
            gpu: GpuMetrics::default(),
        }
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_cpu(mut self, cpu: CpuMetrics) -> Self {
        self.cpu = cpu;
        self
    }

    pub fn with_memory(mut self, memory: MemoryMetrics) -> Self {
        self.memory = memory;
        self
    }

    pub fn with_storage(mut self, storage: StorageMetrics) -> Self {
        self.storage = storage;
        self
    }

    pub fn with_network(mut self, network: NetworkMetrics) -> Self {
        self.network = network;
        self
    }

    pub fn with_gpu(mut self, gpu: GpuMetrics) -> Self {
        self.gpu = gpu;
        self
    }
}

// Conversion from protobuf types
impl From<nexus_common::proto::generic::HardwareMetrics> for Metrics {
    fn from(proto: nexus_common::proto::generic::HardwareMetrics) -> Self {
        let timestamp = proto
            .timestamp
            .map(|ts| {
                DateTime::from_timestamp(ts.seconds, ts.nanos as u32).unwrap_or_else(Utc::now)
            })
            .unwrap_or_else(Utc::now);

        Metrics {
            timestamp,
            cpu: proto.cpu.map(CpuMetrics::from).unwrap_or_default(),
            memory: proto.memory.map(MemoryMetrics::from).unwrap_or_default(),
            storage: proto.storage.map(StorageMetrics::from).unwrap_or_default(),
            network: proto.network.map(NetworkMetrics::from).unwrap_or_default(),
            gpu: proto.gpu.map(GpuMetrics::from).unwrap_or_default(),
        }
    }
}

impl From<Metrics> for nexus_common::proto::generic::HardwareMetrics {
    fn from(metrics: Metrics) -> Self {
        nexus_common::proto::generic::HardwareMetrics {
            timestamp: Some(prost_types::Timestamp {
                seconds: metrics.timestamp.timestamp(),
                nanos: metrics.timestamp.timestamp_subsec_nanos() as i32,
            }),
            cpu: Some(metrics.cpu.into()),
            memory: Some(metrics.memory.into()),
            storage: Some(metrics.storage.into()),
            network: Some(metrics.network.into()),
            gpu: Some(metrics.gpu.into()),
            replicant_id: String::new(),
        }
    }
}

impl From<nexus_common::proto::generic::CpuMetrics> for CpuMetrics {
    fn from(proto: nexus_common::proto::generic::CpuMetrics) -> Self {
        CpuMetrics {
            usage_percent: proto.usage_percent,
            temperature_celsius: proto.temperature_celsius,
            per_core_usage: proto.per_core_usage,
        }
    }
}

impl From<CpuMetrics> for nexus_common::proto::generic::CpuMetrics {
    fn from(cpu: CpuMetrics) -> Self {
        nexus_common::proto::generic::CpuMetrics {
            usage_percent: cpu.usage_percent,
            temperature_celsius: cpu.temperature_celsius,
            per_core_usage: cpu.per_core_usage,
        }
    }
}

impl From<nexus_common::proto::generic::MemoryMetrics> for MemoryMetrics {
    fn from(proto: nexus_common::proto::generic::MemoryMetrics) -> Self {
        MemoryMetrics {
            used_bytes: proto.used_bytes,
            total_bytes: proto.total_bytes,
            usage_percent: proto.usage_percent,
        }
    }
}

impl From<MemoryMetrics> for nexus_common::proto::generic::MemoryMetrics {
    fn from(memory: MemoryMetrics) -> Self {
        nexus_common::proto::generic::MemoryMetrics {
            used_bytes: memory.used_bytes,
            total_bytes: memory.total_bytes,
            usage_percent: memory.usage_percent,
        }
    }
}

impl From<nexus_common::proto::generic::StorageMetrics> for StorageMetrics {
    fn from(proto: nexus_common::proto::generic::StorageMetrics) -> Self {
        StorageMetrics {
            used_bytes: proto.used_bytes,
            total_bytes: proto.total_bytes,
            usage_percent: proto.usage_percent,
            read_bps: proto.read_bps,
            write_bps: proto.write_bps,
        }
    }
}

impl From<StorageMetrics> for nexus_common::proto::generic::StorageMetrics {
    fn from(storage: StorageMetrics) -> Self {
        nexus_common::proto::generic::StorageMetrics {
            used_bytes: storage.used_bytes,
            total_bytes: storage.total_bytes,
            usage_percent: storage.usage_percent,
            read_bps: storage.read_bps,
            write_bps: storage.write_bps,
        }
    }
}

impl From<nexus_common::proto::generic::NetworkMetrics> for NetworkMetrics {
    fn from(proto: nexus_common::proto::generic::NetworkMetrics) -> Self {
        NetworkMetrics {
            rx_bytes: proto.rx_bytes,
            tx_bytes: proto.tx_bytes,
            rx_bandwidth_bps: proto.rx_bandwidth_bps,
            tx_bandwidth_bps: proto.tx_bandwidth_bps,
        }
    }
}

impl From<NetworkMetrics> for nexus_common::proto::generic::NetworkMetrics {
    fn from(network: NetworkMetrics) -> Self {
        nexus_common::proto::generic::NetworkMetrics {
            rx_bytes: network.rx_bytes,
            tx_bytes: network.tx_bytes,
            rx_bandwidth_bps: network.rx_bandwidth_bps,
            tx_bandwidth_bps: network.tx_bandwidth_bps,
        }
    }
}

impl From<nexus_common::proto::generic::GpuMetrics> for GpuMetrics {
    fn from(proto: nexus_common::proto::generic::GpuMetrics) -> Self {
        GpuMetrics {
            compute_usage_percent: proto.compute_usage_percent,
            memory_used_bytes: proto.memory_used_bytes,
            memory_total_bytes: proto.memory_total_bytes,
            memory_usage_percent: proto.memory_usage_percent,
            pcie_rx_bandwidth_bps: proto.pcie_rx_bandwidth_bps,
            pcie_tx_bandwidth_bps: proto.pcie_tx_bandwidth_bps,
            power_usage_watts: proto.power_usage_watts,
            temperature_celsius: proto.temperature_celsius,
        }
    }
}

impl From<GpuMetrics> for nexus_common::proto::generic::GpuMetrics {
    fn from(gpu: GpuMetrics) -> Self {
        nexus_common::proto::generic::GpuMetrics {
            compute_usage_percent: gpu.compute_usage_percent,
            memory_used_bytes: gpu.memory_used_bytes,
            memory_total_bytes: gpu.memory_total_bytes,
            memory_usage_percent: gpu.memory_usage_percent,
            pcie_rx_bandwidth_bps: gpu.pcie_rx_bandwidth_bps,
            pcie_tx_bandwidth_bps: gpu.pcie_tx_bandwidth_bps,
            power_usage_watts: gpu.power_usage_watts,
            temperature_celsius: gpu.temperature_celsius,
        }
    }
}
