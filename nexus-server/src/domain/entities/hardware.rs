use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuType {
    NvidiaRtx6000Ada,
    NvidiaA4000,
}

impl Default for GpuType {
    fn default() -> Self {
        GpuType::NvidiaRtx6000Ada
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpuType {
    AmdEpycMilan,
    AmdEpycGenoa,
}

impl Default for CpuType {
    fn default() -> Self {
        CpuType::AmdEpycMilan
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkType {
    Connectx6,
    Connectx7,
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::Connectx6
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkSpeed {
    Gbe1,
    Gbe10,
    Gbe25,
    Gbe40,
    Gbe100,
    Gbe200,
}

impl Default for NetworkSpeed {
    fn default() -> Self {
        NetworkSpeed::Gbe1
    }
}

impl fmt::Display for NetworkSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkSpeed::Gbe1 => write!(f, "1 GbE"),
            NetworkSpeed::Gbe10 => write!(f, "10 GbE"),
            NetworkSpeed::Gbe25 => write!(f, "25 GbE"),
            NetworkSpeed::Gbe40 => write!(f, "40 GbE"),
            NetworkSpeed::Gbe100 => write!(f, "100 GbE"),
            NetworkSpeed::Gbe200 => write!(f, "200 GbE"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Hardware {
    pub gpu_type: GpuType,
    pub gpu_pcie_id: String,
    pub cpu_type: CpuType,
    pub cpu_socket: String,
    pub cpu_threads: Vec<i32>,
    pub memory_size: i32,
    pub storage_size: i32,
    pub storage_path: String,
    pub network_type: NetworkType,
    pub network_speed: NetworkSpeed,
    pub network_pcie_id: String,
    pub network_interface: String,
    pub network_mac: String,
}

impl Hardware {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gpu(mut self, gpu_type: GpuType, pcie_id: String) -> Self {
        self.gpu_type = gpu_type;
        self.gpu_pcie_id = pcie_id;
        self
    }

    pub fn with_cpu(mut self, cpu_type: CpuType, socket: String, threads: Vec<i32>) -> Self {
        self.cpu_type = cpu_type;
        self.cpu_socket = socket;
        self.cpu_threads = threads;
        self
    }

    pub fn with_memory(mut self, size: i32) -> Self {
        self.memory_size = size;
        self
    }

    pub fn with_storage(mut self, size: i32, path: String) -> Self {
        self.storage_size = size;
        self.storage_path = path;
        self
    }

    pub fn with_network(
        mut self,
        network_type: NetworkType,
        speed: NetworkSpeed,
        pcie_id: String,
        interface: String,
        mac: String,
    ) -> Self {
        self.network_type = network_type;
        self.network_speed = speed;
        self.network_pcie_id = pcie_id;
        self.network_interface = interface;
        self.network_mac = mac;
        self
    }
}

// Conversion from protobuf types
impl From<nexus_common::proto::enums::GpuType> for GpuType {
    fn from(proto: nexus_common::proto::enums::GpuType) -> Self {
        match proto {
            nexus_common::proto::enums::GpuType::NvidiaRtx6000Ada => GpuType::NvidiaRtx6000Ada,
            nexus_common::proto::enums::GpuType::NvidiaA4000 => GpuType::NvidiaA4000,
        }
    }
}

impl From<GpuType> for nexus_common::proto::enums::GpuType {
    fn from(gpu: GpuType) -> Self {
        match gpu {
            GpuType::NvidiaRtx6000Ada => nexus_common::proto::enums::GpuType::NvidiaRtx6000Ada,
            GpuType::NvidiaA4000 => nexus_common::proto::enums::GpuType::NvidiaA4000,
        }
    }
}

impl From<nexus_common::proto::enums::CpuType> for CpuType {
    fn from(proto: nexus_common::proto::enums::CpuType) -> Self {
        match proto {
            nexus_common::proto::enums::CpuType::AmdEpycMilan => CpuType::AmdEpycMilan,
            nexus_common::proto::enums::CpuType::AmdEpycGenoa => CpuType::AmdEpycGenoa,
        }
    }
}

impl From<CpuType> for nexus_common::proto::enums::CpuType {
    fn from(cpu: CpuType) -> Self {
        match cpu {
            CpuType::AmdEpycMilan => nexus_common::proto::enums::CpuType::AmdEpycMilan,
            CpuType::AmdEpycGenoa => nexus_common::proto::enums::CpuType::AmdEpycGenoa,
        }
    }
}

impl From<nexus_common::proto::enums::NetworkType> for NetworkType {
    fn from(proto: nexus_common::proto::enums::NetworkType) -> Self {
        match proto {
            nexus_common::proto::enums::NetworkType::Connectx6 => NetworkType::Connectx6,
            nexus_common::proto::enums::NetworkType::Connectx7 => NetworkType::Connectx7,
        }
    }
}

impl From<NetworkType> for nexus_common::proto::enums::NetworkType {
    fn from(net: NetworkType) -> Self {
        match net {
            NetworkType::Connectx6 => nexus_common::proto::enums::NetworkType::Connectx6,
            NetworkType::Connectx7 => nexus_common::proto::enums::NetworkType::Connectx7,
        }
    }
}

impl From<nexus_common::proto::enums::NetworkSpeed> for NetworkSpeed {
    fn from(proto: nexus_common::proto::enums::NetworkSpeed) -> Self {
        match proto {
            nexus_common::proto::enums::NetworkSpeed::Gbe1 => NetworkSpeed::Gbe1,
            nexus_common::proto::enums::NetworkSpeed::Gbe10 => NetworkSpeed::Gbe10,
            nexus_common::proto::enums::NetworkSpeed::Gbe25 => NetworkSpeed::Gbe25,
            nexus_common::proto::enums::NetworkSpeed::Gbe40 => NetworkSpeed::Gbe40,
            nexus_common::proto::enums::NetworkSpeed::Gbe100 => NetworkSpeed::Gbe100,
            nexus_common::proto::enums::NetworkSpeed::Gbe200 => NetworkSpeed::Gbe200,
        }
    }
}

impl From<NetworkSpeed> for nexus_common::proto::enums::NetworkSpeed {
    fn from(speed: NetworkSpeed) -> Self {
        match speed {
            NetworkSpeed::Gbe1 => nexus_common::proto::enums::NetworkSpeed::Gbe1,
            NetworkSpeed::Gbe10 => nexus_common::proto::enums::NetworkSpeed::Gbe10,
            NetworkSpeed::Gbe25 => nexus_common::proto::enums::NetworkSpeed::Gbe25,
            NetworkSpeed::Gbe40 => nexus_common::proto::enums::NetworkSpeed::Gbe40,
            NetworkSpeed::Gbe100 => nexus_common::proto::enums::NetworkSpeed::Gbe100,
            NetworkSpeed::Gbe200 => nexus_common::proto::enums::NetworkSpeed::Gbe200,
        }
    }
}

impl From<nexus_common::proto::generic::HardwareSpec> for Hardware {
    fn from(proto: nexus_common::proto::generic::HardwareSpec) -> Self {
        Hardware {
            gpu_type: GpuType::from(
                nexus_common::proto::enums::GpuType::try_from(proto.gpu_type).unwrap_or_default(),
            ),
            gpu_pcie_id: proto.gpu_pcie_id,
            cpu_type: CpuType::from(
                nexus_common::proto::enums::CpuType::try_from(proto.cpu_type).unwrap_or_default(),
            ),
            cpu_socket: proto.cpu_socket,
            cpu_threads: proto.cpu_threads,
            memory_size: proto.memory_size,
            storage_size: proto.storage_size,
            storage_path: proto.storage_path,
            network_type: NetworkType::from(
                nexus_common::proto::enums::NetworkType::try_from(proto.network_type)
                    .unwrap_or_default(),
            ),
            network_speed: NetworkSpeed::from(
                nexus_common::proto::enums::NetworkSpeed::try_from(proto.network_speed)
                    .unwrap_or_default(),
            ),
            network_pcie_id: proto.network_pcie_id,
            network_interface: proto.network_interface,
            network_mac: proto.network_mac,
        }
    }
}

impl From<Hardware> for nexus_common::proto::generic::HardwareSpec {
    fn from(hardware: Hardware) -> Self {
        nexus_common::proto::generic::HardwareSpec {
            gpu_type: nexus_common::proto::enums::GpuType::from(hardware.gpu_type) as i32,
            gpu_pcie_id: hardware.gpu_pcie_id,
            cpu_type: nexus_common::proto::enums::CpuType::from(hardware.cpu_type) as i32,
            cpu_socket: hardware.cpu_socket,
            cpu_threads: hardware.cpu_threads,
            memory_size: hardware.memory_size,
            storage_size: hardware.storage_size,
            storage_path: hardware.storage_path,
            network_type: nexus_common::proto::enums::NetworkType::from(hardware.network_type)
                as i32,
            network_speed: nexus_common::proto::enums::NetworkSpeed::from(hardware.network_speed)
                as i32,
            network_pcie_id: hardware.network_pcie_id,
            network_interface: hardware.network_interface,
            network_mac: hardware.network_mac,
        }
    }
}
