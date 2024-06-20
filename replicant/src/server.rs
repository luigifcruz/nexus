use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use tonic::{transport};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use serde::{Serialize, Deserialize};

use crate::services::{MetaService, MetaServer};
use crate::services::{HardwareService, HardwareServer};
use crate::state::{State};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(skip_deserializing, skip_serializing)]
    pub listener: Option<TcpListener>,
}

impl Server {
    pub async fn initialize(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() {
            let port = format!("0.0.0.0:{}", s.cli.args.port);
            s.server.listener = Some(TcpListener::bind(port).await?);
        }

        Ok(())
    }

    pub async fn serve(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        let hardware_service = HardwareService {
            state: state.clone(),
        };

        let meta_service = MetaService {
            state: state.clone(),
        };

        if let Ok(s) = state.lock() {
            println!("[REPLICANT] Server listening on port {}:{}.", s.replicant.get_host(), 
                                                                    s.replicant.get_port());
        }

        let listener = state.lock().unwrap().server.listener.take().unwrap();

        tokio::spawn(async {
            transport::Server::builder()
                .add_service(HardwareServer::new(hardware_service))
                .add_service(MetaServer::new(meta_service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
        });

        Ok(())
    }
}