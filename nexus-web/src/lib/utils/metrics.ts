/**
 * Metrics transformation utilities
 *
 * Converts protobuf metrics to chart-friendly format
 */

import type {
    HardwareMetrics,
    ObservationMetrics,
} from "$lib/proto/generic_pb";

export interface MetricDataPoint {
    timestamp: Date;
    packetDrops: number;
    nvmeUsage: number;
    gpuUsage: number;
    vramUsage: number;
    watts: number;
    temperature: number;
}

/**
 * Transform HardwareMetrics[] from replicant to chart data points
 */
export function transformReplicantMetrics(
    metrics: HardwareMetrics[] | undefined,
): MetricDataPoint[] {
    if (!metrics || metrics.length === 0) {
        return [];
    }

    return metrics.map((m) => ({
        timestamp: m.timestamp
            ? new Date(
                  Number(m.timestamp.seconds) * 1000 +
                      Number(m.timestamp.nanos) / 1000000,
              )
            : new Date(),
        packetDrops: 0, // N/A for replicants
        nvmeUsage: Number(m.storage?.usedBytes || 0),
        gpuUsage: m.gpu?.computeUsagePercent || 0,
        vramUsage: Number(m.gpu?.memoryUsedBytes || 0),
        watts: m.gpu?.powerUsageWatts || 0,
        temperature: m.gpu?.temperatureCelsius || 0,
    }));
}

/**
 * Transform ObservationMetrics[] from observation to chart data points
 */
export function transformObservationMetrics(
    metrics: ObservationMetrics[] | undefined,
): MetricDataPoint[] {
    if (!metrics || metrics.length === 0) {
        return [];
    }

    return metrics.map((m) => ({
        timestamp: m.timestamp
            ? new Date(
                  Number(m.timestamp.seconds) * 1000 +
                      Number(m.timestamp.nanos) / 1000000,
              )
            : new Date(),
        // TODO:API - packetDrops not available in ObservationMetrics
        packetDrops: 0,
        nvmeUsage: Number(m.observationStorageSizeBytes || 0),
        gpuUsage: m.aggregateGpuMetrics?.computeUsagePercent || 0,
        vramUsage: Number(m.aggregateGpuMetrics?.memoryUsedBytes || 0),
        watts: m.aggregateGpuMetrics?.powerUsageWatts || 0,
        temperature: m.aggregateGpuMetrics?.temperatureCelsius || 0,
    }));
}
