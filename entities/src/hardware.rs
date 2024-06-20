use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HardwareGpuType {
    #[default]
    NvidiaRtx6000Ada, 
    NvidiaA4000,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HardwareCpuType {
    #[default]
    AmdEpycMilan,
    AmdEpycGenoa,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HardwareNetworkType {
    #[default]
    Connectx6,
    Connectx7,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HardwareNetworkSpeed {
    #[default]
    Gbe1,
    Gbe10,
    Gbe25,
    Gbe40,
    Gbe100,
    Gbe200,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Hardware {
    gpu_type: HardwareGpuType,
    gpu_pcie_id: String,

    cpu_type: HardwareCpuType,
    cpu_socket: String,
    cpu_threads: Vec<i32>,

    memory_size: i32,

    storage_size: i32,
    storage_path: String,

    network_type: HardwareNetworkType,
    network_speed: HardwareNetworkSpeed,
    network_pcie_id: String,
    network_mac: String,

    // Metadata
    #[serde(skip)]
    up_to_date_db: bool,
}

impl Hardware {
    pub fn get_gpu_type(&self) -> &HardwareGpuType {
        &self.gpu_type
    }

    pub fn get_gpu_pcie_id(&self) -> &str {
        &self.gpu_pcie_id
    }

    pub fn get_cpu_type(&self) -> &HardwareCpuType {
        &self.cpu_type
    }

    pub fn get_cpu_socket(&self) -> &str {
        &self.cpu_socket
    }

    pub fn get_cpu_threads(&self) -> &Vec<i32> {
        &self.cpu_threads
    }

    pub fn get_memory_size(&self) -> i32 {
        self.memory_size
    }

    pub fn get_storage_size(&self) -> i32 {
        self.storage_size
    }

    pub fn get_storage_path(&self) -> &str {
        &self.storage_path
    }

    pub fn get_network_type(&self) -> &HardwareNetworkType {
        &self.network_type
    }

    pub fn get_network_speed(&self) -> &HardwareNetworkSpeed {
        &self.network_speed
    }

    pub fn get_network_pcie_id(&self) -> &str {
        &self.network_pcie_id
    }

    pub fn get_network_mac(&self) -> &str {
        &self.network_mac
    }

    pub fn set_gpu_type(&mut self, gpu_type: HardwareGpuType) {
        self.up_to_date_db = false;
        self.gpu_type = gpu_type;
    }

    pub fn set_gpu_pcie_id(&mut self, gpu_pcie_id: String) {
        self.up_to_date_db = false;
        self.gpu_pcie_id = gpu_pcie_id;
    }

    pub fn set_cpu_type(&mut self, cpu_type: HardwareCpuType) {
        self.up_to_date_db = false;
        self.cpu_type = cpu_type;
    }

    pub fn set_cpu_socket(&mut self, cpu_socket: String) {
        self.up_to_date_db = false;
        self.cpu_socket = cpu_socket;
    }

    pub fn set_cpu_threads(&mut self, cpu_threads: Vec<i32>) {
        self.up_to_date_db = false;
        self.cpu_threads = cpu_threads;
    }

    pub fn set_memory_size(&mut self, memory_size: i32) {
        self.up_to_date_db = false;
        self.memory_size = memory_size;
    }

    pub fn set_storage_size(&mut self, storage_size: i32) {
        self.up_to_date_db = false;
        self.storage_size = storage_size;
    }

    pub fn set_storage_path(&mut self, storage_path: String) {
        self.up_to_date_db = false;
        self.storage_path = storage_path;
    }

    pub fn set_network_type(&mut self, network_type: HardwareNetworkType) {
        self.up_to_date_db = false;
        self.network_type = network_type;
    }

    pub fn set_network_speed(&mut self, network_speed: HardwareNetworkSpeed) {
        self.up_to_date_db = false;
        self.network_speed = network_speed;
    }

    pub fn set_network_pcie_id(&mut self, network_pcie_id: String) {
        self.up_to_date_db = false;
        self.network_pcie_id = network_pcie_id;
    }

    pub fn set_network_mac(&mut self, network_mac: String) {
        self.up_to_date_db = false;
        self.network_mac = network_mac;
    }
}