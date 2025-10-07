use async_trait::async_trait;
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::entities::Hardware;
use crate::repositories::traits::HardwareRepository;

#[derive(Clone)]
pub struct InMemoryHardwareRepository {
    storage: Arc<RwLock<HashMap<String, Hardware>>>,
}

impl InMemoryHardwareRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl HardwareRepository for InMemoryHardwareRepository {
    async fn save(
        &self,
        replicant_id: &str,
        hardware: &Hardware,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        storage.insert(replicant_id.to_string(), hardware.clone());
        Ok(())
    }

    async fn find_by_replicant_id(
        &self,
        replicant_id: &str,
    ) -> Result<Option<Hardware>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.get(replicant_id).cloned())
    }

    async fn update(
        &self,
        replicant_id: &str,
        hardware: &Hardware,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if storage.contains_key(replicant_id) {
            storage.insert(replicant_id.to_string(), hardware.clone());
            Ok(())
        } else {
            Err(format!("Hardware not found for replicant {}", replicant_id).into())
        }
    }
}
