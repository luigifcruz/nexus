/**
 * Enum conversion utilities
 *
 * Converts protobuf enums to human-readable strings
 */

import {
    Status,
    GpuType,
    CpuType,
    NetworkType,
    NetworkSpeed,
} from "$lib/proto/enums_pb";

/**
 * Convert Status enum to readable string
 */
export function statusToString(status: Status): string {
    switch (status) {
        case Status.UNKNOWN:
            return "unknown";
        case Status.STANDBY:
            return "standby";
        case Status.STARTING:
            return "starting";
        case Status.RUNNING:
            return "running";
        case Status.STOPPING:
            return "stopping";
        case Status.STOPPED:
            return "stopped";
        case Status.ERRORED:
            return "errored";
        case Status.MAINTENANCE:
            return "maintenance";
        case Status.DISCONNECTED:
            return "disconnected";
        default:
            return "unknown";
    }
}

/**
 * Convert GpuType enum to readable string
 */
export function gpuTypeToString(gpuType: GpuType): string {
    switch (gpuType) {
        case GpuType.NVIDIA_RTX6000_ADA:
            return "NVIDIA RTX 6000 Ada";
        case GpuType.NVIDIA_A4000:
            return "NVIDIA A4000";
        default:
            return "Unknown GPU";
    }
}

/**
 * Convert CpuType enum to readable string
 */
export function cpuTypeToString(cpuType: CpuType): string {
    switch (cpuType) {
        case CpuType.AMD_EPYC_MILAN:
            return "AMD EPYC Milan";
        case CpuType.AMD_EPYC_GENOA:
            return "AMD EPYC Genoa";
        default:
            return "Unknown CPU";
    }
}

/**
 * Convert NetworkType enum to readable string
 */
export function networkTypeToString(networkType: NetworkType): string {
    switch (networkType) {
        case NetworkType.CONNECTX6:
            return "ConnectX-6";
        case NetworkType.CONNECTX7:
            return "ConnectX-7";
        default:
            return "Unknown Network";
    }
}

/**
 * Convert NetworkSpeed enum to readable string
 */
export function networkSpeedToString(networkSpeed: NetworkSpeed): string {
    switch (networkSpeed) {
        case NetworkSpeed.GBE1:
            return "1 GbE";
        case NetworkSpeed.GBE10:
            return "10 GbE";
        case NetworkSpeed.GBE25:
            return "25 GbE";
        case NetworkSpeed.GBE40:
            return "40 GbE";
        case NetworkSpeed.GBE100:
            return "100 GbE";
        case NetworkSpeed.GBE200:
            return "200 GbE";
        default:
            return "Unknown Speed";
    }
}

/**
 * Convert NetworkSpeed enum to numeric value in Gbps
 */
export function networkSpeedToGbps(networkSpeed: NetworkSpeed): number {
    switch (networkSpeed) {
        case NetworkSpeed.GBE1:
            return 1;
        case NetworkSpeed.GBE10:
            return 10;
        case NetworkSpeed.GBE25:
            return 25;
        case NetworkSpeed.GBE40:
            return 40;
        case NetworkSpeed.GBE100:
            return 100;
        case NetworkSpeed.GBE200:
            return 200;
        default:
            return 0;
    }
}
