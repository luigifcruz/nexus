<script lang="ts">
    import { onMount } from "svelte";
    import { browser } from "$app/environment";
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    import type {
        ObservationData,
        ReplicantData,
        ConnectionStatus,
    } from "$lib/types";
    import { dataStore } from "$lib/stores/data.svelte";
    import { apiSync } from "$lib/services/sync";
    import { transformObservationMetrics } from "$lib/utils/metrics";
    import {
        formatTimestamp,
        formatDuration,
        formatDateTime,
    } from "$lib/utils/formatters";
    import { getStatusIcon, getStatusIconWithClass } from "$lib/utils/status";
    import { calculateProgress } from "$lib/utils/observations";

    import EyeIcon from "@tabler/icons-svelte/icons/eye";
    import ChartBarIcon from "@tabler/icons-svelte/icons/chart-bar";
    import ClockIcon from "@tabler/icons-svelte/icons/clock";
    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";
    import ArrowLeftIcon from "@tabler/icons-svelte/icons/arrow-left";
    import ActivityIcon from "@tabler/icons-svelte/icons/activity";
    import TrendingUpIcon from "@tabler/icons-svelte/icons/trending-up";
    import SatelliteDishIcon from "@lucide/svelte/icons/satellite-dish";
    import SettingsIcon from "@tabler/icons-svelte/icons/settings";
    import HistoryIcon from "@tabler/icons-svelte/icons/history";
    import DatabaseIcon from "@tabler/icons-svelte/icons/database";
    import NetworkIcon from "@tabler/icons-svelte/icons/network";
    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import RadarIcon from "@tabler/icons-svelte/icons/radar";
    import TargetIcon from "@tabler/icons-svelte/icons/target";
    import ExclamationIcon from "@tabler/icons-svelte/icons/alert-triangle";
    import BoltIcon from "@tabler/icons-svelte/icons/bolt";
    import CpuIcon from "@tabler/icons-svelte/icons/cpu";

    import GraphIcon from "@tabler/icons-svelte/icons/chart-dots";
    import CalibrationIcon from "@tabler/icons-svelte/icons/adjustments";
    import ProcessIcon from "@tabler/icons-svelte/icons/playlist";
    import TagIcon from "@tabler/icons-svelte/icons/tag";
    import WarningIcon from "@tabler/icons-svelte/icons/alert-circle";
    import ErrorIcon from "@tabler/icons-svelte/icons/x";
    import PacketIcon from "@tabler/icons-svelte/icons/package";
    import RefreshIcon from "@tabler/icons-svelte/icons/refresh";

    import ReplicantTable from "$lib/components/replicant-table.svelte";
    import MetricsMosaic from "$lib/components/metrics-mosaic.svelte";
    import ProcessingLogs from "$lib/components/processing-logs.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Tabs from "$lib/components/ui/tabs/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import QualityBadge from "$lib/components/quality-badge.svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";
    import { Progress } from "$lib/components/ui/progress/index.js";
    import { SvelteFlow, Controls, Background } from "@xyflow/svelte";
    import "@xyflow/svelte/dist/style.css";
    import * as Chart from "$lib/components/ui/chart/index.js";
    import { Area, AreaChart } from "layerchart";
    import { curveNatural } from "d3-shape";

    // Route params
    let observationId = $derived($page.params.id);

    // Reactive state using Svelte 5 runes
    let observation = $state<ObservationData | null>(null);
    let replicant = $state<ReplicantData | null>(null);
    let connectionStatus = $state<ConnectionStatus>({
        connected: true,
        reconnecting: false,
        error: null,
    });
    let activeTab = $state("routine");
    let loading = $state(false);
    let error = $state<string | null>(null);

    // Mock data for the new cells
    const observationMetrics = $state({
        frequency: (8400 + Math.random() * 100).toFixed(1),
        target: "J1939+2134",
        nvmeUsage: Math.floor(Math.random() * 800 + 200),
        warnings: Math.floor(Math.random() * 5),
        errors: Math.floor(Math.random() * 2),
        packetLoss: (Math.random() * 2).toFixed(2),
        replicants: Math.floor(Math.random() * 8) + 1,
        antennas: Math.floor(Math.random() * 42) + 8,
        tags: [],
    });

    // Mock calibration data for baselines
    let calibrationData = $state(
        Array.from(
            {
                length: Math.floor(
                    (observationMetrics.antennas *
                        (observationMetrics.antennas - 1)) /
                        2,
                ),
            },
            (_, i) => {
                const ant1 = Math.floor(i / 3) + 1;
                const ant2 = (i % 3) + ant1 + 1;
                const letter1 = String.fromCharCode(65 + (i % 3));
                const letter2 = String.fromCharCode(65 + ((i + 1) % 3));
                const baselineId = `${ant1}${letter1}-${ant2}${letter2}`;

                // Generate realistic calibration metrics
                const correlation = 0.7 + Math.random() * 0.3;
                const phaseStability = 0.85 + Math.random() * 0.15;
                const amplitudeRatio = 0.9 + Math.random() * 0.2;
                const rmsNoise = Math.random() * 0.05;

                // Generate time series data for each baseline
                const timeSeriesData = Array.from({ length: 100 }, (_, j) => {
                    const time = Date.now() - (100 - j) * 30000;
                    const phase =
                        Math.sin(j * 0.1 + i) * 10 + Math.random() * 5;
                    const amplitude =
                        correlation * 100 +
                        Math.sin(j * 0.05 + i * 0.3) * 15 +
                        Math.random() * 8;
                    return {
                        timestamp: new Date(time),
                        phase: phase,
                        amplitude: Math.max(0, amplitude),
                        correlation:
                            correlation +
                            Math.sin(j * 0.02) * 0.1 +
                            Math.random() * 0.05,
                    };
                });

                return {
                    baselineId,
                    ant1: `${ant1}${letter1}`,
                    ant2: `${ant2}${letter2}`,
                    correlation: correlation.toFixed(3),
                    phaseStability: phaseStability.toFixed(3),
                    amplitudeRatio: amplitudeRatio.toFixed(3),
                    rmsNoise: rmsNoise.toFixed(4),
                    timeSeriesData,
                };
            },
        ),
    );

    // Real metrics from API
    let metricsHistory = $derived(
        transformObservationMetrics(
            dataStore.getObservationMetrics(observationId),
        ),
    );

    // Real logs from API
    let logs = $derived(dataStore.getObservationLogs(observationId) || []);

    // Flow graph nodes for Stelline Graph
    const stellineNodes = [
        {
            id: "network-op",
            type: "default",
            position: { x: 100, y: 150 },
            data: { label: "Advanced Network\nOperator" },
            style: "background-color: #e3f2fd; border: 2px solid #2196f3; border-radius: 8px; padding: 10px; font-weight: bold; color: #1976d2; width: 140px; text-align: center; white-space: pre-line; font-size: 12px; line-height: 1.3;",
        },
        {
            id: "packet-sorter",
            type: "default",
            position: { x: 300, y: 150 },
            data: { label: "Packet Sorter" },
            style: "background-color: #f3e5f5; border: 2px solid #9c27b0; border-radius: 8px; padding: 10px; font-weight: bold; color: #7b1fa2; width: 120px; text-align: center;",
        },
        {
            id: "kurtosis",
            type: "default",
            position: { x: 500, y: 150 },
            data: { label: "Kurtosis Module" },
            style: "background-color: #e8f5e8; border: 2px solid #4caf50; border-radius: 8px; padding: 10px; font-weight: bold; color: #388e3c; width: 120px; text-align: center;",
        },
        {
            id: "beamformer",
            type: "default",
            position: { x: 700, y: 100 },
            data: { label: "Beamformer" },
            style: "background-color: #fff3e0; border: 2px solid #ff9800; border-radius: 8px; padding: 10px; font-weight: bold; color: #f57c00; width: 120px; text-align: center;",
        },
        {
            id: "frbnn",
            type: "default",
            position: { x: 900, y: 100 },
            data: { label: "FRBNN\nInference" },
            style: "background-color: #fce4ec; border: 2px solid #e91e63; border-radius: 8px; padding: 10px; font-weight: bold; color: #c2185b; width: 120px; text-align: center; white-space: pre-line; font-size: 12px; line-height: 1.3;",
        },
        {
            id: "correlator",
            type: "default",
            position: { x: 700, y: 200 },
            data: { label: "Correlator" },
            style: "background-color: #e0f2f1; border: 2px solid #009688; border-radius: 8px; padding: 10px; font-weight: bold; color: #00695c; width: 120px; text-align: center;",
        },
        {
            id: "hdf5",
            type: "default",
            position: { x: 900, y: 200 },
            data: { label: "HDF5 Writer" },
            style: "background-color: #f1f8e9; border: 2px solid #8bc34a; border-radius: 8px; padding: 10px; font-weight: bold; color: #558b2f; width: 120px; text-align: center;",
        },
    ];

    const stellineEdges = [
        {
            id: "e1",
            source: "network-op",
            target: "packet-sorter",
            style: "stroke: #666; stroke-width: 2px;",
        },
        {
            id: "e2",
            source: "packet-sorter",
            target: "kurtosis",
            style: "stroke: #666; stroke-width: 2px;",
        },
        {
            id: "e3",
            source: "kurtosis",
            target: "beamformer",
            style: "stroke: #666; stroke-width: 2px;",
        },
        {
            id: "e4",
            source: "beamformer",
            target: "frbnn",
            style: "stroke: #666; stroke-width: 2px;",
        },
        {
            id: "e5",
            source: "kurtosis",
            target: "correlator",
            style: "stroke: #666; stroke-width: 2px;",
        },
        {
            id: "e6",
            source: "correlator",
            target: "hdf5",
            style: "stroke: #666; stroke-width: 2px;",
        },
    ];

    // Post-processing flow graph
    const postProcessNodes = [
        {
            id: "end-trigger",
            type: "default",
            position: { x: 200, y: 150 },
            data: { label: "On Observation\nEnd" },
            style: "background-color: #e3f2fd; border: 2px solid #2196f3; border-radius: 8px; padding: 10px; font-weight: bold; color: #1976d2; width: 140px; text-align: center; white-space: pre-line; font-size: 12px; line-height: 1.3;",
        },
        {
            id: "copy-assets",
            type: "default",
            position: { x: 450, y: 150 },
            data: { label: "Copy Assets" },
            style: "background-color: #f3e5f5; border: 2px solid #9c27b0; border-radius: 8px; padding: 10px; font-weight: bold; color: #7b1fa2; width: 120px; text-align: center;",
        },
        {
            id: "hdd-storage",
            type: "default",
            position: { x: 700, y: 150 },
            data: { label: "HDD Storage" },
            style: "background-color: #e8f5e8; border: 2px solid #4caf50; border-radius: 8px; padding: 10px; font-weight: bold; color: #388e3c; width: 120px; text-align: center;",
        },
    ];

    const postProcessEdges = [
        {
            id: "p1",
            source: "end-trigger",
            target: "copy-assets",
            style: "stroke: #666; stroke-width: 2px;",
        },
        {
            id: "p2",
            source: "copy-assets",
            target: "hdd-storage",
            style: "stroke: #666; stroke-width: 2px;",
        },
    ];

    let isDark = $state(false);

    // Initialize theme from document class
    if (browser) {
        isDark = document.documentElement.classList.contains("dark");
    }

    onMount(() => {
        // Watch for theme changes by observing the document's class list
        const observer = new MutationObserver((mutations) => {
            mutations.forEach((mutation) => {
                if (
                    mutation.type === "attributes" &&
                    mutation.attributeName === "class"
                ) {
                    isDark =
                        document.documentElement.classList.contains("dark");
                }
            });
        });

        observer.observe(document.documentElement, {
            attributes: true,
            attributeFilter: ["class"],
        });

        return () => {
            observer.disconnect();
        };
    });

    // Helper functions

    function formatTargetDuration(startTime: Date, endTime: Date): string {
        const diffMs = endTime.getTime() - startTime.getTime();
        const diffMinutes = Math.round(diffMs / 60000);

        if (diffMinutes < 60) {
            return `${diffMinutes}m`;
        } else {
            const hours = Math.floor(diffMinutes / 60);
            const minutes = diffMinutes % 60;
            return minutes > 0 ? `${hours}h ${minutes}m` : `${hours}h`;
        }
    }

    function navigateBack() {
        goto("/observations");
    }

    function navigateToReplicant(replicantName?: string) {
        const name = replicantName || replicant?.replicantId;
        if (name) {
            goto(`/replicants/${name}`);
        }
    }

    function handleLaunchCyberEther() {
        goto(`/observations/${observationId}/cyberether`);
    }

    function getStatusLabel(status: string): string {
        switch (status) {
            case "running":
                return "Running";
            case "standby":
                return "Standby";
            case "starting":
                return "Starting";
            case "stopping":
                return "Stopping";
            case "completed":
                return "Completed";
            case "failed":
            case "errored":
            case "error":
                return "Failed";
            case "stopped":
                return "Stopped";
            default:
                return "Unknown";
        }
    }

    // Load observation data
    $effect(() => {
        if (observationId) {
            loading = true;
            error = null;

            try {
                const obs = dataStore.getObservationById(observationId);
                if (obs) {
                    observation = obs;
                    // Load replicants for this observation
                    if (obs.replicants && obs.replicants.length > 0) {
                        replicant = dataStore.getReplicantById(
                            obs.replicants[0],
                        );
                    }
                } else {
                    error = "Observation not found";
                }
            } catch (e) {
                error = "Failed to load observation";
                console.error("Error loading observation:", e);
            } finally {
                loading = false;
            }
        }
    });

    // Mock timeline data for routine
    const routineData = $state({
        startTime: new Date(Date.now() - 3 * 60 * 60 * 1000), // 3 hours ago
        endTime: new Date(Date.now() + 2 * 60 * 60 * 1000), // 2 hours from now
        currentTime: new Date(),
        events: [
            {
                id: "target1",
                name: "J1939+2134",
                startTime: new Date(Date.now() - 3 * 60 * 60 * 1000),
                endTime: new Date(Date.now() - 70 * 60 * 1000),
                ra: "19h39m38.5s",
                dec: "+21°34'59.1s",
                type: "target",
                frequency: "8.4 GHz",
            },
            {
                id: "steering1",
                name: "Steering",
                startTime: new Date(Date.now() - 70 * 60 * 1000),
                endTime: new Date(Date.now() - 67 * 60 * 1000),
                type: "steering",
                description: "Antenna steering between targets",
            },
            {
                id: "starlink1",
                name: "Possible Starlink Interference",
                startTime: new Date(Date.now() - 40 * 60 * 1000),
                endTime: new Date(Date.now() - 35 * 60 * 1000),
                type: "warning",
                description: "Starlink constellation passing near target beam",
            },
            {
                id: "target2",
                name: "B0329+54",
                startTime: new Date(Date.now() - 67 * 60 * 1000),
                endTime: new Date(Date.now() + 25 * 60 * 1000),
                ra: "03h32m59.4s",
                dec: "+54°34'43.6s",
                type: "target",
                frequency: "8.4 GHz",
            },
            {
                id: "steering2",
                name: "Steering",
                startTime: new Date(Date.now() + 25 * 60 * 1000),
                endTime: new Date(Date.now() + 28 * 60 * 1000),
                type: "steering",
                description: "Antenna steering between targets",
            },
            {
                id: "target3",
                name: "J0437-4715",
                startTime: new Date(Date.now() + 28 * 60 * 1000),
                endTime: new Date(Date.now() + 2 * 60 * 60 * 1000),
                ra: "04h37m15.8s",
                dec: "-47°15'09.1s",
                type: "target",
                frequency: "8.4 GHz",
            },
            {
                id: "starlink2",
                name: "Possible Starlink Interference",
                startTime: new Date(Date.now() + 75 * 60 * 1000),
                endTime: new Date(Date.now() + 80 * 60 * 1000),
                type: "warning",
                description: "Starlink constellation passing near target beam",
            },
        ],
    });

    // Timeline calculations
    const totalDuration = $derived(
        routineData.endTime.getTime() - routineData.startTime.getTime(),
    );
    const currentOffset = $derived(
        routineData.currentTime.getTime() - routineData.startTime.getTime(),
    );
    const currentPercent = $derived(
        Math.max(0, Math.min(100, (currentOffset / totalDuration) * 100)),
    );

    // Mock replicants for this observation
    const observationReplicants = dataStore.replicants
        .filter((r) => r.status === "running")
        .slice(0, observationMetrics.replicants);

    // Helper to check if an event is currently active
    function isEventActive(event: any): boolean {
        return (
            routineData.currentTime >= event.startTime &&
            routineData.currentTime <= event.endTime
        );
    }
</script>

<svelte:head>
    <title>Nexus - {observation?.observationId ?? "Observation"}</title>
</svelte:head>

<div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
    {#if loading}
        <div class="px-4 lg:px-6">
            <div class="flex items-start justify-between mb-6">
                <div class="flex items-center gap-1">
                    <Button onclick={navigateBack} variant="ghost" size="sm">
                        <ArrowLeftIcon class="h-4 w-4" />
                    </Button>
                    <div class="space-y-2">
                        <div class="flex items-center gap-3">
                            <div
                                class="h-5 w-5 bg-muted rounded animate-pulse"
                            ></div>
                            <div
                                class="h-8 w-48 bg-muted rounded animate-pulse"
                            ></div>
                            <div
                                class="h-6 w-20 bg-muted rounded animate-pulse"
                            ></div>
                        </div>
                        <div
                            class="h-4 w-32 bg-muted rounded animate-pulse ml-1"
                        ></div>
                    </div>
                </div>
            </div>
        </div>

        <div class="px-4 lg:px-6 mt-1">
            <div
                class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-3 mb-4"
            >
                {#each Array(8) as _}
                    <div
                        class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50 animate-pulse"
                    >
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/each}
            </div>
        </div>

        <div class="px-4 lg:px-6">
            <div class="space-y-6">
                <div class="h-10 w-full bg-muted rounded animate-pulse"></div>
                <Card.Root>
                    <Card.Header>
                        <div
                            class="h-6 w-32 bg-muted rounded animate-pulse"
                        ></div>
                    </Card.Header>
                    <Card.Content class="space-y-3">
                        {#each Array(5) as _}
                            <div
                                class="h-20 bg-muted rounded animate-pulse"
                            ></div>
                        {/each}
                    </Card.Content>
                </Card.Root>
            </div>
        </div>
    {:else if error || !observation}
        <div class="px-4 lg:px-6">
            <Card.Root
                class="border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950"
            >
                <Card.Header class="text-center py-8">
                    <AlertTriangleIcon
                        class="h-12 w-12 mx-auto text-red-600 mb-4"
                    />
                    <Card.Title class="text-red-800 dark:text-red-200">
                        {error || "Observation not found"}
                    </Card.Title>
                    <Card.Description class="text-red-700 dark:text-red-300">
                        The requested observation could not be found or loaded.
                    </Card.Description>
                </Card.Header>
                <Card.Footer class="flex justify-center">
                    <Button onclick={navigateBack} variant="outline">
                        <ArrowLeftIcon class="h-4 w-4" />
                        Back to Observations
                    </Button>
                </Card.Footer>
            </Card.Root>
        </div>
    {:else}
        {@const statusInfo = getStatusIconWithClass(observation.status)}
        <!-- Header -->
        <div class="px-4 lg:px-6">
            <div class="flex items-start justify-between">
                <div class="flex items-center gap-1">
                    <Button onclick={navigateBack} variant="ghost" size="sm">
                        <ArrowLeftIcon class="h-4 w-4" />
                    </Button>
                    <div>
                        <div class="flex items-center gap-3 mb-2">
                            <div class="flex items-center gap-2">
                                <Tooltip.Provider>
                                    <Tooltip.Root>
                                        <Tooltip.Trigger>
                                            <statusInfo.icon
                                                class="h-5 w-5 mt-1 {statusInfo.class}"
                                            />
                                        </Tooltip.Trigger>
                                        <Tooltip.Content>
                                            <p>
                                                Status: {getStatusLabel(
                                                    observation.status,
                                                )}
                                            </p>
                                        </Tooltip.Content>
                                    </Tooltip.Root>
                                </Tooltip.Provider>
                                <h1 class="text-3xl font-bold tracking-tight">
                                    Observation Dashboard
                                </h1>
                            </div>
                            <Badge
                                variant="secondary"
                                class="text-base font-mono font-bold px-3 py-1"
                            >
                                {observation.observationId}
                            </Badge>
                        </div>

                        <!-- Description and Tags -->
                        <div class="flex items-center gap-4 mb-1">
                            <div class="flex items-center gap-2">
                                <TagIcon
                                    class="h-4 w-4 ml-1 text-muted-foreground"
                                />
                                <div class="flex flex-wrap gap-1">
                                    {#if observationMetrics.tags && observationMetrics.tags.length > 0}
                                        {#each observationMetrics.tags as tag}
                                            <Badge
                                                variant="outline"
                                                class="text-xs"
                                            >
                                                {tag}
                                            </Badge>
                                        {/each}
                                    {:else}
                                        <span
                                            class="text-xs text-muted-foreground italic"
                                            >No tags</span
                                        >
                                    {/if}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Metrics Cells -->
        <div class="px-4 lg:px-6 mt-1">
            <div
                class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-3 mb-4"
            >
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <RadarIcon class="h-6 w-6 text-blue-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.frequency} GHz
                    </span>
                    <span class="text-xs text-muted-foreground">Frequency</span>
                </div>
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <TargetIcon class="h-6 w-6 text-green-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.target}
                    </span>
                    <span class="text-xs text-muted-foreground">Target</span>
                </div>
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <DatabaseIcon class="h-6 w-6 text-purple-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.nvmeUsage} GB
                    </span>
                    <span class="text-xs text-muted-foreground">NVMe Usage</span
                    >
                </div>
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <WarningIcon class="h-6 w-6 text-yellow-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.warnings}
                    </span>
                    <span class="text-xs text-muted-foreground">Warnings</span>
                </div>
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <ErrorIcon class="h-6 w-6 text-red-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.errors}
                    </span>
                    <span class="text-xs text-muted-foreground">Errors</span>
                </div>
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <PacketIcon class="h-6 w-6 text-orange-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.packetLoss}%
                    </span>
                    <span class="text-xs text-muted-foreground"
                        >Packet Loss</span
                    >
                </div>
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <ServerIcon class="h-6 w-6 text-blue-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.replicants}
                    </span>
                    <span class="text-xs text-muted-foreground">Replicants</span
                    >
                </div>
                <div
                    class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/50"
                >
                    <SatelliteDishIcon class="h-6 w-6 text-green-500" />
                    <span
                        class="text-sm font-mono font-semibold text-foreground"
                    >
                        {observationMetrics.antennas}
                    </span>
                    <span class="text-xs text-muted-foreground">Antennas</span>
                </div>
            </div>

            <!-- Tags -->
        </div>

        <!-- Tabs -->
        <div class="px-4 lg:px-6">
            <Tabs.Root bind:value={activeTab} class="space-y-6">
                <Tabs.List class="grid w-full grid-cols-2 md:grid-cols-6">
                    <Tabs.Trigger value="routine">
                        <ClockIcon class="h-4 w-4" />
                        Overview
                    </Tabs.Trigger>
                    <Tabs.Trigger value="overview">
                        <EyeIcon class="h-4 w-4" />
                        Metrics
                    </Tabs.Trigger>
                    <Tabs.Trigger value="resources">
                        <ServerIcon class="h-4 w-4" />
                        Resources
                    </Tabs.Trigger>
                    <Tabs.Trigger value="calibration">
                        <CalibrationIcon class="h-4 w-4" />
                        Calibration
                    </Tabs.Trigger>
                    <Tabs.Trigger value="stelline">
                        <GraphIcon class="h-4 w-4" />
                        Stelline
                    </Tabs.Trigger>
                    <Tabs.Trigger value="postprocess">
                        <ProcessIcon class="h-4 w-4" />
                        Post Processing
                    </Tabs.Trigger>
                </Tabs.List>

                <!-- Overview Tab -->
                <Tabs.Content value="routine" class="space-y-6">
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <ClockIcon class="h-5 w-5" />
                                Observation Timeline
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <div class="space-y-6">
                                <!-- Timeline Container -->
                                <div
                                    class="relative h-20 bg-muted/20 rounded-lg overflow-hidden"
                                >
                                    <!-- Timeline Background -->
                                    <div
                                        class="absolute inset-0 bg-gradient-to-r from-blue-100 via-green-100 to-purple-100 dark:from-blue-950 dark:via-green-950 dark:to-purple-950"
                                    ></div>

                                    <!-- Event Blocks -->
                                    {#each routineData.events as event}
                                        {@const eventStart =
                                            event.startTime.getTime() -
                                            routineData.startTime.getTime()}
                                        {@const eventDuration =
                                            event.endTime.getTime() -
                                            event.startTime.getTime()}
                                        {@const leftPercent =
                                            (eventStart / totalDuration) * 100}
                                        {@const widthPercent =
                                            (eventDuration / totalDuration) *
                                            100}
                                        {@const isTarget =
                                            event.type === "target"}
                                        {@const isSteering =
                                            event.type === "steering"}
                                        {@const isWarning =
                                            event.type === "warning"}

                                        {#if isWarning}
                                            <!-- Starlink Warning Indicator -->
                                            <div
                                                class="absolute top-0 h-full bg-yellow-500/30 shadow-lg z-10"
                                                style="left: {leftPercent}%; width: {widthPercent}%;"
                                                title="{event.name} - {event.description}"
                                            ></div>
                                        {:else}
                                            <!-- Target and Steering Blocks -->
                                            <div
                                                class="absolute top-2 h-16 rounded border-2 border-white dark:border-gray-800 transition-colors flex items-center justify-center"
                                                style="left: {leftPercent}%; width: {widthPercent}%; background: {isTarget
                                                    ? 'linear-gradient(135deg, #3b82f6 0%, #1e40af 100%)'
                                                    : 'linear-gradient(135deg, #6b7280 0%, #374151 100%)'};"
                                                title="{event.name}{event.ra
                                                    ? ` - RA: ${event.ra}, Dec: ${event.dec}, Freq: ${event.frequency}`
                                                    : event.description
                                                      ? ` - ${event.description}`
                                                      : ''}"
                                            >
                                                {#if isTarget}
                                                    <div
                                                        class="p-2 text-white text-center"
                                                    >
                                                        <div
                                                            class="text-sm font-semibold"
                                                        >
                                                            {event.name}
                                                        </div>
                                                        <div
                                                            class="text-xs font-mono opacity-70"
                                                        >
                                                            {event.frequency}
                                                        </div>
                                                    </div>
                                                {/if}
                                            </div>
                                        {/if}
                                    {/each}

                                    <!-- Current Time Cursor -->
                                    <div
                                        class="absolute top-0 h-full w-1 bg-red-500 shadow-lg animate-pulse z-10"
                                        style="left: {currentPercent}%;"
                                    ></div>
                                </div>

                                <!-- Timeline Footer -->
                                <div
                                    class="flex justify-between items-center text-sm text-muted-foreground font-mono"
                                >
                                    <span
                                        >Start: {routineData.startTime.toLocaleString()}</span
                                    >
                                    <span
                                        >End: {routineData.endTime.toLocaleString()}</span
                                    >
                                </div>

                                <!-- Event Details Table -->
                                <div class="rounded-md border">
                                    <Table.Root>
                                        <Table.Header>
                                            <Table.Row>
                                                <Table.Head>Event</Table.Head>
                                                <Table.Head>RA</Table.Head>
                                                <Table.Head>Dec</Table.Head>
                                                <Table.Head
                                                    >Frequency</Table.Head
                                                >
                                                <Table.Head
                                                    >Time Range</Table.Head
                                                >
                                            </Table.Row>
                                        </Table.Header>
                                        <Table.Body>
                                            {#each routineData.events as event, i}
                                                {@const isCurrent =
                                                    routineData.currentTime >=
                                                        event.startTime &&
                                                    routineData.currentTime <=
                                                        event.endTime}
                                                {@const duration =
                                                    formatTargetDuration(
                                                        event.startTime,
                                                        event.endTime,
                                                    )}
                                                {@const isTarget =
                                                    event.type === "target"}
                                                {@const isSteering =
                                                    event.type === "steering"}
                                                {@const isWarning =
                                                    event.type === "warning"}
                                                {@const shouldIndent =
                                                    !isTarget &&
                                                    (isSteering || isWarning)}
                                                <Table.Row>
                                                    <Table.Cell
                                                        class="font-medium"
                                                    >
                                                        <div
                                                            class="flex items-center gap-2 {shouldIndent
                                                                ? 'ml-6'
                                                                : ''}"
                                                        >
                                                            {#if isTarget}
                                                                <TargetIcon
                                                                    class="h-4 w-4 text-blue-500"
                                                                />
                                                            {:else if isSteering}
                                                                <SettingsIcon
                                                                    class="h-4 w-4 text-gray-500"
                                                                />
                                                            {:else if isWarning}
                                                                <WarningIcon
                                                                    class="h-4 w-4 text-yellow-500"
                                                                />
                                                            {/if}
                                                            <span
                                                                class="font-mono text-sm text-foreground"
                                                                >{event.name}</span
                                                            >
                                                            {#if isCurrent && isTarget}
                                                                <Badge
                                                                    variant="default"
                                                                    class="text-xs bg-green-600 hover:bg-green-700 ml-2"
                                                                >
                                                                    Active
                                                                </Badge>
                                                            {/if}
                                                        </div>
                                                    </Table.Cell>
                                                    <Table.Cell>
                                                        <span
                                                            class="font-mono text-sm text-foreground"
                                                            >{event.ra ||
                                                                "—"}</span
                                                        >
                                                    </Table.Cell>
                                                    <Table.Cell>
                                                        <span
                                                            class="font-mono text-sm text-foreground"
                                                            >{event.dec ||
                                                                "—"}</span
                                                        >
                                                    </Table.Cell>
                                                    <Table.Cell>
                                                        <span
                                                            class="font-mono text-sm text-foreground"
                                                            >{event.frequency ||
                                                                "—"}</span
                                                        >
                                                    </Table.Cell>
                                                    <Table.Cell>
                                                        <span
                                                            class="text-xs text-muted-foreground"
                                                        >
                                                            {event.startTime.toLocaleTimeString()}
                                                            - {event.endTime.toLocaleTimeString()}
                                                            ({duration})
                                                        </span>
                                                    </Table.Cell>
                                                </Table.Row>
                                            {/each}
                                        </Table.Body>
                                    </Table.Root>
                                </div>
                            </div>
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>

                <!-- Metrics Tab -->
                <Tabs.Content value="overview" class="space-y-6">
                    <!-- Metrics Mosaic -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <TrendingUpIcon class="h-5 w-5" />
                                Metrics
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <MetricsMosaic {metricsHistory} />
                        </Card.Content>
                    </Card.Root>

                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <HistoryIcon class="h-5 w-5" />
                                Processing Logs
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <ProcessingLogs {logs} />
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>

                <!-- Resources Tab (Merged Replicants and Antennas) -->
                <Tabs.Content value="resources" class="space-y-6">
                    <!-- Replicants Section -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <ServerIcon class="h-5 w-5" />
                                Observation Replicants
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <ReplicantTable
                                replicants={observationReplicants}
                            />
                        </Card.Content>
                    </Card.Root>

                    <!-- Antennas Section -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <SatelliteDishIcon class="h-5 w-5" />
                                Antenna Array Status
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <div class="rounded-md border">
                                <Table.Root>
                                    <Table.Header>
                                        <Table.Row>
                                            <Table.Head>Antenna</Table.Head>
                                            <Table.Head class="text-center"
                                                >Azimuth</Table.Head
                                            >
                                            <Table.Head class="text-center"
                                                >Elevation</Table.Head
                                            >
                                            <Table.Head class="text-center"
                                                >Feed Temp (K)</Table.Head
                                            >
                                            <Table.Head class="text-center"
                                                >Power (dBm)</Table.Head
                                            >
                                        </Table.Row>
                                    </Table.Header>
                                    <Table.Body>
                                        {#each Array(observationMetrics.antennas) as _, i}
                                            {@const isOnline = true}
                                            {@const azimuth = Math.floor(
                                                Math.random() * 360,
                                            )}
                                            {@const elevation = Math.floor(
                                                Math.random() * 90,
                                            )}
                                            {@const azError = (
                                                Math.random() * 0.2
                                            ).toFixed(2)}
                                            {@const elError = (
                                                Math.random() * 0.2
                                            ).toFixed(2)}
                                            {@const signalStrength =
                                                -120 + Math.random() * 80}
                                            {@const feedTemp = (
                                                1 +
                                                Math.random() * 4
                                            ).toFixed(1)}
                                            {@const station =
                                                Math.floor(i / 3) + 1}
                                            {@const antennaLetter =
                                                String.fromCharCode(
                                                    65 + (i % 3),
                                                )}
                                            <Table.Row>
                                                <Table.Cell class="font-medium">
                                                    <div
                                                        class="flex items-center gap-2"
                                                    >
                                                        <SatelliteDishIcon
                                                            class="h-5 w-5 {isOnline
                                                                ? 'text-green-600'
                                                                : 'text-red-600'}"
                                                        />
                                                        <span
                                                            class="font-mono text-sm"
                                                            >{station}{antennaLetter}</span
                                                        >
                                                    </div>
                                                </Table.Cell>
                                                <Table.Cell class="text-center">
                                                    <div
                                                        class="flex flex-col items-center"
                                                    >
                                                        <span
                                                            class="font-mono text-sm"
                                                            >{isOnline
                                                                ? azimuth + "°"
                                                                : "—"}</span
                                                        >
                                                        {#if isOnline}
                                                            <span
                                                                class="font-mono text-xs text-muted-foreground"
                                                                >±{azError}°</span
                                                            >
                                                        {/if}
                                                    </div>
                                                </Table.Cell>
                                                <Table.Cell class="text-center">
                                                    <div
                                                        class="flex flex-col items-center"
                                                    >
                                                        <span
                                                            class="font-mono text-sm"
                                                            >{isOnline
                                                                ? elevation +
                                                                  "°"
                                                                : "—"}</span
                                                        >
                                                        {#if isOnline}
                                                            <span
                                                                class="font-mono text-xs text-muted-foreground"
                                                                >±{elError}°</span
                                                            >
                                                        {/if}
                                                    </div>
                                                </Table.Cell>
                                                <Table.Cell class="text-center">
                                                    <span
                                                        class="font-mono text-sm"
                                                    >
                                                        {feedTemp} K
                                                    </span>
                                                </Table.Cell>
                                                <Table.Cell class="text-center">
                                                    <span
                                                        class="font-mono text-sm"
                                                    >
                                                        {signalStrength.toFixed(
                                                            1,
                                                        )} dBm
                                                    </span>
                                                </Table.Cell>
                                            </Table.Row>
                                        {/each}
                                    </Table.Body>
                                </Table.Root>
                            </div>
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>

                <!-- Calibration Tab -->
                <Tabs.Content value="calibration" class="space-y-6">
                    <!-- Calibration Summary -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <CalibrationIcon class="h-5 w-5" />
                                Calibration Summary
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                                <div
                                    class="flex flex-col items-center gap-2 p-4 rounded-lg bg-muted/50"
                                >
                                    <span
                                        class="text-2xl font-bold text-green-600"
                                    >
                                        {(
                                            calibrationData.reduce(
                                                (sum, b) =>
                                                    sum +
                                                    parseFloat(b.correlation),
                                                0,
                                            ) / calibrationData.length
                                        ).toFixed(3)}
                                    </span>
                                    <span
                                        class="text-xs text-muted-foreground text-center"
                                        >Average Correlation</span
                                    >
                                </div>
                                <div
                                    class="flex flex-col items-center gap-2 p-4 rounded-lg bg-muted/50"
                                >
                                    <span
                                        class="text-2xl font-bold text-blue-600"
                                    >
                                        {(
                                            calibrationData.reduce(
                                                (sum, b) =>
                                                    sum +
                                                    parseFloat(
                                                        b.phaseStability,
                                                    ),
                                                0,
                                            ) / calibrationData.length
                                        ).toFixed(3)}
                                    </span>
                                    <span
                                        class="text-xs text-muted-foreground text-center"
                                        >Phase Stability</span
                                    >
                                </div>
                                <div
                                    class="flex flex-col items-center gap-2 p-4 rounded-lg bg-muted/50"
                                >
                                    <span
                                        class="text-2xl font-bold text-purple-600"
                                    >
                                        {calibrationData.filter(
                                            (b) =>
                                                parseFloat(b.correlation) >
                                                0.85,
                                        ).length}
                                    </span>
                                    <span
                                        class="text-xs text-muted-foreground text-center"
                                        >High Quality Baselines</span
                                    >
                                </div>
                                <div
                                    class="flex flex-col items-center gap-2 p-4 rounded-lg bg-muted/50"
                                >
                                    <span
                                        class="text-2xl font-bold text-orange-600"
                                    >
                                        {(
                                            (calibrationData.reduce(
                                                (sum, b) =>
                                                    sum +
                                                    parseFloat(b.rmsNoise),
                                                0,
                                            ) /
                                                calibrationData.length) *
                                            1000
                                        ).toFixed(1)}
                                    </span>
                                    <span
                                        class="text-xs text-muted-foreground text-center"
                                        >RMS Noise (mJy)</span
                                    >
                                </div>
                            </div>
                        </Card.Content>
                    </Card.Root>

                    <!-- Baseline Correlation Analysis -->
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <GraphIcon class="h-5 w-5" />
                                Baseline Correlation Analysis
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <div
                                class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"
                            >
                                {#each calibrationData.slice(0, 12) as baseline}
                                    <Card.Root>
                                        <Card.Header class="pb-2">
                                            <div
                                                class="flex items-center justify-between"
                                            >
                                                <Card.Title
                                                    class="text-sm font-mono text-foreground"
                                                >
                                                    {baseline.baselineId}
                                                </Card.Title>
                                                <QualityBadge
                                                    value={parseFloat(
                                                        baseline.correlation,
                                                    )}
                                                    class="text-xs"
                                                />
                                            </div>
                                            <div
                                                class="text-xs text-muted-foreground"
                                            >
                                                Correlation: {baseline.correlation}
                                                | Phase: {baseline.phaseStability}
                                            </div>
                                        </Card.Header>
                                        <Card.Content>
                                            <div class="space-y-2">
                                                <Chart.Container
                                                    config={{
                                                        amplitude: {
                                                            label: "Amplitude",
                                                            color: "#3b82f6",
                                                        },
                                                    }}
                                                    class="h-20 w-full"
                                                >
                                                    <AreaChart
                                                        data={baseline.timeSeriesData.slice(
                                                            -30,
                                                        )}
                                                        x="timestamp"
                                                        series={[
                                                            {
                                                                key: "amplitude",
                                                                label: "Amplitude",
                                                                color: "#3b82f6",
                                                            },
                                                        ]}
                                                        props={{
                                                            area: {
                                                                "fill-opacity": 0.3,
                                                                line: {
                                                                    class: "stroke-1",
                                                                },
                                                                curve: curveNatural,
                                                            },
                                                            xAxis: {
                                                                ticks: 0,
                                                                visible: false,
                                                            },
                                                            yAxis: {
                                                                ticks: 0,
                                                                visible: false,
                                                            },
                                                        }}
                                                    >
                                                        {#snippet marks({
                                                            series,
                                                            getAreaProps,
                                                        })}
                                                            <defs>
                                                                <linearGradient
                                                                    id="fillAmplitude{baseline.baselineId}"
                                                                    x1="0"
                                                                    y1="0"
                                                                    x2="0"
                                                                    y2="1"
                                                                >
                                                                    <stop
                                                                        offset="5%"
                                                                        stop-color="#3b82f6"
                                                                        stop-opacity={0.3}
                                                                    />
                                                                    <stop
                                                                        offset="95%"
                                                                        stop-color="#3b82f6"
                                                                        stop-opacity={0.05}
                                                                    />
                                                                </linearGradient>
                                                            </defs>
                                                            {#each series as s, i (s.key)}
                                                                <Area
                                                                    {...getAreaProps(
                                                                        s,
                                                                        i,
                                                                    )}
                                                                    fill="url(#fillAmplitude{baseline.baselineId})"
                                                                />
                                                            {/each}
                                                        {/snippet}
                                                    </AreaChart>
                                                </Chart.Container>
                                                <div
                                                    class="grid grid-cols-2 gap-2 text-xs"
                                                >
                                                    <div
                                                        class="flex justify-between"
                                                    >
                                                        <span
                                                            class="text-muted-foreground"
                                                            >RMS:</span
                                                        >
                                                        <span
                                                            class="font-mono text-foreground"
                                                            >{baseline.rmsNoise}</span
                                                        >
                                                    </div>
                                                    <div
                                                        class="flex justify-between"
                                                    >
                                                        <span
                                                            class="text-muted-foreground"
                                                            >Amp Ratio:</span
                                                        >
                                                        <span
                                                            class="font-mono text-foreground"
                                                            >{baseline.amplitudeRatio}</span
                                                        >
                                                    </div>
                                                </div>
                                            </div>
                                        </Card.Content>
                                    </Card.Root>
                                {/each}
                            </div>
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>

                <!-- Stelline Graph Tab -->
                <Tabs.Content value="stelline" class="space-y-6">
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <GraphIcon class="h-5 w-5" />
                                Signal Processing Pipeline
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <div
                                class="w-full border rounded-lg bg-gray-50 dark:bg-gray-900"
                                style="aspect-ratio: 16/10;"
                            >
                                <SvelteFlow
                                    id="stelline-flow"
                                    nodes={stellineNodes}
                                    edges={stellineEdges}
                                    fitView={false}
                                    defaultZoom={0.8}
                                    attributionPosition="bottom-left"
                                    colorMode={isDark ? "dark" : "light"}
                                >
                                    <Controls position="bottom-right" />
                                    <Background gap={15} size={1.5} />
                                </SvelteFlow>
                            </div>
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>

                <!-- Post-Processing Tab -->
                <Tabs.Content value="postprocess" class="space-y-6">
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <ProcessIcon class="h-5 w-5" />
                                Post Processing Pipeline
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <div
                                class="w-full border rounded-lg bg-gray-50 dark:bg-gray-900"
                                style="aspect-ratio: 16/10;"
                            >
                                <SvelteFlow
                                    id="postprocess-flow"
                                    nodes={postProcessNodes}
                                    edges={postProcessEdges}
                                    fitView={false}
                                    defaultZoom={0.8}
                                    attributionPosition="bottom-left"
                                    colorMode={isDark ? "dark" : "light"}
                                >
                                    <Controls position="bottom-right" />
                                    <Background gap={15} size={1.5} />
                                </SvelteFlow>
                            </div>
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>
            </Tabs.Root>
        </div>
    {/if}
</div>

<style>
    :global(.nvme-progress[data-slot="progress"]) {
        background-color: rgb(147 197 253 / 0.2);
    }

    :global(
        .nvme-progress[data-slot="progress"] [data-slot="progress-indicator"]
    ) {
        background-color: rgb(37 99 235);
        transition: all 0.3s ease;
    }

    :global(
        .nvme-progress.nvme-critical[data-slot="progress"]
            [data-slot="progress-indicator"]
    ) {
        background-color: rgb(220 38 38);
    }

    :global(.dark .nvme-progress[data-slot="progress"]) {
        background-color: rgb(59 130 246 / 0.2);
    }

    :global(
        .dark
            .nvme-progress[data-slot="progress"]
            [data-slot="progress-indicator"]
    ) {
        background-color: rgb(59 130 246);
    }

    :global(
        .dark
            .nvme-progress.nvme-critical[data-slot="progress"]
            [data-slot="progress-indicator"]
    ) {
        background-color: rgb(220 38 38);
    }

    :global(.svelte-flow__node) {
        font-family: inherit;
    }

    :global(.svelte-flow__controls) {
        right: 10px;
        left: auto;
        bottom: 10px;
        top: auto;
    }

    :global(.svelte-flow__controls button) {
        background: white;
        border: 1px solid #ddd;
        border-radius: 4px;
    }

    :global(.dark .svelte-flow__controls button) {
        background: #374151;
        border: 1px solid #4b5563;
        color: white;
    }
</style>
