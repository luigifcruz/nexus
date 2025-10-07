<script lang="ts">
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    import type {
        ReplicantData,
        ObservationData,
        ConnectionStatus,
    } from "$lib/types";
    import { dataStore } from "$lib/stores/data.svelte";
    import { apiSync } from "$lib/services/sync";
    import { transformReplicantMetrics } from "$lib/utils/metrics";
    import { formatBytes, formatUptime, formatTimestamp, formatDateTime } from "$lib/utils/formatters";
    import { getStatusIcon, getStatusIconWithClass } from "$lib/utils/status";

    import TagIcon from "@tabler/icons-svelte/icons/tag";
    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import CpuIcon from "@tabler/icons-svelte/icons/cpu";
    import DeviceDesktopIcon from "@tabler/icons-svelte/icons/device-desktop";
    import DatabaseIcon from "@tabler/icons-svelte/icons/database";
    import NetworkIcon from "@tabler/icons-svelte/icons/network";
    import ActivityIcon from "@tabler/icons-svelte/icons/activity";

    import ChartBarIcon from "@tabler/icons-svelte/icons/chart-bar";


    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";
    import ArrowLeftIcon from "@tabler/icons-svelte/icons/arrow-left";
    import RefreshIcon from "@tabler/icons-svelte/icons/refresh";
    import ThermometerIcon from "@tabler/icons-svelte/icons/thermometer";

    import ObservationTable from "$lib/components/observation-table.svelte";
    import MetricsMosaic from "$lib/components/metrics-mosaic.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Tabs from "$lib/components/ui/tabs/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import StatusBadge from "$lib/components/status-badge.svelte";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";
    import { Progress } from "$lib/components/ui/progress/index.js";

    // Route params
    let replicantId = $derived($page.params.id);

    // Reactive state using Svelte 5 runes
    let replicant = $state<ReplicantData | null>(null);
    let replicantObservations = $state<ObservationData[]>([]);
    let activeTab = $state("metrics");
    let connectionStatus = $state<ConnectionStatus>({
        connected: true,
        reconnecting: false,
        error: "",
    });

    let loading = $state(true);
    let error = $state("");

    // Real metrics from API
    let metricsHistory = $derived(transformReplicantMetrics(dataStore.getReplicantMetrics(replicantId)));

    // Load replicant data when ID changes
    $effect(() => {
        if (replicantId) {
            loading = true;
            error = "";

            const foundReplicant = dataStore.getReplicantById(replicantId);
            if (foundReplicant) {
                replicant = foundReplicant;
                replicantObservations =
                    dataStore.getObservationsByReplicantId(replicantId);
                loading = false;
            } else {
                error = "Replicant not found";
                replicant = null;
                loading = false;
            }
        }
    });

    function getMetricColor(
        value: number,
        type: "usage" | "temperature",
    ): string {
        if (type === "temperature") {
            if (value > 60) return "text-red-600";
            if (value > 45) return "text-yellow-600";
            return "text-green-600";
        }
        // usage metrics
        if (value > 90) return "text-red-600";
        if (value > 75) return "text-yellow-600";
        return "text-green-600";
    }

    // Navigation
    function navigateBack() {
        goto("/replicants");
    }

    function navigateToObservation(id: string) {
        goto(`/observations/${id}`);
    }

    function getStatusLabel(status: string): string {
        switch (status) {
            case "online":
            case "running":
                return "Online";
            case "offline":
            case "stopped":
                return "Offline";
            case "standby":
                return "Standby";
            case "starting":
                return "Starting";
            case "stopping":
                return "Stopping";
            case "maintenance":
                return "Maintenance";
            case "errored":
            case "error":
                return "Error";
            default:
                return "Unknown";
        }
    }
</script>

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
                            <div class="h-5 w-5 bg-muted rounded animate-pulse"></div>
                            <div class="h-8 w-48 bg-muted rounded animate-pulse"></div>
                            <div class="h-6 w-20 bg-muted rounded animate-pulse"></div>
                        </div>
                        <div class="h-4 w-32 bg-muted rounded animate-pulse ml-1"></div>
                    </div>
                </div>
            </div>
        </div>

        <div class="px-4 lg:px-6 mt-1">
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-7 gap-3 mb-4">
                {#each Array(7) as _}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
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
                        <div class="h-6 w-32 bg-muted rounded animate-pulse"></div>
                    </Card.Header>
                    <Card.Content class="space-y-3">
                        {#each Array(5) as _}
                            <div class="h-20 bg-muted rounded animate-pulse"></div>
                        {/each}
                    </Card.Content>
                </Card.Root>
            </div>
        </div>
    {:else if error || !replicant}
        <div class="px-4 lg:px-6">
            <Card.Root
                class="border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950"
            >
                <Card.Header class="text-center py-8">
                    <AlertTriangleIcon
                        class="h-12 w-12 mx-auto text-red-600 mb-4"
                    />
                    <Card.Title class="text-red-800 dark:text-red-200">
                        {error || "Replicant not found"}
                    </Card.Title>
                    <Card.Description class="text-red-700 dark:text-red-300">
                        The requested replicant could not be found or loaded.
                    </Card.Description>
                </Card.Header>
                <Card.Footer class="flex justify-center">
                    <Button onclick={navigateBack} variant="outline">
                        <ArrowLeftIcon class="h-4 w-4" />
                        Back to Replicants
                    </Button>
                </Card.Footer>
            </Card.Root>
        </div>
    {:else}
        {@const statusInfo = getStatusIconWithClass(replicant.status)}
        <!-- Header Section -->
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
                                            <statusInfo.icon class="h-5 w-5 mt-1 {statusInfo.class}" />
                                        </Tooltip.Trigger>
                                        <Tooltip.Content>
                                            <p>Status: {getStatusLabel(replicant.status)}</p>
                                        </Tooltip.Content>
                                    </Tooltip.Root>
                                </Tooltip.Provider>
                                <h1 class="text-3xl font-bold tracking-tight">
                                    Replicant Dashboard
                                </h1>
                            </div>
                            <Badge
                                variant="secondary"
                                class="text-base font-mono font-bold px-3 py-1"
                            >
                                {replicant.replicantId}
                            </Badge>
                        </div>

                        <!-- Description and Tags -->
                        <div class="flex items-center gap-4 mb-1">
                            <div class="flex items-center gap-2">
                                <TagIcon class="h-4 w-4 ml-1 text-muted-foreground" />
                                <div class="flex flex-wrap gap-1">
                                    {#if replicant.tags && replicant.tags.length > 0}
                                        {#each replicant.tags as tag}
                                            <Badge variant="outline" class="text-xs">
                                                {tag}
                                            </Badge>
                                        {/each}
                                    {:else}
                                        <span class="text-xs text-muted-foreground italic">No tags</span>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Current Metrics -->
        <div class="px-4 lg:px-6 mt-1">
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-7 gap-3 mb-4">
                {#if replicant.currentMetrics?.cpuUsage !== undefined}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30">
                        <CpuIcon class="h-6 w-6 text-blue-500" />
                        <span class="text-sm font-mono font-semibold text-foreground">
                            {replicant.currentMetrics.cpuUsage.toFixed(1)}%
                        </span>
                        <span class="text-xs text-muted-foreground">CPU Usage</span>
                    </div>
                {:else}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/if}
                {#if replicant.currentMetrics?.memoryUsage !== undefined}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30">
                        <DatabaseIcon class="h-6 w-6 text-green-500" />
                        <span class="text-sm font-mono font-semibold text-foreground">
                            {replicant.currentMetrics.memoryUsage.toFixed(1)}%
                        </span>
                        <span class="text-xs text-muted-foreground">Memory Usage</span>
                    </div>
                {:else}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/if}
                {#if replicant.currentMetrics?.storageUsage !== undefined}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30">
                        <DatabaseIcon class="h-6 w-6 text-purple-500" />
                        <span class="text-sm font-mono font-semibold text-foreground">
                            {replicant.currentMetrics.storageUsage.toFixed(1)}%
                        </span>
                        <span class="text-xs text-muted-foreground">Storage Usage</span>
                    </div>
                {:else}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/if}
                {#if replicant.currentMetrics?.networkBandwidth !== undefined}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30">
                        <NetworkIcon class="h-6 w-6 text-orange-500" />
                        <span class="text-sm font-mono font-semibold text-foreground">
                            {formatBytes(replicant.currentMetrics.networkBandwidth)}/s
                        </span>
                        <span class="text-xs text-muted-foreground">Network</span>
                    </div>
                {:else}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/if}
                {#if replicant.currentMetrics?.gpuUsage !== undefined}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30">
                        <ChartBarIcon class="h-6 w-6 text-red-500" />
                        <span class="text-sm font-mono font-semibold text-foreground">
                            {replicant.currentMetrics.gpuUsage.toFixed(1)}%
                        </span>
                        <span class="text-xs text-muted-foreground">GPU Usage</span>
                    </div>
                {:else}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/if}
                {#if replicant.currentMetrics?.gpuMemoryUsage !== undefined}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30">
                        <DatabaseIcon class="h-6 w-6 text-indigo-500" />
                        <span class="text-sm font-mono font-semibold text-foreground">
                            {replicant.currentMetrics.gpuMemoryUsage.toFixed(1)}%
                        </span>
                        <span class="text-xs text-muted-foreground">GPU Memory</span>
                    </div>
                {:else}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/if}
                {#if replicant.currentMetrics?.temperature !== undefined}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30">
                        <ThermometerIcon class="h-6 w-6 text-yellow-500" />
                        <span class={`text-sm font-mono font-semibold ${getMetricColor(replicant.currentMetrics.temperature, 'temperature')}`}>
                            {replicant.currentMetrics.temperature.toFixed(1)}°C
                        </span>
                        <span class="text-xs text-muted-foreground">Temperature</span>
                    </div>
                {:else}
                    <div class="flex flex-col items-center gap-1 p-3 rounded-lg bg-muted/30 animate-pulse">
                        <div class="h-6 w-6 bg-muted rounded"></div>
                        <div class="h-4 w-12 bg-muted rounded"></div>
                        <div class="h-3 w-16 bg-muted rounded"></div>
                    </div>
                {/if}
            </div>
        </div>

        <!-- Tabs -->
        <div class="px-4 lg:px-6">
            <Tabs.Root bind:value={activeTab} class="space-y-6">
                <Tabs.List class="grid w-full grid-cols-3">
                    <Tabs.Trigger value="metrics">
                        <ChartBarIcon class="h-4 w-4" />
                        Metrics
                    </Tabs.Trigger>
                    <Tabs.Trigger value="hardware">
                        <ServerIcon class="h-4 w-4" />
                        Hardware
                    </Tabs.Trigger>
                    <Tabs.Trigger value="observations">
                        <ActivityIcon class="h-4 w-4" />
                        Observations
                    </Tabs.Trigger>
                </Tabs.List>

                <!-- Metrics Tab -->
                <Tabs.Content value="metrics" class="space-y-6">
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <ChartBarIcon class="h-5 w-5" />
                                Real-time Metrics
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <MetricsMosaic {metricsHistory} />
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>

                <!-- Hardware Tab -->
                <Tabs.Content value="hardware" class="space-y-6">
                    <Card.Root>
                        <Card.Header>
                            <Card.Title class="flex items-center gap-2">
                                <ServerIcon class="h-5 w-5" />
                                Hardware Specifications
                            </Card.Title>
                        </Card.Header>
                        <Card.Content>
                            {#if replicant.hardware}
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    <!-- Left Column -->
                                    <div class="space-y-6">
                                        <!-- CPU Information -->
                                        <div>
                                            <h4 class="text-sm font-medium text-muted-foreground mb-3">CPU</h4>
                                            <div class="rounded-md border border-muted">
                                                <Table.Root>
                                                    <Table.Header>
                                                        <Table.Row>
                                                            <Table.Head class="font-semibold bg-muted w-1/3 first:rounded-tl-[5px]">Property</Table.Head>
                                                            <Table.Head class="font-semibold bg-muted w-2/3 last:rounded-tr-[5px]">Value</Table.Head>
                                                        </Table.Row>
                                                    </Table.Header>
                                                    <Table.Body>
                                                        <Table.Row>
                                                            <Table.Cell>Type</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.cpuType}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Socket</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.cpuSocket}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Status</Table.Cell>
                                                            <Table.Cell>
                                                                <StatusBadge status={replicant.status} class="text-xs" />
                                                            </Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Threads</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.cpuThreads}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Memory</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.memorySize} MB</Table.Cell>
                                                        </Table.Row>
                                                    </Table.Body>
                                                </Table.Root>
                                            </div>
                                        </div>

                                        <!-- GPU Information -->
                                        <div>
                                            <h4 class="text-sm font-medium text-muted-foreground mb-3">GPU</h4>
                                            <div class="rounded-md border border-muted">
                                                <Table.Root>
                                                    <Table.Header>
                                                        <Table.Row>
                                                            <Table.Head class="font-semibold bg-muted w-1/3 first:rounded-tl-[5px]">Property</Table.Head>
                                                            <Table.Head class="font-semibold bg-muted w-2/3 last:rounded-tr-[5px]">Value</Table.Head>
                                                        </Table.Row>
                                                    </Table.Header>
                                                    <Table.Body>
                                                        <Table.Row>
                                                            <Table.Cell>Type</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.gpuType}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>PCIe ID</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.gpuPcieId}</Table.Cell>
                                                        </Table.Row>
                                                    </Table.Body>
                                                </Table.Root>
                                            </div>
                                        </div>

                                        <!-- Storage Information -->
                                        <div>
                                            <h4 class="text-sm font-medium text-muted-foreground mb-3">Storage</h4>
                                            <div class="rounded-md border border-muted">
                                                <Table.Root>
                                                    <Table.Header>
                                                        <Table.Row>
                                                            <Table.Head class="font-semibold bg-muted w-1/3 first:rounded-tl-[5px]">Property</Table.Head>
                                                            <Table.Head class="font-semibold bg-muted w-2/3 last:rounded-tr-[5px]">Value</Table.Head>
                                                        </Table.Row>
                                                    </Table.Header>
                                                    <Table.Body>
                                                        <Table.Row>
                                                            <Table.Cell>Size</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.storageSize} GB</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Path</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.storagePath}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Type</Table.Cell>
                                                            <Table.Cell class="font-mono">NVMe SSD</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Interface</Table.Cell>
                                                            <Table.Cell class="font-mono">PCIe 4.0 x4</Table.Cell>
                                                        </Table.Row>
                                                    </Table.Body>
                                                </Table.Root>
                                            </div>
                                        </div>
                                    </div>

                                    <!-- Right Column -->
                                    <div class="space-y-6">
                                        <!-- Memory Information -->
                                        <div>
                                            <h4 class="text-sm font-medium text-muted-foreground mb-3">Memory</h4>
                                            <div class="rounded-md border border-muted">
                                                <Table.Root>
                                                    <Table.Header>
                                                        <Table.Row>
                                                            <Table.Head class="font-semibold bg-muted w-1/3 first:rounded-tl-[5px]">Property</Table.Head>
                                                            <Table.Head class="font-semibold bg-muted w-2/3 last:rounded-tr-[5px]">Value</Table.Head>
                                                        </Table.Row>
                                                    </Table.Header>
                                                    <Table.Body>
                                                        <Table.Row>
                                                            <Table.Cell>Size</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.memorySize} MB</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Type</Table.Cell>
                                                            <Table.Cell class="font-mono">DDR4</Table.Cell>
                                                        </Table.Row>
                                                    </Table.Body>
                                                </Table.Root>
                                            </div>
                                        </div>

                                        <!-- Network Information -->
                                        <div>
                                            <h4 class="text-sm font-medium text-muted-foreground mb-3">Network</h4>
                                            <div class="rounded-md border border-muted">
                                                <Table.Root>
                                                    <Table.Header>
                                                        <Table.Row>
                                                            <Table.Head class="font-semibold bg-muted w-1/3 first:rounded-tl-[5px]">Property</Table.Head>
                                                            <Table.Head class="font-semibold bg-muted w-2/3 last:rounded-tr-[5px]">Value</Table.Head>
                                                        </Table.Row>
                                                    </Table.Header>
                                                    <Table.Body>
                                                        <Table.Row>
                                                            <Table.Cell>Type</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.networkType}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Speed</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.networkSpeed}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>Interface</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.networkInterface}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>PCIe ID</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.networkPcieId}</Table.Cell>
                                                        </Table.Row>
                                                        <Table.Row>
                                                            <Table.Cell>MAC Address</Table.Cell>
                                                            <Table.Cell class="font-mono">{replicant.hardware.networkMac}</Table.Cell>
                                                        </Table.Row>
                                                    </Table.Body>
                                                </Table.Root>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            {/if}
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>

                <!-- Observations Tab -->
                <Tabs.Content value="observations" class="space-y-6">
                    <Card.Root>
                        <Card.Header>
                            <div class="flex items-center justify-between">
                                <Card.Title class="flex items-center gap-2">
                                    <ActivityIcon class="h-5 w-5" />
                                    Recent Observations
                                </Card.Title>
                                <Button
                                    onclick={() => goto("/observations")}
                                    variant="outline"
                                    size="sm"
                                >
                                    View All Observations
                                </Button>
                            </div>
                        </Card.Header>
                        <Card.Content>
                            {#if replicantObservations.length > 0}
                                <ObservationTable observations={replicantObservations} limit={10} />
                            {:else}
                                <div class="text-center py-8">
                                    <ActivityIcon class="h-12 w-12 mx-auto text-muted-foreground mb-4" />
                                    <h3 class="text-lg font-semibold mb-2">No Recent Activity</h3>
                                    <p class="text-sm text-muted-foreground">
                                        No observations found for this replicant.
                                    </p>
                                </div>
                            {/if}
                        </Card.Content>
                    </Card.Root>
                </Tabs.Content>
            </Tabs.Root>
        </div>
    {/if}
</div>
