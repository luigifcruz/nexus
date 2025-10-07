use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::entities::Metrics;
use crate::repositories::traits::MetricsRepository;

#[derive(Clone)]
pub struct InMemoryMetricsRepository {
    // Store metrics per replicant, using VecDeque for efficient push/pop operations
    storage: Arc<RwLock<HashMap<String, VecDeque<Metrics>>>>,
    max_metrics_per_replicant: usize,
}

impl InMemoryMetricsRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            max_metrics_per_replicant: 256, // Same as the circular buffer in the original
        }
    }

    pub fn with_max_metrics(max_metrics: usize) -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            max_metrics_per_replicant: max_metrics,
        }
    }
}

#[async_trait]
impl MetricsRepository for InMemoryMetricsRepository {
    async fn save_metrics(
        &self,
        replicant_id: &str,
        metrics: &Metrics,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        let entry = storage
            .entry(replicant_id.to_string())
            .or_insert_with(VecDeque::new);

        // Maintain size limit
        while entry.len() >= self.max_metrics_per_replicant {
            entry.pop_front();
        }

        entry.push_back(metrics.clone());
        Ok(())
    }

    async fn find_latest(
        &self,
        replicant_id: &str,
        limit: usize,
    ) -> Result<Vec<Metrics>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;

        if let Some(metrics_deque) = storage.get(replicant_id) {
            let start_idx = if metrics_deque.len() > limit {
                metrics_deque.len() - limit
            } else {
                0
            };

            Ok(metrics_deque.iter().skip(start_idx).cloned().collect())
        } else {
            Ok(Vec::new())
        }
    }

    async fn find_by_time_range(
        &self,
        replicant_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Metrics>, Box<dyn std::error::Error + Send + Sync>> {
        let storage = self.storage.read().await;

        if let Some(metrics_deque) = storage.get(replicant_id) {
            Ok(metrics_deque
                .iter()
                .filter(|m| m.timestamp >= start && m.timestamp <= end)
                .cloned()
                .collect())
        } else {
            Ok(Vec::new())
        }
    }

    async fn delete_old_metrics(
        &self,
        before: DateTime<Utc>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().await;
        let mut total_deleted = 0u64;

        for metrics_deque in storage.values_mut() {
            let initial_len = metrics_deque.len();

            // Remove metrics older than the specified timestamp
            metrics_deque.retain(|m| m.timestamp >= before);

            total_deleted += (initial_len - metrics_deque.len()) as u64;
        }

        Ok(total_deleted)
    }
}
