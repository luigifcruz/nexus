use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Image {
    pub id: String,
    pub docker_image: String,
    pub docker_entrypoint: String,
    pub docker_args: Vec<String>,
    pub docker_env: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Image {
    pub fn new(
        id: String,
        docker_image: String,
        docker_entrypoint: String,
        docker_args: Vec<String>,
        docker_env: Vec<String>,
    ) -> Self {
        Self {
            id,
            docker_image,
            docker_entrypoint,
            docker_args,
            docker_env,
            created_at: Utc::now(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Image ID cannot be empty".to_string());
        }

        if self.docker_image.is_empty() {
            return Err("Docker image cannot be empty".to_string());
        }

        Ok(())
    }
}

// Conversion from Image to protobuf Image
impl From<Image> for nexus_common::proto::generic::Image {
    fn from(image: Image) -> Self {
        nexus_common::proto::generic::Image {
            image_id: image.id,
            docker_image: image.docker_image,
            docker_entrypoint: image.docker_entrypoint,
            docker_args: image.docker_args,
            docker_env: image.docker_env,
        }
    }
}

// Conversion from protobuf Image to domain Image
impl From<nexus_common::proto::generic::Image> for Image {
    fn from(proto: nexus_common::proto::generic::Image) -> Self {
        Image {
            id: proto.image_id,
            docker_image: proto.docker_image,
            docker_entrypoint: proto.docker_entrypoint,
            docker_args: proto.docker_args,
            docker_env: proto.docker_env,
            created_at: Utc::now(),
        }
    }
}
