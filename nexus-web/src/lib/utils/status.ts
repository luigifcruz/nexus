/**
 * Status-related utility functions for UI components
 */

import PlayIcon from "@tabler/icons-svelte/icons/player-play";
import PauseIcon from "@tabler/icons-svelte/icons/player-pause";
import ClockIcon from "@tabler/icons-svelte/icons/clock";
import CheckIcon from "@tabler/icons-svelte/icons/check";
import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";
// Simple badge variant type to avoid module export issues
type BadgeVariant = "default" | "secondary" | "destructive" | "outline";

/**
 * Status badge configuration
 */
export interface StatusBadgeConfig {
    variant: BadgeVariant;
    className?: string;
    text: string;
}

/**
 * Get complete badge configuration for status
 */
export function getStatusBadgeConfig(status: string): StatusBadgeConfig {
    const normalizedStatus = status?.toLowerCase() || "unknown";
    const capitalizedText =
        normalizedStatus.charAt(0).toUpperCase() + normalizedStatus.slice(1);

    switch (normalizedStatus) {
        case "running":
        case "online":
            return {
                variant: "outline",
                className:
                    "border-green-500 text-green-700 dark:text-green-400",
                text: capitalizedText,
            };

        case "stopped":
            return {
                variant: "outline",
                className: "border-blue-500 text-blue-700 dark:text-blue-400",
                text: capitalizedText,
            };

        case "completed":
            return {
                variant: "outline",
                className: "border-blue-500 text-blue-700 dark:text-blue-400",
                text: capitalizedText,
            };

        case "errored":
        case "error":
        case "failed":
            return {
                variant: "outline",
                className: "border-red-500 text-red-700 dark:text-red-400",
                text: capitalizedText,
            };

        case "standby":
        case "starting":
        case "stopping":
        case "maintenance":
        case "scheduled":
        case "pending":
            return {
                variant: "outline",
                className:
                    "border-yellow-500 text-yellow-700 dark:text-yellow-400",
                text: capitalizedText,
            };

        default:
            return {
                variant: "outline",
                text: "Unknown",
            };
    }
}

/**
 * Get icon component for status
 */
export function getStatusIcon(status: string) {
    const normalizedStatus = status?.toLowerCase() || "unknown";

    switch (normalizedStatus) {
        case "running":
        case "online":
            return PlayIcon;
        case "standby":
        case "starting":
        case "maintenance":
        case "scheduled":
        case "pending":
            return ClockIcon;
        case "stopped":
        case "offline":
        case "completed":
            return CheckIcon;
        case "errored":
        case "error":
        case "failed":
            return AlertTriangleIcon;
        case "stopping":
            return PauseIcon;
        default:
            return PauseIcon;
    }
}

/**
 * Get icon component and color class for status
 * Returns object with { icon: Component, class: string }
 */
export function getStatusIconWithClass(status: string) {
    const icon = getStatusIcon(status);
    const normalizedStatus = status?.toLowerCase() || "unknown";
    let colorClass = "text-gray-500";

    switch (normalizedStatus) {
        case "running":
        case "online":
            colorClass = "text-green-600";
            break;
        case "standby":
        case "starting":
        case "maintenance":
        case "scheduled":
        case "pending":
            colorClass = "text-yellow-600";
            break;
        case "errored":
        case "error":
        case "failed":
            colorClass = "text-red-600";
            break;
        case "stopped":
            colorClass = "text-blue-600";
            break;
        case "offline":
            colorClass = "text-green-600";
            break;
        case "completed":
            colorClass = "text-blue-600";
            break;
    }

    return { icon, class: colorClass };
}
