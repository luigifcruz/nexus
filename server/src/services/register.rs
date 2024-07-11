use tonic::{Request, Response, Status};
use std::sync::{Mutex, Arc};

use crate::state::State;
use crate::services::base::generic::{
    Empty,
};
use crate::services::base::nexus::{
    ListReplicantsRequest,
    ListReplicantsResponse,
    ListInstancesRequest,
    ListInstancesResponse,
    DeleteReplicantRequest,
    DeleteInstanceRequest,
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
    async fn list_replicants(
        &self, 
        _request: Request<ListReplicantsRequest>
    ) -> Result<Response<ListReplicantsResponse>, Status> {
        println!("[NEXUS] Listing Replicants.");

        let response = ListReplicantsResponse::default();

        Ok(Response::new(response))
    }

    async fn list_instances(
        &self, 
        _request: Request<ListInstancesRequest>
    ) -> Result<Response<ListInstancesResponse>, Status> {
        println!("[NEXUS] Listing Instances.");

        let response = ListInstancesResponse::default();

        Ok(Response::new(response))
    }

    async fn delete_replicant(
        &self, 
        request: Request<DeleteReplicantRequest>
    ) -> Result<Response<Empty>, Status> {
        let replicant = request.into_inner();

        println!("[NEXUS] Deleting Replicant: {:?}", replicant);

        let response = Empty::default();

        Ok(Response::new(response))
    }

    async fn delete_instance(
        &self, 
        request: Request<DeleteInstanceRequest>
    ) -> Result<Response<Empty>, Status> {
        let instance = request.into_inner();

        println!("[NEXUS] Deleting Instance: {:?}", instance);

        let response = Empty::default();

        Ok(Response::new(response))
    }

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