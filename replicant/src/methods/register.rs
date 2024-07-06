use tonic::Request;
use std::error::Error;
use crate::state::{State};
use crate::client::Client;
use crate::client::nexus::RegisterReplicantRequest;

pub struct Register {}

impl Register {
    pub async fn check_replicant(_s: &mut State) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

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

    pub async fn unregister_replicant(_s: &mut State) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}