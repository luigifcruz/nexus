use tonic::{Request, Response, Status};
use tracing::debug;

use nexus_common::proto::generic::Empty;
use nexus_common::proto::meta::meta_server::Meta;
use nexus_common::proto::meta::InfoResponse;

pub use nexus_common::proto::meta::meta_server::MetaServer;
pub use nexus_common::proto::meta::FILE_DESCRIPTOR_SET as MetaFileDescriptorSet;

pub struct MetaHandler {
    version: String,
}

impl MetaHandler {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

#[tonic::async_trait]
impl Meta for MetaHandler {
    async fn info(&self, _request: Request<Empty>) -> Result<Response<InfoResponse>, Status> {
        debug!("Info request received!");

        let response = InfoResponse {
            version: self.version.clone(),
            info: "Welcome to NEXUS (Next-gen Execution System)!".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn ping(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        debug!("Ping request received!");

        Ok(Response::new(Empty::default()))
    }
}
