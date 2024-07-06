use tonic::{Request, Response, Status};
use std::sync::Arc;
use std::sync::Mutex;

use crate::state::State;
use crate::services::base::generic::{
    Empty,
};
use crate::services::base::nexus::{
    InfoResponse,
};
pub use crate::services::base::nexus::meta_server::{
    Meta, 
    MetaServer,
};

#[derive(Debug, Default)]
pub struct MetaService {
    pub state: Arc<Mutex<State>>,
}

#[tonic::async_trait]
impl Meta for MetaService {
    async fn info(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<InfoResponse>, Status> {
        println!("[NEXUS] Info request received!");

        let mut response = InfoResponse::default();

        if let Ok(state) = self.state.lock() {
            response.version = state.version.clone();
            response.info = "Welcome to NEXUS (Next-gen Execution System)!".to_string();
        }

        Ok(Response::new(response))
    }

    async fn ping(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        println!("[NEXUS] Ping request received!");

        let response = Empty::default();

        Ok(Response::new(response))
    }
}
