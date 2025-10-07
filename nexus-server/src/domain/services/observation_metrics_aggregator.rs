use chrono::Utc;
use std::sync::Arc;

use crate::domain::entities::Metrics;
use crate::domain::services::{ObservationService, ReplicantService};
use crate::domain::DomainError;

use nexus_common::proto::generic::{
    GpuMetrics as ProtoGpuMetrics, NetworkMetrics as ProtoNetworkMetrics,
    ObservationMetrics as ProtoObservationMetrics, StorageMetrics as ProtoStorageMetrics,
};

pub struct ObservationMetricsAggregator {
    observation_service: Arc<ObservationService>,
    replicant_service: Arc<ReplicantService>,
}

impl ObservationMetricsAggregator {
    pub fn new(
        observation_service: Arc<ObservationService>,
        replicant_service: Arc<ReplicantService>,
    ) -> Self {
        Self {
            observation_service,
            replicant_service,
        }
    }

    /// Aggregate metrics from all replicants in an observation
    pub async fn aggregate_observation_metrics(
        &self,
        observation_id: &str,
    ) -> Result<ProtoObservationMetrics, DomainError> {
        // Get the observation
        let observation = self
            .observation_service
            .get_observation(observation_id)
            .await?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        // Get metrics from all replicants in the observation
        let mut aggregate_gpu = AggregateGpuMetrics::default();
        let mut aggregate_network = AggregateNetworkMetrics::default();
        let mut aggregate_storage = AggregateStorageMetrics::default();
        let mut observation_storage_size = 0u64;

        for replicant_id in &observation.replicant_ids {
            // Get latest metrics for this replicant
            if let Ok(metrics_list) = self.replicant_service.get_metrics(replicant_id, 1).await {
                if let Some(latest_metrics) = metrics_list.first() {
                    // Aggregate GPU metrics
                    aggregate_gpu.add_metrics(&latest_metrics.gpu);

                    // Aggregate Network metrics
                    aggregate_network.add_metrics(&latest_metrics.network);

                    // Aggregate Storage metrics
                    aggregate_storage.add_metrics(&latest_metrics.storage);

                    // Add to observation storage size (simplified - just sum up used storage)
                    observation_storage_size += latest_metrics.storage.used_bytes;
                }
            }
        }

        // Create the aggregated observation metrics
        Ok(ProtoObservationMetrics {
            timestamp: Some(prost_types::Timestamp {
                seconds: Utc::now().timestamp(),
                nanos: 0,
            }),
            aggregate_gpu_metrics: Some(aggregate_gpu.to_proto()),
            aggregate_network_metrics: Some(aggregate_network.to_proto()),
            aggregate_storage_metrics: Some(aggregate_storage.to_proto()),
            aggregate_generic_metrics: vec![],
            generic_metrics: vec![],
            observation_storage_size_bytes: observation_storage_size,
            observation_id: observation_id.to_string(),
        })
    }

    /// Get metrics for all replicants in an observation
    pub async fn get_replicant_metrics_for_observation(
        &self,
        observation_id: &str,
        limit: usize,
    ) -> Result<Vec<(String, Vec<Metrics>)>, DomainError> {
        let observation = self
            .observation_service
            .get_observation(observation_id)
            .await?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        let mut all_metrics = Vec::new();

        for replicant_id in &observation.replicant_ids {
            if let Ok(metrics) = self
                .replicant_service
                .get_metrics(replicant_id, limit)
                .await
            {
                all_metrics.push((replicant_id.clone(), metrics));
            }
        }

        Ok(all_metrics)
    }

    /// Check if observation has active replicants with recent metrics
    pub async fn has_active_metrics(
        &self,
        observation_id: &str,
        threshold_seconds: i64,
    ) -> Result<bool, DomainError> {
        let observation = self
            .observation_service
            .get_observation(observation_id)
            .await?
            .ok_or_else(|| DomainError::new(format!("Observation {} not found", observation_id)))?;

        let threshold_time = Utc::now() - chrono::Duration::seconds(threshold_seconds);

        for replicant_id in &observation.replicant_ids {
            if let Ok(metrics_list) = self.replicant_service.get_metrics(replicant_id, 1).await {
                if let Some(latest_metrics) = metrics_list.first() {
                    if latest_metrics.timestamp > threshold_time {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }
}

// Helper structures for aggregating metrics
#[derive(Default)]
struct AggregateGpuMetrics {
    compute_usage_percent: f32,
    memory_used_bytes: u64,
    memory_total_bytes: u64,
    pcie_rx_bandwidth_bps: u64,
    pcie_tx_bandwidth_bps: u64,
    power_usage_watts: f32,
    temperature_celsius: f32,
    count: u32,
}

impl AggregateGpuMetrics {
    fn add_metrics(&mut self, metrics: &crate::domain::entities::GpuMetrics) {
        self.compute_usage_percent += metrics.compute_usage_percent;
        self.memory_used_bytes += metrics.memory_used_bytes;
        self.memory_total_bytes += metrics.memory_total_bytes;
        self.pcie_rx_bandwidth_bps += metrics.pcie_rx_bandwidth_bps;
        self.pcie_tx_bandwidth_bps += metrics.pcie_tx_bandwidth_bps;
        self.power_usage_watts += metrics.power_usage_watts;
        self.temperature_celsius += metrics.temperature_celsius;
        self.count += 1;
    }

    fn to_proto(&self) -> ProtoGpuMetrics {
        if self.count == 0 {
            return ProtoGpuMetrics::default();
        }

        let count = self.count as f32;
        ProtoGpuMetrics {
            compute_usage_percent: self.compute_usage_percent / count,
            memory_used_bytes: self.memory_used_bytes,
            memory_total_bytes: self.memory_total_bytes,
            memory_usage_percent: if self.memory_total_bytes > 0 {
                (self.memory_used_bytes as f32 / self.memory_total_bytes as f32) * 100.0
            } else {
                0.0
            },
            pcie_rx_bandwidth_bps: self.pcie_rx_bandwidth_bps,
            pcie_tx_bandwidth_bps: self.pcie_tx_bandwidth_bps,
            power_usage_watts: self.power_usage_watts,
            temperature_celsius: self.temperature_celsius / count,
        }
    }
}

#[derive(Default)]
struct AggregateNetworkMetrics {
    rx_bytes: u64,
    tx_bytes: u64,
    rx_bandwidth_bps: f32,
    tx_bandwidth_bps: f32,
}

impl AggregateNetworkMetrics {
    fn add_metrics(&mut self, metrics: &crate::domain::entities::NetworkMetrics) {
        self.rx_bytes += metrics.rx_bytes;
        self.tx_bytes += metrics.tx_bytes;
        self.rx_bandwidth_bps += metrics.rx_bandwidth_bps;
        self.tx_bandwidth_bps += metrics.tx_bandwidth_bps;
    }

    fn to_proto(&self) -> ProtoNetworkMetrics {
        ProtoNetworkMetrics {
            rx_bytes: self.rx_bytes,
            tx_bytes: self.tx_bytes,
            rx_bandwidth_bps: self.rx_bandwidth_bps,
            tx_bandwidth_bps: self.tx_bandwidth_bps,
        }
    }
}

#[derive(Default)]
struct AggregateStorageMetrics {
    used_bytes: u64,
    total_bytes: u64,
    read_iops: u64,
    write_iops: u64,
}

impl AggregateStorageMetrics {
    fn add_metrics(&mut self, metrics: &crate::domain::entities::StorageMetrics) {
        self.used_bytes += metrics.used_bytes;
        self.total_bytes += metrics.total_bytes;
        self.read_iops += metrics.read_bps;
        self.write_iops += metrics.write_bps;
    }

    fn to_proto(&self) -> ProtoStorageMetrics {
        ProtoStorageMetrics {
            used_bytes: self.used_bytes,
            total_bytes: self.total_bytes,
            usage_percent: if self.total_bytes > 0 {
                (self.used_bytes as f32 / self.total_bytes as f32) * 100.0
            } else {
                0.0
            },
            read_bps: self.read_iops,
            write_bps: self.write_iops,
        }
    }
}
