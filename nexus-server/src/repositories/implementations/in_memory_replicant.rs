use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::entities::{Replicant, Status};
use crate::repositories::traits::ReplicantRepository;

#[derive(Clone)]
pub struct InMemoryReplicantRepository {
    storage: Arc<RwLock<HashMap<String, Replicant>>>,
}

impl InMemoryReplicantRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ReplicantRepository for InMemoryReplicantRepository {
    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Replicant>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Replicant>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn save(
        &self,
        replicant: &Replicant,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        storage.insert(replicant.id.clone(), replicant.clone());
        Ok(())
    }

    async fn update(
        &self,
        replicant: &Replicant,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if storage.contains_key(&replicant.id) {
            storage.insert(replicant.id.clone(), replicant.clone());
            Ok(())
        } else {
            Err(format!("Replicant with id {} not found", replicant.id).into())
        }
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        storage.remove(id);
        Ok(())
    }

    async fn find_by_tags(
        &self,
        tags: &[String],
    ) -> Result<Vec<Replicant>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        let result: Vec<Replicant> = storage
            .values()
            .filter(|replicant| tags.iter().any(|tag| replicant.tags.contains(tag)))
            .cloned()
            .collect();
        Ok(result)
    }

    async fn update_status(
        &self,
        id: &str,
        status: Status,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if let Some(replicant) = storage.get_mut(id) {
            replicant.update_status(status);
            Ok(())
        } else {
            Err(format!("Replicant with id {} not found", id).into())
        }
    }

    async fn update_last_seen(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if let Some(replicant) = storage.get_mut(id) {
            replicant.last_seen = Utc::now();
            Ok(())
        } else {
            Err(format!("Replicant with id {} not found", id).into())
        }
    }
}
