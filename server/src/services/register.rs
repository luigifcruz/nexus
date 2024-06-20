use tonic::{Request, Response, Status};
use std::sync::Arc;
use std::sync::Mutex;

use crate::state::State;
use crate::services::base::nexus::{
    RegisterReplicantRequest, 
    RegisterInstanceRequest, 
    RegisterResponse, 
};
pub use crate::services::base::nexus::register_server::{
    Register, 
    RegisterServer,
};

#[derive(Debug, Default)]
pub struct RegisterService {
    pub state: Arc<Mutex<State>>,
}

#[tonic::async_trait]
impl Register for RegisterService {
    async fn register_replicant(
        &self, 
        request: Request<RegisterReplicantRequest>
    ) -> Result<Response<RegisterResponse>, Status> {
        let replicant = request.into_inner();

        println!("[NEXUS] Registering Replicant: {:?}", replicant);

        let response = RegisterResponse {
            success: true,
        };

        Ok(Response::new(response))
    }

    async fn register_instance(
        &self, 
        request: Request<RegisterInstanceRequest>
    ) -> Result<Response<RegisterResponse>, Status> {
        let instance = request.into_inner();

        println!("[NEXUS] Registering Instance: {:?}", instance);

        let response = RegisterResponse {
            success: true,
        };

        Ok(Response::new(response))
    }
}