use tonic::{Request, Response, Status};
use std::sync::Arc;
use std::sync::Mutex;

use crate::state::State;
use crate::services::base::generic::{
    Empty,
};
use crate::services::base::replicant::{
    GetReplicantHardwareResponse, 
    GetReplicantMetricsResponse,
};
pub use crate::services::base::replicant::hardware_server::{
    Hardware, 
    HardwareServer,
};

#[derive(Default)]
pub struct HardwareService {
    pub state: Arc<Mutex<State>>,
}

#[tonic::async_trait]
impl Hardware for HardwareService {
    async fn get_hardware(
        &self, 
        _request: Request<Empty>
    ) -> Result<Response<GetReplicantHardwareResponse>, Status> {
        println!("[REPLICANT] Hardware request received!");

        let mut response = GetReplicantHardwareResponse::default();

        if let Ok(s) = self.state.lock() {
            let hw = s.replicant.get_hardware();

            response.gpu_type = hw.get_gpu_type().clone() as i32;
            response.gpu_pcie_id = hw.get_gpu_pcie_id().to_string();

            response.cpu_type = hw.get_cpu_type().clone() as i32;
            response.cpu_socket = hw.get_cpu_socket().to_string();
            response.cpu_threads = hw.get_cpu_threads().to_vec();

            response.memory_size = hw.get_memory_size();

            response.storage_size = hw.get_storage_size();
            response.storage_path = hw.get_storage_path().to_string();

            response.network_type = hw.get_network_type().clone() as i32;
            response.network_speed = hw.get_network_speed().clone() as i32;
            response.network_pcie_id = hw.get_network_pcie_id().to_string();
            response.network_mac = hw.get_network_mac().to_string();
        }

        Ok(Response::new(response))
    }

    async fn get_metrics(
        &self, 
        _request: Request<Empty>
    ) -> Result<Response<GetReplicantMetricsResponse>, Status> {
        println!("[REPLICANT] Metrics request received!");

        let response = GetReplicantMetricsResponse::default();

        Ok(Response::new(response))
    }
}