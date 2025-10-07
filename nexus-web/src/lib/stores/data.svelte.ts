import type {
    ReplicantData,
    ObservationData,
    ImageData,
    SystemMetrics,
    ActivityItem,
    ConnectionStatus,
    Status,
} from "$lib/types";
import { statusToString, timestampToDate } from "$lib/types";
import type {
    HardwareMetrics,
    ObservationMetrics,
    GenericMetric,
    LogEntry,
    Observation,
    Replicant,
} from "$lib/proto/generic_pb";
import { nexusClient } from "$lib/api/nexus";

interface MetricsStream<T> {
    data: T[];
}

interface LogStream {
    entries: LogEntry[];
}

class DataStore {
    replicants = $state<ReplicantData[]>([]);
    observations = $state<ObservationData[]>([]);
    images = $state<ImageData[]>([]);
    systemMetrics = $state<SystemMetrics>({
        totalBandwidth: 0,
        totalNvmeUsage: 0,
        totalGpuRam: 0,
        activeReplicants: 0,
        onlineReplicants: 0,
        activeObservations: 0,
        scheduledObservations: 0,
        totalReplicants: 0,
        completedObservations: 0,
        failedObservations: 0,
        systemUptime: 0,
        totalStorage: 0,
        usedStorage: 0,
    });
    activity = $state<ActivityItem[]>([]);
    connectionStatus = $state<ConnectionStatus>({
        connected: false,
        reconnecting: false,
        error: "",
    });
    lastDataUpdate = $state<string | null>(null);

    private replicantMetricsMap = $state<Map<string, HardwareMetrics[]>>(
        new Map(),
    );
    private observationMetricsMap = $state<Map<string, ObservationMetrics[]>>(
        new Map(),
    );
    private observationLogsMap = $state<Map<string, LogEntry[]>>(new Map());

    private _signallerSubscription: AbortController | null = null;

    onlineReplicants = $derived(
        this.replicants.filter((r) => r.status === "running"),
    );
    runningObservations = $derived(
        this.observations.filter((o) => o.status === "running"),
    );
    scheduledObservations = $derived(
        this.observations.filter(
            (o) => o.status === "standby" || o.status === "starting",
        ),
    );
    completedObservations = $derived(
        this.observations.filter((o) => o.status === "stopped"),
    );
    failedObservations = $derived(
        this.observations.filter((o) => o.status === "errored"),
    );
    recentActivity = $derived(this.activity.slice(0, 10));

    setReplicants(replicants: ReplicantData[]) {
        console.log(
            "[STORE] setReplicants called with",
            replicants.length,
            "replicants",
        );
        this.replicants = replicants;
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    setObservations(observations: ObservationData[]) {
        console.log(
            "[STORE] setObservations called with",
            observations.length,
            "observations",
        );
        this.observations = observations;
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    setImages(images: ImageData[]) {
        console.log("[STORE] setImages called with", images.length, "images");
        this.images = images;
        this.lastDataUpdate = new Date().toISOString();
    }

    setSystemMetrics(metrics: Partial<SystemMetrics>) {
        this.systemMetrics = { ...this.systemMetrics, ...metrics };
    }

    setConnectionStatus(status: Partial<ConnectionStatus>) {
        console.log("[STORE] setConnectionStatus:", status);
        this.connectionStatus = { ...this.connectionStatus, ...status };
    }

    getReplicantById(id: string): ReplicantData | undefined {
        return this.replicants.find((r) => r.replicantId === id);
    }

    getObservationById(id: string): ObservationData | undefined {
        return this.observations.find((o) => o.observationId === id);
    }

    getObservationsByReplicantId(replicantId: string): ObservationData[] {
        return this.observations.filter((o) =>
            o.replicantIds.includes(replicantId),
        );
    }

    getImageById(id: string): ImageData | undefined {
        return this.images.find((i) => i.imageId === id);
    }

    addReplicant(replicant: ReplicantData) {
        console.log("[STORE] addReplicant:", replicant.replicantId);
        const existingIndex = this.replicants.findIndex(
            (r) => r.replicantId === replicant.replicantId,
        );
        if (existingIndex >= 0) {
            this.replicants = this.replicants.map((r, i) =>
                i === existingIndex ? replicant : r,
            );
        } else {
            this.replicants = [...this.replicants, replicant];
        }
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    updateReplicant(id: string, updates: Partial<ReplicantData>) {
        console.log("[STORE] updateReplicant:", id);
        this.replicants = this.replicants.map((r) =>
            r.replicantId === id ? { ...r, ...updates } : r,
        );
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    removeReplicant(id: string) {
        console.log("[STORE] removeReplicant:", id);
        this.replicants = this.replicants.filter((r) => r.replicantId !== id);
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    addObservation(observation: ObservationData) {
        console.log("[STORE] addObservation:", observation.observationId);
        const existingIndex = this.observations.findIndex(
            (o) => o.observationId === observation.observationId,
        );
        if (existingIndex >= 0) {
            this.observations = this.observations.map((o, i) =>
                i === existingIndex ? observation : o,
            );
        } else {
            this.observations = [...this.observations, observation];
        }
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    updateObservation(id: string, updates: Partial<ObservationData>) {
        console.log("[STORE] updateObservation:", id);
        this.observations = this.observations.map((o) =>
            o.observationId === id ? { ...o, ...updates } : o,
        );
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    removeObservation(id: string) {
        console.log("[STORE] removeObservation:", id);
        this.observations = this.observations.filter(
            (o) => o.observationId !== id,
        );
        this.lastDataUpdate = new Date().toISOString();
        this.updateSystemMetrics();
    }

    addImage(image: ImageData) {
        console.log("[STORE] addImage:", image.imageId);
        const existingIndex = this.images.findIndex(
            (i) => i.imageId === image.imageId,
        );
        if (existingIndex >= 0) {
            this.images = this.images.map((i, idx) =>
                idx === existingIndex ? image : i,
            );
        } else {
            this.images = [...this.images, image];
        }
        this.lastDataUpdate = new Date().toISOString();
    }

    updateImage(id: string, updates: Partial<ImageData>) {
        console.log("[STORE] updateImage:", id);
        this.images = this.images.map((i) =>
            i.imageId === id ? { ...i, ...updates } : i,
        );
        this.lastDataUpdate = new Date().toISOString();
    }

    removeImage(id: string) {
        console.log("[STORE] removeImage:", id);
        this.images = this.images.filter((i) => i.imageId !== id);
        this.lastDataUpdate = new Date().toISOString();
    }

    addActivity(activity: ActivityItem) {
        this.activity = [activity, ...this.activity].slice(0, 100);
    }

    clearActivity() {
        this.activity = [];
    }

    getReplicantMetrics(replicantId: string): HardwareMetrics[] {
        return this.replicantMetricsMap.get(replicantId) ?? [];
    }

    addReplicantMetrics(replicantId: string, metrics: HardwareMetrics[]) {
        console.log(
            "[STORE] addReplicantMetrics:",
            replicantId,
            metrics.length,
            "metrics",
        );
        const existing = this.replicantMetricsMap.get(replicantId) ?? [];
        const updated = [...existing, ...metrics].slice(-100);
        this.replicantMetricsMap.set(replicantId, updated);
        this.replicantMetricsMap = new Map(this.replicantMetricsMap);

        if (metrics.length > 0) {
            const latest = metrics[metrics.length - 1];
            this.updateReplicant(replicantId, {
                currentMetrics: {
                    cpuUsage: latest.cpu?.usagePercent,
                    memoryUsage: latest.memory?.usagePercent,
                    storageUsage: latest.storage?.usagePercent,
                    networkBandwidth:
                        ((latest.network?.rxBandwidthBps ?? 0) +
                            (latest.network?.txBandwidthBps ?? 0)) /
                        1e9,
                    gpuUsage: latest.gpu?.computeUsagePercent,
                    gpuMemoryUsage: latest.gpu?.memoryUsagePercent,
                    temperature:
                        latest.gpu?.temperatureCelsius ??
                        latest.cpu?.temperatureCelsius,
                },
            });
        }
        this.lastDataUpdate = new Date().toISOString();
    }

    getObservationMetrics(observationId: string): ObservationMetrics[] {
        return this.observationMetricsMap.get(observationId) ?? [];
    }

    addObservationMetrics(
        observationId: string,
        metrics: ObservationMetrics[],
    ) {
        console.log(
            "[STORE] addObservationMetrics:",
            observationId,
            metrics.length,
            "metrics",
        );
        const existing = this.observationMetricsMap.get(observationId) ?? [];
        const updated = [...existing, ...metrics].slice(-100);
        this.observationMetricsMap.set(observationId, updated);
        this.observationMetricsMap = new Map(this.observationMetricsMap);
        this.lastDataUpdate = new Date().toISOString();
    }

    getObservationLogs(observationId: string): LogEntry[] {
        return this.observationLogsMap.get(observationId) ?? [];
    }

    addObservationLogs(observationId: string, logs: LogEntry[]) {
        console.log(
            "[STORE] addObservationLogs:",
            observationId,
            logs.length,
            "logs",
        );
        const existing = this.observationLogsMap.get(observationId) ?? [];
        const updated = [...existing, ...logs].slice(-500);
        this.observationLogsMap.set(observationId, updated);
        this.observationLogsMap = new Map(this.observationLogsMap);
        this.lastDataUpdate = new Date().toISOString();
    }

    clearMetrics(type: "replicant" | "observation", id: string) {
        if (type === "replicant") {
            this.replicantMetricsMap.delete(id);
            this.replicantMetricsMap = new Map(this.replicantMetricsMap);
        } else {
            this.observationMetricsMap.delete(id);
            this.observationMetricsMap = new Map(this.observationMetricsMap);
        }
    }

    clearLogs(observationId: string) {
        this.observationLogsMap.delete(observationId);
        this.observationLogsMap = new Map(this.observationLogsMap);
    }

    private updateSystemMetrics() {
        const totalReplicants = this.replicants.length;
        const onlineReplicants = this.onlineReplicants.length;
        const activeObservations = this.runningObservations.length;
        const scheduledObservations = this.scheduledObservations.length;
        const completedObservations = this.completedObservations.length;
        const failedObservations = this.failedObservations.length;

        const totalBandwidth = this.replicants.reduce(
            (sum, r) => sum + (r.currentMetrics?.networkBandwidth ?? 0),
            0,
        );

        const totalNvmeUsage = this.replicants.reduce((sum, r) => {
            const used =
                ((r.currentMetrics?.storageUsage ?? 0) *
                    (r.hardware?.storageSize ?? 0)) /
                100;
            return sum + used;
        }, 0);

        const totalGpuRam = this.replicants.reduce((sum, r) => {
            return sum + 48;
        }, 0);

        this.setSystemMetrics({
            totalReplicants,
            onlineReplicants,
            activeReplicants: onlineReplicants,
            activeObservations,
            scheduledObservations,
            completedObservations,
            failedObservations,
            totalBandwidth,
            totalNvmeUsage,
            totalGpuRam,
        });
    }
}

// Export singleton instance
export const dataStore = new DataStore();
