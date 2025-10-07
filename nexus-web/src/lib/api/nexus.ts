import { createClient, type Client } from "@connectrpc/connect";
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { Nexus } from "$lib/proto/nexus_pb";
import type {
    Observation,
    Replicant,
    Image,
    ObservationMetrics,
    HardwareMetrics,
    GenericMetric,
    LogEntry,
} from "$lib/proto/generic_pb";
import { Status } from "$lib/proto/enums_pb";
import { create } from "@bufbuild/protobuf";
import { TimestampSchema } from "@bufbuild/protobuf/wkt";

const transport = createGrpcWebTransport({
    baseUrl: "http://10.10.1.161:50051",
});

const client = createClient(Nexus, transport);

export interface CreateObservationParams {
    startTime: Date;
    endTime: Date;
    replicantTags?: string[];
    numberOfReplicants: number;
    imageId: string;
    validateOnly?: boolean;
}

export interface CreateImageParams {
    imageId: string;
    dockerImage: string;
    dockerEntrypoint?: string;
    dockerArgs?: string[];
    dockerEnv?: string[];
    validateOnly?: boolean;
}

export interface ListObservationsFilters {
    statusFilter?: Status[];
    startTimeAfter?: Date;
    startTimeBefore?: Date;
    tagFilter?: string[];
}

class NexusClient {
    private client: Client<typeof Nexus>;

    constructor(client: Client<typeof Nexus>) {
        this.client = client;
    }

    // Observations

    async createImage(params: CreateImageParams): Promise<void> {
        console.log("[API] createImage - Input:", params);

        const response = await this.client.createImage({
            imageId: params.imageId,
            dockerImage: params.dockerImage,
            dockerEntrypoint: params.dockerEntrypoint ?? "",
            dockerArgs: params.dockerArgs ?? [],
            dockerEnv: params.dockerEnv ?? [],
            validateOnly: params.validateOnly ?? false,
        });

        console.log("[API] createImage - Response:", response);

        if (!response.ack?.success) {
            console.error("[API] createImage - Error:", response.ack?.message);
            throw new Error(response.ack?.message || "Failed to create image");
        }

        console.log("[API] createImage - Success");
    }

    async destroyImage(imageId: string): Promise<void> {
        console.log("[API] destroyImage - Input:", { imageId });

        const response = await this.client.destroyImage({
            imageId: imageId,
        });

        console.log("[API] destroyImage - Response:", response);

        if (!response.ack?.success) {
            console.error("[API] destroyImage - Error:", response.ack?.message);
            throw new Error(response.ack?.message || "Failed to destroy image");
        }

        console.log("[API] destroyImage - Success");
    }

    async listImages(): Promise<Image[]> {
        console.log("[API] listImages - Input: {}");

        const response = await this.client.listImages({});

        console.log("[API] listImages - Response:", response);

        if (!response.ack?.success) {
            console.error("[API] listImages - Error:", response.ack?.message);
            throw new Error(response.ack?.message || "Failed to list images");
        }

        console.log(
            "[API] listImages - Success, returned",
            response.images.length,
            "images",
        );
        return response.images;
    }

    async createObservation(params: CreateObservationParams): Promise<string> {
        console.log("[API] createObservation - Input:", params);

        const response = await this.client.createObservation({
            startTime: create(TimestampSchema, {
                seconds: BigInt(Math.floor(params.startTime.getTime() / 1000)),
                nanos: (params.startTime.getTime() % 1000) * 1000000,
            }),
            endTime: create(TimestampSchema, {
                seconds: BigInt(Math.floor(params.endTime.getTime() / 1000)),
                nanos: (params.endTime.getTime() % 1000) * 1000000,
            }),
            imageId: params.imageId,
            replicantTags: params.replicantTags ?? [],
            numberOfReplicants: params.numberOfReplicants,
            validateOnly: params.validateOnly ?? false,
        });

        console.log("[API] createObservation - Response:", response);

        if (!response.ack?.success) {
            console.error(
                "[API] createObservation - Error:",
                response.ack?.message,
            );
            throw new Error(
                response.ack?.message || "Failed to create observation",
            );
        }

        // For validation-only requests, observation ID is not returned
        if (!params.validateOnly && !response.observationId) {
            console.error(
                "[API] createObservation - Error: No observation ID returned",
            );
            throw new Error("No observation ID returned");
        }

        console.log(
            "[API] createObservation - Success, ID:",
            response.observationId,
        );
        return response.observationId || "";
    }

    async destroyObservation(observationId: string): Promise<void> {
        console.log("[API] destroyObservation - Input:", { observationId });

        const response = await this.client.destroyObservation({
            observationId: observationId,
        });

        console.log("[API] destroyObservation - Response:", response);

        if (!response.ack?.success) {
            console.error(
                "[API] destroyObservation - Error:",
                response.ack?.message,
            );
            throw new Error(
                response.ack?.message || "Failed to destroy observation",
            );
        }

        console.log("[API] destroyObservation - Success");
    }

    async listObservations(
        filters?: ListObservationsFilters,
    ): Promise<Observation[]> {
        console.log("[API] listObservations - Input:", filters);

        const response = await this.client.listObservations({
            statusFilter: filters?.statusFilter?.map((s) => Status[s]) ?? [],
            startTimeAfter: filters?.startTimeAfter
                ? create(TimestampSchema, {
                      seconds: BigInt(
                          Math.floor(filters.startTimeAfter.getTime() / 1000),
                      ),
                      nanos:
                          (filters.startTimeAfter.getTime() % 1000) * 1000000,
                  })
                : undefined,
            startTimeBefore: filters?.startTimeBefore
                ? create(TimestampSchema, {
                      seconds: BigInt(
                          Math.floor(filters.startTimeBefore.getTime() / 1000),
                      ),
                      nanos:
                          (filters.startTimeBefore.getTime() % 1000) * 1000000,
                  })
                : undefined,
            tagFilter: filters?.tagFilter ?? [],
        });

        console.log("[API] listObservations - Response:", response);

        if (!response.ack?.success) {
            console.error(
                "[API] listObservations - Error:",
                response.ack?.message,
            );
            throw new Error(
                response.ack?.message || "Failed to list observations",
            );
        }

        console.log(
            "[API] listObservations - Success, returned",
            response.observations.length,
            "observations",
        );
        return response.observations;
    }

    // Replicants

    async listReplicants(): Promise<Replicant[]> {
        console.log("[API] listReplicants - Input: {}");

        const response = await this.client.listReplicants({});

        console.log("[API] listReplicants - Response:", response);

        if (!response.ack?.success) {
            console.error(
                "[API] listReplicants - Error:",
                response.ack?.message,
            );
            throw new Error(
                response.ack?.message || "Failed to list replicants",
            );
        }

        console.log(
            "[API] listReplicants - Success, returned",
            response.replicants.length,
            "replicants",
        );
        return response.replicants;
    }

    // Real-time Updates

    subscribeToSignals(): AsyncIterable<{
        timestamp: Date;
        events: Array<{
            type: string;
            observation?: Observation;
            replicant?: Replicant;
        }>;
        logs: LogEntry[];
        observationMetrics: ObservationMetrics[];
        hardwareMetrics: HardwareMetrics[];
    }> {
        return (async function* (client) {
            let retryCount = 0;
            const maxRetries = 5;
            const baseDelay = 1000; // 1 second

            while (retryCount < maxRetries) {
                try {
                    console.log(
                        `[API] subscribeToSignals - Attempt ${retryCount + 1}/${maxRetries}`,
                    );

                    const stream = client.signaller({ version: "1.0" });

                    // Reset retry count on successful connection
                    let hasReceivedData = false;

                    for await (const signal of stream) {
                        if (!hasReceivedData) {
                            console.log(
                                "[API] subscribeToSignals - Successfully connected to stream",
                            );
                            hasReceivedData = true;
                            retryCount = 0; // Reset on successful connection
                        }

                        console.log(
                            "[API] subscribeToSignals - Received signal:",
                            {
                                timestamp: signal.timestamp,
                                eventCount: signal.events.length,
                                logCount: signal.logs.length,
                                observationMetricsCount:
                                    signal.observationMetrics.length,
                                hardwareMetricsCount:
                                    signal.hardwareMetrics.length,
                            },
                        );

                        const timestamp = signal.timestamp
                            ? new Date(
                                  Number(signal.timestamp.seconds) * 1000 +
                                      Math.floor(
                                          Number(signal.timestamp.nanos) /
                                              1000000,
                                      ),
                              )
                            : new Date();

                        const processedSignal = {
                            timestamp,
                            events: signal.events.map((event) => ({
                                type: event.type.toString(),
                                observation:
                                    event.payload?.case === "observation"
                                        ? event.payload.value
                                        : undefined,
                                replicant:
                                    event.payload?.case === "replicant"
                                        ? event.payload.value
                                        : undefined,
                            })),
                            logs: signal.logs,
                            observationMetrics: signal.observationMetrics,
                            hardwareMetrics: signal.hardwareMetrics,
                        };

                        console.log(
                            "[API] subscribeToSignals - Processed signal:",
                            processedSignal,
                        );
                        yield processedSignal;
                    }

                    // If we get here, the stream ended normally
                    console.log(
                        "[API] subscribeToSignals - Stream ended normally",
                    );
                    break;
                } catch (error) {
                    retryCount++;
                    console.error(
                        `[API] subscribeToSignals - Stream error (attempt ${retryCount}/${maxRetries}):`,
                        error,
                    );

                    if (
                        error &&
                        typeof error === "object" &&
                        "message" in error
                    ) {
                        console.error(
                            "[API] subscribeToSignals - Error details:",
                            {
                                message: error.message,
                                code: "code" in error ? error.code : "unknown",
                                details:
                                    "details" in error ? error.details : "none",
                            },
                        );
                    }

                    if (retryCount >= maxRetries) {
                        console.error(
                            "[API] subscribeToSignals - Max retries exceeded, giving up",
                        );
                        throw error;
                    }

                    // Exponential backoff with jitter
                    const delay =
                        baseDelay * Math.pow(2, retryCount - 1) +
                        Math.random() * 1000;
                    console.log(
                        `[API] subscribeToSignals - Retrying in ${Math.round(delay)}ms...`,
                    );
                    await new Promise((resolve) => setTimeout(resolve, delay));
                }
            }
        })(this.client);
    }
}

export const nexusClient = new NexusClient(client);
