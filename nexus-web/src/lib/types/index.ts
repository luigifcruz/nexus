// API-compatible types based on protobuf definitions

import type { Status as ProtoStatus } from "$lib/proto/enums_pb";
import { EventType as ProtoEventType } from "$lib/proto/enums_pb";
import type {
    Observation,
    Replicant,
    Image,
    HardwareMetrics,
} from "$lib/proto/generic_pb";

// Re-export protobuf types for convenience
export type { Observation, Replicant, Image, HardwareMetrics };

// Unified status type for all entities
export type Status =
    | "unknown"
    | "standby"
    | "starting"
    | "running"
    | "stopping"
    | "stopped"
    | "errored"
    | "maintenance"
    | "disconnected";

// Clean TypeScript enum for event types
export enum EventType {
    CREATE = "CREATE",
    UPDATE = "UPDATE",
    DELETE = "DELETE",
}

// Extended types for UI-specific data (not from API)
export interface ReplicantData {
    replicantId: string;
    version: string;
    tags: string[];
    status: Status;
    hardware?: {
        gpuType?: string;
        gpuPcieId?: string;
        cpuType?: string;
        cpuSocket?: string;
        cpuThreads?: number[];
        memorySize?: number;
        storageSize?: number;
        storagePath?: string;
        networkType?: string;
        networkSpeed?: string;
        networkPcieId?: string;
        networkInterface?: string;
        networkMac?: string;
    };
    lastSeen: Date;
    createdAt: Date;
    // UI-specific fields (computed from metrics)
    currentMetrics?: {
        cpuUsage?: number;
        memoryUsage?: number;
        storageUsage?: number;
        networkBandwidth?: number;
        gpuUsage?: number;
        gpuMemoryUsage?: number;
        temperature?: number;
    };
}

export interface ObservationData {
    observationId: string;
    imageId: string;
    startTime: Date;
    endTime: Date;
    replicantTags: string[];
    numberOfReplicants: number;
    status: Status;
    createdAt: Date;
    updatedAt?: Date;
    replicantIds: string[]; // Array of replicant IDs
}

export interface ImageData {
    imageId: string;
    dockerImage: string;
    dockerEntrypoint: string;
    dockerArgs: string[];
    dockerEnv: string[];
    createdAt?: Date;
}

export interface ConnectionStatus {
    connected: boolean;
    reconnecting: boolean;
    error: string;
    lastConnected?: string;
    attempts?: number;
}

export interface SystemMetrics {
    totalBandwidth: number;
    totalNvmeUsage: number;
    totalGpuRam: number;
    activeReplicants: number;
    onlineReplicants: number;
    activeObservations: number;
    scheduledObservations: number;
    totalReplicants?: number;
    completedObservations?: number;
    failedObservations?: number;
    systemUptime?: number;
    totalStorage?: number;
    usedStorage?: number;
}

export interface ActivityItem {
    id: string;
    type: "replicant" | "observation" | "system";
    message: string;
    timestamp: Date;
    status: "success" | "warning" | "error" | "info";
    details?: Record<string, any>;
}

export interface FilterOptions {
    searchTerm: string;
    statusFilter: string;
    dateRange: string;
    locationFilter?: string;
    priorityFilter?: string;
}

export type StatusBadgeVariant =
    | "default"
    | "secondary"
    | "destructive"
    | "outline";

// Helper to convert protobuf Status enum to string
export function statusToString(status: ProtoStatus): Status {
    switch (status) {
        case 0: // UNKNOWN
            return "unknown";
        case 1: // STANDBY
            return "standby";
        case 2: // STARTING
            return "starting";
        case 3: // RUNNING
            return "running";
        case 4: // STOPPING
            return "stopping";
        case 5: // STOPPED
            return "stopped";
        case 6: // ERRORED
            return "errored";
        case 7: // MAINTENANCE
            return "maintenance";
        case 8: // DISCONNECTED
            return "disconnected";
        default:
            return "unknown";
    }
}

// Helper to convert protobuf EventType enum to TypeScript EventType
export function convertEventType(
    protoEventType: ProtoEventType | number | string,
): EventType {
    // Handle numeric values
    if (typeof protoEventType === "number") {
        switch (protoEventType) {
            case 0:
                return EventType.CREATE;
            case 1:
                return EventType.UPDATE;
            case 2:
                return EventType.DELETE;
            default:
                return EventType.CREATE;
        }
    }

    // Handle string values
    if (typeof protoEventType === "string") {
        // First try to parse as numeric string
        const numValue = parseInt(protoEventType, 10);
        if (!isNaN(numValue)) {
            switch (numValue) {
                case 0:
                    return EventType.CREATE;
                case 1:
                    return EventType.UPDATE;
                case 2:
                    return EventType.DELETE;
                default:
                    return EventType.CREATE;
            }
        }

        // Then try string matching
        switch (protoEventType.toLowerCase()) {
            case "create":
                return EventType.CREATE;
            case "update":
                return EventType.UPDATE;
            case "delete":
                return EventType.DELETE;
            default:
                return EventType.CREATE;
        }
    }

    // Handle protobuf enum values
    switch (protoEventType) {
        case ProtoEventType.CREATE:
            return EventType.CREATE;
        case ProtoEventType.UPDATE:
            return EventType.UPDATE;
        case ProtoEventType.DELETE:
            return EventType.DELETE;
        default:
            return EventType.CREATE;
    }
}

// Helper to convert protobuf Timestamp to Date
export function timestampToDate(timestamp?: {
    seconds: bigint;
    nanos: number;
}): Date {
    if (!timestamp) return new Date();
    const milliseconds =
        Number(timestamp.seconds) * 1000 + timestamp.nanos / 1000000;
    return new Date(milliseconds);
}
