pub mod enums {
    tonic::include_proto!("enums");
}

pub mod generic {
    tonic::include_proto!("generic");
}

pub mod nexus {
    tonic::include_proto!("nexus");
}

use std::error::Error;
use std::sync::{Mutex, Arc};
use tonic::transport::Channel;
use serde::{Serialize, Deserialize};

use crate::client::generic::{
    Empty,
};
use crate::client::nexus::meta_client::MetaClient;
use crate::client::nexus::register_client::RegisterClient;
use crate::client::nexus::notify_client::NotifyClient;

use crate::state::State;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Client {
    pub host: String,
    pub port: u16,
    #[serde(skip)]
    pub meta: Option<MetaClient<Channel>>,
    #[serde(skip)]
    pub register: Option<RegisterClient<Channel>>,
    #[serde(skip)]
    pub notify: Option<NotifyClient<Channel>>
}

impl Client {
    pub async fn initialize(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() {
            let nexus_path = format!("{}:{}", s.client.host, s.client.port);

            s.client.meta = Some(MetaClient::connect(nexus_path.clone()).await?);
            s.client.register = Some(RegisterClient::connect(nexus_path.clone()).await?);
            s.client.notify = Some(NotifyClient::connect(nexus_path.clone()).await?);
        }

        println!("[REPLICANT] Successfully connected to Nexus endpoint.");
        
        Ok(())
    }

    pub async fn ping(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() {
            let request = tonic::Request::new(Empty::default());

            if let Some(meta) = &mut s.client.meta {
                meta.ping(request).await?;
            }
        }

        Ok(())
    }
}