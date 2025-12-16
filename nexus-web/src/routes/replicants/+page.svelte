<script lang="ts">
    import { goto } from "$app/navigation";
    import { dataStore } from "$lib/stores/data.svelte";
    import type { ReplicantData, ConnectionStatus } from "$lib/types";
    import {
        formatBytes,
        formatUptime,
        formatTimestamp,
    } from "$lib/utils/formatters";
    import { getStatusIconWithClass } from "$lib/utils/status";

    import SearchIcon from "@tabler/icons-svelte/icons/search";
    import FilterIcon from "@tabler/icons-svelte/icons/filter";
    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";
    import ServerIcon from "@tabler/icons-svelte/icons/server";

    import * as Card from "$lib/components/ui/card/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import ReplicantTable from "$lib/components/replicant-table.svelte";

    // Connection state from data store
    let connectionStatus = $derived(dataStore.connectionStatus);

    let searchTerm = $state("");
    let statusFilter = $state("all");

    // Remove location references from replicants data
    let cleanedReplicants = $derived(
        dataStore.replicants.map((r) => ({
            ...r,
            location: undefined,
        })),
    );

    // Filtered data
    let filteredReplicants = $derived(
        cleanedReplicants.filter((replicant) => {
            const matchesSearch = replicant.replicantId
                .toLowerCase()
                .includes(searchTerm.toLowerCase());

            const matchesStatus =
                statusFilter === "all" || replicant.status === statusFilter;

            return matchesSearch && matchesStatus;
        }),
    );

    // Status counts
    let statusCounts = $derived({
        total: dataStore.replicants.length,
        online: dataStore.replicants.filter(
            (r) => r.status === "online" || r.status === "running",
        ).length,
        offline: dataStore.replicants.filter(
            (r) => r.status === "offline" || r.status === "stopped",
        ).length,
        maintenance: dataStore.replicants.filter(
            (r) => r.status === "maintenance",
        ).length,
        error: dataStore.replicants.filter(
            (r) => r.status === "error" || r.status === "errored",
        ).length,
    });

    function navigateToReplicant(id: string) {
        goto(`/replicants/${id}`);
    }

    function handleStatusFilter(status: string) {
        statusFilter = status;
    }
</script>

<svelte:head>
    <title>Nexus - Replicants</title>
</svelte:head>

<div class="flex flex-col gap-6 py-4 md:py-6">
    <!-- Header -->
    <div class="px-4 lg:px-6">
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-3xl font-bold tracking-tight">Replicants</h1>
                <p class="text-muted-foreground">
                    Monitor and manage your replicant network
                </p>
            </div>
        </div>
    </div>

    <!-- Search and Filters -->
    <div class="px-4 lg:px-6">
        <div class="flex flex-col gap-4">
            <div class="relative flex-1 max-w-sm">
                <SearchIcon
                    class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground"
                />
                <Input
                    bind:value={searchTerm}
                    placeholder="Search replicants..."
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
                            All ({statusCounts.total})
                        </Button>
                        <Button
                            variant={statusFilter === "online"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("online")}
                        >
                            Online ({statusCounts.online})
                        </Button>
                        <Button
                            variant={statusFilter === "offline"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("offline")}
                        >
                            Offline ({statusCounts.offline})
                        </Button>
                        <Button
                            variant={statusFilter === "maintenance"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("maintenance")}
                        >
                            Maintenance ({statusCounts.maintenance})
                        </Button>
                        <Button
                            variant={statusFilter === "error"
                                ? "default"
                                : "outline"}
                            size="sm"
                            onclick={() => handleStatusFilter("error")}
                        >
                            Error ({statusCounts.error})
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <!-- Replicants Table -->
    <div class="px-4 lg:px-6">
        {#if filteredReplicants.length === 0}
            <Card.Root>
                <Card.Header class="text-center py-12">
                    <ServerIcon
                        class="h-12 w-12 text-muted-foreground mx-auto mb-4"
                    />
                    <Card.Title>No replicants found</Card.Title>
                    <Card.Description>
                        {searchTerm || statusFilter !== "all"
                            ? "No replicants match your current filters."
                            : "No replicants are currently registered."}
                    </Card.Description>
                </Card.Header>
            </Card.Root>
        {:else}
            <ReplicantTable replicants={filteredReplicants} />
        {/if}
    </div>
</div>
