use std::error::Error;
use std::sync::{Mutex, Arc};
use tonic::transport;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

use crate::services::{MetaService, MetaServer};
use crate::services::{RegisterService, RegisterServer};
use crate::state::State;

#[derive(Default, Debug)]
pub struct Server {
    pub listener: Option<TcpListener>,
}

impl Server {
    pub async fn initialize(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() {
            let port = format!("{}:{}", s.cli.args.host, s.cli.args.port);
            s.server.listener = Some(TcpListener::bind(port).await?);
        }

        Ok(())
    }

    pub async fn serve(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        let register_service = RegisterService {
            state: state.clone(),
        };

        let meta_service = MetaService {
            state: state.clone(),
        };

        if let Ok(s) = state.lock() {
            println!("[NEXUS] Server listening on port {}:{}.", s.cli.args.host, s.cli.args.port);
        }

        let listener = state.lock().unwrap().server.listener.take().unwrap();

        transport::Server::builder()
            .add_service(RegisterServer::new(register_service))
            .add_service(MetaServer::new(meta_service))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await?;

        Ok(())
    }
}
