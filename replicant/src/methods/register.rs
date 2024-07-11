use crate::client::nexus::{
    DeleteReplicantRequest, ListReplicantsRequest, RegisterReplicantRequest,
};
use crate::state::State;
use std::error::Error;
use tonic::Request;

pub struct Register {}

impl Register {
    /// Check if the current replicant is registered with NEXUS.
    ///
    /// This is done by listing all the replicants listed with NEXUS
    /// and checking if the current replicant ID is there.
    pub async fn check_replicant(s: &mut State) -> Result<(), Box<dyn Error>> {
        let request = Request::new(ListReplicantsRequest {});

        if let Some(register) = &mut s.client.register {
            let response = register.list_replicants(request).await?;

            let replicants_id = response.into_inner().replicants_id;
            let replicant_id = s.replicant.get_replicant_id().to_string();

            if !replicants_id.contains(&replicant_id) {
                return Err("Replicant ID not found in NEXUS.".into());
            }
        }

        Ok(())
    }

    /// Registers the current replicant with NEXUS.
    pub async fn register_replicant(s: &mut State) -> Result<(), Box<dyn Error>> {
        let request = Request::new(RegisterReplicantRequest {
            name: s.replicant.get_name().to_string(),
            replicant_id: s.replicant.get_replicant_id().to_string(),
            version: s.replicant.get_version().to_string(),
            tags: s.replicant.get_tags().to_vec(),
            host: s.replicant.get_host().to_string(),
            port: s.replicant.get_port(),
        });

        if let Some(register) = &mut s.client.register {
            register.register_replicant(request).await?;
        }

        Ok(())
    }

    /// Unregister the current replicant with NEXUS.
    pub async fn unregister_replicant(s: &mut State) -> Result<(), Box<dyn Error>> {
        let request = Request::new(DeleteReplicantRequest {
            replicant_id: s.replicant.get_replicant_id().to_string(),
        });

        if let Some(register) = &mut s.client.register {
            register.delete_replicant(request).await?;
        }

        Ok(())
    }
}
