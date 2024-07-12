use tonic::{Request, Response, Status};
use std::sync::{Mutex, Arc};

use entities::Replicant;
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
        let mut response = ListReplicantsResponse::default();

        if let Ok(s) = self.state.lock() {
            for (id, _) in s.replicants.iter() {
                response.replicants_id.push(id.to_string());
            }
        }

        Ok(Response::new(response))
    }

    async fn list_instances(
        &self,
        request: Request<ListInstancesRequest>
    ) -> Result<Response<ListInstancesResponse>, Status> {
        let replicant_id = request.into_inner().replicant_id;
        let mut response = ListInstancesResponse::default();

        if let Ok(s) = self.state.lock() {
            if let Some(replicant) = s.replicants.get(&replicant_id) {
                if let Some(instances) = &replicant.get_instances() {
                    for (id, _) in instances.iter() {
                        response.instances_id.push(id.to_string());
                    }
                }
            }
        }

        Ok(Response::new(response))
    }

    async fn delete_replicant(
        &self,
        request: Request<DeleteReplicantRequest>
    ) -> Result<Response<Empty>, Status> {
        let replicant_id = request.into_inner().replicant_id;

        println!("[NEXUS] Deleting Replicant: {:?}", replicant_id);

        // TODO: Check if the replicant is running any observation.

        // Check if the replicant exists and remove it.

        if let Ok(mut s) = self.state.lock() {
            if let Some(_) = s.replicants.get(&replicant_id) {
                s.replicants.remove(&replicant_id);
            }
        }

        Ok(Response::new(Empty::default()))
    }

    async fn delete_instance(
        &self,
        request: Request<DeleteInstanceRequest>
    ) -> Result<Response<Empty>, Status> {
        let inner = request.into_inner();

        let replicant_id = inner.replicant_id;
        let instance_id = inner.instance_id;

        println!("[NEXUS] Deleting Instance: {}-{}", replicant_id, instance_id);

        // TODO: Implement.

        Ok(Response::new(Empty::default()))
    }

    async fn register_replicant(
        &self,
        request: Request<RegisterReplicantRequest>
    ) -> Result<Response<RegisterResponse>, Status> {
        let inner = request.into_inner();

        // Check if the ID is already in use.

        if let Ok(s) = self.state.lock() {
            if s.replicants.contains_key(&inner.replicant_id) {
                return Err(Status::already_exists("Replicant ID already in use."));
            }
        }

        println!("[NEXUS] Registering Replicant {:?}", inner.replicant_id);

        // Create the replicant and add it to the state.

        if let Ok(mut s) = self.state.lock() {
            s.replicants.insert(inner.replicant_id.clone(), Replicant::default());
        }

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
