<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { dataStore } from "$lib/stores/data.svelte";
    import { formatBytes } from "$lib/utils/formatters";
    import type {
        ReplicantData,
        ObservationData,
        SystemMetrics,
        ActivityItem,
        ConnectionStatus,
    } from "$lib/types";

    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import DatabaseIcon from "@tabler/icons-svelte/icons/database";
    import EyeIcon from "@tabler/icons-svelte/icons/eye";
    import NetworkIcon from "@tabler/icons-svelte/icons/network";

    import RunningObservationCard from "$lib/components/running-observation-card.svelte";
    import ObservationTable from "$lib/components/observation-table.svelte";
    import EmptyObservationsPlaceholder from "$lib/components/empty-observations-placeholder.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { Button } from "$lib/components/ui/button/index.js";

    let connectionStatus = $derived(dataStore.connectionStatus);
    let runningObservations = $derived(dataStore.runningObservations);

    let recentObservations = $derived(dataStore.observations
        .filter((obs) => obs.status !== "running")
        .sort(
            (a, b) =>
                new Date(b.updatedAt).getTime() -
                new Date(a.updatedAt).getTime(),
        )
        .slice(0, 10));

    let onlineReplicants = $derived(dataStore.onlineReplicants);

    let replicantHealth = $derived.by(() => {
        const total = dataStore.replicants.length;
        const online = onlineReplicants.length;
        const pct = total > 0 ? (online / total) * 100 : 0;
        if (pct >= 80) return { level: "Excellent", color: "green" };
        if (pct >= 50) return { level: "Degraded", color: "orange" };
        return { level: "Critical", color: "red" };
    });

    let runningCount = $derived(dataStore.runningObservations.length);
    let scheduledCount = $derived(dataStore.scheduledObservations.length);

    let observationHealth = $derived.by(() => {
        // Excellent: 1+ running observations
        if (runningCount > 0) return { level: "Excellent", color: "green" };

        // Degraded: 1+ failed observations in the last 24 hours
        const twentyFourHoursAgo = new Date(Date.now() - 24 * 60 * 60 * 1000);
        const recentFailedCount = dataStore.observations.filter(obs => {
            if (obs.status !== "errored" && obs.status !== "failed") return false;
            if (!obs.endTime) return false;
            const endTime = new Date(obs.endTime);
            return endTime >= twentyFourHoursAgo;
        }).length;

        if (recentFailedCount > 0) return { level: "Degraded", color: "orange" };

        // Idle: 0 running observations
        return { level: "Idle", color: "gray" };
    });

    let bandwidthHealth = $derived.by(() => {
        const maxMbps = 12800000;
        const used = dataStore.systemMetrics.totalBandwidth;
        const pct = (used / maxMbps) * 100;
        if (pct < 50) return { level: "Excellent", color: "green" };
        if (pct < 80) return { level: "Degraded", color: "orange" };
        return { level: "Critical", color: "red" };
    });

    let storageHealth = $derived.by(() => {
        const maxGB = 512 * 1024;
        const usedGB = dataStore.systemMetrics.usedStorage || 0;
        const pct = (usedGB / maxGB) * 100;
        if (pct < 70) return { level: "Excellent", color: "green" };
        if (pct < 90) return { level: "Degraded", color: "orange" };
        return { level: "Critical", color: "red" };
    });

    function getHealthClasses(health: { level: string; color: string }) {
        const colorMap = {
            green: {
                iconBg: 'bg-green-100 dark:bg-green-900/30',
                iconColor: 'text-green-600 dark:text-green-400',
                gradient: 'from-green-800/10'
            },
            orange: {
                iconBg: 'bg-orange-100 dark:bg-orange-900/30',
                iconColor: 'text-orange-600 dark:text-orange-400',
                gradient: 'from-orange-800/10'
            },
            red: {
                iconBg: 'bg-red-100 dark:bg-red-900/30',
                iconColor: 'text-red-600 dark:text-red-400',
                gradient: 'from-red-800/10'
            }
        };

        const colors = colorMap[health.color as keyof typeof colorMap] || colorMap.green;

        return {
            border: 'border-gray-200 dark:border-gray-700',
            bg: 'bg-gray-50 dark:bg-gradient-to-br from-gray-950 to-gray-900',
            ...colors
        };
    }

    let replicantClasses = $derived(getHealthClasses(replicantHealth));
    let observationClasses = $derived(getHealthClasses(observationHealth));
    let bandwidthClasses = $derived(getHealthClasses(bandwidthHealth));
    let storageClasses = $derived(getHealthClasses(storageHealth));

    // Navigation helpers
    function navigateToReplicants() {
        goto("/replicants");
    }

    function navigateToObservations() {
        goto("/observations");
    }

    function navigateToReplicant(replicantId: string) {
        goto(`/replicants/${replicantId}`);
    }

    function navigateToObservation(observationId: string) {
        goto(`/observations/${observationId}`);
    }
</script>

<div class="flex flex-col gap-6 py-6">
        <!-- Dashboard Header -->
        <div class="px-4 lg:px-6">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold tracking-tight">Dashboard</h1>
                    <p class="text-muted-foreground">
                        System overview and recent activity
                    </p>
                </div>
                <div class="flex gap-2">
                    <Button onclick={navigateToReplicants} variant="outline">
                        <ServerIcon class="h-4 w-4" />
                        Replicants
                    </Button>
                    <Button onclick={navigateToObservations}>
                        <EyeIcon class="h-4 w-4" />
                        Observations
                    </Button>
                </div>
            </div>
        </div>

        <!-- System Status Cards -->
        <div class="px-4 lg:px-6">
            <div class="grid grid-cols-4 gap-3">
                <!-- System Health -->
                <Card.Root class="group relative overflow-hidden {replicantClasses.border} {replicantClasses.bg} shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <div class="absolute inset-0 bg-gradient-to-r {replicantClasses.gradient} via-transparent to-transparent opacity-40 group-hover:opacity-70 transition-opacity duration-500"></div>
                    <Card.Header class="pb-1 pt-1 relative z-10">
                        <div class="flex items-center">
                            <div class="flex items-center gap-3">
                                <div class="p-2 rounded-lg {replicantClasses.iconBg}">
                                    <ServerIcon class="h-5 w-5 {replicantClasses.iconColor}" />
                                </div>
                                <div>
                                    <Card.Description class="text-sm font-medium text-muted-foreground">
                                        Replicant Health
                                    </Card.Description>
                                </div>
                            </div>
                        </div>
                        <Card.Title class="text-3xl font-bold tabular-nums pt-2">
                            {replicantHealth.level}
                        </Card.Title>
                        <div class="flex flex-row space-x-2">
                            <Badge variant="success">{onlineReplicants.length} Healthy</Badge>
                        </div>
                    </Card.Header>
                </Card.Root>

                <!-- Active Observations -->
                <Card.Root class="group relative overflow-hidden {observationClasses.border} {observationClasses.bg} shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <div class="absolute inset-0 bg-gradient-to-r {observationClasses.gradient} via-transparent to-transparent opacity-40 group-hover:opacity-70 transition-opacity duration-500"></div>
                    <Card.Header class="pb-1 pt-1 relative z-10">
                        <div class="flex items-center">
                            <div class="flex items-center gap-3">
                                <div class="p-2 rounded-lg {observationClasses.iconBg}">
                                    <EyeIcon class="h-5 w-5 {observationClasses.iconColor}" />
                                </div>
                                <div>
                                    <Card.Description class="text-sm font-medium text-muted-foreground">
                                        Observation Health
                                    </Card.Description>
                                </div>
                            </div>
                        </div>
                        <Card.Title class="text-3xl font-bold tabular-nums pt-2">
                            {observationHealth.level}
                        </Card.Title>
                        <div class="flex flex-row space-x-2">
                            <Badge variant="success">{runningCount} Running</Badge>
                            <Badge variant="success">{scheduledCount} Scheduled</Badge>
                        </div>
                    </Card.Header>
                </Card.Root>

                <!-- Total Bandwidth -->
                <Card.Root class="group relative overflow-hidden {bandwidthClasses.border} {bandwidthClasses.bg} shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <div class="absolute inset-0 bg-gradient-to-r {bandwidthClasses.gradient} via-transparent to-transparent opacity-40 group-hover:opacity-70 transition-opacity duration-500"></div>
                    <Card.Header class="pb-1 pt-1 relative z-10">
                        <div class="flex items-center">
                            <div class="flex items-center gap-3">
                                <div class="p-2 rounded-lg {bandwidthClasses.iconBg}">
                                    <NetworkIcon class="h-5 w-5 {bandwidthClasses.iconColor}" />
                                </div>
                                <div>
                                    <Card.Description class="text-sm font-medium text-muted-foreground">
                                        Network Bandwidth
                                    </Card.Description>
                                </div>
                            </div>
                        </div>
                        <Card.Title class="text-3xl font-bold tabular-nums pt-2">
                            {dataStore.systemMetrics.totalBandwidth} <span class="text-xl font-medium text-muted-foreground">Mbps</span>
                        </Card.Title>
                        <div class="text-sm text-muted-foreground italic tabular-nums pt-0">
                            of 12.8 Tbps
                        </div>
                    </Card.Header>
                </Card.Root>

                <!-- Storage Usage -->
                <Card.Root class="group relative overflow-hidden {storageClasses.border} {storageClasses.bg} shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                    <div class="absolute inset-0 bg-gradient-to-r {storageClasses.gradient} via-transparent to-transparent opacity-40 group-hover:opacity-70 transition-opacity duration-500"></div>
                    <Card.Header class="pb-1 pt-1 relative z-10">
                        <div class="flex items-center">
                            <div class="flex items-center gap-3">
                                <div class="p-2 rounded-lg {storageClasses.iconBg}">
                                    <DatabaseIcon class="h-5 w-5 {storageClasses.iconColor}" />
                                </div>
                                <div>
                                    <Card.Description class="text-sm font-medium text-muted-foreground">
                                        NVMe Usage
                                    </Card.Description>
                                </div>
                            </div>
                        </div>
                        <Card.Title class="text-3xl font-bold tabular-nums pt-2">
                            {formatBytes((dataStore.systemMetrics.usedStorage || 0) * 1024 * 1024 * 1024)}
                        </Card.Title>
                        <div class="text-sm text-muted-foreground italic tabular-nums pt-0">
                            of 512 TB
                        </div>
                    </Card.Header>
                </Card.Root>
            </div>
        </div>

        <!-- Running Observations -->
        <div class="px-4 lg:px-6">
            <div class="flex items-center justify-between mb-4">
                <h2 class="text-2xl font-semibold">Running Observations</h2>
                <Button
                    variant="ghost"
                    size="sm"
                    onclick={navigateToObservations}
                >
                    View All
                </Button>
            </div>
            {#if runningObservations.length > 0}
                <div
                    class="grid gap-6 @xl/main:grid-cols-2 @5xl/main:grid-cols-3"
                >
                    {#each runningObservations.slice(0, 6) as observation (observation.observationId)}
                        <RunningObservationCard {observation} />
                    {/each}
                </div>
            {:else}
                <div class="grid gap-6 @xl/main:grid-cols-2 @5xl/main:grid-cols-3">
                    <EmptyObservationsPlaceholder />
                </div>
            {/if}
        </div>

        <div class="px-4 lg:px-6">
            <!-- Other Observations -->
            <div class="flex items-center justify-between mb-4">
                <h2 class="text-2xl font-semibold">Other Observations</h2>
                <Button
                    variant="ghost"
                    size="sm"
                    onclick={navigateToObservations}
                >
                    View All
                </Button>
            </div>
            <ObservationTable observations={recentObservations} limit={10} />
        </div>
    </div>
