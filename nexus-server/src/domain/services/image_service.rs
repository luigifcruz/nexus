use std::sync::Arc;

use crate::domain::entities::Image;
use crate::domain::DomainError;
use crate::repositories::traits::ImageRepository;

pub struct ImageService {
    image_repository: Arc<dyn ImageRepository>,
}

impl ImageService {
    pub fn new(image_repository: Arc<dyn ImageRepository>) -> Self {
        Self { image_repository }
    }

    pub async fn create_image(
        &self,
        id: String,
        docker_image: String,
        docker_entrypoint: String,
        docker_args: Vec<String>,
        docker_env: Vec<String>,
    ) -> Result<Image, DomainError> {
        // Create new image
        let image = Image::new(id, docker_image, docker_entrypoint, docker_args, docker_env);

        // Validate the image
        image.validate().map_err(|e| DomainError::new(e))?;

        // Check if image already exists
        if let Some(_) = self
            .image_repository
            .find_by_id(&image.id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
        {
            return Err(DomainError::new(format!(
                "Image with id {} already exists",
                image.id
            )));
        }

        // Save to repository
        self.image_repository
            .save(&image)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        Ok(image)
    }

    pub async fn get_image(&self, id: &str) -> Result<Option<Image>, DomainError> {
        self.image_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn list_images(&self) -> Result<Vec<Image>, DomainError> {
        self.image_repository
            .find_all()
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn delete_image(&self, id: &str) -> Result<(), DomainError> {
        // Check if image exists
        if self
            .image_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?
            .is_none()
        {
            return Err(DomainError::new(format!("Image with id {} not found", id)));
        }

        self.image_repository
            .delete(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))
    }

    pub async fn validate_image_exists(&self, id: &str) -> Result<bool, DomainError> {
        let image = self
            .image_repository
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::new(e.to_string()))?;

        Ok(image.is_some())
    }
}
