/**
 * Observation-specific utility functions
 */

import type { ObservationData } from "$lib/types";

/**
 * Calculate progress percentage for an observation
 * Returns 0-100 representing completion percentage
 */
export function calculateProgress(observation: ObservationData): number {
    if (observation.status === "stopped") return 100;
    if (observation.status === "errored") return 0;
    if (observation.status === "standby") return 0;

    if (observation.startTime && observation.endTime) {
        const start = new Date(observation.startTime);
        const end = new Date(observation.endTime);
        const now = new Date();
        const elapsed = now.getTime() - start.getTime();
        const total = end.getTime() - start.getTime();
        return Math.min((elapsed / total) * 100, 95);
    }

    return 0;
}
