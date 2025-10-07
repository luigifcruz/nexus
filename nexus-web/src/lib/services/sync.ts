/**
 * API Sync Service
 *
 * Handles synchronization between the gRPC API client and the data store.
 */

import { nexusClient } from "$lib/api/nexus";
import { dataStore } from "$lib/stores/data.svelte";
import {
    statusToString,
    timestampToDate,
    convertEventType,
    EventType as UIEventType,
} from "$lib/types";
import {
    gpuTypeToString,
    cpuTypeToString,
    networkTypeToString,
    networkSpeedToString,
} from "$lib/utils/enums";
import type {
    ReplicantData,
    ObservationData,
    ImageData,
    ActivityItem,
} from "$lib/types";
import type {
    Observation,
    Replicant,
    Image,
    HardwareMetrics,
    ObservationMetrics,
    GenericMetric,
    LogEntry,
} from "$lib/proto/generic_pb";

/**
 * Converts a protobuf Replicant to ReplicantData for the UI
 */
function convertReplicantToData(replicant: Replicant): ReplicantData {
    return {
        replicantId: replicant.replicantId ?? "",
        version: replicant.version,
        tags: replicant.tags,
        status: statusToString(replicant.status),
        hardware: replicant.hardware
            ? {
                  gpuType: gpuTypeToString(replicant.hardware.gpuType),
                  gpuPcieId: replicant.hardware.gpuPcieId,
                  cpuType: cpuTypeToString(replicant.hardware.cpuType),
                  cpuSocket: replicant.hardware.cpuSocket,
                  cpuThreads: replicant.hardware.cpuThreads,
                  memorySize: replicant.hardware.memorySize,
                  storageSize: replicant.hardware.storageSize,
                  storagePath: replicant.hardware.storagePath,
                  networkType: networkTypeToString(
                      replicant.hardware.networkType,
                  ),
                  networkSpeed: networkSpeedToString(
                      replicant.hardware.networkSpeed,
                  ),
                  networkPcieId: replicant.hardware.networkPcieId,
                  networkInterface: replicant.hardware.networkInterface,
                  networkMac: replicant.hardware.networkMac,
              }
            : undefined,
        lastSeen: timestampToDate(replicant.lastSeen),
        createdAt: timestampToDate(replicant.createdAt),
    };
}

/**
 * Converts a protobuf Observation to ObservationData for the UI
 */
function convertObservationToData(observation: Observation): ObservationData {
    return {
        observationId: observation.observationId ?? "",
        imageId: observation.imageId ?? "",
        startTime: timestampToDate(observation.startTime),
        endTime: timestampToDate(observation.endTime),
        replicantTags: observation.replicantTags ?? [],
        numberOfReplicants: observation.numberOfReplicants ?? 0,
        status: statusToString(observation.status),
        createdAt: timestampToDate(observation.createdAt),
        updatedAt: timestampToDate(observation.updatedAt),
        replicantIds: observation.replicantIds ?? [],
    };
}

/**
 * Converts a protobuf Image to ImageData for the UI
 */
function convertImageToData(image: Image): ImageData {
    return {
        imageId: image.imageId ?? "",
        dockerImage: image.dockerImage ?? "",
        dockerEntrypoint: image.dockerEntrypoint ?? "",
        dockerArgs: image.dockerArgs ?? [],
        dockerEnv: image.dockerEnv ?? [],
    };
}

class ApiSyncService {
    private signallerActive = false;

    async initialize() {
        console.log("[SYNC] Initializing API sync service...");
        dataStore.setConnectionStatus({
            connected: false,
            reconnecting: true,
            error: "",
        });

        try {
            console.log("[SYNC] Testing initial connection...");
            const connectionTest = await this.testConnection();

            if (!connectionTest.success) {
                throw new Error(
                    `Initial connection failed: ${connectionTest.error}`,
                );
            }

            console.log("[SYNC] Loading initial data...");
            // Load initial data
            await this.loadAllData();

            console.log("[SYNC] Starting signaller stream...");
            this.startSignaller();

            dataStore.setConnectionStatus({
                connected: true,
                reconnecting: false,
                error: "",
                lastConnected: new Date().toISOString(),
            });

            console.log("[SYNC] API sync service initialized successfully");
        } catch (error) {
            console.error(
                "[SYNC] Failed to initialize API sync service:",
                error,
            );
            dataStore.setConnectionStatus({
                connected: false,
                reconnecting: false,
                error:
                    error instanceof Error
                        ? error.message
                        : "Connection failed",
            });
        }
    }

    /**
     * Load all data from the API
     */
    private async loadAllData() {
        await Promise.all([
            this.loadReplicants(),
            this.loadObservations(),
            this.loadImages(),
        ]);
    }

    /**
     * Load all replicants from the API
     */
    async loadReplicants() {
        try {
            const replicants = await nexusClient.listReplicants();
            const replicantData = replicants.map(convertReplicantToData);
            dataStore.setReplicants(replicantData);
        } catch (error) {
            console.error("Failed to load replicants:", error);
            throw error;
        }
    }

    /**
     * Load all observations from the API
     */
    async loadObservations() {
        try {
            const observations = await nexusClient.listObservations();
            const observationData = observations.map(convertObservationToData);
            dataStore.setObservations(observationData);
        } catch (error) {
            console.error("Failed to load observations:", error);
            throw error;
        }
    }

    /**
     * Load all images from the API
     */
    async loadImages() {
        try {
            const images = await nexusClient.listImages();
            const imageData = images.map(convertImageToData);
            dataStore.setImages(imageData);
        } catch (error) {
            console.error("Failed to load images:", error);
            throw error;
        }
    }

    /**
     * Test basic connectivity to the server
     */
    private async testConnection(): Promise<{
        success: boolean;
        error?: string;
    }> {
        try {
            console.log("[SYNC] Testing server connectivity...");
            await nexusClient.listReplicants();
            console.log("[SYNC] Connection test successful");
            return { success: true };
        } catch (error) {
            console.error("[SYNC] Connection test failed:", error);
            return {
                success: false,
                error: error instanceof Error ? error.message : "Unknown error",
            };
        }
    }

    /**
     * Start the signaller to listen for real-time updates
     */
    private async startSignaller() {
        if (this.signallerActive) {
            console.log("[SYNC] Signaller already active, skipping");
            return;
        }

        console.log("[SYNC] Starting signaller stream...");
        dataStore.setConnectionStatus({
            connected: false,
            reconnecting: true,
            error: "",
        });

        // First test basic connectivity
        const connectionTest = await this.testConnection();
        if (!connectionTest.success) {
            console.error(
                "[SYNC] Basic connectivity test failed:",
                connectionTest.error,
            );
            dataStore.setConnectionStatus({
                connected: false,
                reconnecting: false,
                error: `Connection failed: ${connectionTest.error}. Please check if nexus-server is running and accessible.`,
            });

            // Still try to connect in case it's just a timing issue
            console.log(
                "[SYNC] Attempting signaller stream despite connectivity test failure...",
            );
        }

        try {
            this.signallerActive = true;
            const signals = nexusClient.subscribeToSignals();

            console.log("[SYNC] Signaller stream established successfully");
            dataStore.setConnectionStatus({
                connected: true,
                reconnecting: false,
                error: "",
                lastConnected: new Date().toISOString(),
            });

            let signalCount = 0;
            for await (const signal of signals) {
                signalCount++;
                console.log(`[SYNC] Processing signal #${signalCount}:`, {
                    timestamp: signal.timestamp
                        ? typeof (signal.timestamp as any).toDate === "function"
                            ? (signal.timestamp as any).toDate().toISOString()
                            : signal.timestamp.toString()
                        : "no timestamp",
                    eventCount: signal.events.length,
                    logCount: signal.logs.length,
                    observationMetricsCount: signal.observationMetrics.length,
                    hardwareMetricsCount: signal.hardwareMetrics.length,
                });

                // Handle events from the signaller
                for (const event of signal.events) {
                    const eventType = convertEventType(event.type);
                    console.log(
                        `[SYNC] Processing event #${signalCount}:`,
                        `${event.type} (${eventType})`,
                        {
                            hasReplicant: !!event.replicant,
                            hasObservation: !!event.observation,
                            replicantId: event.replicant?.replicantId,
                            observationId: event.observation?.observationId,
                        },
                    );

                    if (
                        eventType === UIEventType.CREATE ||
                        eventType === UIEventType.UPDATE
                    ) {
                        if (event.replicant) {
                            console.log(
                                "[SYNC] Adding/updating replicant:",
                                event.replicant.replicantId,
                                "status:",
                                event.replicant.status,
                            );
                            const replicantData = convertReplicantToData(
                                event.replicant,
                            );
                            dataStore.addReplicant(replicantData);
                            console.log(
                                "[SYNC] Replicant data updated in store",
                            );
                        }
                        if (event.observation) {
                            console.log(
                                "[SYNC] Adding/updating observation:",
                                event.observation.observationId,
                                "status:",
                                event.observation.status,
                            );
                            const observationData = convertObservationToData(
                                event.observation,
                            );
                            dataStore.addObservation(observationData);
                            console.log(
                                "[SYNC] Observation data updated in store",
                            );
                        }
                    } else if (eventType === UIEventType.DELETE) {
                        if (event.replicant) {
                            console.log(
                                "[SYNC] Removing replicant:",
                                event.replicant.replicantId,
                            );
                            dataStore.removeReplicant(
                                event.replicant.replicantId || "",
                            );
                        }
                        if (event.observation) {
                            console.log(
                                "[SYNC] Removing observation:",
                                event.observation.observationId,
                            );
                            dataStore.removeObservation(
                                event.observation.observationId || "",
                            );
                        }
                    } else {
                        console.warn(
                            "[SYNC] Unknown event type:",
                            event.type,
                            eventType,
                        );
                    }
                }

                if (signal.hardwareMetrics.length > 0) {
                    const metricsByReplicant = new Map<
                        string,
                        HardwareMetrics[]
                    >();

                    for (const metrics of signal.hardwareMetrics) {
                        if (metrics.replicantId) {
                            const existing =
                                metricsByReplicant.get(metrics.replicantId) ||
                                [];
                            existing.push(metrics);
                            metricsByReplicant.set(
                                metrics.replicantId,
                                existing,
                            );
                        }
                    }

                    for (const [replicantId, metrics] of metricsByReplicant) {
                        console.log(
                            "[SYNC] Hardware metrics for replicant:",
                            replicantId,
                            metrics.length,
                            "metrics",
                        );
                        dataStore.addReplicantMetrics(replicantId, metrics);
                    }
                }

                const observationIdsInEvents = new Set(
                    signal.events
                        .filter((event) => event.observation)
                        .map((event) => event.observation!.observationId)
                        .filter((id) => id) as string[],
                );

                if (signal.observationMetrics.length > 0) {
                    const metricsByObservation = new Map<
                        string,
                        ObservationMetrics[]
                    >();

                    for (const metrics of signal.observationMetrics) {
                        if (metrics.observationId) {
                            const existing =
                                metricsByObservation.get(
                                    metrics.observationId,
                                ) || [];
                            existing.push(metrics);
                            metricsByObservation.set(
                                metrics.observationId,
                                existing,
                            );
                        }
                    }

                    for (const [
                        observationId,
                        metrics,
                    ] of metricsByObservation) {
                        console.log(
                            "[SYNC] Observation metrics for:",
                            observationId,
                            metrics.length,
                            "metrics",
                        );
                        dataStore.addObservationMetrics(observationId, metrics);
                    }
                }

                if (signal.logs.length > 0) {
                    const logsByObservation = new Map<string, LogEntry[]>();

                    for (const log of signal.logs) {
                        if (log.observationId) {
                            const existing =
                                logsByObservation.get(log.observationId) || [];
                            existing.push(log);
                            logsByObservation.set(log.observationId, existing);
                        }
                    }

                    for (const [observationId, logs] of logsByObservation) {
                        console.log(
                            "[SYNC] Logs for observation:",
                            observationId,
                            logs.length,
                            "entries",
                        );
                        dataStore.addObservationLogs(observationId, logs);
                    }
                }

                // Update connection timestamp
                dataStore.setConnectionStatus({
                    connected: true,
                    reconnecting: false,
                    error: "",
                    lastConnected: new Date().toISOString(),
                });
            }
        } catch (error) {
            console.error("[SYNC] Signaller stream error:", error);
            this.signallerActive = false;

            // Provide detailed error diagnostics
            let errorMessage = "Stream connection failed";
            let diagnostics = "";

            if (error instanceof Error) {
                errorMessage = error.message;

                // Check for common error patterns
                if (error.message.includes("fetch")) {
                    diagnostics =
                        " (Network/CORS issue - check if server is running on the expected port)";
                } else if (error.message.includes("ConnectError")) {
                    diagnostics =
                        " (gRPC connection error - verify server address and gRPC-Web support)";
                } else if (error.message.includes("input stream")) {
                    diagnostics =
                        " (Stream input error - server may not be sending proper signaller responses)";
                }
            }

            console.error("[SYNC] Error diagnostics:", {
                message: errorMessage,
                diagnostics: diagnostics,
                errorType: error?.constructor?.name || "Unknown",
                stack: error instanceof Error ? error.stack : "No stack trace",
            });

            // Update connection status with error
            dataStore.setConnectionStatus({
                connected: false,
                reconnecting: false,
                error: errorMessage + diagnostics,
            });

            // Attempt to reconnect after a delay
            console.log("[SYNC] Attempting to reconnect in 5 seconds...");
            setTimeout(() => {
                if (!this.signallerActive) {
                    console.log("[SYNC] Reconnecting signaller...");
                    dataStore.setConnectionStatus({
                        connected: false,
                        reconnecting: true,
                        error: "",
                    });
                    this.startSignaller();
                }
            }, 5000);
        }
    }

    /**
     * Create a new observation
     */
    async createObservation(params: {
        startTime: Date;
        endTime: Date;
        replicantTags?: string[];
        numberOfReplicants: number;
        imageId: string;
        validateOnly?: boolean;
    }): Promise<string> {
        const observationId = await nexusClient.createObservation(params);

        // Add activity log
        dataStore.addActivity({
            id: `act-${Date.now()}`,
            type: "observation",
            message: `Observation created with image ${params.imageId}`,
            timestamp: new Date(),
            status: "success",
            details: { observationId },
        });

        // Reload observations to get the new one
        await this.loadObservations();

        return observationId;
    }

    /**
     * Destroy an observation
     */
    async destroyObservation(observationId: string): Promise<void> {
        await nexusClient.destroyObservation(observationId);

        // Add activity log
        const observation = dataStore.getObservationById(observationId);
        dataStore.addActivity({
            id: `act-${Date.now()}`,
            type: "observation",
            message: `Observation ${observation?.observationId || observationId} destroyed`,
            timestamp: new Date(),
            status: "success",
            details: { observationId },
        });

        // Remove from store
        dataStore.removeObservation(observationId);
    }

    /**
     * Create a new image
     */
    async createImage(params: {
        imageId: string;
        dockerImage: string;
        dockerEntrypoint?: string;
        dockerArgs?: string[];
        dockerEnv?: string[];
        validateOnly?: boolean;
    }): Promise<void> {
        await nexusClient.createImage(params);

        // Add activity log
        dataStore.addActivity({
            id: `act-${Date.now()}`,
            type: "system",
            message: `Image ${params.imageId} created`,
            timestamp: new Date(),
            status: "success",
            details: { imageId: params.imageId },
        });

        // Reload images to get the new one
        await this.loadImages();
    }

    /**
     * Destroy an image
     */
    async destroyImage(imageId: string): Promise<void> {
        await nexusClient.destroyImage(imageId);

        // Add activity log
        const image = dataStore.getImageById(imageId);
        dataStore.addActivity({
            id: `act-${Date.now()}`,
            type: "system",
            message: `Image ${image?.imageId || imageId} destroyed`,
            timestamp: new Date(),
            status: "success",
            details: { imageId },
        });

        // Remove from store
        dataStore.removeImage(imageId);
    }

    cleanup() {
        this.signallerActive = false;
    }

    async reconnect() {
        this.cleanup();
        await this.initialize();
    }
}

export const apiSync = new ApiSyncService();

export { convertReplicantToData, convertObservationToData };
