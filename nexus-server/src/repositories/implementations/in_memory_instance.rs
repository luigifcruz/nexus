use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::entities::{Instance, Status};
use crate::repositories::traits::InstanceRepository;

#[derive(Clone)]
pub struct InMemoryInstanceRepository {
    // Since there's only one instance per replicant, we key by replicant_id
    storage: Arc<RwLock<HashMap<String, Instance>>>,
    // Track command acknowledgments
    command_status: Arc<RwLock<HashMap<String, bool>>>,
}

impl InMemoryInstanceRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            command_status: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl InstanceRepository for InMemoryInstanceRepository {
    async fn find_by_id(
        &self,
        replicant_id: &str,
        instance_id: &str,
    ) -> Result<Option<Instance>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        if let Some(instance) = storage.get(replicant_id) {
            if instance.id == instance_id {
                return Ok(Some(instance.clone()));
            }
        }
        Ok(None)
    }

    async fn find_by_replicant_id(
        &self,
        replicant_id: &str,
    ) -> Result<Option<Instance>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.get(replicant_id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Instance>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn save(
        &self,
        instance: &Instance,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        // Since there's only one instance per replicant, check if one already exists
        if storage.contains_key(&instance.replicant_id) {
            return Err(format!(
                "Instance already exists for replicant {}",
                instance.replicant_id
            )
            .into());
        }
        storage.insert(instance.replicant_id.clone(), instance.clone());
        Ok(())
    }

    async fn update(
        &self,
        instance: &Instance,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if storage.contains_key(&instance.replicant_id) {
            storage.insert(instance.replicant_id.clone(), instance.clone());
            Ok(())
        } else {
            Err(format!("Instance not found for replicant {}", instance.replicant_id).into())
        }
    }

    async fn delete(
        &self,
        replicant_id: &str,
        _instance_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        storage.remove(replicant_id);
        Ok(())
    }

    async fn update_status(
        &self,
        replicant_id: &str,
        _instance_id: &str,
        status: Status,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if let Some(instance) = storage.get_mut(replicant_id) {
            instance.update_status(status);
            Ok(())
        } else {
            Err(format!("Instance not found for replicant {}", replicant_id).into())
        }
    }

    async fn update_command_status(
        &self,
        command_id: &str,
        success: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut command_status = self.command_status.write().await;
        command_status.insert(command_id.to_string(), success);
        Ok(())
    }
}
