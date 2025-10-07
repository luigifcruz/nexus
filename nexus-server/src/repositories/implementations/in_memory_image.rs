use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::entities::Image;
use crate::repositories::traits::ImageRepository;

#[derive(Clone)]
pub struct InMemoryImageRepository {
    storage: Arc<RwLock<HashMap<String, Image>>>,
}

impl InMemoryImageRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ImageRepository for InMemoryImageRepository {
    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Image>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Image>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn save(&self, image: &Image) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if storage.contains_key(&image.id) {
            return Err(format!("Image with id {} already exists", image.id).into());
        }
        storage.insert(image.id.clone(), image.clone());
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        storage.remove(id);
        Ok(())
    }
}
