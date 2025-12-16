<script lang="ts">
    import { goto } from "$app/navigation";
    import { dataStore } from "$lib/stores/data.svelte";
    import {
        formatTimestamp,
        formatDateTime,
        formatDuration,
    } from "$lib/utils/formatters";
    import { getStatusIcon } from "$lib/utils/status";
    import { calculateProgress } from "$lib/utils/observations";
    import type {
        ReplicantData,
        ObservationData,
        ConnectionStatus,
        FilterOptions,
    } from "$lib/types";

    import EyeIcon from "@tabler/icons-svelte/icons/eye";
    import SearchIcon from "@tabler/icons-svelte/icons/search";
    import FilterIcon from "@tabler/icons-svelte/icons/filter";
    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";

    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import PlusIcon from "@tabler/icons-svelte/icons/plus";
    import CalendarIcon from "@tabler/icons-svelte/icons/calendar";
    import ActivityIcon from "@tabler/icons-svelte/icons/activity";
    import SatelliteDishIcon from "@lucide/svelte/icons/satellite-dish";

    import RunningObservationCard from "$lib/components/running-observation-card.svelte";
    import ObservationTable from "$lib/components/observation-table.svelte";
    import EmptyObservationsPlaceholder from "$lib/components/empty-observations-placeholder.svelte";
    import * as Card from "$lib/components/ui/card/index.js";

    import StatusBadge from "$lib/components/status-badge.svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Progress } from "$lib/components/ui/progress/index.js";

    // Connection state from data store
    let connectionStatus = $derived(dataStore.connectionStatus);

    // Filter state
    let searchTerm = $state("");
    let statusFilter = $state("all"); // 'all', 'active', 'scheduled', 'completed', 'failed'
    let dateRange = $state("all"); // 'all', 'today', 'week', 'month'

    // Derived computations
    let filteredObservations = $derived(() => {
        return dataStore.observations.filter((observation) => {
            // Search filter
            const matchesSearch =
                searchTerm === "" ||
                observation.observationId
                    ?.toLowerCase()
                    .includes(searchTerm.toLowerCase()) ||
                observation.replicantIds.some((rid) =>
                    getReplicantName(rid)
                        .toLowerCase()
                        .includes(searchTerm.toLowerCase()),
                );

            // Status filter
            const matchesStatus =
                statusFilter === "all" ||
                observation.status === statusFilter ||
                (statusFilter === "errored" &&
                    (observation.status === "failed" ||
                        observation.status === "error"));

            // Date range filter
            let matchesDate = true;
            if (dateRange !== "all") {
                const observationDate = new Date(observation.updatedAt);
                const now = new Date();
                const diffTime = now.getTime() - observationDate.getTime();
                const diffDays = diffTime / (1000 * 60 * 60 * 24);

                switch (dateRange) {
                    case "today":
                        matchesDate = diffDays < 1;
                        break;
                    case "week":
                        matchesDate = diffDays < 7;
                        break;
                    case "month":
                        matchesDate = diffDays < 30;
                        break;
                }
            }

            return matchesSearch && matchesStatus && matchesDate;
        });
    });

    // Status counts
    let statusCounts = $derived(() => ({
        total: dataStore.observations.length,
        active: dataStore.observations.filter((o) => o.status === "running")
            .length,
        scheduled: dataStore.observations.filter((o) => o.status === "standby")
            .length,
        completed: dataStore.observations.filter((o) => o.status === "stopped")
            .length,
        failed: dataStore.observations.filter(
            (o) =>
                o.status === "errored" ||
                o.status === "failed" ||
                o.status === "error",
        ).length,
    }));

    function getReplicantName(replicantId: string): string {
        const replicant = dataStore.getReplicantById(replicantId);
        return replicant?.replicantId || `Replicant ${replicantId.slice(0, 8)}`;
    }

    // Navigation
    function navigateToObservation(id: string) {
        goto(`/observations/${id}`);
    }

    function navigateToReplicant(id: string) {
        goto(`/replicants/${id}`);
    }

    // Filter handlers
    function handleStatusFilter(status: string) {
        statusFilter = status;
    }

    function handleDateFilter(range: string) {
        dateRange = range;
    }

    function handleReplicantClick(event: Event, replicantId: string) {
        event.stopPropagation();
        navigateToReplicant(replicantId);
    }
</script>

<svelte:head>
    <title>Nexus - Observations</title>
</svelte:head>

<div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
    <!-- Header and Actions -->
    <div class="flex flex-col gap-6 px-4 lg:px-6">
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-3xl font-bold tracking-tight">Observations</h1>
                <p class="text-muted-foreground">
                    Monitor and manage observation sessions across all
                    replicants
                </p>
            </div>
            <Button
                onclick={() => goto("/observations/new")}
                disabled={!connectionStatus.connected}
            >
                <PlusIcon class="h-4 w-4" />
                Schedule Observation
            </Button>
        </div>

        <!-- Search and Filters -->
        <div class="flex flex-col gap-4">
            <div class="relative flex-1 max-w-sm">
                <SearchIcon
                    class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground"
                />
                <Input
                    placeholder="Search observations..."
                    bind:value={searchTerm}
                    class="pl-8"
                />
            </div>

            <div
                class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between"
            >
                <div class="flex items-center gap-2">
                    <FilterIcon class="h-4 w-4 text-muted-foreground" />
                    <span class="text-sm text-muted-foreground">Status:</span>
                    <div class="flex gap-2">
                        <Button
                            variant={statusFilter === "all"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("all")}
                        >
                            All ({statusCounts().total})
                        </Button>
                        <Button
                            variant={statusFilter === "running"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("running")}
                        >
                            Active ({statusCounts().active})
                        </Button>
                        <Button
                            variant={statusFilter === "standby"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("standby")}
                        >
                            Scheduled ({statusCounts().scheduled})
                        </Button>
                        <Button
                            variant={statusFilter === "stopped"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("stopped")}
                        >
                            Completed ({statusCounts().completed})
                        </Button>
                        <Button
                            variant={statusFilter === "errored"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("errored")}
                        >
                            Failed ({statusCounts().failed})
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <!-- Running Observations Cards -->
    {#if filteredObservations().length > 0}
        <div class="px-4 lg:px-6">
            <div class="flex items-center justify-between mb-4">
                <h2 class="text-2xl font-semibold">Running Observations</h2>
            </div>
            {#if filteredObservations().filter((obs) => obs.status === "running").length > 0}
                <div
                    class="grid gap-6 @xl/main:grid-cols-2 @5xl/main:grid-cols-3"
                >
                    {#each filteredObservations().filter((obs) => obs.status === "running") as observation (observation.observationId)}
                        <RunningObservationCard {observation} />
                    {/each}
                </div>
            {:else}
                <div
                    class="grid gap-6 @xl/main:grid-cols-2 @5xl/main:grid-cols-3"
                >
                    <EmptyObservationsPlaceholder />
                </div>
            {/if}
        </div>
    {/if}

    <!-- All Other Observations -->
    <div class="px-4 lg:px-6">
        {#if filteredObservations().length === 0}
            <Card.Root>
                <Card.Header class="text-center py-12">
                    <EyeIcon
                        class="h-12 w-12 mx-auto text-muted-foreground mb-4"
                    />
                    <Card.Title>No observations found</Card.Title>
                    <Card.Description>
                        No observations match your current filters. Try
                        adjusting your search criteria or create a new
                        observation.
                    </Card.Description>
                </Card.Header>
            </Card.Root>
        {:else}
            <div class="mb-4">
                <h2 class="text-2xl font-semibold">Scheduled Observations</h2>
            </div>
            {#if filteredObservations().filter((obs) => obs.status === "standby").length > 0}
                <div
                    class="grid gap-6 @xl/main:grid-cols-2 @5xl/main:grid-cols-3"
                >
                    {#each filteredObservations().filter((obs) => obs.status === "standby") as observation}
                        {@const StatusIcon = getStatusIcon(
                            observation.status || "unknown",
                        )}
                        {@const iconColor =
                            observation.status === "standby"
                                ? "text-yellow-600"
                                : "text-muted-foreground"}
                        {@const progress = calculateProgress(observation)}

                        <Card.Root
                            class="cursor-pointer transition-all hover:shadow-md hover:scale-[1.02] border-2 border-yellow-500/15 bg-gradient-to-br from-yellow-500/[0.03] to-yellow-500/[0.06]"
                            onclick={() =>
                                navigateToObservation(
                                    observation.observationId,
                                )}
                        >
                            <Card.Header>
                                <Card.Title class="flex items-center gap-2">
                                    <div
                                        class="p-1.5 rounded-md bg-yellow-100 dark:bg-yellow-900/30"
                                    >
                                        <StatusIcon
                                            class="h-4 w-4 text-yellow-600 dark:text-yellow-400"
                                        />
                                    </div>
                                    {observation.observationId ||
                                        `Observation ${observation.observationId.slice(0, 8)}`}
                                </Card.Title>
                            </Card.Header>

                            <Card.Content>
                                <div class="space-y-4">
                                    <!-- Progress Bar for Running Observations -->
                                    {#if observation.status === "running" && progress > 0}
                                        <div class="space-y-2">
                                            <div
                                                class="flex justify-between text-sm"
                                            >
                                                <span
                                                    class="text-muted-foreground"
                                                    >Progress</span
                                                >
                                                <span class="font-medium"
                                                    >{Math.round(
                                                        progress,
                                                    )}%</span
                                                >
                                            </div>
                                            <Progress
                                                value={progress}
                                                class="h-2"
                                            />
                                        </div>
                                    {/if}

                                    <!-- System Resources -->
                                    <div class="grid grid-cols-2 gap-4 text-sm">
                                        <div
                                            class="flex items-center gap-2 p-2 rounded bg-muted/50"
                                        >
                                            <ServerIcon
                                                class="h-4 w-4 text-blue-500"
                                            />
                                            <span class="font-mono font-medium">
                                                {Math.floor(Math.random() * 8) +
                                                    1}
                                            </span>
                                            <span class="text-muted-foreground"
                                                >Replicants</span
                                            >
                                        </div>
                                        <div
                                            class="flex items-center gap-2 p-2 rounded bg-muted/50"
                                        >
                                            <SatelliteDishIcon
                                                class="h-4 w-4 text-green-500"
                                            />
                                            <span class="font-mono font-medium">
                                                {Math.floor(
                                                    Math.random() * 42,
                                                ) + 8}
                                            </span>
                                            <span class="text-muted-foreground"
                                                >Antennas</span
                                            >
                                        </div>
                                    </div>

                                    <!-- Schedule Table -->
                                    <div class="space-y-3">
                                        <div
                                            class="border rounded-lg overflow-hidden"
                                        >
                                            <div
                                                class="grid grid-cols-1 text-xs"
                                            >
                                                <div
                                                    class="p-2 border-b bg-muted/50 font-medium text-muted-foreground"
                                                >
                                                    Start Date
                                                </div>
                                                <div
                                                    class="p-2 border-b font-mono"
                                                >
                                                    {#if observation.startTime}
                                                        {formatDateTime(
                                                            observation.startTime,
                                                        )}
                                                    {:else}
                                                        Not scheduled
                                                    {/if}
                                                </div>
                                                <div
                                                    class="p-2 border-b bg-muted/50 font-medium text-muted-foreground"
                                                >
                                                    Duration
                                                </div>
                                                <div class="p-2 font-mono">
                                                    {#if observation.startTime && observation.endTime}
                                                        {formatDuration(
                                                            observation.startTime,
                                                            observation.endTime,
                                                        )}
                                                    {:else}
                                                        Unknown duration
                                                    {/if}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </Card.Content>
                        </Card.Root>
                    {/each}
                </div>
            {:else}
                <p class="text-sm text-muted-foreground">
                    No scheduled observations at this time.
                </p>
            {/if}
        {/if}

        <!-- Other Observations Table -->
        {#if filteredObservations().length > 0}
            <div class="mt-8 mb-4">
                <h2 class="text-2xl font-semibold">Other Observations</h2>
            </div>
            <ObservationTable
                observations={filteredObservations().filter(
                    (obs) =>
                        obs.status !== "running" && obs.status !== "standby",
                )}
                limit={10}
            />
        {/if}
    </div>
</div>
