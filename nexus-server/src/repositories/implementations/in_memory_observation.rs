use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::entities::{Observation, ObservationStatus};
use crate::repositories::traits::ObservationRepository;

#[derive(Clone)]
pub struct InMemoryObservationRepository {
    storage: Arc<RwLock<HashMap<String, Observation>>>,
}

impl InMemoryObservationRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ObservationRepository for InMemoryObservationRepository {
    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Observation>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Observation>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn save(
        &self,
        observation: &Observation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if storage.contains_key(&observation.id) {
            return Err(format!("Observation with id {} already exists", observation.id).into());
        }
        storage.insert(observation.id.clone(), observation.clone());
        Ok(())
    }

    async fn update(
        &self,
        observation: &Observation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        if !storage.contains_key(&observation.id) {
            return Err(format!("Observation with id {} not found", observation.id).into());
        }
        storage.insert(observation.id.clone(), observation.clone());
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        storage.remove(id);
        Ok(())
    }

    async fn find_by_status(
        &self,
        status: ObservationStatus,
    ) -> Result<Vec<Observation>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        let result: Vec<Observation> = storage
            .values()
            .filter(|obs| std::mem::discriminant(&obs.status) == std::mem::discriminant(&status))
            .cloned()
            .collect();
        Ok(result)
    }

    async fn find_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Observation>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;
        let result: Vec<Observation> = storage
            .values()
            .filter(|obs| obs.start_time >= start && obs.start_time <= end)
            .cloned()
            .collect();
        Ok(result)
    }
}
