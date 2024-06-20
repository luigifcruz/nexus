use tonic::{Request, Response, Status};
use std::sync::Arc;
use std::sync::Mutex;

use crate::state::State;
use crate::services::base::generic::{
    Empty,
};
use crate::services::base::replicant::{
    InfoResponse,
};
pub use crate::services::base::replicant::meta_server::{
    Meta, 
    MetaServer,
};

#[derive(Default)]
pub struct MetaService {
    pub state: Arc<Mutex<State>>,
}

#[tonic::async_trait]
impl Meta for MetaService {
    async fn info(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<InfoResponse>, Status> {
        println!("[REPLICANT] Info request received!");

        let mut response = InfoResponse::default();

        if let Ok(s) = self.state.lock() {
            response.name = s.replicant.get_name().to_string();
            response.replicant_id = s.replicant.get_replicant_id().to_string();
            response.version = s.replicant.get_version().to_string();
            response.tags = s.replicant.get_tags().to_vec();
            response.healthy = s.healthy;
            response.info = format!("Welcome to a Replicant!");
        }

        Ok(Response::new(response))
    }

    async fn ping(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        println!("[REPLICANT] Ping request received!");

        let response = Empty {};

        Ok(Response::new(response))
    }
}
