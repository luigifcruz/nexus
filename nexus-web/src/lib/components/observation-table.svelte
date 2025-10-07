<script lang="ts">
    import type { ObservationData } from "$lib/types";
    import { goto } from "$app/navigation";
    import { formatDuration, formatDateTime } from "$lib/utils/formatters";
    import * as Table from "$lib/components/ui/table/index.js";
    import StatusBadge from "$lib/components/status-badge.svelte";

    interface Props {
        observations: ObservationData[];
        limit?: number;
    }

    let { observations, limit = 10 }: Props = $props();

    function navigateToObservation(id: string) {
        goto(`/observations/${id}`);
    }

    const displayedObservations = $derived(
        observations
            .toSorted((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
            .slice(0, limit)
    );
</script>

<div class="rounded-md border">
    <Table.Root>
        <Table.Header>
            <Table.Row>
                <Table.Head>Status</Table.Head>
                <Table.Head>Name</Table.Head>
                <Table.Head>Started</Table.Head>
                <Table.Head>Duration</Table.Head>
                <Table.Head>Replicants</Table.Head>
                <Table.Head>Antennas</Table.Head>
                <Table.Head>Disk Usage</Table.Head>
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#each displayedObservations as observation}
                <Table.Row
                    class="cursor-pointer hover:bg-muted/50"
                    onclick={() => navigateToObservation(observation.observationId)}
                >
                    <Table.Cell>
                        <StatusBadge status={observation.status || "unknown"} showIcon />
                    </Table.Cell>
                    <Table.Cell class="font-medium">
                        {observation.observationId}
                    </Table.Cell>
                    <Table.Cell class="text-sm font-mono">
                        {#if observation.startTime}
                            {formatDateTime(observation.startTime)}
                        {:else}
                            N/A
                        {/if}
                    </Table.Cell>
                    <Table.Cell class="text-sm font-mono">
                        {#if observation.startTime && observation.endTime}
                            {formatDuration(observation.startTime, observation.endTime)}
                        {:else}
                            N/A
                        {/if}
                    </Table.Cell>
                    <Table.Cell class="text-sm">
                        <span class="font-mono">{observation.numberOfInstances}</span>
                    </Table.Cell>
                    <!-- TODO:API - Number of Antennas not provided by API -->
                    <Table.Cell class="text-sm text-muted-foreground">
                        <span class="font-mono">N/A</span>
                    </Table.Cell>
                    <!-- TODO:API - Disk Usage not provided by API -->
                    <Table.Cell class="text-sm text-muted-foreground">
                        <span class="font-mono text-xs">N/A</span>
                    </Table.Cell>
                </Table.Row>
            {/each}
        </Table.Body>
    </Table.Root>
</div>
